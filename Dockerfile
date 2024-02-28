FROM rust:1.74-bullseye AS builder

WORKDIR /usr/src/axum-prototype-auth

COPY Cargo.toml Cargo.lock ./

COPY src ./src

COPY migration ./migration

COPY entity ./entity

RUN cargo build --locked --release

FROM debian:bullseye-slim

WORKDIR /usr/src/app

COPY --from=builder /usr/src/axum-prototype-auth/target/release/axum-prototype-auth /usr/src/app/axum-prototype-auth

EXPOSE 80

CMD ["./axum-prototype-auth"]