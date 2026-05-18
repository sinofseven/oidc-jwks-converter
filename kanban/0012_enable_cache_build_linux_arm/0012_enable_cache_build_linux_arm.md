# build-linux-armジョブのキャッシュ最適化

## 目的
build-linux-armジョブを高速化したい

## 要望
build-linux-armジョブの第二ステップでキャッシュを効かせるようにして欲しい

## 備考
タスクファイルのあるディレクトリの `docker_rust_cache_qa.html` を参考にして一つ目のアプローチをベースにして

## プラン
参考資料「docker_rust_cache_qa.html」のアプローチ1（GitHub Actions cache + ボリュームマウント）を採用。build-linux-arm ジョブに以下の変更を実施：

1. checkout の直後にキャッシュステップを追加（registry と build artifacts）
2. docker run コマンドに `-v` オプションを追加してホスト側の ~/.cargo をコンテナ内にマウント

詳細は log.md を参照。

## 完了サマリー
**完了日時**: 2026-05-18T15:56:30+09:00

build-linux-arm ジョブのキャッシュ最適化が完了しました。以下の変更を加えました：

- GitHub Actions cache@v4 で ~/.cargo/registry、~/.cargo/git、target/arm-unknown-linux-musleabihf をキャッシュ
- docker run コマンドにホスト側キャッシュのボリュームマウントを追加

これにより、2回目以降の build-linux-arm ジョブが高速化されます。
