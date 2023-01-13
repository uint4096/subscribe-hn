use teloxide::{prelude::*, utils::command::BotCommands};

pub struct SubscriptionBot(pub Bot);

impl SubscriptionBot {
    fn create() -> Bot {
        Bot::from_env()
    }

    pub async fn init() -> () {
        let bot = SubscriptionBot::create();
        Command::repl(bot, answer).await;
        println!("Repl initialized!");
    }

    pub fn new() -> Self {
        Self(SubscriptionBot::create())
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

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(
                msg.chat.id,
                 Command::descriptions().to_string()
            ).await?
        }
        Command::Subscribe(topic) => {
            bot.send_message(
                msg.chat.id,
                 topic,
            ).await?
        }
        Command::Unsubscribe(topic) => {
            bot.send_message(
                msg.chat.id,
                 topic,
            ).await?
        },
        Command::List => {
            bot.send_message(
                msg.chat.id,
                String::from("List"),
            ).await?
        }
    };

    Ok(())
}
