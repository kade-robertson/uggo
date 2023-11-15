check-all:
	cargo clippy --frozen --tests --examples --all-features --message-format=json --all-targets; \