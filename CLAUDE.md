# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository overview

Metadata Portal is a self-hosted website that publishes the latest runtime metadata for Substrate-based chains as scannable QR codes for the Parity Signer (air-gapped) signing device. It has two parts that share one repo:

- **`cli/`** — `metadata-cli`, a Rust binary with subcommands that maintain the QR-code corpus checked into `public/qr/` and the `public/data.json` consumed by the frontend.
- **`src/`** — A Create-React-App + Tailwind frontend deployed to GitHub Pages (`gh-pages` branch). It is a static site that fetches `data.json` and `portals.json` at runtime.

The two halves communicate via files under `public/`: the CLI writes QR images and `data.json`; the frontend reads them.

Note: `rust/` at the repo root contains only stale `target/` directories — the live Rust code is `cli/`. The top-level `Cargo.toml` is a workspace with one member: `cli`.

## Common commands

Frontend (run from repo root):
- `yarn` — install deps
- `yarn start` — dev server (requires `public/data.json`; either run the CLI first or `cp public/test-file.json public/data.json`)
- `yarn build` — production build
- `yarn lint` — eslint over `src/**/*.{ts,tsx}`
- `yarn prettier` — format `src/**/*.{ts,tsx}`
- `yarn test` — Jest via react-scripts

CLI (Rust, run from repo root via the `Makefile` so `DYLD_FALLBACK_LIBRARY_PATH` is set on macOS — direct `cargo` invocations may fail to link against `libclang.dylib`):
- `make updater` — `cargo run --release update` (defaults to `--source node`; use `cargo run --release -- update --source github` for runtime WASM from GitHub releases)
- `make collector` — regenerates `public/data.json` from QRs in `public/qr/` plus live RPC specs
- `make signer` — interactive: scans a signature QR via webcam (OpenCV) and converts unsigned QRs to signed ones
- `make verifier` — validates all signed QRs against the configured public key
- `make cleaner` — removes obsolete QRs not referenced by `data.json`
- `make tests` — `cargo test --release`
- Single test: `cargo test --release -p metadata-cli <test_name>` (e.g., `test_collector`)
- Lint: `cargo fmt --all -- --check` and `cargo clippy -- -D warnings` (CI requires both clean)

Local frontend bootstrap (per `README.md`): `make updater && make collector && yarn start`.

## CLI architecture

Entry point: `cli/src/main.rs` dispatches `clap` subcommands from `opts.rs` to module-per-command handlers:

- `updater/` — adds new unsigned QRs when a chain has a metadata version not yet in `public/qr/`. Two sources: `update_from_node` (RPC via `qr_reader_pc`/`generate_message`) and `update_from_github` (downloads runtime WASM from GitHub releases configured per-chain). Both call `generate::generate_metadata_qr` and embed provenance via `source.rs::save_source_info` (zTXt PNG chunk).
- `signer/` — operator workflow: lists unsigned QRs, opens each in a browser, reads a signature QR via OpenCV webcam, then uses `generate_message::full_run` to produce a signed QR, preserving the source zTXt chunk.
- `verifier/` — checks every signed QR's signature against `verifier.public_key` from `config.toml`.
- `collector/export.rs` — the source of truth for `public/data.json`: walks `qr_dir`, picks the right metadata QR per chain (preferring signed; keeping versions ≥ live; one symlink `<chain>_metadata_latest.apng` per chain pointing at the live version), and serializes an `ExportData` map keyed by `chain.portal_id()`.
- `cleaner/` — reads `data.json` via `file::files_to_keep` and removes everything else from `qr_dir`.
- `deployment_checker/` — exits with code **12** when on-disk specs differ from the deployed `data.json` so the `update.yml` workflow can detect "needs redeploy" without needing a non-zero failure semantics.

Cross-cutting:
- `common/path.rs` — `QrFileName` parses/produces filenames of the form `[unsigned_]<chain>_<metadata_NNN|specs>.<apng|png>`. The `unsigned_` prefix is the only signal of signed-vs-unsigned. `chain` here is the `portal_id` (relay-prefixed for parachains, e.g. `polkadot-statemint`).
- `config.rs` — `AppConfig::load` canonicalizes `data_file`/`public_dir`/`qr_dir` against the config's parent dir so the CLI works from anywhere. `Chain.portal_id()` is `<relay>-<name>` for parachains, plain `name` for relay chains; `formatted_title()` is what the UI shows.
- `fetch.rs` — `RpcFetcher` accepts the `rpc_endpoint` field as either a string or a list (`string_or_vec` deserializer in `config.rs`); falls back through endpoints on failure.
- `source.rs` — `Source::{Wasm, Rpc}` is JSON-serialized into the PNG `Source` zTXt chunk so the frontend can show provenance.

## Frontend architecture

`src/index.tsx` mounts `<App>` inside `BrowserRouter`. `src/components/App.tsx` fetches `data.json` (chains) and `portals.json` (links to other portal instances), then drives a single-network detail view (`Network`) with a sidebar (`NetworkSelect`/`PortalSelect`). The current chain syncs to the URL hash. `src/scheme.ts` defines the TS shape mirroring `cli/src/export.rs::ExportData`.

## Adding/removing a chain

Edit `config.toml`. A new entry needs `name`, `rpc_endpoint`, optional `title`, `color`, `relay_chain`, `token_unit`/`token_decimals`, and `[chains.github_release]` (with `genesis_hash`) if you also want GitHub-release-based updates. After editing, run `make updater && make signer && make collector` to populate QRs and refresh `data.json`.

## Operational flow (CI)

`.github/workflows/update.yml` runs hourly: it runs `update --source node` then `update --source github`, commits to a long-lived `sign-me-YYYY-MM-DD` branch (creates a draft PR if none exists), and notifies Matrix. A release manager checks out that branch, runs `make signer` locally with a Parity Signer device, pushes the signed QRs, and the PR is merged. `deploy.yml` then runs `make verifier && make collector` and pushes the build to `gh-pages`. `verify.yml` runs `cargo run --release -- verify` on PRs that touch `public/qr/**` or `config.toml`.

## Platform notes

- **macOS:** OpenCV via `brew install opencv`. The `Makefile` exports `DYLD_FALLBACK_LIBRARY_PATH` to the Xcode toolchain so `libclang.dylib` resolves. Prefer `make <target>` over raw `cargo` for CLI work locally.
- **Linux:** `libopencv-dev clang libclang-dev` (Ubuntu).
- The CLI uses Unix `symlink` in `collector/export.rs` (latest-metadata pointer) — Windows is not supported.
