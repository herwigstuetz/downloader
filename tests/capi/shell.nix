{}:
let
  nixpkgs = import <nixpkgs> { };
  pkgs = nixpkgs.pkgs;

  project = import ./default.nix { };
in
pkgs.stdenv.mkDerivation {
  name = "cmake-shell";
  buildInputs =(with pkgs; [
    cmake
    ccls
  ]);
}
