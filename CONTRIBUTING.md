# 翻訳ガイドライン

> [!NOTE]
> 当プロジェクトの[README](./README.md)や「[はじめに：Typst Japan Communityより](https://typst-jp.github.io/docs/)」、[Typst公式](https://typst.app/)の[ライセンス](https://github.com/typst/typst/blob/main/LICENSE)や[コントリビューション・ガイド](https://github.com/typst/typst/blob/main/CONTRIBUTING.md)も併せてご参照ください。

Typst日本語ドキュメント翻訳プロジェクトにご興味をお持ちいただき、どうもありがとうございます。

このプロジェクトは、[Typst GmbH](https://typst.app/legal/)の許諾を得て、最新の[公式のドキュメント](https://typst.app/docs/)より翻訳を行うことで、非公式な日本語ドキュメントを提供することを目的としています。まさに、あなたのようなボランティアの皆様のご協力の元、成り立っています。当ガイドラインをご一読の上、翻訳・校正・提案及びその他の作業にご参加いただければ幸いです。

この翻訳ガイドラインは、翻訳に参加する皆様に、翻訳の進め方に対する説明やより良質な翻訳を行うためのガイダンスを提供します。

## 翻訳の進め方

翻訳は[GitHub上の当リポジトリ](https://github.com/typst-jp/typst-jp.github.io)を中心に行います。実際の翻訳作業やそれに対する議論や提案などは、主にGitHubの[Issue](https://github.com/typst-jp/typst-jp.github.io/issues)や[Pull Request](https://github.com/typst-jp/typst-jp.github.io/pulls)機能を通じて行います。また、[Discordサーバー「くみはんクラブ」](https://discord.gg/9xF7k4aAuH)の`#typst-翻訳`チャンネルでも、質問の対応や合意の形成などを行うことがあります。

### 翻訳提案の手順

1. このGitHubリポジトリをフォークします。
2. ドキュメントの実体は、主にMarkdownおよびYAMLの2種類のファイルから構成されています。それぞれ、下記の注意書きに従って翻訳作業をお願いします。
    1. `./docs/i18n/**/`内のYAMLファイル群は、Typstの言語リファレンスの本体です。その中に含まれている、**既存の`*-ja.yaml`ファイルを直接書き換えて翻訳してください**。**`*-en.yaml`や`*-zh.yaml`は翻訳しないでください**。
        - 例：[Reference > Model](https://typst.app/docs/reference/model/)を翻訳する際は、`./docs/i18n/category/model-ja.yaml`を編集してください。`model-en.yaml`や`model-zh.yaml`は放置してください。
    2. `./docs`内のMarkdownファイル群は、Typstのチュートリアルや入門ガイドなど、言語リファレンス以外のページの本体です。**既存のMarkdownファイルを直接書き換えて翻訳してください**。
    それに加えて、`./docs/src/lib.rs`ファイルの[`urlify`関数](https://github.com/search?q=repo%3Atypst-jp/typst-jp.github.io%20urlify&type=code)を編集して、中国語版の記事タイトルを日本語版のものに書き換えてください。このプロセスを抜かすと、WebページのURLが正しく生成されません。
    3. 「サードパーティパッケージ」のページの翻訳を追加する場合は、`./static/assets/index2ja.json`も編集する必要があります。
3. 翻訳の際は、[後述のガイドライン](#スタイルマニュアル)を参照し、[v0.12.0時点での公式ドキュメント](https://github.com/typst/typst/tree/v0.12.0/docs)から翻訳してください。
4. 翻訳作業の途中でも、Draft Pull Requestを作成して、翻訳の進捗状況を共有することができます。
5. 翻訳作業が終わったら、Pull Requestを作成し、送信してください。

ご質問などがある場合は、[「くみはんクラブ」のDiscordサーバー](https://discord.gg/9xF7k4aAuH)に参加してご連絡ください。

もちろん、Discordサーバーに参加していない方からのPull Requestも大いに歓迎します。

### 技術的な詳細

[中国語版](https://github.com/typst-doc-cn/typst-doc-cn.github.io?tab=readme-ov-file#%E6%8A%80%E6%9C%AF%E7%BB%86%E8%8A%82)を参照してください。

### ローカル環境でWebページを生成する

当プロジェクトは[typst/typst](https://github.com/typst/typst/)の派生リポジトリであり、Webページを生成するためにはRustが必要です。導入していない場合は、[Install Rust - Rust Programming Language](https://www.rust-lang.org/tools/install)に従ってインストールしてください。

また、当プロジェクトが独自に導入しているRustを除く開発ツールおよびコマンドは[mise](https://mise.jdx.dev/)で一元管理しています。導入していない場合は、[Getting Started | mise-en-place](https://mise.jdx.dev/getting-started.html)に従ってインストールしてください。

#### TL;DR

> [!NOTE]
> こちらの説明は要約版です。詳細を知りたい場合は、次以降の見出しを参照してください。

当プロジェクトのルートディレクトリに移動し、以下のコマンドを実行します。このコマンドは初回のみ実行する必要があります。

```sh
mise trust
mise install
```

Webサイトを再生成するには、以下のコマンドを実行します。

```sh
mise run generate
```

Webサイトをローカルサーバーでプレビューするには、以下のコマンドを実行します。

```sh
mise run preview
```

#### miseによる開発環境のセットアップ

> [!NOTE]
> 以下の内容はmise v2024.11.5に基づいています。内容の不備を発見した場合は、Issueを立ててください。

miseが導入されている環境で初めて当プロジェクトのルートディレクトリに移動すると、以下のように構成ファイルを信頼することを求められます。

```plaintext
mise ERROR Config file /path/to/typst-jp.github.io/.mise.toml is not trusted.
Trust it with `mise trust`.
mise ERROR Run with --verbose or MISE_VERBOSE=1 for more information
```

この場合は、指示に従って`mise trust`を実行してください。`mise trust`の詳細は、[mise trust | mise-en-place](https://mise.jdx.dev/cli/trust.html)を参照してください。

```sh
mise trust
```

miseの設定ファイルでPythonのvirtualenvの自動アクティベートを有効にしていますが、この時点では`.venv/`ディレクトリが作成されていないため、WARNが発生するはずです。

```plaintext
mise trusted /path/to/typst-jp.github.io/.mise.toml
mise WARN  no venv found at: /path/to/typst-jp.github.io/.venv

To create a virtualenv manually, run:
python -m venv /path/to/typst-jp.github.io/.venv
```

次に、`mise install`を実行して、miseで管理されているツールをインストールおよびアクティベートします。

```sh
mise install
```

#### TypstのソースコードからドキュメントデータのJSONファイルを生成する

ドキュメントデータのJSONファイルは、typst-docsによりTypstのソースコード内のコメントおよび`docs/`にあるMarkdownファイル群から生成されます。

`mise run generate-docs`を実行すると、ドキュメントデータのJSONファイルが`assets/docs.json`に生成されます。

```sh
mise run generate-docs
```

#### ドキュメントデータのJSONファイルからWebサイトを生成する

Webサイトの生成にはPythonとJinja2を使用しています。また、パッケージ管理にuvを使用しています。

`mise run generate-web`を実行すると、`assets/docs.json`を基にWebサイトのデータが`dist/`に生成されます。

```sh
mise run generate-web
```

生成されたWebサイトをプレビューするには、`mise run preview`を実行します。

```sh
mise run preview
```

#### Webサイトの生成までを一括で行う

`mise run generate`を実行すると、`generate-docs`および`generate-web`を一括で実行します。

```sh
mise run generate
```

#### Dev Containerについて

> [!WARNING]
> Dev Containerは現在miseに対応していません。そのため、Dev Containerを使用する場合は、`.mise.toml`を参考に手動で上記の手順を実行してください。

上記のローカル環境を構築するDockerfileも整備しております。詳細は[.devcontainer/README.md](.devcontainer/README.md)をご参照ください。

## スタイルマニュアル

スタイルマニュアルでは、当プロジェクトにおける翻訳の品質確保のための、統一したスタイルの参照基準を提供します。具体的には、基本、文体、表記、用語の4つの観点から、翻訳の際に留意すべき事項を示します。

本スタイルマニュアルは絶対的なルールではなく、翻訳全体の整合性を保つための基本方針として提供しているものです。そのため、本マニュアルの内容に必ず従う義務はなく、ケース・バイ・ケースで適用して翻訳を行ってください。本マニュアルの内容に疑問がある場合は、IssueやPull Requestなどで他の翻訳者に意見を求めることもできます。

### 基本

1. 翻訳は、原則として説明文章や表などに限ります。コードやコマンドなどの技術的な表現は、原文のままとします。
3. コード記述例の中に出てくる英文のコメントは、日本語に翻訳する必要はありません。
4. 既存の翻訳を参照し、一貫性を保つようにしてください。
5. 疑問点、不明点などがある場合は、必要に応じて、積極的にIssuesやDiscordなどで議論・相談してください。
6. 構成や段分けなどについては、原文の構成をなるべく保つようにしてください。

### 文体

文体については、以下のガイドラインに従ってください。

1. 基本的に「です」「ます」調の敬体を使用すること。ただし、引用、見出し、箇条書きなどに関しては、その限りではありません。
2. 一般的に用いられる現代日本語共通語に基づき、平易的な表現を心がけること。

### 表記

約物および日本語の表記については、以下のルールに従ってください。

1. 和欧混植がなされている訳文において、和文と欧文の間には半角スペースを手動で挿入しないこと。
2. 和文において、句点は「。」を、読点は「、」を使用し、他の記号も原則として全角を使用すること。
3. コロン「：」やセミコロン「；」は、原則として使用しないこと。ただし、「例：」など、文中以外で用いる場合はその限りではありません。
4. 数字や欧文（ラテン文字、キリル文字、ギリシア文字など）は、原則として半角を使用すること。
5. 原則、現代仮名遣いおよび常用漢字表に基づいた表記を使用すること。送り仮名や仮名書き、ひらがな・カタカナの使い分けは、一般的な書き方に従ってください。ただし、引用、特定の用語や固有名詞については、その限りではありません。

### 用語

用語については、以下のガイドラインに従ってください。

1. [用語集](https://typst-jp.github.io/docs/glossary/)を参照すること。
    1. 用語集にあってかつ適切と思われる場合は、その通りに翻訳してください。
    2. 用語集にあっても不適切と思われる場合は、IssueやDiscordで相談してください
    3. 必要と思われるのに用語集にない場合は、既存の翻訳を参照し、追加を提案してください。
2. 用語と用語でないものを、柔軟に見分けて訳し分けること。
3. 現代日本語の一般的な、わかりやすい用語を使用すること。

### 参考

* [JTFスタイルガイド](https://www.jtf.jp/tips/styleguide)
* [ウィキペディア日本語版のスタイルマニュアル](https://ja.wikipedia.org/wiki/Wikipedia:%E3%82%B9%E3%82%BF%E3%82%A4%E3%83%AB%E3%83%9E%E3%83%8B%E3%83%A5%E3%82%A2%E3%83%AB)
* [ウィキペディア日本語版の表記ガイド](https://ja.wikipedia.org/wiki/Wikipedia:%E8%A1%A8%E8%A8%98%E3%82%AC%E3%82%A4%E3%83%89)
* [Microsoft Localization Style Guides](https://learn.microsoft.com/ja-jp/globalization/reference/microsoft-style-guides)
* [WordPress 翻訳ハンドブック](https://ja.wordpress.org/team/handbook/translation/)
* [Vue.js 公式サイト日本語翻訳ガイド](https://github.com/vuejs-translations/docs-ja/blob/main/.github/CONTRIBUTING.md)
* [ja.react.dev 翻訳スタイルガイド](https://github.com/reactjs/ja.react.dev/wiki/%E7%BF%BB%E8%A8%B3%E3%82%B9%E3%82%BF%E3%82%A4%E3%83%AB%E3%82%AC%E3%82%A4%E3%83%89)
