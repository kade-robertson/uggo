FROM lukemathwalker/cargo-chef:latest-rust-alpine3.17 AS chef
WORKDIR /prod

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /prod/recipe.json recipe.json
RUN apk add openssl-dev
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --package ugg-proxy --release

FROM alpine:3.17 AS runner
COPY --from=builder /prod/target/x86_64-unknown-linux-musl/release/ugg-proxy /bin
CMD ["/bin/ugg-proxy"]
