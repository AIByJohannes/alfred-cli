.PHONY: build install test clean

build:
	cargo build --workspace

install:
	cargo install --path crates/alfred-cli --locked

test:
	cargo test --workspace

clean:
	cargo clean
