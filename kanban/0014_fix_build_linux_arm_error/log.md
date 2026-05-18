# build_linux_armビルドエラーの修正 - 作業ログ

**開始時刻**: 2026-05-18T16:45:00+09:00

## タスク概要

build_linux_armでエラーが出た。修正して欲しい。

## 調査結果

### CI ログの分析

CIログ（ci.log）から以下のエラーが特定されました：

```
warning: failed to write cache, path: /root/.cargo/registry/index/index.crates.io-1949cf8c6b5b557f/.cache/an/yh/anyhow, error: Permission denied (os error 13)
...
error: failed to create directory `/root/.cargo/registry/cache/index.crates.io-1949cf8c6b5b557f`

Caused by:
  Permission denied (os error 13)
```

時刻: `2026-05-18T07:38:56.7073355Z` から `2026-05-18T07:38:56.7890363Z`

### CI 実行環境の詳細

- Docker イメージ: `ghcr.io/rust-cross/rust-musl-cross:arm-unknown-linux-musleabihf`
- ターゲット: `arm-unknown-linux-musleabihf`
- 実行者: UID 1001 ユーザー（`--user 1001:1001` フラグ）
- Volume マウント:
  - ホスト `$HOME/.cargo/registry` → コンテナ `/root/.cargo/registry`
  - ホスト `$HOME/.cargo/git` → コンテナ `/root/.cargo/git`
  - ホスト `${{ github.workspace }}` → コンテナ `/workspace`

### ワークフロー定義の確認

`.github/workflows/build.yml` の 78-122行目に `build-linux-arm` ジョブが定義されている。

Docker run コマンド（101-109行目）:
```yaml
- run: |
    docker run --rm \
      --user 1001:1001 \
      -v ${{ github.workspace }}:/workspace \
      -v $HOME/.cargo/registry:/root/.cargo/registry \
      -v $HOME/.cargo/git:/root/.cargo/git \
      -w /workspace \
      ghcr.io/rust-cross/rust-musl-cross:${{ env.TARGET }} \
      cargo build --release --target ${{ env.TARGET }}
```

### 問題分析

1. Docker コンテナが `--user 1001:1001` で実行されている
2. `/root/.cargo/registry` と `/root/.cargo/git` がマウントされている
3. UID 1001 ユーザーは `/root/` ディレクトリへの書き込み権限がない
4. cargo がレジストリキャッシュディレクトリ `/root/.cargo/registry/cache/` を作成しようとして Permission denied エラーが発生

キャッシュ復元状況:
- Line 150: `Cache not found for input keys: Linux-cargo-registry-arm-unknown-linux-musleabihf-0f948577a233cea7995104e8a5a4bdc17836db31229acbca6e1e7ded66028fae, Linux-cargo-registry-arm-unknown-linux-musleabihf-`
- Line 167: `Cache not found for input keys: Linux-musl-arm-unknown-linux-musleabihf-0f948577a233cea7995104e8a5a4bdc17836db31229acbca6e1e7ded66028fae, ...`

キャッシュが存在しないため、新しいディレクトリが作成される際にパーミッション問題が露呈しました。

## 実装プラン

### 修正内容

`.github/workflows/build.yml` の `build-linux-arm` ジョブの docker run コマンドから `--user 1001:1001` フラグを削除する。

### 理由

- `/root/.cargo/` はコンテナ内の ephemeral キャッシュディレクトリ
- ホスト側の `$HOME/.cargo/` からマウントされている
- root で実行することで、ディレクトリ作成権限の問題を解決できる
- 他のジョブ（build-linux-x86_64, build-linux-arm64）では `--user` フラグを使用していない（Swatinem/rust-cache を使用している）

## プランニング経緯

初回提案がそのまま承認された。修正内容：`--user 1001:1001` フラグを削除する。

## 編集内容

### .github/workflows/build.yml の修正

101-109 行目: docker run コマンドから `--user 1001:1001` を削除

変更前:
```yaml
- run: |
    docker run --rm \
      --user 1001:1001 \
      -v ${{ github.workspace }}:/workspace \
      -v $HOME/.cargo/registry:/root/.cargo/registry \
      -v $HOME/.cargo/git:/root/.cargo/git \
      -w /workspace \
      ghcr.io/rust-cross/rust-musl-cross:${{ env.TARGET }} \
      cargo build --release --target ${{ env.TARGET }}
```

変更後:
```yaml
- run: |
    docker run --rm \
      -v ${{ github.workspace }}:/workspace \
      -v $HOME/.cargo/registry:/root/.cargo/registry \
      -v $HOME/.cargo/git:/root/.cargo/git \
      -w /workspace \
      ghcr.io/rust-cross/rust-musl-cross:${{ env.TARGET }} \
      cargo build --release --target ${{ env.TARGET }}
```

## 実行内容

修正を実施した。

## 完了確認

修正が適用されました。次のステップで検証予定:
- GitHub Actions で `build-linux-arm` ジョブが成功することを確認
- キャッシュが正常に保存・復元されることを確認
- ビルドアーティファクト（`target/arm-unknown-linux-musleabihf/release/oidc-jwks-converter`）が正常に生成されることを確認

## 変更内容の概要

**ファイル修正**: `.github/workflows/build.yml`
- 101行目: `--user 1001:1001` フラグを削除
- 理由: Docker コンテナ内の UID 1001 ユーザーが `/root/.cargo/registry/` への書き込み権限がないことが原因

