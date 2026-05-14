# Linuxビルド失敗の修正

## 目的

配布用にlinuxでもビルドできないと困る。

## 要望

Github Actionsにおいてlinuxでビルドに失敗した。どうしたらいい？

## プラン

`Cargo.toml` の `reqwest` 依存を `native-tls` から `rustls-tls` に切り替える。

```toml
reqwest = { version = "0.11", default-features = false, features = ["json", "rustls-tls"] }
```

## 完了サマリー

完了日時: 2026-05-14T17:55:06+09:00

`Cargo.toml` の `reqwest` 依存を変更し、`native-tls`（OpenSSL依存）から `rustls-tls`（ピュアRust実装）に切り替えた。
これにより musl ターゲットでのクロスコンパイル時に OpenSSL のネイティブライブラリが不要となり、Linuxビルドが成功するようになる。
