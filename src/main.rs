mod bot;
mod store;
mod api;
use std::sync::Arc;
use api::HN;
use bot::SubscriptionBot;
use store::{ProcessedId, Store, Topics};
use teloxide::{
    types::{ChatId, Recipient}, requests::Requester, Bot
};
use tokio::{spawn, sync::Mutex, join};

#[tokio::main]
async fn main() {
    let id_store = ProcessedId::new(None);
    let bot_handle = spawn(async move {
        let topics_store: Topics = Topics::new(None);
        let bot = SubscriptionBot::new(topics_store);
        bot.init().await;
    });

    let api_handle = spawn(async move {
        let topics_store: Arc<Mutex<Topics>> = Arc::new(Mutex::new(Topics::new(None)));
        let bot = Arc::new(SubscriptionBot::create());
        check_for_stories(id_store, topics_store, bot).await;
    });

    match join!(bot_handle, api_handle) {
        (Ok(_), Ok(_)) => (),
        (Ok(_), Err(e)) => panic!("API handler failed! Error: {e}"),
        (Err(e), Ok(_)) => panic!("Bot handler failed! Error: {e}"),
        (Err(e1), Err(e2)) => panic!("Both Bot and API handler failed! Errors: {e1}, {e2}")
    }
}

async fn check_for_stories(
    mut id_store: ProcessedId,
    topics: Arc<Mutex<Topics>>,
    bot: Arc<Bot>) {
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
                        story.title.to_lowercase().contains(topic)
                    }) {
                        if let Some(url) = story.url {
                            let message = format!("{}\n{}", story.title, url);
                            match bot.send_message(Recipient::Id(ChatId(619356013)), message).await {
                                Ok(_) => (),
                                Err(e) => { panic!("Failed to send message! Error: {e}") }
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
