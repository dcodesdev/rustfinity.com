# --- Stage 1 ---
FROM rust:slim-buster AS builder
WORKDIR /app

COPY crates/rustfinity-runner .
RUN cargo build --release

# --- Stage 2 ---

FROM rust:slim-buster AS runner
LABEL rustfinity-runner="true"

# Install OpenSSL development packages and pkg-config
RUN apt-get update && apt-get install -y \
    heaptrack \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY crates/syntest crates/syntest
COPY crates/rustfinity-runner/docker/Cargo.toml Cargo.toml

RUN cargo new challenges/playground && rm -rf challenges/playground/.git

WORKDIR challenges/playground
RUN cargo add syn quote

WORKDIR /app
RUN cargo build

COPY --from=builder /app/target/release/rustfinity-runner /app/

# The final structure:
# /app/rustfinity-runner (executable)
# /app/challenges/playground (project)
# /app/crates/syntest (library)
