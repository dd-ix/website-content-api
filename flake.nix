{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-24.05";
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
        inherit (self.packages."${prev.system}") website-content-api;
      };

      nixosModules = rec {
        website-content-api = import ./module.nix;
        default = website-content-api;
      };
    };
}
