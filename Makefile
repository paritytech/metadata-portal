.PHONY: tests

UNAME := $(shell uname)
ifeq ($(UNAME), Darwin)
	export DYLD_FALLBACK_LIBRARY_PATH=/Library/Developer/CommandLineTools/usr/lib
endif

all: tests verifier updater collector cleaner signer

verifier:
	cargo run --release verify

updater:
	cargo run --release update

collector:
	cargo run --release collect

cleaner:
	cargo run --release clean

signer:
	cargo run --release sign

tests:
	cargo test --release
