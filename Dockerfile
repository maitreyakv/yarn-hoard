# ============================================================================ #
#                                     COMMON                                   #
# ============================================================================ #

FROM rust:1.85-alpine3.20 AS base
RUN apk add musl-dev alpine-sdk openssl-dev perl 
RUN cargo install --locked cargo-binstall && \
      cargo binstall trunk
RUN rustup target add wasm32-unknown-unknown
WORKDIR /app
COPY . .

# ============================================================================ #
#                                   API SERVER                                 #
# ============================================================================ #

# FROM base AS builder
# RUN cargo install --path api
#
# FROM alpine:3.20 AS deploy
# COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
# CMD ["server"]

# ============================================================================ #
#                                     SITE                                     #
# ============================================================================ #
