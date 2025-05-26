FROM rust:1.85-alpine3.20 AS base
RUN apk add musl-dev alpine-sdk openssl-dev perl
WORKDIR /app
COPY . .

FROM base AS dev 
WORKDIR api/server
RUN cargo build
CMD ["cargo", "run"]

# FROM base AS builder
# RUN cargo install --path api
#
# FROM alpine:3.20 AS deploy
# COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
# CMD ["server"]
