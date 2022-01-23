use std::path::PathBuf;
use std::sync::Arc;

use futures::{channel::oneshot, StreamExt};
use rand::{Rng, SeedableRng};
use structopt::StructOpt;
use telegram_bot::*;
use thiserror::Error;
use tokio::{signal::unix::SignalKind, sync::Mutex};

use crate::converter;

mod db;
mod web;

#[derive(StructOpt)]
#[structopt()]
pub struct RunOpts {
    #[structopt(short, long, env = "TELEGRAM_BOT_TOKEN")]
    token: String,

    #[structopt(long, env = "IMAGES_BASE_URL", default_value = "localhost:3000/images")]
    images_url: String,

    #[structopt(
        long,
        env = "DATABASE_URL",
        default_value = "sqlite:utc-telegram-bot.db"
    )]
    database_url: String,

    #[structopt(short, long, default_value = "localhost:3000")]
    bind: String,

    #[structopt(short, long, default_value = "public")]
    serve_root: PathBuf,
}

fn parse_message(msg: &str) -> (Option<String>, Option<String>) {
    enum State {
        ParsingName,
        EatingWhitespace,
    }

    let mut name = String::with_capacity(80);
    let mut state = State::ParsingName;

    for (i, c) in msg.char_indices() {
        match state {
            State::ParsingName => {
                if c.is_ascii_whitespace() {
                    state = State::EatingWhitespace;
                } else {
                    name.push(c);
                }
            }
            State::EatingWhitespace => {
                if !c.is_ascii_whitespace() {
                    return (Some(name), Some(msg[i..].to_owned()));
                }
            }
        }
    }

    (None, None)
}

struct Context {
    /// Telegram API instance
    api: Api,
    /// Random number generator for response IDs
    rng: Mutex<rand::rngs::StdRng>,
    /// Daemon options
    opt: RunOpts,
    /// Transform list
    transforms: converter::TransformList,
    /// Database interface
    db: Mutex<db::Db>,
}

impl Context {
    pub async fn new(opt: RunOpts) -> Result<Self, RunError> {
        let db = db::Db::new(&opt.database_url).await?;

        Ok(Self {
            api: Api::new(&opt.token),
            rng: Mutex::new(rand::rngs::StdRng::from_entropy()),
            opt,
            transforms: converter::TransformList::new(),
            db: tokio::sync::Mutex::new(db),
        })
    }

    pub fn stream(&self) -> telegram_bot::UpdatesStream {
        self.api.stream()
    }
}

