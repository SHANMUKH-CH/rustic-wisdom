FROM rust:1.70 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/app/target/release/rustic-wisdom /usr/local/bin/rustic-wisdom
CMD ["rustic-wisdom"]
