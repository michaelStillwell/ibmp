FROM rust:latest

WORKDIR /api

COPY Cargo.toml Cargo.lock ./

RUN mkdir src \
  && echo "fn main() {}" > src/main.rs \
  && cargo build --release

COPY . .

RUN cargo build --release

EXPOSE 3000

CMD ["./target/release/server"]
