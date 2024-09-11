# Stage 1: Clone the repository
FROM alpine/git AS git
WORKDIR /app
RUN git clone https://github.com/dcodesdev/rustfinity.com

# Stage 2: Build the Runner CLI and the rustfinity.com project
FROM rust:slim-buster
LABEL rustfinity-runner="true"

# Install OpenSSL development packages and pkg-config
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Build the Runner CLI
WORKDIR /app/runner
COPY . .
RUN cargo build --release

# Move the Runner CLI executable
RUN mv target/release/rustfinity-runner /app/

# Build the rustfinity.com project
WORKDIR /app/rustfinity.com
COPY --from=git /app/rustfinity.com .
RUN cargo build

# Create a new project for (rustfinity.com/playground)
WORKDIR /app
RUN cargo new playground

# The final structure:
# /app/rustfinity-runner (executable)
# /app/rustfinity.com/ (project directory)
# /app/playground/ (project directory)
