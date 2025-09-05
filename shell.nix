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
  ];

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];
  env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
