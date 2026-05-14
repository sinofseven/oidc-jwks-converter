# log: Linuxビルド失敗の修正

開始: 2026-05-14T17:54:43+09:00
完了: 2026-05-14T17:55:06+09:00

## タスク概要

Github Actionsにおいてlinuxでビルドに失敗した。どうしたらいい？

## 調査結果

### actions_raw.log の確認

エラーメッセージ（608〜722行付近）:

```
warning: openssl-sys@0.9.115: Could not find directory of OpenSSL installation, and this `-sys` crate cannot proceed without this knowledge.
error: failed to run custom build command for `openssl-sys v0.9.115`
Caused by:
  process didn't exit successfully: ... (exit status: 101)

  Could not find openssl via pkg-config:
  pkg-config has not been configured to support cross-compilation.

  $HOST = x86_64-unknown-linux-gnu
  $TARGET = x86_64-unknown-linux-musl
  openssl-sys = 0.9.115
```

### .github/workflows/build.yml の確認

Linux ビルドジョブは3つある：
- `build-linux-x86_64`: Ubuntu 22.04, ターゲット `x86_64-unknown-linux-musl`
- `build-linux-arm64`: Ubuntu 22.04-arm, ターゲット `aarch64-unknown-linux-musl`
- `build-linux-arm`: Ubuntu 22.04-arm, ターゲット `arm-unknown-linux-musleabihf`

いずれも `musl-tools` をインストールして musl ターゲットに向けてビルドしている。

### Cargo.toml の確認

```toml
reqwest = { version = "0.11", features = ["json"] }
```

`reqwest 0.11` はデフォルト機能に `native-tls` が含まれており、OpenSSL のネイティブライブラリへの依存が発生する。
musl ターゲットでクロスコンパイルする際、pkg-config が musl 用 OpenSSL を検出できないためビルドが失敗している。

### src/ コードの確認

`reqwest` の使用箇所 (`src/jwks.rs`):
- `reqwest::Client::new()` でクライアントを作成して HTTP GET リクエストを送信
- `reqwest::Error` をエラー型として使用

API 互換性は `rustls-tls` に切り替えても保たれる。

## 実装プラン

`Cargo.toml` の `reqwest` 依存を変更し、`native-tls`（OpenSSL依存）から `rustls-tls`（ピュアRust実装TLS）に切り替える。

```toml
# 変更前
reqwest = { version = "0.11", features = ["json"] }

# 変更後
reqwest = { version = "0.11", default-features = false, features = ["json", "rustls-tls"] }
```

`rustls` はピュアRustで実装されているため OpenSSL のネイティブライブラリが不要。
musl ターゲットを含む全プラットフォームでビルドできる。

## プランニング経緯

初回提案がそのまま承認された。

## 会話内容

1. ユーザーが `kanban/0006_fix_linux_build_failure/actions_raw.log` にログを保存していると伝えた
2. ログを確認し、`openssl-sys` のビルド失敗を発見
3. `reqwest` が `native-tls` を使用していることを確認
4. `rustls-tls` への切り替えプランを提案 → 承認

## 編集したファイル

- `Cargo.toml`: `reqwest` の依存を `native-tls` から `rustls-tls` に変更

## 実行したコマンド

（なし）

## 判断・意思決定

- `native-tls` → `rustls-tls` 切り替えを選択。GitHub Actions でOpenSSLをインストールする方法もあるが、依存ライブラリを減らしてピュアRustにする方が全プラットフォームで安定する。

## エラー・問題

（なし）
