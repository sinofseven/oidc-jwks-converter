mod converter;
mod jwks;

use clap::Parser;
use std::path::{Path, PathBuf};

/// OIDC JWKS to PEM Converter
///
/// Fetches a JWKS (JSON Web Key Set) from the given URL and generates
/// individual PEM files for each key.
///
/// EXAMPLES:
///     oidc-jwks-converter https://example.com/.well-known/openid-configuration
///
///     oidc-jwks-converter https://example.com/.well-known/openid-configuration -o ./keys
#[derive(Parser, Debug)]
#[command(name = "oidc-jwks-converter", version, about)]
struct Cli {
    /// OIDC provider's JWKS endpoint URL
    jwks_url: String,

    /// Output directory for PEM files (default: current directory)
    #[arg(short, long, value_name = "OUTPUT")]
    output: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let output_dir = cli.output.unwrap_or_else(|| PathBuf::from("."));

    if let Err(e) = run(&cli.jwks_url, &output_dir).await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

async fn run(jwks_url: &str, output_dir: &Path) -> anyhow::Result<()> {
    println!("Fetching JWKS from: {jwks_url}");

    let jwks = jwks::fetch_jwks(jwks_url).await?;

    println!("Found {} key(s)", jwks.keys.len());

    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir)?;
    }

    for jwk in &jwks.keys {
        let kid = jwk.kid.as_deref().unwrap_or("unknown");

        match converter::convert_jwk_to_pem(jwk) {
            Ok(pem) => {
                let filename = format!("{kid}.pem");
                let path = output_dir.join(&filename);
                std::fs::write(&path, &pem)?;
                println!("Saved: {}", path.display());
            }
            Err(converter::ConversionError::UnsupportedKeyType(kty)) => {
                eprintln!("Warning: Skipping key '{kid}' with unsupported type: {kty}");
            }
            Err(e) => {
                eprintln!("Warning: Failed to convert key '{kid}': {e}");
            }
        }
    }

    Ok(())
}
