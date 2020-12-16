{}:
let
  nixpkgs = import <nixpkgs> { };
  pkgs = nixpkgs.pkgs;

  project = import ./default.nix { };
in
pkgs.stdenv.mkDerivation {
  name = "rust-shell";
  nativeBuildInputs = project.nativeBuildInputs ++ (with pkgs; [
    # rust
    cargo-audit
    cargo-edit
    cargo-tarpaulin
    clippy
    rust-analyzer
    rustfmt

    # nix
    nixpkgs-fmt
  ]);
  buildInputs = project.buildInputs;
}
