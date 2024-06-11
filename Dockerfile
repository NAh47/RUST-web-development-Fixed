# syntax=docker/dockerfile:1

FROM rust:1.77.0-alpine AS build
WORKDIR /app

RUN apk add --no-cache musl-dev openssl-dev build-base

COPY . .
COPY .env .

RUN cargo build --release

FROM alpine:3.18

RUN apk add --no-cache libssl1.1 curl

COPY --from=build /app/target/release/testdb /usr/local/bin/testdb

CMD ["testdb"]
