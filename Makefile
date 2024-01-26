.PHONY: all check clean

all: build

check: fmt test clippy

test:
	(command -v cargo-nextest && cargo nextest run --all-features) || cargo test --all-features

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --workspace --all-targets --tests -- -D warnings

doc:
	RUSTDOCFLAGS="-D warnings" cargo doc --document-private-items --all --no-deps

clean:
	cargo clean

build:
	cargo build --release

build-mac:
	cargo build --release --target aarch64-apple-darwin

unused_dep:
	cargo machete