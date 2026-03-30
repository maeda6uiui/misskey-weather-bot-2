FROM rust:1.92 AS builder

WORKDIR /misskey-weather-bot-2
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /misskey-weather-bot-2
COPY --from=builder /misskey-weather-bot-2/target/release/weather-bot /misskey-weather-bot-2/weather-bot
COPY Data/ /misskey-weather-bot-2/Data/
ENV RUST_LOG=info
CMD ["weather-bot"]
