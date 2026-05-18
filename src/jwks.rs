//! OIDC JWKS (JSON Web Key Set) の取得とパースモジュール
//!
//! OIDC プロバイダーの JWKS エンドポイントから公開鍵セットを取得し、
//! JSON 形式でパースします。RSA および EC 鍵に対応し、
//! 各鍵の検証に必要なメタデータを保持します。

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwksResponse {
    pub keys: Vec<Jwk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jwk {
    pub kty: String,
    #[serde(rename = "use")]
    pub use_: Option<String>,
    pub kid: Option<String>,
    pub alg: Option<String>,
    pub n: Option<String>,
    pub e: Option<String>,
    pub crv: Option<String>,
    pub x: Option<String>,
    pub y: Option<String>,
}

#[derive(Debug, Error)]
pub enum JwksError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
}

pub async fn fetch_jwks(url: &str) -> Result<JwksResponse, JwksError> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;

    let jwks = response.json::<JwksResponse>().await?;

    Ok(jwks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwks_response_deserialize() {
        let json_str = r#"{
            "keys": [
                {
                    "kty": "RSA",
                    "kid": "test-key-1",
                    "n": "0vx7agoebGcQSuuPiLJXZptN",
                    "e": "AQAB",
                    "use": "sig",
                    "alg": "RS256"
                }
            ]
        }"#;

        let response: JwksResponse = serde_json::from_str(json_str).unwrap();
        assert_eq!(response.keys.len(), 1);
        assert_eq!(response.keys[0].kty, "RSA");
        assert_eq!(response.keys[0].kid, Some("test-key-1".to_string()));
        assert_eq!(response.keys[0].n, Some("0vx7agoebGcQSuuPiLJXZptN".to_string()));
        assert_eq!(response.keys[0].e, Some("AQAB".to_string()));
    }

    #[test]
    fn test_jwk_optional_fields() {
        let json_str = r#"{
            "keys": [
                {
                    "kty": "RSA",
                    "n": "0vx7agoebGcQSuuPiLJXZptN",
                    "e": "AQAB"
                }
            ]
        }"#;

        let response: JwksResponse = serde_json::from_str(json_str).unwrap();
        assert_eq!(response.keys.len(), 1);
        assert_eq!(response.keys[0].kid, None);
        assert_eq!(response.keys[0].use_, None);
        assert_eq!(response.keys[0].alg, None);
    }

    #[test]
    fn test_jwks_response_multiple_keys() {
        let json_str = r#"{
            "keys": [
                {
                    "kty": "RSA",
                    "kid": "rsa-key",
                    "n": "0vx7agoebGcQSuuPiLJXZptN",
                    "e": "AQAB"
                },
                {
                    "kty": "EC",
                    "kid": "ec-key",
                    "crv": "P-256",
                    "x": "WKn-ZIGevcwGIyyrzFoZNBdaq9_TsqzGl96oc0CWuis",
                    "y": "y77t-RvAHRKTsSGdIYUfweuOvwrvDD-Q3Hv5J0fSKbE"
                }
            ]
        }"#;

        let response: JwksResponse = serde_json::from_str(json_str).unwrap();
        assert_eq!(response.keys.len(), 2);
        assert_eq!(response.keys[0].kty, "RSA");
        assert_eq!(response.keys[1].kty, "EC");
    }

    #[test]
    fn test_jwk_use_field_rename() {
        let json_str = r#"{
            "keys": [
                {
                    "kty": "RSA",
                    "use": "sig",
                    "n": "0vx7agoebGcQSuuPiLJXZptN",
                    "e": "AQAB"
                }
            ]
        }"#;

        let response: JwksResponse = serde_json::from_str(json_str).unwrap();
        assert_eq!(response.keys[0].use_, Some("sig".to_string()));
    }
}
