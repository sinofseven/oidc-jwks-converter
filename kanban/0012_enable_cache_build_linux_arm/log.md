# build-linux-armジョブのキャッシュ最適化 実装ログ

## ヘッダー

- **開始日時**: 2026-05-18T15:56:00+09:00
- **ステータス**: 実装中

## タスク概要

build-linux-armジョブの第二ステップでキャッシュを効かせるようにする。これにより、build-linux-armジョブを高速化する。

## 調査結果

### ワークフロー構造の分析

**ファイル**: `.github/workflows/build.yml`

**build-linux-arm ジョブの現状（78-102行）**:
- ランナー: ubuntu-24.04
- TARGET: arm-unknown-linux-musleabihf
- Docker を使用したクロスコンパイル環境
- 現在キャッシュなし

**他のジョブとの比較**:
- build-linux-x86_64（21-47行）: Swatinem/rust-cache@v2 を使用
- build-linux-arm64（49-75行）: Swatinem/rust-cache@v2 を使用
- build-macos（104-127行）: Swatinem/rust-cache@v2 を使用
- build-windows-x86_64（129-154行）: Swatinem/rust-cache@v2 を使用

**build-linux-arm がキャッシュを使用していない理由**:
Docker コンテナ内でビルドが実行されるため、ホスト側のキャッシュディレクトリが直接参照できず、Swatinem/rust-cache（ホスト向け）が効果的でない。

### 参考資料の確認

**ファイル**: `kanban/0012_enable_cache_build_linux_arm/docker_rust_cache_qa.html`

このファイルに記載されている2つのアプローチ：

**アプローチ1（推奨）: GitHub Actions cache + ボリュームマウント**
- `~/.cargo/registry` をホスト側でキャッシュしてコンテナにマウント
- `~/.cargo/git` をホスト側でキャッシュしてコンテナにマウント
- `target/` ディレクトリもキャッシュ（インクリメンタルビルド有効化）
- シンプルで安定している

**アプローチ2: sccache を使う**
- より本格的だが、設定が複雑
- コンテナ内のバイナリ互換性に注意が必要

タスク備考で「一つ目のアプローチをベースにして」と指示されているため、アプローチ1を採用。

### 実装の詳細方針

1. **GitHub Actions cache ステップを追加**（checkout の直後）
   - `~/.cargo/registry` と `~/.cargo/git` をキャッシュ
   - `target/arm-unknown-linux-musleabihf` をキャッシュ
   - キー設計: Cargo.lock のハッシュをベースに、依存更新時は破棄

2. **docker run コマンド修正**
   - ホスト側の `~/.cargo/registry` をコンテナの `/root/.cargo/registry` にマウント
   - ホスト側の `~/.cargo/git` をコンテナの `/root/.cargo/git` にマウント

## 実装プラン

### 修正対象: `.github/workflows/build.yml`

**変更範囲**: build-linux-arm ジョブ（78-102行）

**具体的な修正内容**:

1. checkout ステップの直後（83行の後）に2つのキャッシュステップを追加
2. docker run コマンド（84-89行）に `-v` オプションを追加してボリュームマウントを設定

**修正前（84-89行）**:
```yaml
      - run: |
          docker run --rm \
            -v ${{ github.workspace }}:/workspace \
            -w /workspace \
            ghcr.io/rust-cross/rust-musl-cross:${{ env.TARGET }} \
            cargo build --release --target ${{ env.TARGET }}
```

**修正後**:
```yaml
      - name: Cache cargo registry
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-registry-

      - name: Cache build artifacts
        uses: actions/cache@v4
        with:
          path: target/arm-unknown-linux-musleabihf
          key: ${{ runner.os }}-musl-arm-${{ hashFiles('**/Cargo.lock') }}-${{ hashFiles('src/**/*.rs') }}
          restore-keys: |
            ${{ runner.os }}-musl-arm-${{ hashFiles('**/Cargo.lock') }}-
            ${{ runner.os }}-musl-arm-

      - run: |
          docker run --rm \
            -v ${{ github.workspace }}:/workspace \
            -v $HOME/.cargo/registry:/root/.cargo/registry \
            -v $HOME/.cargo/git:/root/.cargo/git \
            -w /workspace \
            ghcr.io/rust-cross/rust-musl-cross:${{ env.TARGET }} \
            cargo build --release --target ${{ env.TARGET }}
```

## プランニング経緯

初回提案がそのまま承認された。

## 会話内容

