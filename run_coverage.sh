#!/bin/bash

# Build the Docker image
docker build -t subversive .

# Run the test
docker run --rm --cap-add=SYS_PTRACE --security-opt seccomp=unconfined -v $(pwd):/app -w /app subversive cargo tarpaulin -- .