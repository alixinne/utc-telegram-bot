use futures::StreamExt;
use rand::Rng;
use structopt::StructOpt;
use telegram_bot::*;

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

pub async fn run(opt: RunOpts) -> Result<(), failure::Error> {
    let api = Api::new(opt.token);

    // Load transforms
    let transforms = converter::TransformList::new();

    // Rng for IDs
    let mut rng = rand::thread_rng();

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        match update.kind {
            UpdateKind::Message(message) => {
                if let MessageKind::Text { ref data, .. } = message.kind {
                    // Print received text message to stdout.
                    trace!("<{}>: {}", &message.from.first_name, data);

                    // Find name
                    match match parse_message(data) {
                        (Some(transform_name), Some(msg)) => {
                            match transforms.transform_string(&transform_name, &msg) {
                                Ok(result) => api.send(message.text_reply(result)),
                                Err(error) => api.send(message.text_reply(format!("{}", error))),
                            }
                        }
                        _ => api.send(message.text_reply("Usage: transform_name message")),
                    }
                    .await
                    {
                        Ok(_) => {}
                        Err(error) => warn!("error handling request: {:?}", error),
                    }
                }
            }
            UpdateKind::InlineQuery(query) => {
                info!("inline query: {:?}", query);

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

                let mut results = vec![];
                for r in matches {
                    let id: String = std::iter::repeat(())
                        .map(|()| rng.sample(rand::distributions::Alphanumeric))
                        .take(16)
                        .collect();

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

                let query_details = if log_enabled!(log::Level::Error) {
                    Some(format!("request: {:?}", query))
                } else {
                    None
                };

                let answer = query.answer(results);
                debug!("answer: {:?}", answer);

                let query_details = if log_enabled!(log::Level::Error) {
                    Some(format!(
                        "{}, response: {:?}",
                        query_details.unwrap(),
                        answer
                    ))
                } else {
                    None
                };

                match api.send(answer).await {
                    Ok(_) => {}
                    Err(error) => {
                        let emsg = format!("{}", error);

                        if !emsg.contains("Bad Request: SEND_MESSAGE_MEDIA_INVALID") {
                            error!("api error: {} ({})", emsg, query_details.unwrap());
                        }
                    }
                }
            }
            other => {
                debug!("unsupported update kind: {:?}", other);
            }
        }
    }

    Ok(())
}
