{}:
let
  nixpkgs = import <nixpkgs> { };
  pkgs = nixpkgs.pkgs;

  downloader = import ./default.nix { };
  downloader-c = import ./tests/capi/default.nix { };

in
pkgs.stdenv.mkDerivation {
  name = "rust-shell";
  nativeBuildInputs =
    downloader.nativeBuildInputs ++
    downloader-c.nativeBuildInputs ++
    (with pkgs; [
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
  buildInputs =
    downloader.buildInputs ++
    downloader-c.buildInputs;
}
