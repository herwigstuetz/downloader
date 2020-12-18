{ rustPlatform, pkg-config, openssl }:
# See https://eipi.xyz/blog/packaging-a-rust-project-for-nix/ for more information
rustPlatform.buildRustPackage rec {
  name = "rust-project-${version}";
  version = "0.0.1";
  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];
  src = ./.;
  cargoSha256 = "1bw3whndydxn7i8gb0w659flm4bb17l1bggkn9y7x7145n9x80fa";
}
