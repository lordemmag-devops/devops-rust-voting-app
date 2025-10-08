# builder
FROM rust:1.90 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# runtime
FROM debian:bookworm-slim
COPY --from=builder /usr/src/app/target/release/backend-rust /usr/local/bin/backend-rust
EXPOSE 8080
CMD ["backend-rust"]
