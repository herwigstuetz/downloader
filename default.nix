{ nixpkgs ? import <nixpkgs> { } }:
let
  inherit (nixpkgs) pkgs;
  project = pkgs.callPackage ./derivation.nix { };
in
project
