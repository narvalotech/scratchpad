{
  pkgs,
  lib,
  config,
  ...
}:
{
  # Add the MUSL target to your Rust toolchain
  languages = {
    rust = {
      enable = true;
      targets = [ "aarch64-unknown-linux-musl" ];
      channel = "stable";
    };
  };

  # Pull in the aarch64 MUSL cross-compiler toolchain
  packages = [
    pkgs.pkgsCross.aarch64-multiplatform-musl.stdenv.cc
  ];

  # Wire up CC and CARGO linker for the musl target
  env = {
    CC_aarch64_unknown_linux_musl = "${pkgs.pkgsCross.aarch64-multiplatform-musl.stdenv.cc}/bin/aarch64-unknown-linux-musl-gcc";
    CXX_aarch64_unknown_linux_musl = "${pkgs.pkgsCross.aarch64-multiplatform-musl.stdenv.cc}/bin/aarch64-unknown-linux-musl-g++";
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER = "${pkgs.pkgsCross.aarch64-multiplatform-musl.stdenv.cc}/bin/aarch64-unknown-linux-musl-gcc";
  };

  scripts = {
    build-static = {
      exec = "cargo build --target aarch64-unknown-linux-musl --release";
      description = "Build a static release binary for ARM64 Linux";
    };
  };
}
