# GitHub Repository About 設定ガイド

このドキュメントは、GitHub リポジトリの「About」セクションに表示される説明文に関するガイドです。

## GitHub About について

GitHub リポジトリページの右側には「About」セクションが表示されます。ここに設定されたテキストは、リポジトリの目的を訪問者に簡潔に伝える重要な役割を果たします。

## 推奨される About 説明文

### 案 A: 最短版（推奨）
```
Fetch OIDC provider public keys and convert them to PEM format
```

**用途**: GitHub リポジトリの About として最適  
**文字数**: 約 65 文字  
**特徴**: 簡潔で機能が明確、GitHub about の推奨長（160文字以下）に適合

### 案 B: やや詳しい版
```
CLI tool to extract and convert OIDC public keys to PEM certificate format
```

**用途**: Homebrew Formula の `desc` フィールド、crates.io での説明としても利用可能  
**文字数**: 約 80 文字  
**特徴**: ツールの役割（CLI tool）を明記、より詳細

### 案 C: シンプル版
```
Convert authentication provider public keys to certificate files for secure integration
```

**用途**: 技術者以外にも分かりやすい説明が必要な場合  
**文字数**: 約 90 文字  
**特徴**: OIDC/JWKS などの技術用語を避け、より広い層に訴求

## GitHub UI での設定方法

1. リポジトリページにアクセス
2. 右側の「About」セクションの **歯車アイコン（⚙️）** をクリック
3. 表示されたダイアログで以下を設定：
   - **Description**: 上記いずれかのテキスト（推奨：案 A）を入力
   - **Website**: https://github.com/sinofseven/oidc-jwks-converter（任意）
   - **Topics**: `oidc`, `jwks`, `pem`, `cli`, `rust`（任意）
4. 「Save changes」をクリック

## Homebrew Formula での利用

`Formula/oidc-jwks-converter.rb` が公開される際、以下のように設定されます：

```ruby
class OidcJwksConverter < Formula
  desc "CLI tool to extract and convert OIDC public keys to PEM certificate format"
  # ...
end
```

（案 B を使用予定）

## crates.io での利用

Rust パッケージレジストリ crates.io では、`Cargo.toml` の `description` フィールドが使用されます。

現在の設定（同様に案 B を採用）：
```toml
[package]
description = "CLI tool to extract and convert OIDC public keys to PEM certificate format"
```

## 関連リソース

- [GitHub リポジトリメタデータ](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-repositories)
- [Homebrew Formula スタイルガイド](https://docs.brew.sh/Formula-Cookbook)
- [crates.io パッケージ メタデータ](https://doc.rust-lang.org/cargo/reference/manifest.html)
