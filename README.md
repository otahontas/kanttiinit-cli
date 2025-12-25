# Kanttiinit CLI

Rust port of [c++ based kanttiinit cli](https://github.com/kanttiinit/cli), a cli to browse Helsinki area student restaurant menus. Supports the same features (and couple of extra ones) as the c++ version.

## Features

- Query-based search by restaurant or area name
- Day selection (today, tomorrow, etc.)
- Course filtering by keyword
- Language support (Finnish and English)
- No external binary dependencies (curl not required)

## Installation

### From GitHub Releases (Recommended)

Download pre-built binaries for your platform from the [releases page](https://github.com/Kanttiinit/cli/releases).

### From source

```bash
git clone https://github.com/otahontas/kanttiinit-cli && \
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
  -q, --query <QUERY>          Search restaurants by restaurant or area name
  -d, --day <DAY>              Specify day offset (0=today, 1=tomorrow, etc.) [default: 0]
  -f, --filter <FILTER>        Filter courses by keyword
  -n, --number <NUMBER>        Show only n restaurants
  -v, --version                Print version
  -a, --address                Show restaurant address
  -u, --url                    Show restaurant URL
  -h, --hide-closed            Hide closed restaurants when searching for today's menus
      --set-lang <SET_LANG>    Save the preferred language (fi or en)
      --help                   Display help
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

- Rust 1.70 or later
- Cargo

### Building

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Linting and Formatting

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings
```

## Contributing

1. Fork the repository
2. Create your feature branch
3. Make your changes
4. Use conventional commits: `feat: add feature`, `fix: bug fix`, `docs: update docs`
5. Push and open a Pull Request

## Acknowledgments

- [Kanttiinit.fi](https://kanttiinit.fi) for the API
- Original C++ CLI contributors
