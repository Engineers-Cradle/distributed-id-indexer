FROM rust:1.80.1-slim-bullseye

# View app name in Cargo.toml
ARG APP_NAME=distributed-id-indexer-pubsub

WORKDIR /app

COPY . .
RUN cargo build --release
RUN cp ./target/release/$APP_NAME /distributed-id-indexer-pubsub

CMD ["/distributed-id-indexer-pubsub"]