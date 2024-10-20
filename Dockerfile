FROM rust:slim-bullseye AS builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release
FROM debian:bullseye-slim 
RUN apt-get update && apt-get install -y gcc make libc6-dev  \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/app
COPY --from=builder /usr/src/myapp/target/release/actix_test .
EXPOSE 8080
CMD ["./actix_test"]