{
  description = "Linkdoku";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    (flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust-toolchain = (pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" "x86_64-unknown-linux-musl" ];
        });
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rust-toolchain;
          rustc = rust-toolchain;
        };
        linkdoku = rustPlatform.buildRustPackage {
          pname = "linkdoku";
          version = "git";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "diesel_async_migrations-0.10.0" =
                "sha256-iGiGatNBcPCteHw5HyzB/SOMopmO110jlRQeW0It1Gw=";
              "lz-str-0.1.0" =
                "sha256-17BMDZhV+3G92u7N26SeIQZuXUXH2JfOOLu4adZFWOE=";
            };

          };

          nativeBuildInputs = with pkgs; [ trunk wasm-bindgen-cli binaryen ];
          buildPhase = ''
            make release Q=
          '';
          installPhase = ''
            make install DESTDIR="$out" Q=
          '';
        };
      in {
        packages = {
          inherit linkdoku;
          default = linkdoku;
        };
      }));
}
