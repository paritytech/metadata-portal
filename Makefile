.PHONY: tests

UNAME := $(shell uname)
ifeq ($(UNAME), Darwin)
	export DYLD_FALLBACK_LIBRARY_PATH=$(shell xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib
endif

all: tests verifier updater collector cleaner

%:
	cargo run --manifest-path rust/$(*)/Cargo.toml -- --config=config.toml

tests:
	cargo test --manifest-path rust/Cargo.toml
