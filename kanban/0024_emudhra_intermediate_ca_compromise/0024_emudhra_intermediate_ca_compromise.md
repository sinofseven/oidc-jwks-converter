# eMudhra Intermediate CA 侵害問題への対応要否の確認

## 目的
CA に関して大きな問題が起きたようなので、本ツール（oidc-jwks-converter）が対応を必要とするかを知りたい。

## 要望
「eMudhra Intermediate CA Compromise」という問題について、対応が必要になることがあるかを調査する。

参考: https://emudhra.we-have-a-problem.com/

## プラン
- 調査結果を `log.md` と本ファイルに記録する
- `cargo update -p rustls-webpki -p webpki-root-certs` で TLS 検証ライブラリをパッチ版へ更新する（`Cargo.lock` のみ）
- 利用中の JWKS エンドポイントの証明書チェーンに eMudhra 中間 CA が含まれないかをユーザー側で確認する

## 完了サマリー
- **完了日時**: 2026-10-22T14:41:47+09:00
- **判定**: 本ツールが eMudhra 問題へ直接対応する必要はない。ただし JWKS 取得時の HTTPS 接続先の証明書チェーンに侵害された中間 CA が含まれる場合は、偽の JWKS を受け取るリスクがあるため、利用中の JWKS エンドポイントの発行元を確認すること。
- **インシデントの概要**: eMudhra の中間 CA 8 本の秘密鍵が鍵生成の不具合で証明書から復元可能。例として `EM-DV-TLS-CA-G3B`。CT ログに該当期間の記録がなく、CT 強制クライアントは攻撃不可能と記載。JWKS / JWT / OIDC への言及はなし。
- **実施した変更**: `Cargo.lock` の更新のみ。`rustls-webpki` 0.103.13 → 0.103.16、`webpki-root-certs` 1.0.7 → 1.0.10。依存解決により `aws-lc-rs` 1.17.0 → 1.18.2、`aws-lc-sys` 0.41.0 → 0.46.0、`pkg-config` 0.3.34 を追加。
- **検証**: `cargo build` 成功、`cargo test` 全件 passed（unit 14 / integration 3）、`cargo clippy` 警告なし、`cargo fmt --check` 差分なし、実 JWKS（Google）から 2 件の PEM 生成を確認。
- **未完了・留意点**:
  - `webpki-root-certs` の依存経路は既定ターゲットでは表示されず、`--target all` での最終確認は未完了。
  - ページのパッチ版情報はページ記載に基づく。解決されたバージョンは一致したが、ページ内容自体は独立検証していない。
  - 漏えいした中間 CA の完全な一覧（`impacted-intermediates.json`）は未取得。
  - コミットは未実施（ユーザーの指示があるまで行わない）。
