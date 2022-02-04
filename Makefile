.PHONY: verifier updater collector tests

all: tests verifier updater collector

updater:
	cargo run --manifest-path rust/qr_updater/Cargo.toml -- --config=config.toml

verifier:
	cargo run --manifest-path rust/qr_verifier/Cargo.toml -- --config=config.toml

collector:
	cargo run --manifest-path rust/collector/Cargo.toml -- --config=config.toml

tests:
	cargo test --manifest-path rust/Cargo.toml
