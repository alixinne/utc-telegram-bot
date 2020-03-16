#[macro_use]
extern crate log;

mod converter;

use futures::StreamExt;
use rand::Rng;
use telegram_bot::*;

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

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt()]
struct RunOpts {
    #[structopt(short, long, env = "TELEGRAM_BOT_TOKEN")]
    token: String,

    #[structopt(long, env = "IMAGES_BASE_URL")]
    images_url: String,
}

#[derive(StructOpt)]
#[structopt()]
enum Opt {
    GenerateImages,
    Run(RunOpts),
}

async fn run(opt: RunOpts) -> Result<(), failure::Error> {
    let api = Api::new(opt.token);

    // Load maps
    let maps = converter::MapList::new();

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
                    match parse_message(data) {
                        (Some(map_name), Some(msg)) => match maps.map_string(&map_name, &msg) {
                            Ok(result) => api.send(message.text_reply(result)),
                            Err(error) => api.send(message.text_reply(format!("{}", error))),
                        },
                        _ => api.send(message.text_reply("Usage: map_name message")),
                    }
                    .await?;
                }
            }
            UpdateKind::InlineQuery(query) => {
                info!("inline query: {:?}", query);

                let data = &query.query;
                let matches = match parse_message(data) {
                    (Some(map_name), Some(msg)) => {
                        let fuzzy_matches = maps.get_fuzzy_matches(&map_name, &msg);
                        if fuzzy_matches.is_empty() {
                            maps.get_all_matches(data)
                        } else {
                            fuzzy_matches
                        }
                    }
                    _ => maps.get_all_matches(data),
                };

                let mut results = vec![];
                for r in matches {
                    let id: String = std::iter::repeat(())
                        .map(|()| rng.sample(rand::distributions::Alphanumeric))
                        .take(16)
                        .collect();

                    let photo_url = opt.images_url.clone() + &r.map.short_name + ".jpg";

                    results.push(InlineQueryResult::from(InlineQueryResultVideo {
                        id,
                        thumb_url: photo_url.clone(),
                        mime_type: "text/html".to_owned(),
                        video_url: photo_url,
                        title: r.map.full_name.clone(),
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

                let answer = query.answer(results);
                match api.send(answer).await {
                    Ok(_) => {}
                    Err(error) => error!("api error: {}", error),
                }
            }
            other @ _ => {
                debug!("unsupported update kind: {:?}", other);
            }
        }
    }

    Ok(())
}

#[paw::main]
#[tokio::main]
async fn main(opt: Opt) -> Result<(), failure::Error> {
    // Initialize logger
    env_logger::init();

    match opt {
        Opt::Run(run_opts) => run(run_opts).await?,
        Opt::GenerateImages => {
            converter::MapList::new().render_images();
        }
    }

    Ok(())
}
