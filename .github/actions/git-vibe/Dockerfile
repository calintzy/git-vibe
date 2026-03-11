FROM rust:1.82-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl jq git && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/git-vibe /usr/local/bin/
COPY .github/actions/git-vibe/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
