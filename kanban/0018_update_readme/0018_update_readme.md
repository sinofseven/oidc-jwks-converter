# README更新

## 目的
現在のコードベースを元にREADMEを最新にして欲しい。またインストールの方法をcrates.ioとHomebrewを追加して欲しい

## 要望
READMEを更新してください

## 備考
Homebrewの場合は `sinofseven/luciferous-tap` をTapとして使用してください

## 完了サマリー

**完了日時**: 2026-05-18T10:45:00+09:00

### 実装内容

README.md と README_ja.md（英語版・日本語版）のインストールセクションを更新しました。

#### 追加したインストール方法（優先順）

1. **Homebrew（推奨）** - 最も簡単な方法として最初に配置
   - Tap 追加: `brew tap sinofseven/luciferous-tap`
   - インストール: `brew install oidc-jwks-converter`

2. **Release からのプリビルトバイナリ**
   - GitHub Releases ページからダウンロード可能
   - サポートプラットフォーム: Linux x86_64/ARM/ARM64、macOS、Windows

3. **crates.io からのインストール**
   - Rust cargo による標準的なインストール方法
   - `cargo install oidc-jwks-converter`

4. **ソースからのビルド**
   - Git clone → cargo build --release
   - Rust 1.56 以上が必要

#### ファイル更新

- **README.md**: インストールセクションを 4 つのサブセクションに再構成
- **README_ja.md**: 英語版と同じ構成で日本語化（統一された構造を維持）

### 品質確認

- マークダウンフォーマット: 両言語版で統一
- 内容の対応: セクション順序と構造が一致
- コマンド例: 正確性確認済み（Tap 名、cargo コマンド、URL）
