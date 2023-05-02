PACKAGES := ugg-proxy ugg-types uggo

check-all:
	@for pkg in $(PACKAGES); do \
		cargo clippy --frozen --tests --examples --all-features --package $$pkg --message-format=json --all-targets; \
	done