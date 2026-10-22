# eMudhra Intermediate CA 侵害問題への対応要否の確認 - 作業ログ

## 開始日時
2026-10-22T14:40:29+09:00

## タスク概要
（kanban ファイルの要望を転記）

要望: 「eMudhra Intermediate CA Compromise」という問題について、対応が必要になることがあるかを調査する。
参考: https://emudhra.we-have-a-problem.com/

目的: CA に関して大きな問題が起きたようなので、本ツール（oidc-jwks-converter）が対応を必要とするかを知りたい。

## 調査結果

### インシデントページ（web-fetch エージェント経由で取得、HTTP 200）
- 本文: 「eight eMudhra intermediate CA certificates の秘密鍵が漏えいした。Mozilla, Microsoft, Apple, Chrome の各 root program と多くの downstream trust store に信頼されている root に連鎖する。」
- 確認できた intermediate 名は `EM-DV-TLS-CA-G3B`（例として記載）のみ。全一覧は `impacted-intermediates.json`（未取得）。
- Windows 回避スクリプト側の記述は 16 本（8 本 + Sectigo cross-sign と推定）。ページ本文では明記なし。
- 秘密鍵の漏えい原因: 「intermediate の秘密鍵が CA により誤って生成され、証明書から誰でも復元できる」。2025 年 4 月発行。CT ログに該当期間のエントリなし → CT 強制クライアント（Firefox, Chrome, Edge, Safari）は攻撃不可能と記述。ただし攻撃の有無は不明とのこと。
- 日付: ページ日付 2026-10-20。root program への報告 2026-10-17。Firefox/Chrome は 10-17 からブロック、Apple は 10-19、Microsoft は 10-20 から。downstream への開示 10-19。revoke 済みだが日付の記載なし。
- 影響: 侵害された intermediate の秘密鍵を持つ攻撃者は任意の Web サイトやサービスを偽装できる。OneCRL / CRLite / CRLSets で失効処理。失効情報を見ないクライアントはソフトウェア更新が必要。
- 利用者への要求: ブラウザ・OS を最新化。確認用に `curl https://dv-g3b-emudhra.we-have-a-problem.com/` 等。Rust `rustls-webpki` 0.103.16、Rust `webpki-root-certs` 1.0.10 などのパッチ版一覧（crates.io では未検証）。
- JWKS / JWT / OIDC への言及はなし。TLS 証明書チェーンへの言及のみ。「独自の CA バンドルを使うアプリケーションは個別の更新が必要」との記述あり。

### 本リポジトリの確認
- `grep`（rs/toml/md/yml/yaml）で eMudhra への言及はなし。
- `Cargo.toml`: `reqwest = { version = "0.13", default-features = false, features = ["json", "rustls"] }`。JWKS 取得は HTTPS。
- 更新前の `Cargo.lock`:
  - `rustls-webpki 0.103.13`
  - `webpki-root-certs 1.0.7`
  - `rustls-platform-verifier 0.7.0`（reqwest の依存。OS の trust store を使う経路で `webpki-root-certs` も依存）
  - `aws-lc-rs 1.17.0`, `aws-lc-sys 0.41.0`
- `.github/workflows/build.yml`: `dtolnay/rust-toolchain@stable` でビルド。

### 判定
- 本ツールの JWT 署名検証ロジックへの直接の影響はない。
- JWKS 取得時の HTTPS 接続先の証明書チェーンに、侵害された eMudhra intermediate が含まれる場合のみ、偽の JWKS を受け取るリスクがある。利用中の JWKS エンドポイントの証明書発行元で判定できる。
- 本ツールは CLI で、ユーザー自身の環境で動く。ツールの依存ライブラリの TLS 検証ライブラリのパッチ版へ更新することが、ページの推奨に沿う対応。

## 実装プラン
（kanban `## プラン` の要約版に対応する完全版）

承認された計画（`/Users/natsume.yuta/.claude/plans/drifting-frolicking-octopus.md`）の内容:

1. 調査結果の記録（必須）: kanban の `## 完了サマリー` と `log.md` に事実と判定を記録する。
2. 依存の更新（推奨）: `cargo update -p rustls-webpki -p webpki-root-certs` を実行し、`Cargo.lock` の差分のみを対象とする。コミットはユーザーの明示指示があるまで行わない。
3. JWKS 取得先の確認（ユーザー作業）: 利用中の JWKS エンドポイントの証明書チェーンに eMudhra 中間 CA が含まれないかを `openssl s_client -connect <host>:443 -showcerts` で確認する。

検証計画:
- `cargo build` と `cargo test`
- `cargo tree -i rustls-webpki` と `cargo tree -i webpki-root-certs` でバージョン確認
- 実 JWKS（Google）で `cargo run -- <URL> -o <scratchpad>/keys` が従来どおり動くこと

未解決の質問: 依存更新（推奨 2）を本タスクに含めるか。承認されたプランに含まれていたため、本タスクで実施した。

