{ lib, rustPlatform, ... }:

let
  manifest = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  pname = "ddix-website-content-api";
  inherit (manifest) version;

  src = lib.cleanSource ./.;
  cargoLock.lockFile = ./Cargo.lock;

  cargoBuildFlags = "-p ${manifest.name}";
  cargoTestFlags = "-p ${manifest.name}";

  meta.mainProgram = manifest.name;
}
