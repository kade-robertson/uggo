PACKAGES := ugg-proxy ugg-types uggo

check-all:
	@for pkg in $(PACKAGES); do \
		cargo clippy --frozen --tests --examples --package $$pkg --message-format=json --all-targets; \
	done