## プランニング経緯
- 最初に提示したプラン: 上記の 3 点（記録 / 依存更新 / JWKS 取得先の確認）。
- ユーザーのフィードバック・リジェクト: なし。ExitPlanMode で承認された。
- 最終プランへの変更: 検証計画の出力先を `/tmp` ではなく scratchpad に変更（システム指示に従う）。

## 会話内容
- ユーザー: `/kanban-kit:add-kanban` で「eMudhra Intermediate CA Compromise」の対応要否を調べたいとしてタスクを作成依頼。目的（CA に関する大きな問題、対応要否を知りたい）と参考 URL を提示。
- Claude: 採番を `ls kanban/` で行い、最大番号 0023 から 0024 を採番。`kanban/0024_emudhra_intermediate_ca_compromise/` にタスクファイルを作成。
- Claude: AskUserQuestion で `/kanban` 続行を確認。ユーザーは「はい（すぐ実行する）」を選択。
- Claude: `kanban-kit:kanban` を args `0024` で起動。`## 完了サマリー` は未存在のためプランニングへ。目的セクションは存在するため、プランモードに入って調査を実施。
- Claude: web-fetch エージェントでインシデントページを取得（HTTP 200）。並行して `grep` でリポジトリ内の関連語を検索（Grep ツールは使用不可だったため Bash の grep を使用）。
- Claude: `Cargo.toml` / `Cargo.lock` / CI 設定を確認し、判定と計画を作成。
- Claude: ExitPlanMode で承認された。
- Claude: 承認後、ログを作成。`cargo update -p rustls-webpki -p webpki-root-certs` を実行。

## 実装フェーズ

### 編集したファイル
- `Cargo.lock`: `cargo update -p rustls-webpki -p webpki-root-certs` で更新。
  - `rustls-webpki` 0.103.13 → 0.103.16
  - `webpki-root-certs` 1.0.7 → 1.0.10
  - 同時に（cargo の依存解決により）`aws-lc-rs` 1.17.0 → 1.18.2、`aws-lc-sys` 0.41.0 → 0.46.0、`pkg-config` 0.3.34 を追加（5 パッケージがロック対象）。
- `kanban/0024_emudhra_intermediate_ca_compromise/log.md`（本ファイル）
- `kanban/0024_emudhra_intermediate_ca_compromise/0024_emudhra_intermediate_ca_compromise.md`（`## 完了サマリー` 追記予定）

### 実行したコマンドと結果
- `TZ=Asia/Tokyo date +"%Y-%m-%dT%H:%M:%S+09:00"` → `2026-10-22T14:40:29+09:00`
- `cargo update -p rustls-webpki -p webpki-root-certs` → 上記のバージョン変更。`Locking 5 packages to highest Rust 1.99.1 compatible versions`。
- `cargo build` → 成功（25.18s）。
- `cargo test` → unit tests 14 件 passed、integration tests 3 件 passed、失敗 0。
- `cargo clippy` → 警告なし（Finished）。
- `cargo fmt --check` → 差分なし（FMT_OK）。
- `cargo tree -i rustls-webpki` → `rustls-webpki v0.103.16`（rustls v0.23.40 経由）。
- `cargo tree -i webpki-root-certs` → 既定ターゲットでは表示されず（プラットフォーム依存）。`cargo tree --target all -i webpki-root-certs` を実行し、出力が先頭のダウンロード行のみだったため、依存経路の最終確認は未完了。
- 実 JWKS 検証: `./target/debug/oidc-jwks-converter https://www.googleapis.com/oauth2/v3/certs -o <scratchpad>/keys` → `Found 2 key(s)`、2 件の `.pem` を保存。（scratchpad: `/private/tmp/claude-501/-Users-natsume-yuta-spaces-private-oidc-jwks-converter/6df96be8-c295-4210-bc5b-a323cc662b1b/scratchpad/keys`）

### 判断・意思決定
- 依存更新は承認されたプランの推奨 2 として実施した。未解決の質問は「含めるか」だったが、承認されたプラン本文に含まれていたため実施。
- 更新先のバージョンは `cargo update` が解決した最新互換版（rustls-webpki 0.103.16、webpki-root-certs 1.0.10）。インシデントページの推奨版と一致した。
- コミットは行わない（CLAUDE.md とユーザー指示による）。

### エラー・問題
- Grep ツールは利用不可だったため、Bash の `grep` で代替。
- `cargo tree -i webpki-root-certs`（既定ターゲット）は該当なし。`--target all` の出力は先頭のダウンロード行のみで、経路の最終確認は未完了。
- ページの「パッチ版」情報は web-fetch エージェントの報告に基づく。`cargo update` で実際に該当バージョンに解決されたことは確認できたが、ページ自体の内容の正確性は独立には未検証。

## 完了日時
2026-10-22T14:41:47+09:00（kanban ファイルの `## 完了サマリー` に同内容を記載）
