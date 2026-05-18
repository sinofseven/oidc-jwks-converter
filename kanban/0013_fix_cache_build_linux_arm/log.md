# build-linux-armのキャッシュ機構修正 - 実装ログ

## ヘッダー

- **タスク**: 0013_fix_cache_build_linux_arm
- **開始日時**: 2026-05-18T10:30:00+09:00
- **ステータス**: 進行中

---

## タスク概要

build-linux-arm CI ジョブのキャッシュ機構がうまく動いていない。キャッシュは一応復元されているように見えるが、ビルド時間が1分以上かかり、効率化されていない。キャッシュを修正して、ビルド高速化を実現する。

---

## 調査結果

### フェーズ1 の調査概要

#### 1. CI ログ分析（ci.log）
- **キャッシュ復元状況**: ✅ Cache hit occurred
  - `Linux-cargo-registry-...`: 復元成功（39MB）
  - `Linux-musl-arm-...`: 復元成功（77MB）
- **ビルド時間**: 1分13秒（Docker イメージ pull 後）
- **状態**: キャッシュがヒットしているにもかかわらず、フルリビルド状態でのビルド実行

#### 2. ワークフロー設定分析（.github/workflows/build.yml）
**build-linux-arm ジョブの特異性:**

| ジョブ | キャッシュツール | ビルド方式 | 環境 |
|------|----------|---------|------|
| build-linux-x86_64 | Swatinem/rust-cache@v2 | ネイティブ | ubuntu-24.04 |
| build-linux-arm64 | Swatinem/rust-cache@v2 | ネイティブ | ubuntu-22.04-arm |
| **build-linux-arm** | **actions/cache@v5（手動）** | **Docker cross-compile** | **ubuntu-24.04** |
| build-macos | Swatinem/rust-cache@v2 | ネイティブ | macos-latest |

**build-linux-arm の詳細設定:**

```yaml
# 行84-100: キャッシュ設定
- name: Cache cargo registry
  uses: actions/cache@v5
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
    key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
    restore-keys: |
      ${{ runner.os }}-cargo-registry-

- name: Cache build artifacts
  uses: actions/cache@v5
  with:
    path: target/arm-unknown-linux-musleabihf
    key: ${{ runner.os }}-musl-arm-${{ hashFiles('**/Cargo.lock') }}-${{ hashFiles('src/**/*.rs') }}
    restore-keys: |
      ${{ runner.os }}-musl-arm-${{ hashFiles('**/Cargo.lock') }}-
      ${{ runner.os }}-musl-arm-

# 行102-108: Docker実行
docker run --rm \
  -v ${{ github.workspace }}:/workspace \
  -v $HOME/.cargo/registry:/root/.cargo/registry \
  -v $HOME/.cargo/git:/root/.cargo/git \
  -w /workspace \
  ghcr.io/rust-cross/rust-musl-cross:${{ env.TARGET }} \
  cargo build --release --target ${{ env.TARGET }}
```

#### 3. 根本原因の特定

**Docker 権限ミスマッチ:**
- GitHub Actions runner ユーザー（runner：UID 1001）がキャッシュを復元・保存
- Docker コンテナ内は root（UID 0）で実行
- ホストと Docker 間でマウントされたファイル（target/など）の UID/GID が異なる
- cargo がコンテナ内で新規ファイル作成時に権限エラーが発生する可能性が高い

**Swatinem が Docker 非対応:**
- GitHub Actions 公式ドキュメントで Docker 環境での使用が言及されていない
- Swatinem/rust-cache Issue #9: 「No cache is ever found with containers」が報告済み
- Docker コンテナ内でのキャッシュ検出バグが存在

**キャッシュキーの問題:**
- キー：`${{ runner.os }}-musl-arm-${{ hashFiles('**/Cargo.lock') }}-${{ hashFiles('src/**/*.rs') }}`
- `runner.os` が "Linux" で、他の Linux ジョブ（x86_64、arm64）と重複する可能性
- `src/**/*.rs` の任意の変更でキャッシュキーが変わり、キャッシュヒット率が低下

---

## 実装プラン

### 修正1: Docker UID マッピング（最優先）

Docker の `--user 1001:1001` オプションで cargo 実行ユーザーをホストの runner ユーザー（UID 1001）に統一する。

**効果:**
- ホストと Docker 内でファイル権限が同一化
- キャッシュ復元後、cargo が権限エラーなくファイルを更新可能
- インクリメンタルビルドが正常に機能

**リスク:** 低（Docker コマンドの引数追加のみ）

### 修正2: キャッシュキーの改善

**現在:**
```
registry: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
target:   ${{ runner.os }}-musl-arm-${{ hashFiles('**/Cargo.lock') }}-${{ hashFiles('src/**/*.rs') }}
```

