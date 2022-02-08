.PHONY: tests

all: tests verifier updater collector cleaner

%:
	cargo run --manifest-path rust/$(*)/Cargo.toml -- --config=config.toml

tests:
	cargo test --manifest-path rust/Cargo.toml
