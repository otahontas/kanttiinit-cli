{ pkgs, ... }:
{
  languages.rust = {
    enable = true;
    channel = "stable";
  };

  devenv-base.treefmt = {
    programs = {
      taplo.enable = true;
      rustfmt.enable = true;
    };
  };

  packages = [
    pkgs.cargo-edit
    pkgs.cargo-watch
  ];

  enterShell = ''
    echo "kanttiinit-cli dev shell. Run 'devenv tasks list' to see available tasks."
  '';

  enterTest = ''
    cargo --version
    rustc --version
    treefmt --version

    command -v cargo-watch
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
