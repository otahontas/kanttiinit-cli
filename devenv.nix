{ pkgs, inputs, ... }:

let
  treefmt-nix = import inputs.treefmt-nix;
  treefmtEval = treefmt-nix.evalModule pkgs {
    projectRootFile = "devenv.nix";
    settings.global.excludes = [
      "*.lock"
      "target/"
      ".devenv/"
      ".direnv/"
    ];
    programs = {
      nixfmt.enable = true;
      prettier.enable = true;
      taplo.enable = true;
      rustfmt.enable = true;
    };
  };
in
{
  languages.rust = {
    enable = true;
    channel = "stable";
  };

  packages = [
    treefmtEval.config.build.wrapper
    pkgs.cargo-watch
    pkgs.cargo-edit
    pkgs.commitlint
    pkgs.gitleaks
  ];

  tasks = {
    "kanttiinit:build" = {
      exec = "cargo build";
    };
    "kanttiinit:lint" = {
      exec = "cargo clippy --all-targets --all-features -- -D warnings";
    };
    "kanttiinit:test" = {
      exec = "cargo test";
    };
    "kanttiinit:watch" = {
      exec = "cargo watch -x run";
    };
    "kanttiinit:fmt" = {
      exec = "treefmt";
    };
    "kanttiinit:fmt:check" = {
      exec = "treefmt --fail-on-change";
    };
  };

  git-hooks.hooks = {
    check-merge-conflicts.enable = true;
    deadnix.enable = true;
    statix.enable = true;
    typos.enable = true;

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
