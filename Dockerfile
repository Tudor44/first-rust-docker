FROM rust:1.51.0
WORKDIR /usr/src/app
COPY . .
RUN cargo install
CMD ["first-rust-docker"]
