FROM rust:slim-buster
LABEL rustfinity-runner="true"

# Install OpenSSL development packages and pkg-config
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    heaptrack \
    && rm -rf /var/lib/apt/lists/*

# Build the Runner CLI
WORKDIR /app

COPY crates/ crates/
COPY crates/rustfinity-runner/docker/Cargo.toml Cargo.toml

RUN cargo new challenges/playground && rm -rf challenges/playground/.git

WORKDIR /app/crates/rustfinity-runner
RUN cargo build --release

# Move the Runner CLI executable
RUN mv /app/target/release/rustfinity-runner /app/

WORKDIR /app
RUN cargo build

# The final structure:
# /app/rustfinity-runner (executable)
# /app/challenges/playground (project)
# /app/crates/syntest (library)
