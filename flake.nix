{
  description = "The Mist Programming Language";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    devenv = {
      url = "github:cachix/devenv";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ {
    devenv,
    flake-parts,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        devenv.flakeModule
      ];

      systems = [
        "aarch64-darwin"
      ];

      perSystem = _: {
        devenv.shells.default = {
          languages.rust = {
            enable = true;
            version = "latest";
          };

          pre-commit.hooks = {
            alejandra.enable = true;
            cargo-check.enable = true;
            clippy.enable = true;
            editorconfig-checker.enable = true;
            markdownlint.enable = true;
          };
        };
      };
    };
}
