FROM rust:latest

WORKDIR /app

RUN cargo install diesel_cli --no-default-features --features postgres

COPY . .

RUN cargo build --release

CMD ["./target/release/rust-rest-api"]