**改善後:**
```
registry: ${{ runner.os }}-cargo-registry-arm-${{ hashFiles('**/Cargo.lock') }}
target:   ${{ runner.os }}-musl-arm-${{ hashFiles('**/Cargo.lock') }}
```

**改善内容:**
- "arm" プレフィックス追加で他の Linux ジョブとの キー重複を防止
- `src/**/*.rs` ハッシュ削除で、Cargo.lock 変更時のみキー再生成
- 保守性向上、キャッシュヒット率向上

---

## プランニング経緯

### 初回提案（修正内容：Swatinem への移行）

プランモード初期案では、build-linux-arm のキャッシュを `Swatinem/rust-cache@v2` に変更することを提案しました。理由は「他のジョブと統一できる」「キャッシュキー計算の自動化」でした。

### ユーザーからの指摘

「Swatinem は Docker でビルドする場合でも使えるのですか？もっと詳細に調査してもらえますか？」

### 詳細調査結果

Explore agent による詳細調査で、以下が判明：
- **Swatinem は Docker 環境で公式にはサポートされていない**
- Issue #9「No cache is ever found with containers」が報告済み
- ネイティブ環境向けに設計されており、Docker での使用は「ベストエフォート」レベル

### 最終プラン（修正内容：Docker UID マッピング + キャッシュキー改善）

安定性と実装リスク優先で、短期修正を採用：
1. Docker `--user 1001:1001` 追加（権限問題の根本解決）
2. キャッシュキー改善（敏感なファイルハッシュ削除、arm プレフィックス追加）
3. 長期的には Swatinem 検討（別タスク）

---

## 会話内容

**ユーザー:**
「build-linux-armにキャッシュ機構がうまく動いていない。修正して欲しい。」

**Claude（初期提案）:**
「Swatinem/rust-cache への移行を提案します。」

**ユーザー（指摘）:**
「Swatinem は Docker でビルドする場合でも使えるのですか？もっと詳細に調査してもらえますか？」

**Claude（調査報告）:**
「Swatinem は Docker 環境でのサポート未確立。権限ミスマッチとキャッシュキー改善で対応すべき。」

**ユーザー（承認）:**
「了解。短期修正（Docker UID マッピング + キャッシュキー改善）で進める。」

---

## 実装内容

### 実装日時

2026-05-18 10:30～11:00（予定）

### 変更ファイル

`.github/workflows/build.yml`

#### 変更1: キャッシュキー registry（行90）

**修正前:**
```yaml
key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
```

**修正後:**
```yaml
key: ${{ runner.os }}-cargo-registry-arm-${{ hashFiles('**/Cargo.lock') }}
```

**実施日時:** 2026-05-18 10:32

#### 変更2: キャッシュキー build artifacts（行97）

**修正前:**
```yaml
key: ${{ runner.os }}-musl-arm-${{ hashFiles('**/Cargo.lock') }}-${{ hashFiles('src/**/*.rs') }}
```

**修正後:**
```yaml
key: ${{ runner.os }}-musl-arm-${{ hashFiles('**/Cargo.lock') }}
```

**実施日時:** 2026-05-18 10:33

#### 変更3: Docker run コマンド（行102-108）

**修正前:**
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

**修正後:**
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

**実施日時:** 2026-05-18 10:34

---

## テスト・検証予定

### 1. ローカル動作確認

（不可 - Docker コンテナ実行が必要）

### 2. CI 実行テスト

テストタグ `test-cache-v1` で build-linux-arm ジョブを実行し、以下を確認：

- [ ] Docker `--user 1001:1001` オプションが設定されている
- [ ] キャッシュ復元メッセージが表示される
- [ ] ビルド時間が85秒以下に短縮される
- [ ] 2回目以降の実行でキャッシュ効果が維持される

### 3. 追加確認項目

- [ ] cargo が権限エラーなく実行される
- [ ] キャッシュ保存時の エラーが発生しない
- [ ] 他の CI ジョブ（x86_64、arm64 など）に影響がない

---

## 次のステップ

1. ✅ ワークフローファイルの修正完了
2. ⏳ タスクログファイル（本ファイル）の作成完了
3. ⏳ kanban タスクファイルへ `## プラン` セクション追記
4. ⏳ git commit （修正内容）
5. ⏳ テスト実行（タグ push）
6. ⏳ 結果確認とログ最終化
7. ⏳ kanban タスクファイルへ `## 完了サマリー` 追記

---

## メモ

- `--user 1001:1001` オプションが musl-cross イメージで正常に機能するかは未確認
- 万が一失敗した場合は、Docker イメージ側で `chown` やユーザー設定を行う必要が生じる可能性あり
- 権限エラーが残る場合は、別途ファイル権限明示化（chmod など）の実装が必要
