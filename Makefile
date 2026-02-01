.PHONY: build build-cli install test clean run dev

build:
	cargo build --workspace

build-cli:
	cargo build -p alfred-cli

run:
	cargo run -p alfred-cli

dev: run

install:
	cargo install --path crates/alfred-cli --locked

test:
	cargo test --workspace

clean:
	cargo clean
