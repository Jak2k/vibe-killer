# SPDX-FileCopyrightText: 2026 The Vibe Killer contributers
# SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-GAL
# SPDX-FileContributor: Jakob Schwanenberg
# SPDX-FileContributor: oxalica
# SPDX-FileContributor: atcol

{
  description = "Vibe Killer";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        formatter = pkgs.nixfmt;

        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              pkg-config

              # Useful tools
              just
              cargo-watch

              # Toolchain
              just-lsp
              (rust-bin.stable.latest.default.override {
                extensions = [ "rust-src" ];
              })
              patchelf

              # Quality assurance
              reuse
              taplo
            ];
          };
      }
    );
}
