{ pkgs, config, lib, ... }:

{
  devshells.default = {
    commands = [
      {
        help = "Generate openapi spec";
        name = "openapi-gen";
        command = ''
          ${lib.getExe pkgs.openapi-generator-cli} generate -g rust-axum -i openapi.json -o openapi -c openapi-generator-config.yaml
        '';
      }
    ];
    env = [
      {
        name = "DATABASE_URL";
        value = "sqlite://data.db";
      }
    ];
    packages = with pkgs; [
      openssl
      pkg-config
      sqlite.out
      sqlite-web
      sqlx-cli
      (rust-bin.stable.latest.default.override {
        extensions = [ "rust-analyzer" "rust-src" "rustfmt" "clippy" ];
      })

      hurl

      nodejs
      package-version-server

      pre-commit
    ];
    devshell.startup.pre-commit-hooks.text = config.pre-commit.installationScript;
  };
}
