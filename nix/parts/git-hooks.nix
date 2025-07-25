{ ... }:

{
  pre-commit = {
    check.enable = true;

    settings = {
      addGcRoot = true;

      hooks = {
        check-json.enable = true;
        check-toml.enable = true;
        rustfmt.enable = true; # currently broken because rustfmt is already bundled with rust-analyzer
      };
    };
  };
}
