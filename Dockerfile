FROM rust:1.74-bullseye AS builder

WORKDIR /usr/src/axum-prototype-auth

COPY Cargo.toml Cargo.lock ./

COPY src ./src

COPY migration ./migration

COPY entity ./entity

RUN cargo build --locked --release

RUN cp ./target/release/axum-prototype-auth /bin/server

FROM debian:bullseye-slim

WORKDIR /usr/src/app

COPY --from=builder /bin/server /bin/

EXPOSE 3000

CMD ["/bin/server"]