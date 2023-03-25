FROM rust:1.68-slim-buster as builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY src src

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && \
    apt-get install -y libpq-dev && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/actix /usr/local/bin/

ENV DATABASE_URL=/data/actix.db

EXPOSE 8080

CMD ["actix"]

