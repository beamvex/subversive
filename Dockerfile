FROM rust:trixie

RUN cargo install cargo-tarpaulin

WORKDIR /app
COPY scripts/coverage.sh /coverage.sh
RUN chmod +x /coverage.sh

