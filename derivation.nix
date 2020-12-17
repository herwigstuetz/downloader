{ rustPlatform, pkg-config, openssl }:
# See https://eipi.xyz/blog/packaging-a-rust-project-for-nix/ for more information
rustPlatform.buildRustPackage rec {
  name = "rust-project-${version}";
  version = "0.0.1";
  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];
  src = ./.;
  cargoSha256 = "1g3v4vpyqr5gw9gakb0acfqisyfv1lb9qslhck999a3qzq583j49";
}
