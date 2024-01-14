FROM rust:1.74-bullseye

WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .
RUN cargo install sea-orm-cli
EXPOSE 3000

RUN cargo build

ENTRYPOINT ["bash", "./entrypoint.sh" ]