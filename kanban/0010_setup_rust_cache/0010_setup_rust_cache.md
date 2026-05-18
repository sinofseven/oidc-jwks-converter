# CIでrust-cacheを使う

## 目的
連続してCIを動かすときに高速化したい。しかしCIやタグを打った後のCDなどで問題が起きないようにしたい。

## 要望
`Swatinem/rust-cache` を使ってCIでキャッシュを使うようにしてください

## プラン

5つのビルドジョブ（build-linux-x86_64, build-linux-arm64, build-linux-arm, build-macos, build-windows-x86_64）の `actions/checkout` の直後に `Swatinem/rust-cache@v2` を追加します。

**キャッシュの安全性:**
- Cargo.lock ベースの自動キャッシュキー生成
- タグ打ち後のリリースでも Cargo.lock が同じなら安全に再利用
- 依存関係変更時は新規キャッシュが自動作成される

**期待される効果:**
- PR・push 時の CI 実行時間が 30-60% 削減（キャッシュ命中時）
- タグ打ち後の CD フローに問題なし

## 完了サマリー

**実施日時**: 2026-05-18

5つのビルドジョブに `Swatinem/rust-cache@v2` を追加しました。

**修正ファイル:**
- `.github/workflows/build.yml` - 5つのビルドジョブに `- uses: Swatinem/rust-cache@v2` を追加

**検証方法:**
1. PR・push で CI 実行、複数回実行で実行時間の短縮を確認
2. タグ打ってリリース実行、バイナリが期待通り動作することを確認
