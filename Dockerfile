FROM rust:1.67

WORKDIR /app
COPY . .

RUN cargo run --release

CMD ["./target/release/dockte"]