# Kanttiinit CLI

Rust port of [c++ based kanttiinit cli](https://github.com/kanttiinit/cli), a cli to browse Helsinki area student restaurant menus. Supports the same features (and couple of extra ones) as the c++ version.

## Features

- Query-based search by restaurant or area name
- Geolocation search using OpenStreetMap (so no api key needed)
- Day selection (today, tomorrow, etc.)
- Course filtering by keyword
- Distance display for geolocation searches
- language support (Finnish and English)
- No external binary dependencies (curl not required)

## Installation

### Using Homebrew (macOS/Linux)

```bash
# Add the tap
brew tap otahontas/kanttiinit-cli

# Install kanttiinit
brew install kanttiinit
```

To upgrade to the latest version:
```bash
brew update
brew upgrade kanttiinit
```

### From GitHub Releases

Download pre-built binaries for your platform from the [releases page](https://github.com/otahontas/kanttiinit-cli/releases).

### From source

```bash
git clone https://github.com/otahontas/kanttiinit-cli && \
cd kanttiinit-cli && \
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

Find restaurants near a location:
```bash
kanttiinit -g "Otakaari 8"
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
  -g, --geo <GEO>              Search restaurants by location
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
- [OpenStreetMap Nominatim](https://nominatim.openstreetmap.org/) for free geocoding services
- Original C++ CLI contributors
