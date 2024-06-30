# Typst 日本語ドキュメント (非公式)

[![CI/CD for Documentation](https://github.com/typst-jp/typst-jp.github.io/actions/workflows/deploy.yml/badge.svg?branch=main&event=push)](https://github.com/typst-jp/typst-jp.github.io/actions/workflows/deploy.yml)

組版システム [Typst](https://typst.app/docs) の非公式な日本語ドキュメントです。[Typst GmbH](https://typst.app/legal/) の許諾を得て作成されています。

このリポジトリは[中国語版](https://github.com/typst-doc-cn/typst-doc-cn.github.io)からフォークして作成され、2024年6月時点での最新版である [Typst v0.11.1](https://typst.app/docs/changelog/#v0.11.1) の公式ドキュメントを元に日本語訳を行います。

実際に作動している Web 版は、以下の URL から閲覧できます。
> https://typst-jp.github.io/docs/

## 翻訳に参加するには

1. この GitHub リポジトリをフォークします。
1. ドキュメントの実体は、Markdown および Yaml の2種類のファイルから構成されています。それぞれ、下記の注意書きに従って翻訳作業をお願いします。
    1. `./docs/i18n/**/` 内の Yaml ファイル群は、Typst の言語リファレンスの本体です。その中に含まれている、**既存の `*-ja.yaml` ファイルを直接書き換えて翻訳してください**。**`*-en.yaml` や `*-zh.yaml` は翻訳しないでください**。
        - 例: https://typst.app/docs/reference/model/ を翻訳する際は、`./docs/i18n/category/model-ja.yaml` を編集してください。`model-en.yaml` や `model-zh.yaml` は放置してください。
    2. `./docs` 内の Markdown ファイル群は、Typst のチュートリアルや入門ガイドなど、言語リファレンス以外のページの本体です。**既存の Markdown ファイルを直接書き換えて翻訳してください**。
    それに加えて、`./crates/typst-docs/src/lib.rs` ファイルの [`urlify` 関数](https://github.com/search?q=repo%3Atypst-jp/typst-jp.github.io%20urlify&type=code) を編集して、中国語版の記事タイトルを日本語版のものに書き換えてください。このプロセスを抜かすと、Web ページの URL が正しく生成されません。
1. 「サードパーティ パッケージ」のページの翻訳を追加する場合は、`./static/assets/index2ja.json` も編集する必要があります。
1. 翻訳の際は、以下のガイドラインを遵守するようにしてください。
    1. [v0.11.1 時点での公式ドキュメント](https://github.com/typst/typst/tree/v0.11.1/docs)から翻訳すること。
    1. 和文と欧文の間には半角スペースを挿入すること。
    1. 句読点は「, .」ではなく、和文の「、。」を使用すること。
    1. 不明な用語については、用語集または他のページの翻訳も参照すること。必要に応じて、Discord サーバ (後述) や Issue で相談すること。
    1. Typst のコード記述例の中に出てくる英文は、日本語に翻訳する必要はありません。
1. 翻訳作業が終わったら、Pull Request を送信してください。

ご質問などがある場合は、[「くみはんクラブ」のDiscordサーバー](https://discord.gg/9xF7k4aAuH)に参加してご連絡ください。

もちろん、Discord サーバに参加していない方からの Pull Request も大いに歓迎します。

## 技術的な詳細

[中国語版](https://github.com/typst-doc-cn/typst-doc-cn.github.io?tab=readme-ov-file#%E6%8A%80%E6%9C%AF%E7%BB%86%E8%8A%82)を参照してください。

## ローカル環境でドキュメントを生成する

変更した Markdown/Yaml ファイルから、ローカル環境で Web サイトのデータを生成することも可能です。翻訳の際にこの作業は必須ではありませんが、書き換えたファイルが Web ページとして正しく表示されるのか確認するのに役立ちます。

まず、このリポジトリのクローンを作成し、`cargo` ツールチェーン、Python および Python パッケージの `jinja2` と `pyyaml` をインストールする必要があります。
```
# `./docs` 以下のディレクトリを変更した場合は、次の 2 行のコマンドを実行する必要があります
cargo test --package typst-docs --lib -- tests::test_docs --exact --nocapture
# `./docs/i18n` ディレクトリのみを変更した場合は、このコマンド行を実行するだけで済みます
python ./gen.py
```

最終的にコンパイルされたファイルは `./dist` にあります。

Node.js がインストールされている場合は、`npx serve ./dist` でプレビューできます。
