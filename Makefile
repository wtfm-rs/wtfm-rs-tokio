all: 	check clippy fmt test doc
	cp -r target/doc .

install:
	cargo add tokio --features=full

check:
	cargo check

clippy:
	cargo clippy

fmt:
	cargo fmt

doc:
	cargo doc --examples

review:
	cargo doc --examples --open

test:
	cargo test
	cargo run --example example-echo-hello

clean:
	cargo clean
	rm -rf doc
