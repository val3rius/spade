target/release/spade:
	cargo build --release

lint-rust:
	cargo clippy --fix --allow-dirty --allow-staged
.PHONY: lint-rust

lint-js:
	./examples/theme/node_modules/.bin/eslint ./examples/theme/src/**/*.ts --fix
.PHONY: lint-js

lint: lint-rust lint-js
.PHONY: lint

test:
	cargo test
.PHONY: test