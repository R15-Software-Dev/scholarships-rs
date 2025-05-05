﻿FROM rust:1.86 AS builder

# Install node for tailwind capability
RUN apt-get update -y && apt-get install -y --no-install-recommends clang libc6 nodejs npm
RUN node -v
RUN npm -v

RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

RUN cargo binstall cargo-leptos -y

RUN rustup target add wasm32-unknown-unknown

RUN mkdir -p /app
WORKDIR /app
COPY . .

RUN npm install tailwindcss @tailwindcss/cli

RUN cargo leptos build --release -vv

# Start second phase of build - install ca-certs and copy all build outputs from previous step into a
# Debian image in the /app directory.
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Remember to replace project_name with the actual project name
COPY --from=builder /app/target/release/scholarships-rs /app/

# This folder contains the JS, WASM, and other such files
COPY --from=builder /app/target/site /app/site

# Copy Cargo.toml just in case it's needed
COPY --from=builder /app/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

# Don't forget about project_name
RUN ls -lh /app/

# For some reason this starts in the /bin/usr directory, so use the relative path
CMD ["../app/scholarships-rs"]
