# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-02-12

### Added

- Hide restaurants without menu (`--hide-no-menu`) option

### Removed

- Geolocation-based search (`-g`, `--geo`) — area search via `-q` is sufficient

## [0.1.0] - 2025-11-13

### Added

- Initial release of Kanttiinit CLI in Rust
- Query-based restaurant search (`-q`, `--query`)
- Geolocation-based restaurant search (`-g`, `--geo`) using OpenStreetMap's Nominatim API
- Day offset support (`-d`, `--day`) to view menus for different days
- Course filtering by keyword (`-f`, `--filter`)
- Limit number of restaurants displayed (`-n`, `--number`)
- Display restaurant addresses (`-a`, `--address`)
- Display restaurant URLs (`-u`, `--url`)
- Hide closed restaurants option (`-h`, `--hide-closed`)
- Language preference setting (`--set-lang`) with support for Finnish (fi) and English (en)
- Colorized terminal output with opening hours countdown
- Distance display for geolocation-based searches
- Comprehensive test coverage (unit and integration tests)
- Linting and formatting configuration (rustfmt, clippy)

### Changed

- Migrated from C++ to Rust for better memory safety and modern tooling
- Replaced curl dependency with native Rust HTTP client (ureq)
- Switched from Google Maps API to OpenStreetMap Nominatim API (no API key required)

[0.2.0]: https://github.com/Kanttiinit/cli/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Kanttiinit/cli/releases/tag/v0.1.0
