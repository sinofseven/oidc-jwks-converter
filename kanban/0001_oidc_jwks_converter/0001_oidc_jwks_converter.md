# OIDC JWKS PEM 変換ツールの実装

## 目的

JWT Tokenの検証に使用するためにpemファイルが欲しい。しかしJWKSのURLしかわかっていないのでpemファイルを生成してほしい。

## 要望

OIDCのJWKSのurlを受け取り、各キーのpemファイル (`<key id>.pem`)を生成するCLI Toolを書いて欲しい。

## プラン

1. Cargo.toml に依存クレート追加（serde, reqwest, tokio, rsa, p256, base64, clap, thiserror, anyhow）
2. `src/jwks.rs`: JWKS取得・構造体定義
3. `src/converter.rs`: JWK→PEM変換（RSA/EC対応）
4. `src/main.rs`: CLI引数処理・メインロジック

## 完了サマリー

**完了日時**: 2026-05-14T16:36:33+09:00

- OIDC JWKS URL からRSA/EC公開鍵をPEMに変換するCLIツールを実装
- `cargo run -- <JWKS_URL>` でカレントディレクトリに `<kid>.pem` を出力
- `-o/--output` オプションで出力ディレクトリ指定可能
- `--help` でヘルプメッセージ表示
- Google の JWKS エンドポイント（`https://www.googleapis.com/oauth2/v3/certs`）で動作確認済み
- clippy 警告ゼロ
