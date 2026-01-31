{ pkgs ? import <nixpkgs> { } }:

let
  dlopenLibraries = with pkgs; [
    libxkbcommon

    pkg-config
    dbus
  ];

in pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rustup
    rust-analyzer
    dbus
  ];

  # additional libraries that your project
  # links to at build time, e.g. OpenSSL
  buildInputs = [dlopenLibraries];
}
