test:
	cargo test $(day) -- --show-output

lint:
	cargo fmt -- --check
	cargo clippy --all-targets --all-features -- -D warnings
