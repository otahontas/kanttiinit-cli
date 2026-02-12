{ pkgs, ... }:

{
  packages = [
    pkgs.cargo-edit
    pkgs.cargo-watch
  ];

  languages.rust.enable = true;
}
