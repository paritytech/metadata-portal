.PHONY: verifier generator

all: verifier generator

generator:
	cargo run --manifest-path rust/qr_generator/Cargo.toml -- --config=config.toml

verifier:
	cargo run --manifest-path rust/qr_verifier/Cargo.toml -- --config=config.toml

collector:
	cargo run --manifest-path rust/data_collector/Cargo.toml -- --config=config.toml
