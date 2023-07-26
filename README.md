# Subscribe HN

![GitHub Actions](https://github.com/uint4096/subscribe-HN/actions/workflows/build.yml/badge.svg?label=actions)
[![GitHub Release](https://img.shields.io/github/v/release/uint4096/subscribe-hn?include_prereleases&label=release)]()

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
TELOXIDE_TOKEN=<telegram_bot_token> CHAT_ID=<telegram_chat_id> <path-to-the-binary>
```

If you have cloned the repo, you can run this command from the project's root directory:

```
TELOXIDE_TOKEN=<telegram_bot_token> CHAT_ID=<telegram_chat_id> ./target/release/subscribe_hn
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

## Running with Docker

SubscribeHN comes with a pre-built Docker image. To get the bot running with Docker, run:

```shell
docker pull uint4096/subscribehn:latest
docker run -e TELOXIDE_TOKEN=<telegram_bot_token> -e CHAT_ID=<telegram_chat_id> uint4096/subscribehn:latest
```
Replace the latest tag with the correct semver to run older releases.

To ensure that your subscriptions are persistent, you can specify the volume while running `docker run`:

```shell
docker run -e TELOXIDE_TOKEN=<telegram_bot_token> -e CHAT_ID=<telegram_chat_id> -v /home_dir/subscribe_hn:/root/.config/subscribe_hn:rw uint4096/subscibehn:latest
```
You can also use docker compose to run the container. See `./config/docker-compose-example.yml` for an example docker compose config.


## License

This tool is distributed under the MIT License. See the LICENSE file for more information.
