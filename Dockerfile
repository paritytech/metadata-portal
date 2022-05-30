FROM rust:1.60

# Build time options to avoid dpkg warnings and help with reproducible builds.
ENV DEBIAN_FRONTEND=noninteractive \
    CARGO_HOME="/app/target"

# Create CARGO_HOME folder and don't download rust docs
RUN mkdir -pv "${CARGO_HOME}" \
    && rustup set profile minimal

# Install system packages
RUN apt-get update \
    && apt-get install -y \
        --no-install-recommends \
        libopencv-dev clang libclang-dev libssl-dev ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

ENTRYPOINT ["cargo", "run", "--release"]
