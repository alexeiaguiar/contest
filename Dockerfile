# syntax=docker/dockerfile:1

# Builder stage
FROM rust:1.83 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/contest /app/contest
CMD ["/app/contest"]