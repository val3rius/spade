target/release/spade:
	cargo build --release

lint:
	cargo clippy
.PHONY: lint

test:
	cargo test
.PHONY: test