 FROM rust:1.34
 WORKDIR  .
 COPY ./target/debug/examples/basic_usage ./basic_usage
 CMD ["./basic_usage"]

