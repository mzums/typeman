{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  # Get dependencies from the main package
  inputsFrom = [(pkgs.callPackage ./default.nix {})];
  # Additional tooling
  buildInputs = with pkgs; [
    # Uncomment whatever you want/need
    # rust-analyzer # LSP Server
    # rustfmt # Formatter
    # clippy # Linter
    fontconfig
    alsa-lib
    libxkbcommon
    xorg.libXi
  ];

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];
  env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  shellHook = ''
    export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath (with pkgs; [fontconfig alsa-lib xorg.libX11 libxkbcommon xorg.libXi])}:$LD_LIBRARY_PATH
  '';
}
