FROM lukemathwalker/cargo-chef:latest-rust-1-bullseye AS chef
WORKDIR /prod

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /prod/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --package ugg-proxy --release

FROM debian:bullseye AS runner
RUN apt-get update && apt-get -y install ca-certificates
COPY --from=builder /prod/target/release/ugg-proxy /bin
CMD ["/bin/ugg-proxy"]