{ pkgs, inputs, lib, config, ... }:

let
  flutterWritableRoot = config.devenv.dotfile + "/flutter-sdk";
in
{
  packages = with pkgs; [
    git
    jdk17
    gradle
  ];

  languages.dart = {
    enable = true;
    package = pkgs.dart;
  };

  env.ANDROID_EMULATOR_HOME = "/home/jon/.android";
  env.ANDROID_AVD_HOME = "/home/jon/.android/avd";
  env.GRADLE_USER_HOME = config.devenv.dotfile + "/.gradle";
  env.CHROME_EXECUTABLE = "firefox";

  # devenv's android module points FLUTTER_ROOT straight into the read-only
  # /nix/store, which breaks the flutter-plugin-loader composite build.
  # Override it with a writable copy so Gradle can create its .gradle metadata.
  env.FLUTTER_ROOT = lib.mkForce flutterWritableRoot;

  enterShell = ''
    if [ ! -x "${flutterWritableRoot}/bin/flutter" ]; then
      echo "Setting up a writable Flutter SDK copy (one-time)..."
      rm -rf "${flutterWritableRoot}"
      mkdir -p "$(dirname "${flutterWritableRoot}")"
      cp -r --no-preserve=ownership "${config.android.flutter.package}" "${flutterWritableRoot}"
      chmod -R u+w "${flutterWritableRoot}"
    fi
  '';

  android = {
    enable = true;
    flutter.enable = true;   # back to default nixpkgs flutter (3.44.x) — matches your pubspec
    platforms.version = [ "36" "35" ];
    buildTools.version = [ "36.0.0" "35.0.0" ];
    ndk.enable = true;
    googleAPIs.enable = true;
  };
}
