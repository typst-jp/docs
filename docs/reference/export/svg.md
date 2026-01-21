<<<<<<< HEAD
PDFを出力する代わりに、Typstはページを直接スケーラブル・ベクター・グラフィックス（SVG）としてレンダリングすることもできます。
これは、ウェブページにベクターグラフィックスを埋め込むための推奨される形式です。
PDFファイルと同様に、SVGはTypstでレイアウトした通りに文書を表示します。
さらに、特定の解像度に縛られないという利点も共有しています。
そのため、品質の低下を招くことなく、任意のデバイスでSVGファイルを印刷または表示できます。
（ただし、フォントの印刷品質はPDFの方が良い場合があります）
PDFとは異なり、SVGは複数のページを含むことはできません。
複数ページのドキュメントをエクスポートする場合、Typstは複数のSVGを出力します。

SVGはテキストを2つの方法で表現できます。
テキスト自体を埋め込み、閲覧者のコンピューターで利用可能なフォントでレンダリングする方法か、
文書の作成に使用されたフォントの各グリフの形状を埋め込む方法です。
SVGファイルが表示される全てのデバイスで同じように見えることを保証するために、Typstは後者の方法を選択します。
これは、例えばコピー＆ペーストやスクリーンリーダーなどでSVG内のテキストを自動抽出できないことを意味します。
テキストのアクセシビリティが必要な場合は、
代わりにPDFまたはHTMLファイルをエクスポートしてください。

SVGは透明な背景を持つことができます。
デフォルトでは、Typstは不透明な白背景のSVGを出力します。
`[#set page(fill: none)]`を使用して背景を透明にすることができます。
詳細は[`page`関数のリファレンスページ]($page.fill)を確認してください。

# SVG形式でのエクスポート
## コマンドライン
`compile`または`watch`サブコマンドで`--format svg`を指定するか、
`.svg`で終わる出力ファイル名を指定してください。

文書が複数ページからなる場合、Typstは複数の画像ファイルを生成します。
このとき、出力ファイル名は次のうち少なくとも1つを含むテンプレート文字列でなければなりません。
- `[{p}]`はページ番号に置き換えられます。
- `[{0p}]`は（全ての番号が同じ長さになるように）
  ゼロ埋めされたページ番号に置き換えられます。
- `[{t}]`は総ページ数に置き換えられます。

SVG形式でエクスポートする際には、以下の設定オプションが指定可能です。

- `--pages`の後に、カンマ区切りのページ番号またはダッシュによる番号範囲を指定することで、エクスポートするページを指定します。
  範囲指定は半開区間にすることもできます。
  例：`2,3,7-9,11-`。

## Webアプリ
「File」>「Export as」>「SVG」をクリックするか、
クイックダウンロードボタンの横にある下向き矢印をクリックして「Export as SVG」を選択します。
SVG形式でエクスポートする際には、以下の設定項目を指定できます。

- エクスポートするページ。有効なオプションは「All pages（全てのページ）」、「Current page（現在のページ）」、および「Custom ranges（カスタム範囲）」です。
  カスタム範囲は、カンマ区切りの番号リストまたはダッシュで区切られた番号範囲です。
  範囲は半開区間にすることもできます。例：`2,3,7-9,11-`。
=======
Instead of creating a PDF, Typst can also directly render pages to scalable
vector graphics (SVGs), which are the preferred format for embedding vector
graphics in web pages. Like PDF files, SVGs display your document exactly how
you have laid it out in Typst. Likewise, they share the benefit of not being
bound to a specific resolution. Hence, you can print or view SVG files on any
device without incurring a loss of quality. (Note that font printing quality may
be better with a PDF.) In contrast to a PDF, an SVG cannot contain multiple
pages. When exporting a multi-page document, Typst will emit multiple SVGs.

SVGs can represent text in two ways: By embedding the text itself and rendering
it with the fonts available on the viewer's computer or by embedding the shapes
of each glyph in the font used to create the document. To ensure that the SVG
file looks the same across all devices it is viewed on, Typst chooses the latter
method. This means that the text in the SVG cannot be extracted automatically,
for example by copy/paste or a screen reader. If you need the text to be
accessible, export a PDF or HTML file instead.

SVGs can have transparent backgrounds. By default, Typst will output an SVG with
an opaque white background. You can make the background transparent using
`[#set page(fill: none)]`. Learn more on the
[`page` function's reference page]($page.fill).

# Exporting as SVG
## Command Line
Pass `--format svg` to the `compile` or `watch` subcommand or provide an output
file name that ends with `.svg`.

If your document has more than one page, Typst will create multiple image files.
The output file name must then be a template string containing at least one of
- `[{p}]`, which will be replaced by the page number
- `[{0p}]`, which will be replaced by the zero-padded page number (so that all
  numbers have the same length)
- `[{t}]`, which will be replaced by the total number of pages

When exporting to SVG, you have the following configuration options:

- Which pages to export by specifying `--pages` followed by a comma-separated
  list of numbers or dash-separated number ranges. Ranges can be half-open.
  Example: `2,3,7-9,11-`.

## Web App
Click "File" > "Export as" > "SVG" or click the downwards-facing arrow next to
the quick download button and select "Export as SVG". When exporting to SVG, you
have the following configuration options:

- Which pages to export. Valid options are "All pages", "Current page", and
  "Custom ranges". Custom ranges are a comma-separated list of numbers or
  dash-separated number ranges. Ranges can be half-open. Example: `2,3,7-9,11-`.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
