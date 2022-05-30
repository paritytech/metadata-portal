#  Metadata Portal 🌗

Metadata Portal is a self-hosted web page that shows you the latest metadata for a given network.

This is an important addition to Signer, which can update the metadata inside only through a special video QR code without going online. 
Parity will host its own version of the page for all chains for which we sign the metadata. 
External users (chain owners) will be able to deploy their versions of metadata portal if they want.

## How does it work?

It all starts with the Github repository. Any user can clone it and run their Metadata Portal. We also host our own version, so let's break down the principles of working on it.

Metadata Portal supports two metadata sources in parallel. Both are equally important for different types of users.

### 1. Parsing it from chain and generating QR codes itself

This flow is important for all users who want to always have the latest metadata in their signing devices to parse and sign their transactions right away.

- Cron job
  - runs every N hours and checks every known network for the latest metadata version
  - If any network has a new version of metadata that has not yet been published on the Metadata Portal
    - generates unsigned metadata QR code
    - creates new pull request to the repo
    - sends notification to a Matrix channel
- Release manager
  - checkouts pull request's branch locally
  - runs `make signer` locally to sign new metadata using his signing air-gapped device
  - commit and push changes to the same branch
- Owner of the repository
  - accept and merge the PR
- Github action is triggered to regenerate and re-deploy the Github Page

### 2. Showing manually uploaded and signed QR codes via PRs

This flow is for security-oriented users and Parity itself. It allows chain owners to sign their metadata updates and host QR codes for their users.

- Release manager generates a new signed QR code manually in an air-gapped environment using his signing device
- He opens a PR and signs commits by his YubiKey to prove its validity
- Owner of the repository accepts the PR
- Github action is triggered to regenerate and re-deploy the Github Page

## Deployment
### Requirements
1. install https://github.com/paritytech/parity-signer to your signing device

### Steps

You can use Github Pages to host the metadata-portal for your set of chains
1. Fork this repo
2. Edit `config.toml`
   1. Add/remove chains
   2. Edit signer's name and public key. The key can be exported from [parity-signer](https://github.com/paritytech/parity-signer)
3. Configure GitHub Pages to build from `gh-pages` branch (`Settings` -> `Pages` -> `Source`)
4. Edit domain name in:
   1. `homepage` field in `package.json`
   2. `public/CNAME` file
5. Notifications to Matrix:
   1. You can disable it by setting `NOTIFY_MATRIX: false` in `.github/workflows/update.yml`
   2. Otherwise, add `MATRIX_SERVER`, `MATRIX_ROOM_ID`, `MATRIX_ACCESS_TOKEN` values to project Actions secrets

## Development
### Dependencies
The main requirement is the OpenCV. You can check this manual: https://crates.io/crates/opencv


#### Arch Linux:

OpenCV package in Arch is suitable for this.

`pacman -S clang qt5-base opencv`

#### Ubuntu:

`sudo apt install libopencv-dev clang libclang-dev`

#### Other Linux:
You have several options of getting the OpenCV library:

* install it from the repository, make sure to install `-dev` packages because they contain headers necessary
  for the crate build (also check that your package contains `pkg_config` or `cmake` files).

* build OpenCV manually and set up the following environment variables prior to building the project with
  `opencv` crate:
    * `PKG_CONFIG_PATH` for the location of `*.pc` files or `OpenCV_DIR` for the location of `*.cmake` files
    * `LD_LIBRARY_PATH` for where to look for the installed `*.so` files during runtime

Additionally, please make sure to install `clang` package or its derivative that contains `libclang.so` and
`clang` binary.
* Gentoo, Fedora: `clang`
* Debian, Ubuntu: `clang` and `libclang-dev`

#### MacOs:

`brew install opencv`

If you're getting `dyld: Library not loaded: @rpath/libclang.dylib`:
OS can't find libclang.dylib dynamic library because it resides in a non-standard path, set up the DYLD_FALLBACK_LIBRARY_PATH environment variable to point to the path where libclang.dylib can be found, e.g. for XCode:

`export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/"`


### Frontend
Before running the frontend locally, you need to generate a data file:

    make collector

 And then run the app in the development mode

`yarn start`
