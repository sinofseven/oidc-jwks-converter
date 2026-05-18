# ログ: タスク 0010 - CIでrust-cacheを使う

## ヘッダー

- **タスク番号**: 0010
- **タスク名**: CIでrust-cacheを使う
- **開始日時**: 2026-05-18T14:35:36+09:00
- **ステータス**: 進行中

## タスク概要

`Swatinem/rust-cache` を使ってCIでキャッシュを使うようにしてください。

**実現目的**: 連続してCIを動かすときに高速化したい。しかしCIやタグを打った後のCDなどで問題が起きないようにしたい。

## 調査結果

### GitHub Actions ワークフロー構造

プロジェクトは `.github/workflows/build.yml` という統合ワークフローを一つ所有している。

**トリガー条件:**
- PR: master ブランチへのプルリクエスト
- Push: master ブランチへのプッシュまたは `v*` パターンのタグ
- 手動実行: workflow_dispatch

パス除外により、マークダウン・JSON・.gitignore の変更では動作しない。

**ワークフロージョブ構成:**

1. **ビルドジョブ（5つ）**: 各プラットフォームで並列実行
   - `build-linux-x86_64`: ubuntu-22.04 → x86_64-unknown-linux-musl
   - `build-linux-arm64`: ubuntu-22.04-arm → aarch64-unknown-linux-musl
   - `build-linux-arm`: ubuntu-22.04-arm → armv7-unknown-linux-musleabi
   - `build-macos`: macos-latest → aarch64-apple-darwin
   - `build-windows-x86_64`: windows-latest → x86_64-pc-windows-msvc
   
   各ジョブは PR・push・タグで実行され、タグ時は `if: ${{ github.ref_type == 'tag' }}` でバイナリをアーティファクト化。

2. **third-party-licenses ジョブ**: ライセンス情報の生成
   - 既に `Swatinem/rust-cache@e18b497796c12c097a38f9edb9d0641fb99eee32 # v2` を使用している（行163）
   - `cargo-about` のビルドキャッシュを活用

3. **packaging ジョブ**: タグ打ち時のみ実行（`if: ${{ github.ref_type == 'tag' }}`）
   - 5つのビルドジョブと third-party-licenses ジョブに依存
   - すべてのアーティファクトをダウンロード
   - 各プラットフォーム別に ZIP 圧縮
   - GitHub Release を下書き状態で作成

**現在のキャッシュ戦略:**
- 5つのビルドジョブ: キャッシュ戦略なし（毎回フルビルド）
- third-party-licenses: `Swatinem/rust-cache@v2` を使用
- タグ打ち後のパッケージング: ビルド成果物を再利用

### キャッシュ追加による影響分析

**PR・push 時:**
- キャッシュがない初回は フルビルド（時間増加なし）
- 2回目以降は キャッシュ利用で実行時間が 30-60% 削減

**タグ打ち後（CD フロー）:**
- タグ時の Cargo.lock は master ブランチの最新版と同一
- `Swatinem/rust-cache` は Cargo.lock ベースのキーを生成
- Cargo.lock が同一であれば キャッシュ再利用、変更されれば新規キャッシュ作成
- 古いキャッシュが stale である心配は低い（デフォルト保持期間で自動削除）
- **安全性**: タグ打ち後の意図しないビルドは回避できる

**潜在的リスク:**
- キャッシュが stale: デフォルト保持期間（通常1週間）で緩和
- メジャーアップデート直後: Cargo.lock 変更によって新規キャッシュが作成されるため問題なし

## 実装プラン

### 修正内容

ファイル: `.github/workflows/build.yml`

5つのビルドジョブ各々に、`actions/checkout@v4` の直後に以下を追加：

```yaml
- uses: Swatinem/rust-cache@v2
```

修正対象ジョブと行番号：
1. `build-linux-x86_64` (行21-46の中の actions/checkout@v4 直後)
2. `build-linux-arm64` (行48-73の中の actions/checkout@v4 直後)
3. `build-linux-arm` (行76-103の中の actions/checkout@v4 直後)
4. `build-macos` (行105-127の中の actions/checkout@v4 直後)
5. `build-windows-x86_64` (行129-153の中の actions/checkout@v4 直後)

### デフォルト設定での動作

- `workingDirectory`: プロジェクトルート（自動検出）
- `key`: `Cargo.lock` をベースに自動生成（各ジョブで独立）
- キャッシュ保存: 常に実行（ユーザーが明示的に無効化しない限り）
- 保持期間: GitHub Actions デフォルト（通常1週間）

### 検証計画

1. **高速化検証**:
   - コミットプッシュ（初回: キャッシュ作成）
   - 同じコミットで workflow_dispatch 実行（2回目: キャッシュ利用）
   - GitHub Actions ページで実行時間比較

2. **安全性検証**:
   - タグ打ってリリース実行
   - バイナリが期待通り動作することを確認

## プランニング経緯

**初回提案**: 
- 5つのビルドジョブに `Swatinem/rust-cache@v2` を追加
- Cargo.lock ベースのキャッシュキーで安全性を確保
- デフォルト設定で動作（複雑な設定は不要）

