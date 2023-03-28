{...}: {
  languages = {
    rust = {
      enable = true;
      version = "latest";
    };
  };

  pre-commit.hooks = {
    alejandra.enable = true;
    cargo-check.enable = true;
    clippy.enable = true;
    editorconfig-checker.enable = true;
    markdownlint.enable = true;
    yamllint.enable = true;
  };
}
