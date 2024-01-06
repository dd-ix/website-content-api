{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-23.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = (import nixpkgs) {
            inherit system;
          };
        in
        {
          packages = rec {
            foundation = pkgs.callPackage ./derivation.nix {
              cargoToml = ./Cargo.toml;
            };
            default = foundation;
          };
        }
      ) // {
      overlays.default = _: prev: {
        foundation = self.packages."${prev.system}".foundation;
      };

      nixosModules = rec {
        foundation = import ./nixos-module;
        default = foundation;
      };
    };
}
