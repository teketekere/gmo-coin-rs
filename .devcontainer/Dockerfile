FROM rust:1.47-buster

RUN apt-get update -y && \
    apt-get install -y \
    nano \
    lldb \
    pkg-config \
    libssl-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN rustup component add \
    rustfmt \
    rls \
    rust-analysis \
    rust-src \
    clippy
