# CHANGELOG更新（v0.2.1）

## 目的
依存ライブラリなどを更新したので、パッチバージョンアップのためにCHANGELOGを書いてほしい。

## 要望
v0.2.1用に CHANGELOG.md を更新してほしい。

## 完了サマリー

- 完了日時: 2026-09-04T15:01:20+09:00
- `Cargo.toml` のバージョンを `0.2.0` → `0.2.1` に変更
- `CHANGELOG.md` に `## [0.2.1] - 2026-09-04` セクションを追加。内容は依存ライブラリ更新（p256 0.13→0.14、base64 0.22→0.23、anyhow・serde・serde_json・clap・thiserrorのパッチ更新）、p256 0.14アップグレードに伴うビルドエラー修正、CI設定更新（actions/checkout v7、各種digest更新）を記載
- `cargo build` でビルド成功を確認済み
- 詳細は `kanban/0022_update_changelog_v0_2_1/log.md` を参照
