{ pkgs ? import ./pkgs.nix {}, ci ? false }:

with pkgs;
mkShell {
  nativeBuildInputs = [
    gitAndTools.gh
    # Rust
    rustc
    cargo
    # Deps
    pkg-config
    openssl
    fontconfig
    freetype
  ];
  # Don't set rpath for native addons
  NIX_DONT_SET_RPATH = true;
  NIX_NO_SELF_RPATH = true;
  RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
  LD_LIBRARY_PATH = lib.makeLibraryPath [ 
    openssl
    fontconfig
    freetype
  ];
  shellHook = ''

  '';
}
