# Kanttiinit CLI

[![CI](https://github.com/otahontas/kanttiinit-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/otahontas/kanttiinit-cli/actions/workflows/ci.yml)
[![Security Audit](https://github.com/otahontas/kanttiinit-cli/actions/workflows/security-audit.yml/badge.svg)](https://github.com/otahontas/kanttiinit-cli/actions/workflows/security-audit.yml)

Rust port of [c++ based kanttiinit cli](https://github.com/kanttiinit/cli), a cli to browse Helsinki area student restaurant menus. Supports the same features (and couple of extra ones) as the c++ version.

![Demo](demo.gif)

## Features

- Query-based search by restaurant or area name
- Day selection (today, tomorrow, etc.)
- Course filtering by keyword
- Language support (Finnish and English)
- No external binary dependencies (curl not required)

## Installation

### From GitHub Releases (Recommended)

Download pre-built binaries for your platform from the [releases page](https://github.com/otahontas/kanttiinit-cli/releases).

### From source

```bash
git clone https://github.com/Kanttiinit/cli && \
cd cli && \
cargo install --path .
```

## Usage

### Basic Examples

Search for restaurants in a specific area:

```bash
kanttiinit -q otaniemi
```

Search for restaurants by name:

```bash
kanttiinit -q unicafe
```

Filter menu items by keyword:

```bash
kanttiinit -q töölö -f salad
```

View menus for tomorrow:

```bash
kanttiinit -q alvari -d 1
```

### All Options

```
Options:
  -q, --query <QUERY>        Search restaurants by restaurant or area name
  -d, --day <DAY>            Specify day (0=today, 1=tomorrow, -1=yesterday, etc.) [default: 0]
  -f, --filter <FILTER>      Filter courses by keyword (case insensitive)
  -n, --head <HEAD>          Show first n restaurants
  -v, --version              Print version
  -a, --address              Show restaurant address in the output
  -u, --url                  Show restaurant URL in the output
      --hide-closed          Hide closed restaurants when searching for todays menus
      --hide-no-menu         Hide restaurants without menu when searching for todays menus
      --set-lang <SET_LANG>  Save the preferred language [possible values: fi, en]
  -h, --help                 Print help
```

### Setting Language

Set your preferred language (Finnish or English):

```bash
kanttiinit --set-lang fi
kanttiinit --set-lang en
```

The language preference is saved in `~/.config/kanttiinit/config.toml`.

## Development

### Prerequisites

This project uses [devenv](https://devenv.sh) to manage the development environment. Install [devenv](https://devenv.sh) and optionally [direnv](https://direnv.net) for automatic shell activation.

All development tools used in this repo are provided by devenv (except devenv and direnv themselves), so no extra global tool installs are needed.

### Setup

```bash
# With direnv (recommended)
direnv allow

# Without direnv
devenv shell
```

### Available tasks

List all tasks with `devenv tasks list`. Common ones:

```bash
devenv tasks run kanttiinit:build       # Build the project
devenv tasks run kanttiinit:test        # Run tests
devenv tasks run kanttiinit:lint        # Run clippy with strict warnings
devenv tasks run kanttiinit:fmt         # Format all files
devenv tasks run kanttiinit:fmt:check   # Check formatting
devenv tasks run kanttiinit:watch       # Watch for changes and run
```

### Building a release binary

```bash
cargo build --release
```

## Contributing

1. Fork the repository
2. Create your feature branch
3. Make your changes
4. Use conventional commits: `feat: add feature`, `fix: bug fix`, `docs: update docs`
5. Push and open a Pull Request

Tickets are handled with [tk](https://github.com/wedow/ticket) CLI tool.

## Acknowledgments

- [Kanttiinit.fi](https://kanttiinit.fi) for the API
- Original C++ CLI contributors
