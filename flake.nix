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
            website-content-api = pkgs.callPackage ./derivation.nix {
              cargoToml = ./Cargo.toml;
            };
            default = website-content-api;
          };
        }
      ) // {
      overlays.default = _: prev: {
        website-content-api = self.packages."${prev.system}".website-content-api;
      };

      nixosModules = rec {
        website-content-api = import ./nixos-module;
        default = website-content-api;
      };
    };
}
