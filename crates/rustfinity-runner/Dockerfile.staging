FROM alpine/git AS git

WORKDIR /app

RUN git clone https://github.com/dcodesdev/rustfinity.com

# --- 
FROM rust:slim-buster
LABEL rustfinity-runner="true"

# --- Building the Runner CLI first ---
WORKDIR /app/build

COPY . .

RUN cargo build --release

COPY /app/build/target/release/rustfinity-runner /app/

# Runner CLI executable binary is now available at /app/rustfinity-runner

# --- Building the rustfinity.com project ---
COPY --from=git /app/rustfinity.com /app/

WORKDIR /app/rustfinity.com

RUN cargo build
