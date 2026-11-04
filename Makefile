.PHONY: tests

UNAME := $(shell uname)
ifeq ($(UNAME), Darwin)
	export DYLD_FALLBACK_LIBRARY_PATH=$(shell xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib
	# Homebrew's `opencv` formula is OpenCV 5, which the pinned `opencv` crate (0.94.x) does not support.
	# `opencv@4` is keg-only, so point pkg-config at it when it is installed (Apple Silicon or Intel prefix).
	OPENCV4_PKGCONFIG := $(firstword $(wildcard /opt/homebrew/opt/opencv@4/lib/pkgconfig /usr/local/opt/opencv@4/lib/pkgconfig))
	ifneq ($(OPENCV4_PKGCONFIG),)
		export PKG_CONFIG_PATH := $(OPENCV4_PKGCONFIG)$(if $(PKG_CONFIG_PATH),:$(PKG_CONFIG_PATH))
	endif
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

docker.%:
	docker-compose run --rm $(*)
