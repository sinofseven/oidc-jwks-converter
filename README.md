English | [日本語](README_ja.md)

# oidc-jwks-converter

A command-line tool to fetch OIDC (OpenID Connect) JWKS (JSON Web Key Set) and convert it to PEM format.

## Overview

Retrieves public key information from an OpenID Connect provider's JWKS endpoint and generates individual PEM files in formats supporting RSA and EC (Elliptic Curve) cryptographic methods. Each key uses its Key ID (kid) as the filename.

## Features

- **Automatic JWKS Fetching**: Retrieve JWKS from OpenID Connect providers
- **Multiple Cryptographic Methods**: Handle RSA and EC (P-256) keys
- **Batch Processing**: Convert multiple keys to PEM format at once
- **Error Handling**: Skip unsupported key types and continue processing

## Installation

### Prerequisites

- Rust 1.56 or later

### Build

```bash
cargo build --release
```

The executable will be generated at `target/release/oidc-jwks-converter`.

## Usage

### Basic Usage

```bash
oidc-jwks-converter <JWKS_URL>
```

Specify a JWKS URL to run. Each key will be saved as `{key_id}.pem` in the current directory.

### Specifying Output Directory

```bash
oidc-jwks-converter <JWKS_URL> -o ./keys
oidc-jwks-converter <JWKS_URL> --output /path/to/keys
```

Use the `-o` or `--output` option to specify the output directory. The directory will be created automatically if it doesn't exist.

### Example

```bash
# Fetch keys from Google's JWKS (example)
oidc-jwks-converter https://www.googleapis.com/oauth2/v3/certs -o ./google_keys

# Output example
# Fetching JWKS from: https://www.googleapis.com/oauth2/v3/certs
# Found 2 key(s)
# Saved: ./google_keys/key_id_1.pem
# Saved: ./google_keys/key_id_2.pem
```

## Supported Formats

### Supported Key Types and Signing Algorithms

| Key Type | Signing Algorithm | Description |
|----------|-------------------|-------------|
| RSA | RS256, RS384, RS512 | RSA-PSS signature |
| EC | ES256, ES384, ES512 | ECDSA signature |

### Output Format

Generated PEM files are in standard text PEM encoding (PKCS#8) format.

Example:
```
-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...
-----END PUBLIC KEY-----
```

## Development

### Commands

```bash
# Build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Lint
cargo clippy
```

### Project Structure

- `src/main.rs`: CLI entry point
- `src/jwks.rs`: JWKS fetching logic
- `src/converter.rs`: Key conversion logic

## License

This project is licensed under the [MIT License](LICENSE).
