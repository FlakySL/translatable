{
  description = "Flake configuration file for translatable development.";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    fenix.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, fenix, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        crane = inputs.crane.mkLib pkgs;

        toolchain = with fenix.packages.${system};
          combine [
            stable.rustc
            stable.cargo
            stable.rust-src
            complete.rustfmt
            stable.clippy
            stable.rust-analyzer
          ];

        craneLib = crane.overrideToolchain toolchain;
      in {
        devShells.default = craneLib.devShell {
          packages = with pkgs; [ toolchain gnumake ];

          env = { LAZYVIM_RUST_DIAGNOSTICS = "bacon-ls"; };
        };
      });
}
