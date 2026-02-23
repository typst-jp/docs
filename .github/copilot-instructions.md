# Copilot instructions

このリポジトリは、[Typst](https://typst.app/)の公式ドキュメントを日本語に翻訳するプロジェクトおける、コーディングエージェント用の初期セットアップガイドです。エージェントは作業を開始する前に必ずこのファイルを参照し、記載された手順や制約に従ってください。

## リポジトリの概要

- リポジトリの目的は、Typstの公式ドキュメントを日本語へ翻訳し、Webサイトとして公開することです。
- 依存ツールの管理やタスクランナーとしてmiseを使用します。RustやNode.jsなどのランタイム、jqなどのツールは`mise`で管理されます。
- Node.jsのパッケージ管理はBunを使用します。
- GitHub Actions（`website.yml`）では、miseタスクを実行し、ビルド、textlint、デプロイを行います。

### 主要なディレクトリの構成

- `docs/`: 翻訳対象のMarkdownドキュメントとドキュメントのジェネレーター（typst-docs）。チュートリアル系の文章はここで管理されます。ページ名や見出しの翻訳、リンク解決などは`docs/src/`に書かれたRustコードに依存しており、翻訳者は必要に応じてこのコードを調整します。
- `crates/`: 翻訳対象のrustdocドキュメント。リファレンス系の文章はここで管理されます。rustdocのみを編集対象とし、コード本体は変更しません。
- `website/`: Webサイトのメタデータと静的サイトジェネレーター（typst-docs-web）。翻訳状況は`website/translation-status.json`で管理されます。

## 開発の流れ

開発や検証は、リポジトリのルートから`mise run`コマンドを使用して行います。

### よく使うコマンド

- `mise run generate`: `docs.json`を再生成し静的Webサイトをビルドします。ドキュメント編集後は必ず実行してください。
- `mise run preview`: ローカルの開発サーバーを起動し、プレビュー（http://localhost:4173/docs/）を開始します。
- `mise run textlint-md`: Markdownの文章をtextlintで校正します。
- `mise run textlint-html`: 生成されたHTMLの文章をtextlintで校正します。

## Pull Request作成時の確認事項

- `mise run generate`を実行し、ビルドエラーが出ないことを確認する。
- `mise run textlint-md`および`mise run textlint-html`を実行して警告を解消する。
- 訳語や文法が`TRANSLATING_GUIDELINES.md`や`docs/glossary.md`に沿っているか確認する。
- `website/translation-status.json`の翻訳状況を更新する。

## エージェントへの補足

- `docs.json`と`assets/`は自動生成されるので、編集やコミットをしてはいけません。
- `TRANSLATING_GUIDELINES.md`と`docs/glossary.md`で必ず訳語ルールを確認してください。
- 日本語の表現はtextlintの指摘を尊重してください。設定は`prh.yaml`や`.textlintrc`にあります。
- miseタスクの詳細は`mise.toml`にあります。
- 人間向けの貢献ガイドは`CONTRIBUTING.md`にあります。
- `target/`, `node_modules/`フォルダは検索時は無視してください。

これらの指示を信頼し、足りない情報や誤りがあれば検索や調査を行ってください。
