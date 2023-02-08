{
	description = "The Mist Programming Language";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

		ocaml-overlay = {
			url = "github:nix-ocaml/nix-overlays";
			inputs.nixpkgs.follows = "nixpkgs";
		};
		rust-overlay = {
			url = "github:oxalica/rust-overlay";
			inputs.nixpkgs.follows = "nixpkgs";
		};

		flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = inputs @ {
		nixpkgs,
		ocaml-overlay,
		rust-overlay,
		flake-utils,
		...
	}: flake-utils.lib.eachDefaultSystem (
		system:
			let
				pkgs = import nixpkgs {
					inherit system;
					overlays = [
						ocaml-overlay.overlays.${system}
						(import rust-overlay)
					];
				};
			in with pkgs; {
				devShells.default = mkShell {
					buildInputs = [
						dune_3

						(rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
						rust-analyzer
					] ++ (with ocaml-ng.ocamlPackages_5_0; [
						findlib
						menhir
						ocaml
						ocaml-lsp

						base
						zarith
					]);
				};
			}
	);
}
