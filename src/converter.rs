//! RSA/EC公開鍵のPEM形式への変換モジュール
//!
//! JWK (JSON Web Key) 形式で提供される RSA または EC の公開鍵を、
//! PEM 形式に変換します。各鍵タイプに応じた変換ロジックを提供し、
//! エラーハンドリングも包含しています。

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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_rsa_jwk() -> Jwk {
        Jwk {
            kty: "RSA".to_string(),
            use_: Some("sig".to_string()),
            kid: Some("test-rsa-key".to_string()),
            alg: Some("RS256".to_string()),
            n: Some("0vx7agoebGcQSuuPiLJXZptN9nndrQmbXEps2aiAFbWhM78LhWx4cbbfAAtVT86zwu1RK7aPFFxuhDR1L6tSoc_BJECPebWKRXjBZCiFV4n3oknjhMstn64tZ_2W-5JsGY4Hc5n9yBXArwl93lqt7_RN5w6Cf0h4QyQ5v-65YGjQR0_FDW2QvzqY368QQMicAtaSqzs8KJZgnYb9c7d0zgdAZHzu6qMQvRL5hajrn1n91CbOpbISD08qNLyrdkt-bFTWhAI4vMQFh6WeZu0fM4lFd2NcRwr3XPksINHaQ-G_xBniIqbw0Ls1jF44-csFCur-kEgU8awapJzKnqDKgw".to_string()),
            e: Some("AQAB".to_string()),
            crv: None,
            x: None,
            y: None,
        }
    }

    fn create_test_ec_jwk() -> Jwk {
        Jwk {
            kty: "EC".to_string(),
            use_: Some("sig".to_string()),
            kid: Some("test-ec-key".to_string()),
            alg: Some("ES256".to_string()),
            n: None,
            e: None,
            crv: Some("P-256".to_string()),
            x: Some("WKn-ZIGevcwGIyyrzFoZNBdaq9_TsqzGl96oc0CWuis".to_string()),
            y: Some("y77t-RvAHRKTsSGdIYUfweuOvwrvDD-Q3Hv5J0fSKbE".to_string()),
        }
    }

    #[test]
    fn test_convert_rsa_valid() {
        let jwk = create_test_rsa_jwk();
        let result = convert_rsa(&jwk);
        assert!(result.is_ok());
        let pem = result.unwrap();
        assert!(pem.contains("-----BEGIN PUBLIC KEY-----"));
        assert!(pem.contains("-----END PUBLIC KEY-----"));
    }

    #[test]
    fn test_convert_rsa_missing_n() {
        let mut jwk = create_test_rsa_jwk();
        jwk.n = None;
        let result = convert_rsa(&jwk);
        assert!(result.is_err());
        match result {
            Err(ConversionError::MissingField("n")) => {},
            _ => panic!("Expected MissingField('n')"),
        }
    }

    #[test]
    fn test_convert_rsa_missing_e() {
        let mut jwk = create_test_rsa_jwk();
        jwk.e = None;
        let result = convert_rsa(&jwk);
        assert!(result.is_err());
        match result {
            Err(ConversionError::MissingField("e")) => {},
            _ => panic!("Expected MissingField('e')"),
        }
    }

    #[test]
    fn test_convert_rsa_invalid_base64() {
        let mut jwk = create_test_rsa_jwk();
        jwk.n = Some("!!!invalid base64!!!".to_string());
        let result = convert_rsa(&jwk);
        assert!(result.is_err());
        match result {
            Err(ConversionError::Base64DecodeError(_)) => {},
            _ => panic!("Expected Base64DecodeError"),
        }
    }

    #[test]
    fn test_convert_ec_valid() {
        let jwk = create_test_ec_jwk();
        let result = convert_ec(&jwk);
        assert!(result.is_ok());
        let pem = result.unwrap();
        assert!(pem.contains("-----BEGIN PUBLIC KEY-----"));
        assert!(pem.contains("-----END PUBLIC KEY-----"));
    }

    #[test]
    fn test_convert_ec_missing_x() {
        let mut jwk = create_test_ec_jwk();
        jwk.x = None;
        let result = convert_ec(&jwk);
        assert!(result.is_err());
        match result {
            Err(ConversionError::MissingField("x")) => {},
            _ => panic!("Expected MissingField('x')"),
        }
    }

    #[test]
    fn test_convert_ec_missing_y() {
        let mut jwk = create_test_ec_jwk();
        jwk.y = None;
        let result = convert_ec(&jwk);
        assert!(result.is_err());
        match result {
            Err(ConversionError::MissingField("y")) => {},
            _ => panic!("Expected MissingField('y')"),
        }
    }

    #[test]
    fn test_convert_jwk_to_pem_rsa() {
        let jwk = create_test_rsa_jwk();
        let result = convert_jwk_to_pem(&jwk);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_jwk_to_pem_ec() {
        let jwk = create_test_ec_jwk();
        let result = convert_jwk_to_pem(&jwk);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_jwk_to_pem_unsupported_type() {
        let mut jwk = create_test_rsa_jwk();
        jwk.kty = "UNSUPPORTED".to_string();
        let result = convert_jwk_to_pem(&jwk);
        assert!(result.is_err());
        match result {
            Err(ConversionError::UnsupportedKeyType(kty)) => {
                assert_eq!(kty, "UNSUPPORTED");
            },
            _ => panic!("Expected UnsupportedKeyType"),
        }
    }
}
