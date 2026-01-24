FROM rust:trixie

RUN cargo install cargo-tarpaulin

WORKDIR /app


