{
	description = "The Mist Programming Language";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

		rust-overlay = {
			url = "github:oxalica/rust-overlay";
			inputs.nixpkgs.follows = "nixpkgs";
		};

		flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = inputs @ {
		nixpkgs,
		rust-overlay,
		flake-utils,
		...
	}: flake-utils.lib.eachDefaultSystem (
		system:
			let
				pkgs = import nixpkgs {
					inherit system;
					overlays = [
						(import rust-overlay)
					];
				};
			in with pkgs; {
				devShells.default = mkShell {
					buildInputs = [
						(rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
						rust-analyzer
					];
				};
			}
	);
}
