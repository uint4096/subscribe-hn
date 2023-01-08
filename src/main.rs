mod bot;
use bot::SubscriptionBot;

#[tokio::main]
async fn main() {
    SubscriptionBot::init().await;
}
