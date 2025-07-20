{
  description = "Rust webserver implementation for trmnl.";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ ];
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, ... }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            inputs.rust-overlay.overlays.default
          ];
        };
        packages.default = pkgs.hello;
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            openssl
            pkg-config
            openapi-generator-cli
            sqlite.out
            sqlite-web
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-analyzer" "rust-src" "rustfmt" "clippy" ];
            })

            nodejs
          ];
        };
      };
    };
}
