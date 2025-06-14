.PHONY: all build prove verify verify-rust clean

all: build prove verify verify-rust

build:
	cargo build --release
	cd program && cargo prove build

prove:
	cargo run --release --bin prove -- --index 12

verify:
	go run verifier/main.go

verify-rust:
	cargo run --release --bin verify

clean:
	rm -rf output/
	rm -rf target/
	cd program && rm -rf target/ 