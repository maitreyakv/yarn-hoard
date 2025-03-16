FROM rust:1.85-alpine3.20 AS builder

RUN apk add musl-dev
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM alpine:3.20

COPY --from=builder /usr/local/cargo/bin/yarn-hoard-api /usr/local/bin/yarn-hoard-api
CMD ["yarn-hoard-api"]
