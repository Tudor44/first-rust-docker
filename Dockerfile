FROM rust:1.51.0 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/first-rust-docker /usr/local/bin/first-rust-docker

CMD ["first-rust-docker"]
