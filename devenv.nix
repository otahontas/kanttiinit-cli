{ pkgs, inputs, ... }:

let
  treefmt-nix = import inputs.treefmt-nix;
  treefmtEval = treefmt-nix.evalModule pkgs {
    projectRootFile = "devenv.nix";
    settings.global.excludes = [
      "*.lock"
      "target/"
      ".devenv*"
      ".direnv/"
    ];
    programs = {
      nixfmt.enable = true;
      prettier.enable = true;
      taplo.enable = true;
      rustfmt.enable = true;
    };
  };

  # Ticket CLI tool
  tk = pkgs.stdenv.mkDerivation {
    name = "tk";
    version = "master";
    src = pkgs.fetchurl {
      url = "https://raw.githubusercontent.com/wedow/ticket/master/ticket";
      sha256 = "a8e825ac2a18b1360d8cd0ba8c65d82099752ec93cfb5b80584885dd791cfc92";
    };
    dontUnpack = true;
    installPhase = ''
      install -Dm755 $src $out/bin/tk
    '';
  };
in
{
  languages.rust = {
    enable = true;
    channel = "stable";
  };

  packages = [
    treefmtEval.config.build.wrapper
    pkgs.cargo-edit
    pkgs.cargo-watch
    pkgs.commitlint
    pkgs.git
    pkgs.gitleaks
    tk
  ];

  enterShell = ''
    echo "kanttiinit-cli dev shell. Run 'devenv tasks list' to see available tasks."
  '';

  enterTest = ''
    cargo --version
    rustc --version
    treefmt --version

    command -v cargo-watch
    command -v commitlint
    command -v gitleaks
    command -v deadnix
    command -v statix
    command -v typos
    command -v tk
    command -v git
  '';

  tasks = {
    "kanttiinit:build" = {
      description = "Build the project";
      exec = "cargo build";
    };
    "kanttiinit:lint" = {
      description = "Run clippy with strict warnings";
      exec = "cargo clippy --all-targets --all-features -- -D warnings";
    };
    "kanttiinit:test" = {
      description = "Run tests";
      exec = "cargo test";
    };
    "kanttiinit:watch" = {
      description = "Watch for changes and run";
      exec = "cargo watch -x run";
    };
    "kanttiinit:fmt" = {
      description = "Format all files";
      exec = "treefmt";
    };
    "kanttiinit:fmt:check" = {
      description = "Check formatting without modifying files";
      exec = "treefmt --fail-on-change";
    };
  };

  git-hooks.hooks = {
    check-merge-conflicts.enable = true;
    deadnix.enable = true;
    detect-private-keys.enable = true;
    typos.enable = true;

    statix = {
      enable = true;
      entry = "${pkgs.statix}/bin/statix check --format errfmt --ignore .devenv,.devenv.*,.direnv,target .";
      pass_filenames = false;
    };

    treefmt = {
      enable = true;
      package = treefmtEval.config.build.wrapper;
    };

    commitlint = {
      enable = true;
      stages = [ "commit-msg" ];
      entry = "${pkgs.commitlint}/bin/commitlint --extends @commitlint/config-conventional --edit";
    };

    gitleaks = {
      enable = true;
      entry = "${pkgs.gitleaks}/bin/gitleaks protect --staged --verbose";
    };

    clippy = {
      enable = true;
      entry = "cargo clippy --all-targets --all-features -- -D warnings";
      pass_filenames = false;
    };

    cargo-test = {
      enable = true;
      name = "cargo-test";
      entry = "cargo test";
      stages = [ "pre-push" ];
      pass_filenames = false;
    };
  };
}
