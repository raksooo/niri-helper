{ pkgs, lib, ... }:

with lib;

let
  name = "niri-helper";
  compilation = with pkgs; [
    gcc
    cargo
    rustc
  ];
in
{
  package = pkgs.rustPlatform.buildRustPackage {
    pname = name;
    version = "0.1.2";

    src = ./.;
    cargoLock.lockFile = ./Cargo.lock;

    meta.mainProgram = "niri-helper";
  };

  env = pkgs.mkShell {
    name = "env";
    buildInputs = compilation;
  };
}
