{ pkgs ? import <nixpkgs> {} }:
with pkgs;
let
  shell = pkgs.stdenv.mkDerivation {
    name = "shell";
    buildInputs = [
      qt5.full
      qt5.qtcharts
      qt5.qttools
      qtcreator
      gst_all_1.gstreamer
      gst_all_1.gst-plugins-good
      gst_all_1.gst-plugins-bad
      gst_all_1.gst-plugins-ugly
      gst_all_1.gst-editing-services
      glib-networking
      pkgconfig
      clang
      gtk3
      ncurses
      #glibc
      cmake
      rustup
    ];
  shellHook = ''
    export XDG_DATA_DIRS=${gsettings-desktop-schemas}/share/gsettings-schemas/${gsettings-desktop-schemas.name}:${gtk3}/share/gsettings-schemas/${gtk3.name}:$XDG_DATA_DIRS
    export LIBCLANG_PATH="${pkgs.llvmPackages.libclang}/lib";
  '';

  };
in shell
