# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

`oidc-jwks-converter` は OIDC (OpenID Connect) の JWKS (JSON Web Key Set) のURLを渡されると各キーのpemファイル(`{key_id}.pem`)を生成するCLIツールです。

## コマンド

```bash
# ビルド
cargo build

# リリースビルド
cargo build --release

# 実行
cargo run

# テスト実行
cargo test

# 特定のテストのみ実行
cargo test <テスト名>

# Lint (clippy)
cargo clippy

# フォーマット
cargo fmt

# フォーマットチェック
cargo fmt --check
```

## 開発フロー

開発には `kanban-kit` を使用する。

### タスク管理

- タスク作成: `/add-kanban` スキルを使って `kanban/` ディレクトリにタスクファイルを作成する
- タスク実行: `/kanban` スキルを使ってタスクを進める（プランニング → 承認 → 実装の順）

詳細なワークフローは `kanban-kit` プラグインの `references/kanban-workflow.md` を参照。

### ディレクトリ構造

```
kanban/
  {xxxx}_{title}/          # 4桁0パディング連番 + snake_case タイトル
    {xxxx}_{title}.md      # タスクファイル（## 目的 と ## 要望 セクション必須）
    log.md                 # 作業ログ（完全な記録、要約・省略禁止）
```

### 注意事項

- git commit はスキルが自動実行しない。ユーザーが明示的に指示した場合のみ実行する
