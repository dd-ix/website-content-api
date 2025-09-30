{ lib, rustPackages_1_88, ... }:

let
  inherit (rustPackages_1_88) rustPlatform;
  manifest = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  inherit (manifest) version;

  src = lib.cleanSource ./.;
  cargoLock.lockFile = ./Cargo.lock;

  cargoBuildFlags = "-p ${pname}";
  cargoTestFlags = "-p ${pname}";

  meta = {
    mainProgram = "website-content-api";
  };
}
