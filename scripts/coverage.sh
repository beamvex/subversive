#!/bin/bash
cp /src/Cargo.toml /app/Cargo.toml
cp /src/Cargo.lock /app/Cargo.lock
rsync -r /src/src/* /app/src/
cargo tarpaulin --out html --output-dir /src/coverage