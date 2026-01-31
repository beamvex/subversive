FROM rust:trixie

RUN cargo install cargo-tarpaulin

RUN apt-get update 
RUN apt-get install -y rsync

WORKDIR /app
COPY scripts/coverage.sh /coverage.sh
RUN chmod +x /coverage.sh

