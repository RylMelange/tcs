.PHONY: all build run nvim silentbuild

nvim: silentbuild run

all: build run

silentbuild:
	RUSTFLAGS=-Awarnings cargo build --quiet

build:
	cargo build

run:
	./target/debug/tcs

