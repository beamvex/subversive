#!/bin/bash
cp -r /src/src/* /app/src/
cargo tarpaulin --out html --output-dir /src/coverage