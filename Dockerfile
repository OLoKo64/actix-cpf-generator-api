FROM rust:1.65.0 as builder

WORKDIR /api

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
RUN cargo build --release
RUN mv ./target/release/cpf-generator-api .

FROM debian:buster-slim
WORKDIR /api
RUN apt update
RUN apt install -y libssl-dev
COPY --from=builder /api/cpf-generator-api /api/cpf-generator-api

CMD cd /api && ./cpf-generator-api
