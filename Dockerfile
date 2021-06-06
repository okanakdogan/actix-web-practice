FROM rust:1.51.0 as builder

WORKDIR /usr/src/bjira

COPY . .

RUN  cargo install cargo-watch && cargo install --path .
