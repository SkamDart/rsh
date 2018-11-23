.PHONY: build clean deploy doc run test

all: test run

build:
	cargo build

clean:
	cargo clean

deploy: release test run

doc: build
	cargo doc

release:
	cargo build --release

run:
	cargo run

test: build
	cargo test
