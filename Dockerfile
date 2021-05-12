FROM rust:1.51.0 as builder

# muslc is required in order to build the rust image.
# RUN apt-get update && apt-get -y install ca-certificates cmake musl-tools libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/bjira

COPY . .

#ENV PKG_CONFIG_ALLOW_CROSS=1

RUN cargo install --path . && cargo install cargo-watch

# CMD ["cargo watch -x 'run --bin bjira'"]
# CMD ["bjira"]
# FROM alpine:3.8

# RUN apk --no-cache add ca-certificates 
# COPY --from=builder /target/x86_64-unknown-linux-musl/release/rust-actix-web .

# CMD ["/bjira"]