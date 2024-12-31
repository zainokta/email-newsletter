FROM rust:1.83.0-alpine3.20 AS builder

WORKDIR /app

RUN apk update && apk add --no-cache lld clang

COPY . .

ENV SQLX_OFFLINE true

RUN cargo build --release

FROM scratch AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/zero2prod zero2prod

COPY configuration configuration

ENV APP_ENVIRONMENT production

ENTRYPOINT ["./zero2prod"]