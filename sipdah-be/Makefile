#!make

ifneq (,$(wildcard ./.env))
    include .env
    export
endif

DEBUG_MODE = debug
RELEASE_MODE = release

run:
	cargo run
run-release:
	./target/release/social-media

debug:
	cargo build --$$(DEBUG_MODE)

release:
	cargo build --$$(RELEASE_MODE)

clean:
	cargo clean

test:
	cargo test
