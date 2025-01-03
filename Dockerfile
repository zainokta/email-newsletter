FROM lukemathwalker/cargo-chef:latest-rust-1.83.0-alpine3.21 as chef

WORKDIR /app

RUN apk update && apk add --no-cache lld clang

FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

ENV SQLX_OFFLINE true

RUN cargo build --release --bin zero2prod

FROM scratch AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/zero2prod zero2prod

COPY configuration configuration

ENV APP_ENVIRONMENT production

ENTRYPOINT ["./zero2prod"]