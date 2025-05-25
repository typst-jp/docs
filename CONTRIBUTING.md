# 貢献ガイドライン

> [!NOTE]
> 当プロジェクトの[README](./README.md)や「[はじめに：Typst Japan Communityより](https://typst-jp.github.io/docs/)」、[Typst公式](https://typst.app/)の[ライセンス](https://github.com/typst/typst/blob/main/LICENSE)や[コントリビューション・ガイド](https://github.com/typst/typst/blob/main/CONTRIBUTING.md)も併せてご参照ください。

Typst日本語ドキュメント翻訳プロジェクトにご興味をお持ちいただき、どうもありがとうございます。

このプロジェクトは、[Typst GmbH](https://typst.app/legal/)の許諾を得て、最新の[公式のドキュメント](https://typst.app/docs/)より翻訳を行うことで、非公式な日本語ドキュメントを提供することを目的としています。まさに、あなたのようなボランティアの皆様のご協力の元、成り立っています。当ガイドラインをご一読の上、翻訳・校正・提案およびその他の作業にご参加いただければ幸いです。

## 翻訳の進め方

翻訳は[GitHub上の当リポジトリ](https://github.com/typst-jp/typst-jp.github.io)を中心に行います。実際の翻訳作業やそれに対する議論や提案などは、主にGitHubの[Issue](https://github.com/typst-jp/typst-jp.github.io/issues)や[Pull Request](https://github.com/typst-jp/typst-jp.github.io/pulls)機能を通じて行います。また、[Discordサーバー「くみはんクラブ」](https://discord.gg/9xF7k4aAuH)の`#typst-翻訳`チャンネルでも、質問の対応などが可能です。

### 翻訳提案の手順

> [!WARNING]
> ここに記載されている内容は改訂中です。現在の手順は最新の[Pull Request](https://github.com/typst-jp/typst-jp.github.io/pulls?q=sort%3Aupdated-desc+is%3Apr+is%3Aopen)を参照してください。

1. このGitHubリポジトリをフォークします。
2. ドキュメントの実体は、主にMarkdownファイルおよびコンパイラのソースコード内のコメントの2種類から構成されています。それぞれ、下記の注意書きに従って翻訳作業をお願いします。
    1. `./crates/typst-library/src/`内の`.rs`ファイル群は、Typstのコンパイラのソースコードです。ソースコード内に含まれている、**既存のコメントを直接書き換えて翻訳してください**。
        - 例1：[Reference > Foundations](https://typst.app/docs/reference/foundations/)を翻訳する際は、`./crates/typst-library/src/foundations/mod.rs`のコメントを編集してください。
        - 例2：[Reference > Foundations > Arguments](https://typst.app/docs/reference/foundations/arguments/)を翻訳する際は、`./crates/typst-library/src/foundations/args.rs`のコメントを編集してください。
    2. `./docs`内のMarkdownファイル群は、Typstのチュートリアルや入門ガイドなど、言語リファレンス以外のページの本体です。**既存のMarkdownファイルを直接書き換えて翻訳してください**。
    3. 上記いずれの場合においても、[website/translation-status.json](/website/translation-status.json)の該当箇所を`"translated"`に変更してください。
3. 翻訳の際の文体や表記は[翻訳ガイドライン](./TRANSLATING_GUIDELINES.md)を参照してください。ドキュメントの最新バージョンへの追従は管理者が一括で行っているため、日本語ドキュメントと公式ドキュメントのバージョンが異なる場合でも、日本語ドキュメントで管理されている原文を優先してください。
4. 翻訳作業の途中でも、Draft Pull Requestを作成して、翻訳の進捗状況を共有することもできます。
5. 翻訳作業が終わったら、Pull Requestを作成し、送信してください。

ご質問などがある場合は、[「くみはんクラブ」のDiscordサーバー](https://discord.gg/9xF7k4aAuH)に参加してご連絡ください。

もちろん、Discordサーバーに参加していない方からのPull Requestも大いに歓迎します。

### 技術的な詳細

[`./website/`のREADME](https://github.com/typst-jp/typst-jp.github.io/blob/main/website/README.md)を参照してください。

### ローカル環境でWebページを生成する

当プロジェクトの開発ツールおよびコマンドは[mise](https://mise.jdx.dev/)で一元管理しています。導入していない場合は、[Getting Started | mise-en-place](https://mise.jdx.dev/getting-started.html)に従ってインストールしてください。

コマンドラインでの操作を避けたい方や[Docker](https://docs.docker.com/)で作業したい方へ向けて、[Dev Containerの環境](#dev-containerによる開発環境のセットアップ)もご用意しております。

> [!NOTE]
> Windowsのネイティブ環境で実行する場合には、[開発者モード](https://learn.microsoft.com/ja-jp/windows/apps/get-started/enable-your-device-for-development)に設定する必要があります。
> その他Windowsに起因する潜在的なトラブルを回避するため、Windowsユーザーには[WSL](https://learn.microsoft.com/ja-jp/windows/wsl/install)やDev Containerの使用を推奨します。

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
> 以下の内容はmise v2025.5.6に基づいています。内容の不備を発見した場合は、Issueを立ててください。

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

Webサイトの生成にはNode.jsとViteとHonoを使用しています。また、パッケージ管理にBunを使用しています。

`mise run generate-web`を実行すると、`assets/docs.json`を基にWebサイトのデータが`website/dist/`に生成されます。

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

#### Dev Containerによる開発環境のセットアップ

Dockerコンテナー上に上記と同一の環境を構築して作業することも可能です。
以下の操作はDockerがインストール済み、かつDockerデーモンを起動していることが前提となります。
このプロジェクトでは[Dev Container](https://code.visualstudio.com/docs/devcontainers/containers)もご使用いただけます。
Visual Studio Codeにおける操作フロー例は以下の通りです。

1. Ctrl+Shift+Pを押してから`> Dev Containers: Reopen in Container`を実行します。
2. Webサーバーが起動したらブラウザで http://localhost:5173 にアクセスします。
3. 翻訳したファイルの変更を反映させるためにはCtrl+Shift+Bで再ビルドしてください。
4. 体裁を確認したい場合、Ctrl+Shift+Pを押してから`> Tasks: Run task`を実行し以下のいずれかを選択します。
    - `textlint-md` : Markdownファイルを翻訳した場合
    - `textlint-html` : Rustソースコードを翻訳した場合
5. 自動修正を実施したい場合も同様に以下から選択します。
    - `textlint-md:fix` : Markdownファイルを自動修正します。
    - Rustコードの自動修正は対応していなため、該当箇所を手動で修正してください。

`> Tasks: Run task`はDocker環境でなく、miseで環境構築した際にも使用できます。
