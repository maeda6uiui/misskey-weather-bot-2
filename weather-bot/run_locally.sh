#!/bin/bash

RUST_LOG=info cargo run --no-default-features --features local -- \
    --weather-api-query Tokyo \
    --weather-api-days 1 \
    --misskey-server-url https://misskey-dabansky.com
