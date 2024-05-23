{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    licensure-dev = {
      url = "github:chasinglogic/licensure";
      flake = false;
    };
 
  };

  description = "Advent of Code 2023 solutions written in Rust";

  outputs = { self, nixpkgs, flake-utils, naersk, licensure-dev }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk' = pkgs.callPackage naersk {};
        rust-toolchain = pkgs.symlinkJoin {
          name = "rust-toolchain";
          paths = [ pkgs.rustc pkgs.cargo pkgs.cargo-watch pkgs.rust-analyzer  pkgs.rustPlatform.rustcSrc pkgs.clippy
          pkgs.rustfmt];
        };
      in rec {
        defaultPackage = naersk'.buildPackage ./.;
        licensure = naersk'.buildPackage {
          src = licensure-dev;
          buildInputs = [ pkgs.openssl.dev pkgs.pkg-config ];
        };
        devShell = with pkgs; mkShell {
          buildInputs = [ rust-toolchain pkgs.openssl.dev pkgs.pkg-config licensure ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
  });
}
