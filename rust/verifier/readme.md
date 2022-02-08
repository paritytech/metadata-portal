# QR verifier

QR verifier checks that all signed metadata QRs have valid filename and signature.

## Getting Started

### Dependencies

The main requirement is the OpenCV. You can check this manuals: https://crates.io/crates/opencv and https://docs.opencv.org.

#### Arch Linux:

OpenCV package in Arch is suitable for this crate. It requires some dependencies.

* `pacman -S clang qt5-base opencv`

#### Other Linux systems:

* For Debian/Ubuntu also you need: `clang` and `libclang-dev`
* For Gentoo/Fedora also you need: `clang`
* It is preferable to build latest version of opencv+opencv_contrib from source. OpenCV package from the system repository may not contain the necessary libraries.\
Use this manual: https://docs.opencv.org/4.5.3/d7/d9f/tutorial_linux_install.html

#### Examples

* `cargo run -- --config=../../config.toml`
