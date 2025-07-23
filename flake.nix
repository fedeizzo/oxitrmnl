{
  description = "Rust webserver implementation for trmnl.";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    devshell.url = "github:numtide/devshell";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.devshell.flakeModule
      ];
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, ... }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            inputs.rust-overlay.overlays.default
          ];
        };
        packages.default = pkgs.hello;
        devshells.default = {
          env = [
            {
              name = "DATABASE_URL";
              value = "sqlite://data.db";
            }
          ];
          packages = with pkgs; [
            openssl
            pkg-config
            openapi-generator-cli
            sqlite.out
            sqlite-web
            sqlx-cli
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-analyzer" "rust-src" "rustfmt" "clippy" ];
            })

            hurl

            nodejs
            package-version-server
          ];
        };
      };
    };
}
