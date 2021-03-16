use std::sync::{Arc, Mutex};

use futures::StreamExt;
use rand::{Rng, SeedableRng};
use structopt::StructOpt;
use telegram_bot::*;
use thiserror::Error;

use crate::converter;

#[derive(StructOpt)]
#[structopt()]
pub struct RunOpts {
    #[structopt(short, long, env = "TELEGRAM_BOT_TOKEN")]
    token: String,

    #[structopt(long, env = "IMAGES_BASE_URL")]
    image_url: String,
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
}

impl Context {
    pub fn new(opt: RunOpts) -> Self {
        Self {
            api: Api::new(&opt.token),
            rng: Mutex::new(rand::rngs::StdRng::from_entropy()),
            opt,
            transforms: converter::TransformList::new(),
        }
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
            trace!("<{:?}>: inline query: `{:?}`", &query.from.username, query);

            let mut results = vec![];

            let data = &query.query;
            let matches = match parse_message(data) {
                (Some(transform_name), Some(msg)) => {
                    let fuzzy_matches = transforms.get_fuzzy_matches(&transform_name, &msg);
                    if fuzzy_matches.is_empty() {
                        transforms.get_all_matches(data)
                    } else {
                        fuzzy_matches
                    }
                }
                _ => transforms.get_all_matches(data),
            };

            // Compute result set
            for r in matches {
                let id = {
                    let mut rng = ctx.rng.lock().unwrap();

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

                let photo_url = opt.image_url.clone() + &r.transform.short_name + ".jpg";

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
    #[error(transparent)]
    Telegram(#[from] telegram_bot::Error),
}

pub async fn run(opt: RunOpts) -> Result<(), RunError> {
    // Context for request handling
    let ctx = Arc::new(Context::new(opt));

    // Fetch new updates via long poll method
    let mut stream = ctx.stream();
    while let Some(update) = stream.next().await {
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
                error!("error decoding update: {:?}", error);
            }
        }
    }

    Ok(())
}