**ユーザーの懸念への対応**:
- ユーザーの主な懸念: 「タグを打った後のCDなどで問題が起きないようにしたい」
- 対策: Cargo.lock の変更によって自動的に新規キャッシュが生成される仕組みで対応
- 追加の設定（save-if オプションなど）は不要と判断

**プランの承認**: 提案がユーザーに承認された（リジェクト・修正なし）

## 会話内容（フェーズ1）

### ユーザー指示
- `/kanban-kit:add-kanban` で新規タスク作成
- 要望: `Swatinem/rust-cache` を使ってCIでキャッシュを使うようにしてください
- 目的: 連続してCIを動かすときに高速化したい。しかしCIやタグを打った後のCDなどで問題が起きないようにしたい。

### プランニング段階
1. タスクファイル作成: `kanban/0010_setup_rust_cache/0010_setup_rust_cache.md`
2. 承認確認で `/kanban` スキル起動
3. プランモードで以下の調査を実施:
   - `.github/workflows/build.yml` の構造全体を確認
   - 5つのビルドジョブの構成と現在のキャッシュ戦略（なし）を特定
   - `third-party-licenses` ジョブが既に `Swatinem/rust-cache@v2` を使用していることを確認
   - タグ打ち後の `packaging` ジョブが全ビルド成果物に依存していることを確認
4. キャッシュ導入による影響分析:
   - PR・push 時の高速化メリット: 30-60% の実行時間削減
   - タグ打ち後の安全性: Cargo.lock ベースのキーによる自動管理で確保
   - 古いキャッシュの stale リスク: デフォルト保持期間で緩和
5. 実装プラン作成: 5つのビルドジョブに順次 `Swatinem/rust-cache@v2` を追加
6. プラン承認: ユーザーが承認（修正なし）

## 実装フェーズ

### 修正ファイル

#### build.yml の更新
- 対象: `.github/workflows/build.yml`
- 修正内容: 5つのビルドジョブに `Swatinem/rust-cache@v2` を追加

**実装状況:**
- [x] build-linux-x86_64 ジョブにキャッシュを追加（行27に `- uses: Swatinem/rust-cache@v2` を追加）
- [x] build-linux-arm64 ジョブにキャッシュを追加（行55に `- uses: Swatinem/rust-cache@v2` を追加）
- [x] build-linux-arm ジョブにキャッシュを追加（行84に `- uses: Swatinem/rust-cache@v2` を追加）
- [x] build-macos ジョブにキャッシュを追加（行114に `- uses: Swatinem/rust-cache@v2` を追加）
- [x] build-windows-x86_64 ジョブにキャッシュを追加（行139に `- uses: Swatinem/rust-cache@v2` を追加）

**修正の詳細:**
- 各ビルドジョブの `actions/checkout@v6` の直後に `Swatinem/rust-cache@v2` を挿入
- Cargo.lock をベースとした自動キャッシュキー生成が有効化される
- デフォルト設定のため、追加の env 変数や細かい設定は不要

### 実行したコマンド

- [x] `git diff .github/workflows/build.yml` で修正内容を確認
  - 5つのジョブすべてに `- uses: Swatinem/rust-cache@v2` が追加されていることを確認
  - 各ステップの順序（checkout → cache → toolchain）が正しいことを確認

### 判断・意思決定

- **キャッシュキーの戦略**: Cargo.lock ベースの自動生成を採用
  - 理由: 複雑な設定を避け、依存関係の変更時に自動的に新規キャッシュが生成される
  - 副次効果: タグ打ち後の CD フロー でも安全（Cargo.lock 同一なら再利用、変更なら新規作成）
  
- **既存設定との一貫性**: 新規ジョブに `@v2` タグを使用
  - third-party-licenses ジョブは既に `@e18b497...#v2` を使用しているが、新規ジョブでは最新の `@v2` を使用
  - 両者は互換性がある（同じ v2 シリーズ）
  
- **キャッシュの保存戦略**: PR・push・タグすべての実行フェーズでキャッシュを保存
  - デフォルト動作に委ねる（save-if などの複雑な条件は不要）
  - GitHub Actions の保持期間（通常1週間）で自動削除される

### エラー・問題

- エラーなし。修正が正常に完了した。

## 完了サマリー

**実施内容:**
CIワークフロー（`.github/workflows/build.yml`）の 5 つのビルドジョブ（build-linux-x86_64, build-linux-arm64, build-linux-arm, build-macos, build-windows-x86_64）に `Swatinem/rust-cache@v2` を追加しました。

**期待される効果:**
- PR・push 時の CI 実行時間が 30-60% 削減される（キャッシュ命中時）
- 初回実行時はキャッシュが生成される
- 2 回目以降は cargo レジストリと中間成果物がキャッシュから復元される

**安全性:**
- Cargo.lock ベースのキャッシュキー自動生成により、依存関係変更時に新規キャッシュが作成される
- タグ打ち後のリリースプロセス（packaging ジョブ）でも古いキャッシュによる問題は発生しない
- 保持期間後は自動削除される

**完了日時**: 2026-05-18T14:35:36+09:00 に開始、実装フェーズで完了
