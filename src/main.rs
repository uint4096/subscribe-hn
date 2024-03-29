mod api;
mod bot;
mod store;
use api::HN;
use bot::SubscriptionBot;
use std::{env, sync::Arc};
use store::{ProcessedId, SentStories, Store, Topics};
use teloxide::{
    requests::{Requester, RequesterExt},
    types::{ChatId, Recipient, ParseMode},
    Bot,
};
use tokio::{
    join, spawn,
    sync::Mutex,
    task::JoinSet,
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
            let sent_stories = Arc::new(Mutex::new(SentStories::new(None)));
            check_for_stories(&mut id_store, sent_stories, topics_store, bot, chat_id).await;

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
    sent_stories: Arc<Mutex<SentStories>>,
    topics: Arc<Mutex<Topics>>,
    bot: Arc<Bot>,
    chat_id: i64,
) {
    let new_stories = HN::get_story_ids().await;
    let mut set = JoinSet::new();
    if let Some(id) = id_store.fetch() {
        for story_id in &new_stories {
            let story_id = *story_id;
            if story_id == id {
                println!("Reached last story: {id}");
                break;
            }

            let topics = topics.clone();
            let sent_stories = sent_stories.clone();
            let bot = bot.clone();

            set.spawn(async move {
                if let Some(story) = HN::get_story(story_id).await {
                    let mut topics = topics.lock().await;
                    let mut sent_store = sent_stories.lock().await;
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
                            let sent_stories = match sent_store.fetch() {
                                Some(stories) => stories,
                                None => vec![],
                            };

                            if let Some(url) = story.url {
                                if !sent_stories.contains(&story.title.to_lowercase()) {
                                    let link = format!("https://news.ycombinator.com/item?id={}", story.id);
                                    let message = format!("{}\n[Article]({})  [Post]({})", story.title, url, link);
                                    match bot
                                        .parse_mode(ParseMode::MarkdownV2)
                                        .send_message(Recipient::Id(ChatId(chat_id)), message)
                                        .await
                                    {
                                        Ok(_) => sent_store.update(&story.title.to_lowercase()),
                                        Err(e) => {
                                            panic!("Failed to send message! Error: {e}")
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
    }

    let mut counter = 0;
    while let Some(_) = set.join_next().await {
        counter += 1;
        println!("Processed {counter} stories.");
    }

    if new_stories.len() > 0  { id_store.update(&new_stories[0]) };
    ()
}
