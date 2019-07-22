 FROM rust:1.34
 WORKDIR  .
 COPY . .
 RUN cargo test
 RUN cargo run --example basic_usage


