.PHONY: tests

UNAME := $(shell uname)
ifeq ($(UNAME), Darwin)
	export DYLD_FALLBACK_LIBRARY_PATH=$(shell xcode-select --print-path)/usr/lib
endif

# "all" target builds the project
# "clean" target cleans installed build to prepare for a fresh build
# "remove" target removes data files including QR bar codes

#all: tests verifier updater collector cleaner signer

all:
	cargo build --release  

clean:
	rm -rf target

remove:
	rm -f public/data.json public/qr/*

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

autosigner:
	cargo run --release auto-sign

tests:
	cargo test --release

docker.%:
	docker-compose run --rm $(*)
