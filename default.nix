{pkgs ? import <nixpkgs> {}}: let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
  pkgs.rustPlatform.buildRustPackage rec {
    pname = manifest.name;
    version = manifest.version;
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;
    nativeBuildInputs = with pkgs; [
      pkg-config
      writableTmpDirAsHomeHook
    ];
    buildInputs = with pkgs; [
      fontconfig
      alsa-lib
      libx11
      libxkbcommon
    ];
  }