#[derive(Error, Debug)]
pub enum RequestError {
    #[error(transparent)]
    Telegram(#[from] telegram_bot::Error),
}

async fn handle_request(
    ctx: Arc<Context>,
    update: telegram_bot::Update,
) -> Result<(), RequestError> {
    let api = &ctx.api;
    let opt = &ctx.opt;
    let transforms = &ctx.transforms;

    // If the received update contains a new message...
    match update.kind {
        UpdateKind::Message(message) => {
            if let MessageKind::Text { ref data, .. } = message.kind {
                trace!("<{:?}>: `{}`", &message.from.username, data);

                // Find name
                match parse_message(data) {
                    (Some(transform_name), Some(msg)) => {
                        match transforms.transform_string(&transform_name, &msg) {
                            Ok(result) => api.send(message.text_reply(result)),
                            Err(error) => api.send(message.text_reply(format!("{}", error))),
                        }
                    }
                    _ => api.send(message.text_reply("Usage: transform_name message")),
                }
                .await?;
            }
        }
        UpdateKind::InlineQuery(query) => {
            trace!("<{:?}>: inline query: `{:?}`", query.from, query);

            {
                let mut db = ctx.db.lock().await;
                if let Err(error) = db.record_query(&query).await {
                    error!(
                        "<{:?}>: failed saving details to database: {:?}",
                        query.from, error
                    );
                }
            }

            let mut results = vec![];

            let data = &query.query;
            let (matches, request_empty) = match parse_message(data) {
                (Some(transform_name), Some(msg)) => (
                    {
                        let fuzzy_matches = transforms.get_fuzzy_matches(&transform_name, &msg);
                        if fuzzy_matches.is_empty() {
                            transforms.get_all_matches(data)
                        } else {
                            fuzzy_matches
                        }
                    },
                    false,
                ),
                _ => (
                    if data.is_empty() {
                        vec![]
                    } else {
                        transforms.get_all_matches(data)
                    },
                    data.is_empty(),
                ),
            };

            if request_empty {
                // The request is empty, do not add results, they would be invalid
            } else {
                // Compute result set
                for r in matches {
                    let id = {
                        let mut rng = ctx.rng.lock().await;

                        // safety: we only generate alphanumeric chars, they are valid UTF-8
                        unsafe {
                            String::from_utf8_unchecked(
                                std::iter::repeat(())
                                    .map(|()| rng.sample(rand::distributions::Alphanumeric))
                                    .take(16)
                                    .collect(),
                            )
                        }
                    };

                    let photo_url = opt.images_url.clone() + &r.transform.short_name + ".jpg";

                    results.push(InlineQueryResult::from(InlineQueryResultVideo {
                        id,
                        thumb_url: photo_url.clone(),
                        mime_type: "text/html".to_owned(),
                        video_url: photo_url,
                        title: r.transform.full_name.clone(),
                        video_duration: None,
                        video_height: None,
                        video_width: None,
                        description: Some(r.result.clone()),
                        caption: None,
                        parse_mode: None,
                        reply_markup: None,
                        input_message_content: Some(InputMessageContent::from(
                            InputTextMessageContent {
                                message_text: r.result,
                                parse_mode: Some(ParseMode::Markdown),
                                disable_web_page_preview: false,
                            },
                        )),
                    }));
                }
            }

            // Store query details before it's sent off, in case something goes wrong
            let error_request = format!("{:?}", &query);

            // Generate response object
            let answer = query.answer(results);

            // Store response too
            let error_response = format!("{:?}", &answer);

            match api.send(answer).await {
                Ok(_) => {}
                Err(error) => {
                    error!(
                        "api error({}): query: {}, response: {}",
                        error, error_request, error_response
                    );
                }
            }
        }
        other => {
            debug!("unsupported update kind: {:?}", other);
        }
    }

    Ok(())
}

#[derive(Error, Debug)]
pub enum RunError {
    #[error("database error: {0}")]
    Db(#[from] db::Error),
    #[error("telegram api error: {0}")]
    Telegram(#[from] telegram_bot::Error),
    #[error("web server error: {0}")]
    Web(#[from] web::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

async fn process_updates(ctx: Arc<Context>) -> Result<(), RunError> {
    // Declare SIGTERM handle
    let mut handler = tokio::signal::unix::signal(SignalKind::terminate())?;

    // Fetch new updates via long poll method
    let mut stream = ctx.stream();

    while let Some(update) = tokio::select! {
        update = stream.next() => update,
        _ = handler.recv() => {
            info!("got SIGTERM, terminating");
            return Ok(());
        }
    } {
        match update {
            Ok(update) => {
                tokio::spawn({
                    let ctx = ctx.clone();
                    async move {
                        match handle_request(ctx, update).await {
                            Err(error) => {
                                error!("error processing request: {:?}", error);
                            }
                            _ => {
                                // ok, nothing to do
                            }
                        }
                    }
                });
            }
            Err(error) => {
                // TODO: Make a PR to telegram-bot to allow returning internal errors
                let error_string = error.to_string();
                if error_string == "Unauthorized" {
                    // Invalid token, this is a permanent error
                    return Err(error.into());
                } else {
                    error!("error decoding update: {:?}", error);
                }
            }
        }
    }

    Ok(())
}

pub async fn run(opt: RunOpts) -> Result<(), RunError> {
    // Spawn web server
    let (server, cancel) = web::run(&opt).await?;
    let _server = tokio::spawn(async move {
        match server.await {
            Ok(_) => {
                debug!("web server terminated");
            }
            Err(err) => {
                error!("error running web server: {:?}", err);
            }
        }
    });

    // Guard web server for termination
    struct Guard(Option<oneshot::Sender<()>>);

    impl Drop for Guard {
        fn drop(&mut self) {
            self.0.take().and_then(|tx| tx.send(()).ok());
        }
    }

    // Instantiate guard
    let _cancel = Guard(Some(cancel));

    // Context for request handling
    let ctx = Arc::new(Context::new(opt).await?);

    // Process incoming updates
    process_updates(ctx).await?;

    Ok(())
}
