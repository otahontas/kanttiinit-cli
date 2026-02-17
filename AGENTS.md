## Git

- Push directly to main — no branches or pull requests needed
- CI runs tests and handles releases automatically
- For each new release version, keep version fields in sync: update `Cargo.toml` and `flake.nix` (and `CHANGELOG.md`).

## Tools

- Use `devenv` (and `direnv` when enabled) for repo commands and tooling.
- Tickets are handled with `tk` CLI tool.
- `done` is not a valid ticket status in `tk`; use `closed` instead.
