build:
	cargo build --verbose --all --release

check:
	cargo clippy --all-targets --all-features -- -D warnings -A clippy::trivial-regex

test:
	cargo test --verbose --all
