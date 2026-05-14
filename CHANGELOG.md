# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-05-14

### Added
- **JWKS Fetching**: Automatically fetch JSON Web Key Set (JWKS) from OpenID Connect (OIDC) providers
- **PEM Conversion**: Convert JWKS public keys to PEM format for use with standard cryptographic libraries
- **RSA Support**: Handle RSA public keys (RSASSA-PKCS1-v1.5)
- **EC Support**: Handle Elliptic Curve public keys (P-256 only)
- **Batch Processing**: Process multiple keys from a JWKS endpoint in a single run
- **Graceful Handling**: Automatically skip unsupported key types and continue processing
- **Directory Creation**: Automatically create output directory if it doesn't exist
- **Async Support**: Non-blocking I/O using Tokio for improved performance
- **Comprehensive Testing**: 20 unit and integration tests covering core functionality

### Supported
- **Platforms**: Linux (x86_64, ARM64, ARM32), macOS (Intel & Apple Silicon), Windows
- **Key Types and Algorithms**: 
  - RSA keys: All RSA key sizes (2048-bit and higher) with any signing algorithm (RS256, RS384, RS512, etc.)
  - EC keys: P-256 curve (secp256r1) only
- **Signing Algorithms**: RS256, RS384, RS512 (RSA), ES256 (EC P-256)

### Known Limitations
- **EC Curves**: Only P-256 (secp256r1) elliptic curves are supported. Other curves (P-384, P-521, etc.) are not supported and will be skipped.
- **Unsupported Algorithms**: ES384, ES512, and other non-standard key types will be skipped during processing
- **Key Operations**: This tool is designed for public key extraction only. Private key operations are not supported.

### Installation
The release includes precompiled binaries for multiple platforms. You can also build from source:

```bash
# Build from source
cargo build --release

# The executable will be available at:
# ./target/release/oidc-jwks-converter
```

### Usage

Basic usage:

```bash
./oidc-jwks-converter <JWKS_URL> <OUTPUT_DIRECTORY>
```

Example:

```bash
./oidc-jwks-converter \
  "https://accounts.google.com/.well-known/openid-configuration/keys" \
  "./keys"
```

This will:
1. Fetch the JWKS from the provided URL
2. Extract each key and convert it to PEM format
3. Save each key as `{key_id}.pem` in the output directory

### Technical Details
- **HTTP Client**: Built with `reqwest` using rustls for secure, OpenSSL-independent communication
- **Key Processing**: Utilizes `rsa` and `p256` crates for cryptographic operations
- **Error Handling**: Comprehensive error reporting for debugging and troubleshooting
- **CLI Framework**: Built with `clap` for intuitive command-line interface

### Testing
This release includes:
- 17 unit tests for core conversion logic and JWKS parsing
- 3 integration tests for command-line interface validation
- Cross-platform verification via GitHub Actions

### Known Issues
None at this time. Please report any issues on the GitHub repository.

### Future Roadmap
Potential enhancements for future releases:
- Support for additional elliptic curves (P-384, P-521)
- Support for additional key types (OKP - Octet Key Pair)
- Batch URL processing from a configuration file
- Output format options (JWK, DER, etc.)
