# build-linux-arm ジョブを rust-musl-cross で安定化

## 目的
ビルドを安定化させるため

## 要望
`build-linux-arm` ジョブで `https://github.com/rust-cross/rust-musl-cross` を使うように修正してください

## プラン

1. `.github/workflows/build.yml` の `build-linux-arm` ジョブを以下のように修正：
   - ランナーを `ubuntu-22.04-arm` から `ubuntu-22.04` に変更
   - `dtolnay/rust-toolchain` ステップを削除
   - ツールチェーンインストールステップを削除
   - `cargo build` を Docker コンテナ内で実行（`docker run -v ... ghcr.io/rust-cross/rust-musl-cross:latest cargo build`）
   - 環境変数 `CARGO_TARGET_ARM_UNKNOWN_LINUX_MUSLEABIHF_LINKER` を削除

2. 期待される効果：
   - Docker コンテナによる統一されたビルド環境
   - ツールチェーンバージョン固定による安定性向上
   - ビルド再現性の向上

## 完了サマリー

**完了日時**: 2026-05-18T19:35:00+09:00

**実装内容**:
- `.github/workflows/build.yml` の `build-linux-arm` ジョブを修正
- ランナーを `ubuntu-22.04-arm` から `ubuntu-22.04` に変更
- `dtolnay/rust-toolchain` ステップを削除
- ツールチェーンインストール（apt-get）を削除
- `cargo build` を `docker run ... ghcr.io/rust-cross/rust-musl-cross:latest cargo build` で実行
- 環境変数 `CARGO_TARGET_ARM_UNKNOWN_LINUX_MUSLEABIHF_LINKER` を削除

**検証**:
- ワークフロー構文の確認（YAML 形式は正しい）
- Docker コマンド構文確認（`--rm -v ... -w ...` のマウント設定は正しい）
- 次のワークフロー実行時にビルド成功を確認する
