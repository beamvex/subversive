#!/bin/bash

# Build the Docker image
docker build -t subversive .

# Run the test

docker run --rm --cap-add=SYS_PTRACE \
  --security-opt seccomp=unconfined \
  -v SUBVERSIVE_VOLUME:/app \
  -v "$(pwd)":/src \
  -w /app \
  subversive \
  /coverage.sh 