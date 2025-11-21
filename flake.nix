{
  inputs = {
    nixpkgs.url = "github:NuschtOS/nuschtpkgs/backports-25.11";
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
            ddix-website-content-api = pkgs.callPackage ./package.nix { };
            default = ddix-website-content-api;
          };
        }
      ) // {
      overlays.default = _: prev: {
        inherit (self.packages."${prev.system}") ddix-website-content-api;
      };

      nixosModules = rec {
        ddix-website-content-api = ./module.nix;
        default = ddix-website-content-api;
      };
    };
}
