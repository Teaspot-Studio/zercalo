with import ./nix/pkgs.nix {};

stdenv.mkDerivation rec {
  name = "zercalo-rust-env";
  env = buildEnv { name = name; paths = buildInputs; };

  buildInputs = [
    rustup
    rust-analyzer
    SDL2
    SDL2_image
    libGL
    xorg.libX11
    xorg.libXi
    xorg.libXinerama
    xorg.libXext
    xorg.libXcursor
    xorg.libXrandr
    pkgs.vulkan-loader
  ];

  APPEND_LIBRARY_PATH = lib.makeLibraryPath [
    libGL
    xorg.libX11
    xorg.libXi
    xorg.libXinerama
    xorg.libXext
    xorg.libXcursor
    xorg.libXrandr
    pkgs.vulkan-loader
  ];

  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$APPEND_LIBRARY_PATH"
  '';
}
