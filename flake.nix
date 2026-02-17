{
  description = "CLI for browsing Helsinki area student restaurant menus from Kanttiinit.fi";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "kanttiinit";
            version = "0.3.4";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            meta = {
              description = "CLI for browsing Helsinki area student restaurant menus from Kanttiinit.fi";
              homepage = "https://github.com/otahontas/kanttiinit-cli";
              license = pkgs.lib.licenses.mit;
              mainProgram = "kanttiinit";
            };
          };
        }
      );
    };
}
