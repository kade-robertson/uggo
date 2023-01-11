FROM rust:bullseye AS builder
RUN apt-get update && apt-get -y install ca-certificates libssl-dev
WORKDIR /prod
COPY . .
RUN cargo build --package ugg-proxy --release

FROM debian:bullseye AS runner
RUN apt-get update && apt-get -y install ca-certificates
COPY --from=builder /prod/target/release/ugg-proxy /bin
CMD ["/bin/ugg-proxy"]