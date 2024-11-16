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

RUN mkdir challenges

WORKDIR /app/crates/rustfinity-runner
RUN cargo build --release

# Move the Runner CLI executable
RUN mv /app/crates/rustfinity-runner/target/release/rustfinity-runner /app/

# Create a new project for (rustfinity.com/playground)
WORKDIR /app/challenges
RUN cargo new playground && rm -rf playground/.git

WORKDIR /app

# Build the rustfinity.com project
RUN cd challenges/playground && cargo build
RUN cd crates/syntest && cargo build

# The final structure:
# /app/rustfinity-runner (executable)
# /app/challenges/playground (project)
# /app/crates/syntest (library)
