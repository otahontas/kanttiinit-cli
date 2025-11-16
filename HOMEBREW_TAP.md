# Homebrew Tap Guide

This repository doubles as a Homebrew tap, allowing users to easily install `kanttiinit` via Homebrew.

## For Users

### Installing

```bash
brew tap otahontas/kanttiinit-cli
brew install kanttiinit
```

### Upgrading

```bash
brew update
brew upgrade kanttiinit
```

### Uninstalling

```bash
brew uninstall kanttiinit
brew untap otahontas/kanttiinit-cli
```

## For Maintainers

### Updating the Formula

When releasing a new version, the Homebrew formula needs to be updated:

1. **Create a new release** with a version tag (e.g., `v0.1.0`)

2. **Calculate the SHA256 checksum** of the source tarball:
   ```bash
   # Replace VERSION with the actual version number
   VERSION=0.1.0
   curl -L https://github.com/otahontas/kanttiinit-cli/archive/refs/tags/v${VERSION}.tar.gz | shasum -a 256
   ```

3. **Update the formula** (`Formula/kanttiinit.rb`):
   - Update the `url` to point to the new version tag
   - Update the `sha256` with the checksum from step 2
   - Update the version number if it changed

4. **Test the formula locally**:
   ```bash
   brew install --build-from-source Formula/kanttiinit.rb
   brew test kanttiinit
   ```

5. **Commit and push the changes**

### Formula Structure

The formula uses the standard Homebrew formula structure:
- Builds from source using Cargo
- Requires Rust as a build dependency
- Installs the binary to the Homebrew cellar
- Includes a basic version test

### Testing

After updating the formula, test it:

```bash
# Install from the updated formula
brew reinstall kanttiinit

# Verify the version
kanttiinit --version

# Run the formula test
brew test kanttiinit
```
