# Dockerfile
FROM rust:1.51 as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /app/target/release/neurox /usr/local/bin/neurox

CMD ["neurox"]