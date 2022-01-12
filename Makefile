CARGO_FILE=rust/qr_generator/Cargo.toml

.PHONY: rust.run rust.run

all: rust

rust: rust.test rust.run

rust.test:
	cargo test --manifest-path $(CARGO_FILE)

rust.run:
	cargo run --manifest-path $(CARGO_FILE) -- --config-file config.toml
