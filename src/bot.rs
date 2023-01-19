use teloxide::{prelude::*, utils::command::BotCommands, types::Me};
use crate::store::{Topics, Store};

pub struct SubscriptionBot {
    pub bot: Bot,
    topics_store: Topics,
}

impl SubscriptionBot {
    pub fn create() -> Bot {
        Bot::from_env()
    }

    pub async fn init(self) -> () {
        let handler = Update::filter_message()
            .branch(dptree::entry().filter_command::<Command>().endpoint(command_handler));

        Dispatcher::builder(self.bot, handler)
            .dependencies(dptree::deps![self.topics_store])
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }

    pub fn new(topics_store: Topics) -> Self {
        let bot = SubscriptionBot::create();
        Self {
            bot,
            topics_store,
        }
    }
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Supported commands:")]
enum Command {
    #[command(description = "Display help.")]
    Help,
    #[command(description = "Subscribe to a topic")]
    Subscribe(String),
    #[command(description = "Unsubscribe from a topic")]
    Unsubscribe(String),
    #[command(description = "List all subscribed topics")]
    List,
}
 
async fn command_handler(mut store: Topics, bot: Bot, _: Me, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Subscribe(topic) => {
            store.update(&topic.to_lowercase());
            bot.send_message(msg.chat.id, format!("Subscribed to {topic}")).await?
        },
        Command::Unsubscribe(topic) => {
            store.delete(&topic.to_lowercase());
            bot.send_message(msg.chat.id, format!("Unsubscribed from {topic}")).await?
        },
        Command::List => {
            match store.fetch() {
                Some(list) => bot.send_message(msg.chat.id, list.join("\n")).await?,
                None => bot.send_message(msg.chat.id, format!("You haven't subscribed to anything")).await?,
            }
        }
    };

    Ok(())
}
