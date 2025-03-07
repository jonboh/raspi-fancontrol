{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    supportedSystems = ["x86_64-linux" "aarch64-linux"];
    forEachSupportedSystem = f:
      nixpkgs.lib.genAttrs supportedSystems (system:
        f {
          pkgs = import nixpkgs {
            inherit system;
            overlays = [rust-overlay.overlays.default self.overlays.default];
          };
          pkgs-cross = import nixpkgs {
            inherit system;
            crossSystem = {
              config = "aarch64-unknown-linux-gnu";
            };
            overlays = [rust-overlay.overlays.default self.overlays.default];
          };
        });
  in {
    overlays.default = final: prev: {
      rustToolchain = final.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
    };

    devShells = forEachSupportedSystem ({
      pkgs,
      pkgs-cross,
    }: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          pkgs-cross.rustToolchain
          cargo-deny
          rust-analyzer
          bacon
        ];

        env = {
          # Required by rust-analyzer
          RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
        };
      };
    });
    packages = forEachSupportedSystem ({pkgs, ...}: {
      rp-fancontrol = import ./default.nix {rustPlatform = pkgs.rustPlatform;};
    });
  };
}
