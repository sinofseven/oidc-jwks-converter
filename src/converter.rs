use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use p256::{pkcs8::EncodePublicKey as P256EncodePublicKey, PublicKey as P256PublicKey};
use rsa::{BigUint, RsaPublicKey};
use thiserror::Error;

use crate::jwks::Jwk;

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Missing required field '{0}'")]
    MissingField(&'static str),
    #[error("Base64 decode failed: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),
    #[error("RSA key construction failed: {0}")]
    RsaError(#[from] rsa::errors::Error),
    #[error("EC key construction failed: {0}")]
    EcError(String),
    #[error("PEM encoding failed: {0}")]
    PemError(#[from] rsa::pkcs8::spki::Error),
    #[error("Unsupported key type: {0}")]
    UnsupportedKeyType(String),
}

pub fn convert_jwk_to_pem(jwk: &Jwk) -> Result<String, ConversionError> {
    match jwk.kty.as_str() {
        "RSA" => convert_rsa(jwk),
        "EC" => convert_ec(jwk),
        other => Err(ConversionError::UnsupportedKeyType(other.to_string())),
    }
}

fn convert_rsa(jwk: &Jwk) -> Result<String, ConversionError> {
    let n_b64 = jwk.n.as_deref().ok_or(ConversionError::MissingField("n"))?;
    let e_b64 = jwk.e.as_deref().ok_or(ConversionError::MissingField("e"))?;

    let n_bytes = URL_SAFE_NO_PAD.decode(n_b64)?;
    let e_bytes = URL_SAFE_NO_PAD.decode(e_b64)?;

    let n = BigUint::from_bytes_be(&n_bytes);
    let e = BigUint::from_bytes_be(&e_bytes);

    let public_key = RsaPublicKey::new(n, e)?;
    let pem = public_key.to_public_key_pem(rsa::pkcs8::LineEnding::LF)?;

    Ok(pem)
}

fn convert_ec(jwk: &Jwk) -> Result<String, ConversionError> {
    let x_b64 = jwk.x.as_deref().ok_or(ConversionError::MissingField("x"))?;
    let y_b64 = jwk.y.as_deref().ok_or(ConversionError::MissingField("y"))?;

    let x_bytes = URL_SAFE_NO_PAD.decode(x_b64)?;
    let y_bytes = URL_SAFE_NO_PAD.decode(y_b64)?;

    // Build uncompressed SEC1 point: 04 || x || y
    let mut sec1_bytes = vec![0x04];
    sec1_bytes.extend_from_slice(&x_bytes);
    sec1_bytes.extend_from_slice(&y_bytes);

    let public_key = P256PublicKey::from_sec1_bytes(&sec1_bytes)
        .map_err(|e| ConversionError::EcError(e.to_string()))?;

    let pem = public_key
        .to_public_key_pem(p256::pkcs8::LineEnding::LF)
        .map_err(|e| ConversionError::EcError(e.to_string()))?;

    Ok(pem)
}
