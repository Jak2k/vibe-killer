# SPDX-FileCopyrightText: 2026 The Vibe Killer contributers
# SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-GAL
# SPDX-FileContributor: Jakob Schwanenberg

# Display this help screen
help:
    @just --list

# Verify that all licenses are properly declared
[group("dev")]
lint:
    reuse lint
    cargo clippy

# Run in development mode
[group("dev")]
dev:
    cargo run

# Format everthing
[group("dev")]
format:
    cargo fmt
    taplo fmt src/**/*.toml Cargo.toml
    nix fmt flake.nix

# Build the binary
[group("build")]
build:
    cargo build --release
    cp ./target/release/vk ./target/release/vk-nix
    patchelf --set-interpreter /lib64/ld-linux-x86-64.so.2 ./target/release/vk
    patchelf --remove-rpath ./target/release/vk

# Generate documentation
docs:
    cargo doc
