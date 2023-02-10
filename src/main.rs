mod api;
mod bot;
mod store;
use api::HN;
use bot::SubscriptionBot;
use std::{env, sync::Arc};
use store::{ProcessedId, Store, Topics};
use teloxide::{
    requests::Requester,
    types::{ChatId, Recipient},
    Bot,
};
use tokio::{
    join, spawn,
    sync::Mutex,
    time::{sleep, Duration},
};

#[tokio::main]
async fn main() {
    let mut id_store = ProcessedId::new(None);
    let bot_handle = spawn(async move {
        let topics_store: Topics = Topics::new(None);
        let bot = SubscriptionBot::new(topics_store);
        bot.init().await;
    });

    let chat_id = match env::var("CHAT_ID") {
        Ok(val) => val
            .parse::<i64>()
            .unwrap_or_else(|_| panic!("CHAT_ID must be a number!")),
        Err(_) => {
            panic!("No chat id specified! You must specify the CHAT_ID as an environment var.")
        }
    };

    let api_handle = spawn(async move {
        loop {
            println!("Fetching new stories.");
            let topics_store: Arc<Mutex<Topics>> = Arc::new(Mutex::new(Topics::new(None)));
            let bot = Arc::new(SubscriptionBot::create());
            check_for_stories(&mut id_store, topics_store, bot, chat_id).await;

            sleep(Duration::from_secs(60)).await;
        }
    });

    match join!(bot_handle, api_handle) {
        (Ok(_), Ok(_)) => (),
        (Ok(_), Err(e)) => panic!("API handler failed! Error: {e}"),
        (Err(e), Ok(_)) => panic!("Bot handler failed! Error: {e}"),
        (Err(e1), Err(e2)) => panic!("Both Bot and API handler failed! Errors: {e1}, {e2}"),
    }
}

async fn check_for_stories(
    id_store: &mut ProcessedId,
    topics: Arc<Mutex<Topics>>,
    bot: Arc<Bot>,
    chat_id: i64,
) {
    let new_stories = HN::get_story_ids().await;

    if let Some(id) = id_store.fetch() {
        for story_id in &new_stories {
            let story_id = *story_id;
            if story_id == id {
                println!("Reached last story: {id}");
                break;
            }

            let topics = topics.clone();
            let bot = bot.clone();

            spawn(async move {
                let story = HN::get_story(story_id).await;
                let mut topics = topics.lock().await;
                if let Some(topics) = topics.fetch() {
                    if topics.iter().any(|topic| {
                        story
                            .title
                            .to_lowercase()
                            .split(|c| c == ' ' || c == '/' || c == '-')
                            .any(|word| word == topic)
                            || if let Some(text) = &story.text {
                                text.to_lowercase()
                                    .split_ascii_whitespace()
                                    .any(|word| word == topic)
                            } else {
                                false
                            }
                    }) {
                        if let Some(url) = story.url {
                            let message = format!("{}\n{}", story.title, url);
                            match bot
                                .send_message(Recipient::Id(ChatId(chat_id)), message)
                                .await
                            {
                                Ok(_) => (),
                                Err(e) => {
                                    panic!("Failed to send message! Error: {e}")
                                }
                            }
                        }
                    }
                }
            });
        }
    }

    id_store.update(&new_stories[0]);

    ()
}
