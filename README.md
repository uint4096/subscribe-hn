# Subscribe HN

A Telegram bot that sends you Hacker News posts based on the topics you subscribe to.

## Setup

You can create a Telegram bot by following the instructions on [@Botfather](https://telegram.me/botfather).

Once you create the bot, you will receive a token to allow HTTP API access. You will need this token to get the bot up and running.


## Building and Running

If you have Rust installed on your machine, you can build the binary by cloning this repo and running:

```
cargo build --release
```

You can also download the latest release [here](https://github.com/uint4096/subscribe-HN/releases).

You must specify these environment variables to be able to run the built executable:

- **TELOXIDE_TOKEN** - The bot token generated while setting up the bot with [@Botfather](https://telegram.me/botfather).
- **CHAT_ID** - The Telegram chat id. You can find your chat id by using [@get_id_bot](https://t.me/get_id_bot).

Once you have these details, you can run:
```
TELOXIDE_TOKEN=<your-bot-id> CHAT_ID=<your-chat-id> <path-to-the-binary>
```

If you have cloned the repo, you can run this command from the project's root directory:

```
TELOXIDE_TOKEN=<your-bot-id> CHAT_ID=<your-chat-id> ./target/release/subscribe_hn
```

## Supported Commands

The bot currently supports these commands:

| Command                  | Description                                           |
|--------------------------|-------------------------------------------------------|
| `/help`                  | List all commands along with the command description. |
| `/subscribe <topic>`     | Subscribe to a topic.                                 |
| `/unsubscribe <topic>`   | Unsubscribe from a topic                              |
| `/list`                  | List all subscribed topics                            |


## Running as a systemd Service

Check the sample config file in `./config/subscribe-hn-example.service` to run the bot as a systemd service.

## License

This tool is distributed under the MIT License. See the LICENSE file for more information.
