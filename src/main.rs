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
use tokio::{spawn, sync::Mutex};

#[tokio::main]
async fn main() {
    let id_store = ProcessedId(None);
    let handle = spawn(async move {
        let topics_store: Topics = Topics(None);
        let bot = SubscriptionBot::new(topics_store);
        bot.init().await;
    });

    // println!("{:?}", args().collect::<String>());
    // SubscriptionBot::init().await;
    match handle.await {
        Ok(_) => (),
        Err(e) => {
            panic!("Error in the spawned thread! Error: {e}");
        }
    }

    let topics_store: Arc<Mutex<Topics>> = Arc::new(Mutex::new(Topics(None)));
    let bot = Arc::new(SubscriptionBot::create());
    check_for_stories(id_store, topics_store, bot).await;
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
                        let message = format!("{}\n{}", story.title, story.url);
                        match bot.send_message(Recipient::Id(ChatId(1223123)), message).await {
                            Ok(_) => (),
                            Err(e) => { panic!("Failed to send message! Error: {e}") }
                        }
                    }
                }
            });
        }
    }

    id_store.update(&new_stories[0]);

    ()
}
