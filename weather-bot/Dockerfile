FROM rust:1.92-bullseye AS builder

WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM public.ecr.aws/lambda/provided:al2023

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/weather-bot /var/task/bootstrap
COPY Data/ /var/task/Data/
ENV RUST_LOG=info
CMD ["bootstrap"]
