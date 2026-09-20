use std::path::PathBuf;
use std::sync::Arc;

use futures::channel::oneshot;
use rand::{Rng, SeedableRng};
use structopt::StructOpt;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::requests::{Request, Requester};
use teloxide::respond;
use teloxide::types::{
    InputMessageContent, InputMessageContentText, MediaKind, MediaText, Message, ParseMode,
};
use teloxide::{
    Bot,
    dispatching::{Dispatcher, UpdateFilterExt},
    dptree,
    types::{InlineQuery, InlineQueryResult, InlineQueryResultVideo, Update},
};
use thiserror::Error;
use tokio::sync::Mutex;

use crate::{converter, manifest::Manifest};

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
    bot: teloxide::Bot,
    /// Random number generator for response IDs
    rng: Mutex<rand::rngs::StdRng>,
    /// Daemon options
    opt: RunOpts,
    /// Transform list
    transforms: Arc<converter::TransformList>,
    /// Database interface
    db: Mutex<db::Db>,
    /// Image manifest
    manifest: Manifest,
}

impl Context {
    pub async fn new(opt: RunOpts) -> Result<Self, RunError> {
        let db = db::Db::new(&opt.database_url).await?;

        // Try loading an image manifest
        let manifest =
            Manifest::load(opt.serve_root.join("images/.manifest.json")).unwrap_or_default();

        // Create bot instance
        let client = teloxide::net::default_reqwest_settings().build().unwrap();
        let bot = teloxide::Bot::with_client(&opt.token, client);

        Ok(Self {
            bot,
            rng: Mutex::new(rand::rngs::StdRng::from_entropy()),
            opt,
            transforms: Arc::new(converter::TransformList::new()),
            db: tokio::sync::Mutex::new(db),
            manifest,
        })
    }
}

#[derive(Error, Debug)]
pub enum RunError {
    #[error("database error: {0}")]
    Db(#[from] db::Error),
    #[error("web server error: {0}")]
    Web(#[from] web::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

// Handles commands sent to the bot in a chat: @unicode_text_converter_bot <type> <message>
async fn handle_inline_query(
    ctx: Arc<Context>,
    query: InlineQuery,
) -> Result<(), teloxide::RequestError> {
    let transforms = ctx.transforms.clone();

    trace!("<{}>: inline query: `{:?}`", query.from.id, query);

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

            // Compute photo url with added hash
            let filename = r.transform.short_name.clone() + ".jpg";
            let mut photo_url = ctx.opt.images_url.clone() + &filename;

            if let Some(hash) = ctx.manifest.hash(&filename) {
                photo_url.push('?');
                photo_url.extend(hash.chars().take(12));
            }

            results.push(InlineQueryResult::from(InlineQueryResultVideo {
                id,
                video_url: photo_url.parse().unwrap(),
                mime_type: "text/html".parse().unwrap(),
                thumbnail_url: photo_url.parse().unwrap(),
                title: r.transform.full_name.clone(),
                caption: None,
                parse_mode: None,
                caption_entities: None,
                show_caption_above_media: false,
                video_width: None,
                video_height: None,
                video_duration: None,
                description: Some(r.result.clone()),
                reply_markup: None,
                input_message_content: Some(InputMessageContent::Text(InputMessageContentText {
                    message_text: r.result,
                    parse_mode: Some(ParseMode::MarkdownV2),
                    entities: None,
                    link_preview_options: None,
                })),
            }));
        }
    }

    // Store query details before it's sent off, in case something goes wrong
    let error_request = format!("{:?}", query);

    // Generate response object
    let answer = ctx.bot.answer_inline_query(query.id.clone(), results);

    match answer.send().await {
        Ok(_) => {}
        Err(error) => {
            error!("api error({}): query: {}", error, error_request);
        }
    }

    respond(())
}

// Handles commands sent to the bot in a chat: @unicode_text_converter_bot <type> <message>
async fn handle_message(ctx: Arc<Context>, message: Message) -> Result<(), teloxide::RequestError> {
    trace!(
        "<{:?}>: message: `{:?}`",
        message.from.as_ref().map(|user| user.id),
        message
    );

    if let teloxide::types::MessageKind::Common(message_common) = &message.kind
        && let MediaKind::Text(MediaText { text, .. }) = &message_common.media_kind
        && text == "/start"
        && let Some(chat_id) = message.chat_id()
    {
        ctx.bot.send_message(chat_id, "Hello! I'm @unicode_text_converter_bot. To send messages using my various unicode alphabets, add me to the target chat and use the @unicode_text_converter_bot query to choose the script you want.").send().await?;
    }

    Ok(())
}

async fn process_updates(ctx: Arc<Context>) -> Result<(), RunError> {
    // Fetch new updates via long poll method
    let handler = dptree::entry()
        .branch(Update::filter_inline_query().chain(dptree::endpoint(
            |_bot: Bot, ctx: Arc<Context>, query: InlineQuery| handle_inline_query(ctx, query),
        )))
        .branch(Update::filter_message().branch(dptree::endpoint(
            |_bot: Bot, ctx: Arc<Context>, message: Message| handle_message(ctx, message),
        )));

    Dispatcher::builder(ctx.bot.clone(), handler)
        .dependencies(dptree::deps![ctx])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

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
