# CHANGELOG.md を v0.2.2 用に更新

## 目的
v0.2.2 をタグ付けしたいので、リリースに合わせて CHANGELOG.md を更新してほしい。

## 要望
v0.2.2 用に CHANGELOG.md を更新する。

## プラン
- `CHANGELOG.md` の `## [0.2.1]` 直前に `## [0.2.2] - 2026-10-22` セクションを追加する
- `### Security`: rustls-webpki / webpki-root-certs の更新（eMudhra 中間 CA 侵害対応、kanban 0024）
- `### Changed`: 依存更新（reqwest, tokio, serde_json, clap, thiserror、aws-lc-rs/sys の依存解決更新）と CI のランナー・digest 更新
- `### Known Issues`: 0.2.1 と同じ文言
- 検証: 見出し・箇条書き形式を 0.2.1 と揃える。`git diff` で `CHANGELOG.md` のみ変わることを確認。記載バージョンは `Cargo.lock` と照合
- コミットは行わない

## 完了サマリー
- **完了日時**: 2026-10-22T14:48:57+09:00
- `CHANGELOG.md` に `## [0.2.2] - 2026-10-22` セクションを追加。`### Security`（rustls-webpki 0.103.16 / webpki-root-certs 1.0.10、eMudhra 中間 CA 侵害対応）、`### Changed`（reqwest 0.13.5、tokio 1.54.1、serde_json 1.0.154、clap 4.6.7、thiserror 2.0.21、aws-lc-rs/sys の依存解決更新、ubuntu-26.04 ランナーと digest 更新）、`### Known Issues`（0.2.1 と同じ文言）を記載
- `git diff` で変更は `CHANGELOG.md` のみ（追加 12 行）
- `Cargo.toml` のバージョンは `0.2.2` で既に反映済み。今回は変更していない
- 未実施: タグ付け（v0.2.2）とコミットはユーザーの指示があるまで行わない
- 詳細は `kanban/0025_update_changelog_v0_2_2/log.md` を参照
