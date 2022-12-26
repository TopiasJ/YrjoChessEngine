FROM rust as builder
WORKDIR /src
COPY . .

RUN cargo install --profile release-lto --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/yrjo_chess_engine /usr/local/bin/yrjo_chess_engine
CMD [ "yrjo_chess_engine" ]