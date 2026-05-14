[English](README.md) | 日本語

# oidc-jwks-converter

OIDC (OpenID Connect) の JWKS (JSON Web Key Set) を取得して、PEM フォーマットに変換するコマンドラインツールです。

## 概要

OpenID Connect プロバイダーの JWKS エンドポイントから公開鍵情報を取得し、RSA および EC (Elliptic Curve) の暗号化方式に対応した個別のPEMファイルを生成します。各鍵には Key ID (kid) をファイル名として使用します。

## 機能

- **JWKS 自動取得**: OpenID Connect プロバイダーから JWKS を取得
- **複数の暗号方式対応**: RSA および EC (P-256) の鍵を処理
- **バッチ処理**: 複数の鍵を一括で PEM フォーマットに変換
- **エラーハンドリング**: サポートされていない鍵タイプはスキップし、処理を継続

## インストール

### 前提条件

- Rust 1.56 以上

### ビルド

```bash
cargo build --release
```

実行可能ファイルは `target/release/oidc-jwks-converter` に生成されます。

## 使用方法

### 基本的な使い方

```bash
oidc-jwks-converter <JWKS_URL>
```

JWKS URL を指定して実行します。各鍵は `{key_id}.pem` というファイル名で現在のディレクトリに保存されます。

### 出力ディレクトリの指定

```bash
oidc-jwks-converter <JWKS_URL> -o ./keys
oidc-jwks-converter <JWKS_URL> --output /path/to/keys
```

`-o` または `--output` オプションで出力先ディレクトリを指定できます。ディレクトリが存在しない場合は自動的に作成されます。

### 使用例

```bash
# Google の JWKS から鍵を取得（例）
oidc-jwks-converter https://www.googleapis.com/oauth2/v3/certs -o ./google_keys

# 出力例
# Fetching JWKS from: https://www.googleapis.com/oauth2/v3/certs
# Found 2 key(s)
# Saved: ./google_keys/key_id_1.pem
# Saved: ./google_keys/key_id_2.pem
```

## 対応フォーマット

### サポートキータイプと署名アルゴリズム

| キータイプ | 署名アルゴリズム | 説明 |
|-----------|-----------------|------|
| RSA | RS256, RS384, RS512 | RSA-PSS による署名 |
| EC | ES256, ES384, ES512 | ECDSA による署名 |

### 出力形式

生成されるPEMファイルはテキスト形式の標準的な PEM エンコーディング（PKCS#8）です。

例：
```
-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...
-----END PUBLIC KEY-----
```

## 開発

### コマンド

```bash
# ビルド
cargo build

# リリースビルド
cargo build --release

# テスト実行
cargo test

# コードフォーマット
cargo fmt

# フォーマットチェック
cargo fmt --check

# Lint
cargo clippy
```

### プロジェクト構成

- `src/main.rs`: CLIエントリーポイント
- `src/jwks.rs`: JWKS 取得ロジック
- `src/converter.rs`: 鍵変換ロジック
