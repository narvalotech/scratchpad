{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.protobuf
    pkgs.abseil-cpp
    pkgs.cmake
    pkgs.sbcl
    pkgs.git
  ];

  # Where `cmake --build . --target install` puts protoc-gen-cl-pb.
  # This is the PATH entry you're modifying — change it if you install
  # to somewhere other than ~/.local.
  CL_PROTOBUFS_INSTALL_PREFIX = "${builtins.getEnv "HOME"}/.local";

  shellHook = ''
    export PATH="$CL_PROTOBUFS_INSTALL_PREFIX/bin:$PATH"
  '';
}
