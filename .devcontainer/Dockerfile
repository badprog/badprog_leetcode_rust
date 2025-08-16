# Base image
FROM rust:1.89.0
# FROM mcr.microsoft.com/devcontainers/rust:1

# Run
RUN apt-get update && apt-get install -y \
    git \
    tree \
    nano \
    jq \
    && rm -rf /var/lib/apt/lists/*

# Where to work
WORKDIR /workspace

# Copy
COPY . .

# Rust stuff
RUN rustup component add rustfmt clippy