{ rustPlatform }:
# See https://eipi.xyz/blog/packaging-a-rust-project-for-nix/ for more information
rustPlatform.buildRustPackage rec {
  name = "rust-project-${version}";
  version = "0.0.1";
  buildInputs = [ ];
  src = ./.;
  cargoSha256 = "1h4aibmbpz3hrkg0hh8bfw6268yq4nl9p9cdq1j9pyq1ylkmzghg";
}