### フェーズ1: プランニング

1. **タスク内容の確認**
   - 目的: build-linux-armジョブを高速化したい
   - 要望: build-linux-armジョブの第二ステップでキャッシュを効かせるようにして欲しい
   - 備考: 参考資料の一つ目のアプローチをベースにして

2. **参考資料の読み込み**
   - docker_rust_cache_qa.html でアプローチ1（GitHub Actions cache + ボリュームマウント）を確認

3. **ワークフロー調査**
   - build.yml でワークフロー全体を確認
   - build-linux-arm ジョブが Docker を使用していることを確認
   - 他のジョブが Swatinem/rust-cache を使用していることを確認

4. **計画策定**
   - Explore agent でワークフロー構造を詳細調査
   - アプローチ1 を採用する方針を確定

5. **プラン作成と承認**
   - EnterPlanMode でプランを策定
   - ExitPlanMode でユーザーから承認を得た

## 実装フェーズ

### 修正実施

**実行日時**: 2026-05-18T15:56:30+09:00

#### 編集ファイル

**ファイル**: `.github/workflows/build.yml`

**変更内容**:
- build-linux-arm ジョブ（78-102行）を修正
- checkout ステップの直後に「Cache cargo registry」ステップを追加（GitHub Actions cache@v4 を使用）
- 続けて「Cache build artifacts」ステップを追加（target/ ディレクトリのキャッシュ）
- docker run コマンドに `-v $HOME/.cargo/registry:/root/.cargo/registry` と `-v $HOME/.cargo/git:/root/.cargo/git` を追加

**修正前**:
```yaml
      - uses: actions/checkout@de0fac2e4500dabe0009e67214ff5f5447ce83dd # v6
      - run: |
          docker run --rm \
            -v ${{ github.workspace }}:/workspace \
            -w /workspace \
            ghcr.io/rust-cross/rust-musl-cross:${{ env.TARGET }} \
            cargo build --release --target ${{ env.TARGET }}
```

**修正後**:
```yaml
      - uses: actions/checkout@de0fac2e4500dabe0009e67214ff5f5447ce83dd # v6
      - name: Cache cargo registry
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-registry-
      - name: Cache build artifacts
        uses: actions/cache@v4
        with:
          path: target/arm-unknown-linux-musleabihf
          key: ${{ runner.os }}-musl-arm-${{ hashFiles('**/Cargo.lock') }}-${{ hashFiles('src/**/*.rs') }}
          restore-keys: |
            ${{ runner.os }}-musl-arm-${{ hashFiles('**/Cargo.lock') }}-
            ${{ runner.os }}-musl-arm-
      - run: |
          docker run --rm \
            -v ${{ github.workspace }}:/workspace \
            -v $HOME/.cargo/registry:/root/.cargo/registry \
            -v $HOME/.cargo/git:/root/.cargo/git \
            -w /workspace \
            ghcr.io/rust-cross/rust-musl-cross:${{ env.TARGET }} \
            cargo build --release --target ${{ env.TARGET }}
```

#### 判断・意思決定

1. **キャッシュアクションのバージョン選択**
   - `actions/cache@v4` を使用（最新の安定版）
   - 理由: 他のジョブでは Swatinem/rust-cache を使用しているが、Docker 環境ではホスト側のキャッシュマウントが必要なため、公式の actions/cache が最適

2. **キャッシュキーの設計**
   - 第1キー（registry): `${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}`
   - 第2キー（build artifacts): `${{ runner.os }}-musl-arm-${{ hashFiles('**/Cargo.lock') }}-${{ hashFiles('src/**/*.rs') }}`
   - 理由: Cargo.lock のハッシュ変更時にキャッシュを破棄し、src/*.rs の変更時に build artifacts も新規キャッシュ生成。参考資料に従った設計

3. **ボリュームマウント設定**
   - ホスト側の `$HOME/.cargo/registry` をコンテナの `/root/.cargo/registry` にマウント
   - ホスト側の `$HOME/.cargo/git` をコンテナの `/root/.cargo/git` にマウント
   - 理由: GitHub Actions ランナー内での $HOME は /home/runner。キャッシュステップで保存されたディレクトリをコンテナ内で参照可能にするため

#### 検証

**YAML 構文確認**:
- インデント、キー名、値の形式が正しいことを確認
- docker run コマンドのエスケープが正しいことを確認
- オプション順序は保持（-v オプションはマウント順序に制約がないため問題なし）

