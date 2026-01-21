<div class="info-box">

<<<<<<< HEAD
TypstのHTMLエクスポートは現在積極的に開発が行われています。
この機能はまだ非常に不完全であり、機能フラグを有効にした場合に、実験目的でのみ利用可能です。
本番環境における利用ではこの機能を使用しないでください。
CLIでは`--features html`オプションを指定するか環境変数`TYPST_FEATURES`に`html`を設定することでHTMLエクスポートを試すことができます。
Webアプリでは、現時点でHTMLエクスポートは利用できません。
HTMLエクスポートに関する進捗を追跡し、計画されている機能について詳しく知るには、
[tracking issue](https://github.com/typst/typst/issues/5512)を参照してください。
</div>

HTMLファイルでは文書を構造的に記述します。
TypstのHTMLエクスポートの目的は、入力文書の構造を捉え、
この構造を保持する、意味付けが豊かなHTMLを生成することです。
生成されるHTMLは、アクセシビリティに優れ、人間が読みやすく、手作業での編集や下流のツールによる処理が可能であるべきです。

対照的に、PDF、PNG、SVGエクスポートは全て、
完全にレイアウト済みの文書の _視覚的_ 表現を生成します。
このようなフォーマットの意図の違いにより、
Typstは既存のTypst文書に対して完璧なHTMLを単純に生成することはできません。
Typstがあなたのコンテンツにとって最適に意味付けされたHTML表現が何であるかを常に知ることはできないためです。

代わりに、Typstは _あなた_ に完全な制御を委ねます。
[`target`]($target)関数で現在のエクスポート形式を確認でき、これがHTMLに設定されている場合、
[生のHTML要素]($html.elem)を生成できます。
これらの要素は主にテンプレートやshowルールでの使用を想定しています。
このようにすることで、
文書の内容はエクスポート先に依存せず、PDFとHTMLの両方で共有できます。

現時点では、
Typstは常に単一のHTMLファイルを出力します。
複数のHTML文書やアセットを含むディレクトリの出力、
他のHTML文書に統合できるフラグメント出力は今後の実装が予定されています。

Typstは現状ではCSSスタイルシートを出力せず、意味付けされたマークアップの生成に重点を置いています。
もちろん、独自にCSSスタイルを書くこともでき、
そうすればPDFとHTMLの間で _コンテンツ_ を共有する利点をそのまま享受できます。
将来的には、既存のsetルールをより多く考慮に入れた上で、
CSSを自動的に出力するオプションを提供する予定です。

# HTML形式でのエクスポート
## コマンドライン
`compile`または`watch`サブコマンドに`--format html`オプションを指定するか、`.html`で終わる出力ファイル名を指定します。
この実験的なエクスポートターゲットを有効にするには、`--features html`オプションを指定するか、
環境変数`TYPST_FEATURES=html`を設定する必要があることに注意してください。

`typst watch`を使用すると、Typstはライブリロード対応のHTTPサーバーを起動します。
設定は以下の方法で可能です。

- `--port`オプションでポート番号を変更します。
（デフォルトでは3000-3005の範囲で最初に利用可能なポートが使用されます）。
- `--no-reload`オプションでライブリロードスクリプトの注入を無効にします。
（いずれにせよディスクに書き込まれるHTMLには影響しません）。
- `--no-serve`オプションでサーバー機能が完全に無効になります。

## Webアプリ
現在利用できません。

# HTML固有の機能
Typstでは、グローバルな`html`モジュールを通じてHTMLに固有の機能を提供しています。
そのモジュールに含まれる定義については、以下を参照してください。
=======
Typst's HTML export is currently under active development. The feature is still
very incomplete and only available for experimentation behind a feature flag. Do
not use this feature for production use cases. In the CLI, you can experiment
with HTML export by passing `--features html` or setting the `TYPST_FEATURES`
environment variables to `html`. In the web app, HTML export is not available at
this time. Visit the [tracking issue](https://github.com/typst/typst/issues/5512)
to follow progress on HTML export and learn more about planned features.
</div>

HTML files describe a document structurally. The aim of Typst's HTML export is
to capture the structure of an input document and produce semantically rich HTML
that retains this structure. The resulting HTML should be accessible,
human-readable, and editable by hand and downstream tools.

PDF, PNG, and SVG export, in contrast, all produce _visual_ representations of a
fully-laid out document. This divergence in the formats' intents means that
Typst cannot simply produce perfect HTML for your existing Typst documents. It
cannot always know what the best semantic HTML representation of your content
is.

Instead, it gives _you_ full control: You can check the current export format
through the [`target`] function and when it is set to HTML, generate [raw HTML
elements]($html.elem). The primary intended use of these elements is in
templates and show rules. This way, the document's contents can be fully
agnostic to the export target and content can be shared between PDF and HTML
export.

Currently, Typst will always output a single HTML file. Support for outputting
directories with multiple HTML documents and assets, as well as support for
outputting fragments that can be integrated into other HTML documents is
planned.

Typst currently does not output CSS style sheets, instead focussing on emitting
semantic markup. You can of course write your own CSS styles and still benefit
from sharing your _content_ between PDF and HTML. For the future, we plan to
give you the option of automatically emitting CSS, taking more of your existing
set rules into account.

# Exporting as HTML
## Command Line
Pass `--format html` to the `compile` or `watch` subcommand or provide an output
file name that ends with `.html`. Note that you must also pass `--features html`
or set `TYPST_FEATURES=html` to enable this experimental export target.

When using `typst watch`, Typst will spin up a live-reloading HTTP server. You
can configure it as follows:

- Pass `--port` to change the port. (Defaults to the first free port in the
  range 3000-3005.)
- Pass `--no-reload` to disable injection of a live reload script. (The HTML
  that is written to disk isn't affected either way.)
- Pass `--no-serve` to disable the server altogether.

## Web App
Not currently available.

# HTML-specific functionality
Typst exposes HTML-specific functionality in the global `html` module. See below
for the definitions it contains.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
