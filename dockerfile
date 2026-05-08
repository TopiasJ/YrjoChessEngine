FROM rust:1.86-slim-bookworm AS builder
WORKDIR /src
COPY . .
RUN cargo install --profile release-lto --path .

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/yrjo_chess_engine /usr/local/bin/yrjo_chess_engine
ENTRYPOINT ["yrjo_chess_engine"]
