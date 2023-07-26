FROM rust:latest as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:12-slim

ENV FOUNDATION_CONTENT_DIRECTORY=/content
ENV FOUNDATION_LISTEN_ADDR=0.0.0.0:8080

COPY --from=builder /app/target/release/foundation /foundation

CMD ["/foundation"]
