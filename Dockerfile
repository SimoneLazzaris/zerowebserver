FROM rust:buster as builder
WORKDIR /src
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /src/target/release/zerowebserver /bin/zerowebserver
ENTRYPOINT /bin/zerowebserver
