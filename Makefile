.PHONY: all native

all: cargo_build_release

native:
	RUSTFLAGS="-C target-cpu=native" cargo build --release --no-default-features

cargo_build_release:
	cargo build --release
