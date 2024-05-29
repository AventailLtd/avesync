FROM rust:1.78.0-slim-bookworm

WORKDIR /usr/src/app

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

EXPOSE 3001

VOLUME ["/usr/local/cargo"]