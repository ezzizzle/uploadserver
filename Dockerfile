FROM rust:1.40 as builder
WORKDIR /usr/src/uploadserver
COPY . .
RUN cargo test -- --test-threads=1
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/uploadserver /usr/local/bin/uploadserver
CMD ["uploadserver"]