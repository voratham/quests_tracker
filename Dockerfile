FROM rust:1.82-bullseye as builder

WORKDIR /usr/src/quests_tracker
COPY . .
RUN cargo install --path .


FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl-dev \
    libpq-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/quests_tracker /usr/local/bin/quests_tracker

CMD ["quests_tracker"]