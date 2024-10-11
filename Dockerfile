FROM rust:1.81.0 as builder

WORKDIR /usr/src/app

RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./
COPY src src
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/tofl-gpt-parser /usr/local/bin/tofl-gpt-parser

CMD ["tofl-gpt-parser"]