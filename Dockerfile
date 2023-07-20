FROM rust:latest
ARG TELOXIDE_TOKEN
ARG CHAT_ID
env TELOXIDE_TOKEN $TELOXIDE_TOKEN
env CHAT_ID $CHAT_ID
WORKDIR /usr/app
COPY Cargo.lock Cargo.toml ./
COPY src ./src
RUN cargo build --release
CMD ./target/release/subscribe_hn
