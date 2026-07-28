 {
  pkgs,
  lib,
  config,
  ...
}:
{
  # https://devenv.sh/packages/
  packages = [
    pkgs.flutter
  ];

  # https://devenv.sh/languages/
  languages = {
    dart.enable = true;
  };

  # See full reference at https://devenv.sh/reference/options/
}
