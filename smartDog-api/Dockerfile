# Build fase for Rust
FROM rust:latest as builder

WORKDIR /app
COPY . .
RUN cargo install --path .

# Create Container
FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/smartdog-api /usr/local/bin/smartdog-api

CMD ["smartdog-api"]
