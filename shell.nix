let
  oxalica_overlay = import (builtins.fetchTarball
    "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ oxalica_overlay ]; };

in
with nixpkgs;

stdenv.mkDerivation {
  name = "rust-env";
  buildInputs = [
    (rust-bin.stable.latest.default.override { extensions = [ "rust-src" ]; })

    # Possibly needed for some crates like reqwest
    pkg-config
    openssl
    cargo-expand
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;

}

