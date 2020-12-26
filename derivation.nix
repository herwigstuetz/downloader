{ rustPlatform, pkg-config, openssl }:
# See https://eipi.xyz/blog/packaging-a-rust-project-for-nix/ for more information
rustPlatform.buildRustPackage rec {
  name = "rust-project-${version}";
  version = "0.0.1";
  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];
  src = ./.;
  cargoSha256 = "070f32n51dnh2rfykgwnyzd6wm52h7qwrfsiinx3yc9gkdgxr13p";
}
