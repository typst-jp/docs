<<<<<<< HEAD
PDFを作成する代わりに、Typstはページを直接PNG形式のラスター画像にレンダリングすることもできます。
PNGは可逆圧縮の画像形式で、1枚に1ページを含むことができます。
複数ページの文書をエクスポートする場合、Typstは複数のPNGを出力します。
PNGは、Typstの出力を画像編集ソフトウェアで編集したい場合や、
Typstの他のエクスポート形式が使用できない場合に適しています。

Typstの他のエクスポート形式とは異なり、PNGの解像度は特定のものに固定されます。
PNGにエクスポートする際、解像度をインチあたりのピクセル数（PPI）として設定できます。
PNGを閲覧する媒体の解像度がエクスポートしたPNGの解像度を上回る場合、
画質の低下が見て取れます。
Typstは各ページの物理的な寸法とPPIに基づいてPNGの解像度を計算します。
PPI値を選ぶ際の目安が必要であれば、以下を参考にしてください。

- デスクトップ印刷では、300または600の値が一般的です。
- 精細なグラフィックの業務用印刷では1200PPIまで用いられることがあります。
- ポスターなどの離れた距離のみから閲覧される文書の場合、
  300未満の値を選択してもよいでしょう。
- 画面上で閲覧される文書の場合、
  スマートフォンの一般的なPPI値は400-500です。

PNGはピクセルのラスターのみを含むため、
例えばコピー＆ペーストやスクリーンリーダーなどでテキストを自動抽出することは（OCRを用いない限り）できません。
テキストのアクセシビリティが必要な場合は、代わりにPDFまたはHTMLファイルをエクスポートしてください。

PNGは透明な背景を持つことができます。
デフォルトでは、Typstは不透明な白背景のPNGを出力します。
`[#set page(fill: none)]`を使用して背景を透明にすることができます。
詳細は[`page`関数のリファレンスページ]($page.fill)を確認してください。

# PNG形式でのエクスポート
## コマンドライン
`compile`または`watch`サブコマンドで`--format png`を指定するか、
`.png`で終わる出力ファイル名を指定してください。

文書が複数ページからなる場合、Typstは複数の画像ファイルを生成します。
このとき、出力ファイル名は次のうち少なくとも1つを含むテンプレート文字列でなければなりません。
- `[{p}]`：はページ番号に置き換えられます。
- `[{0p}]`：は（全ての番号が同じ長さになるように）
  ゼロ埋めされたページ番号に置き換えられます。
- `[{t}]`：は総ページ数に置き換えられます。

PNG形式でエクスポートする際には、以下の設定オプションが指定可能です。

- `--ppi`に続けて1インチあたりのピクセル数を指定することで、レンダリング解像度を指定します。
  デフォルトの値は`144`です。

- `--pages`の後に、カンマ区切りのページ番号またはダッシュによる番号範囲を指定することで、エクスポートするページを指定します。
  範囲指定は半開区間にすることもできます。
  例：`2,3,7-9,11-`。

## Webアプリ
「File」>「Export as」>「PNG」をクリックするか、
クイックダウンロードボタンの横にある下向き矢印をクリックして「Export as PNG」を選択します。
PNG形式でエクスポートする際には、以下の設定項目を指定できます。

- 1インチあたりのピクセル数を指定することで、レンダリング解像度を指定します。
  デフォルトの値は`144`です。

- エクスポートするページ。有効なオプションは「All pages（全てのページ）」、「Current page（現在のページ）」、および「Custom ranges（カスタム範囲）」です。
  カスタム範囲は、カンマ区切りの番号リストまたはダッシュで区切られた番号範囲です。
  範囲は半開区間にすることもできます。例：`2,3,7-9,11-`。
=======
Instead of creating a PDF, Typst can also directly render pages to PNG raster
graphics. PNGs are losslessly compressed images that can contain one page at a
time. When exporting a multi-page document, Typst will emit multiple PNGs. PNGs
are a good choice when you want to use Typst's output in an image editing
software or when you can use none of Typst's other export formats.

In contrast to Typst's other export formats, PNGs are bound to a specific
resolution. When exporting to PNG, you can configure the resolution as pixels
per inch (PPI). If the medium you view the PNG on has a finer resolution than
the PNG you exported, you will notice a loss of quality. Typst calculates the
resolution of your PNGs based on each page's physical dimensions and the PPI. If
you need guidance for choosing a PPI value, consider the following:

- A value of 300 or 600 is typical for desktop printing.
- Professional prints of detailed graphics can go up to 1200 PPI.
- If your document is only viewed at a distance, e.g. a poster, you may choose a
  smaller value than 300.
- If your document is viewed on screens, a typical PPI value for a smartphone is
  400-500.

Because PNGs only contain a pixel raster, the text within cannot be extracted
automatically (without OCR), for example by copy/paste or a screen reader. If
you need the text to be accessible, export a PDF or HTML file instead.

PNGs can have transparent backgrounds. By default, Typst will output a PNG with
an opaque white background. You can make the background transparent using
`[#set page(fill: none)]`. Learn more on the
[`page` function's reference page]($page.fill).

# Exporting as PNG
## Command Line
Pass `--format png` to the `compile` or `watch` subcommand or provide an output
file name that ends with `.png`.

If your document has more than one page, Typst will create multiple image files.
The output file name must then be a template string containing at least one of
- `[{p}]`, which will be replaced by the page number
- `[{0p}]`, which will be replaced by the zero-padded page number (so that all
  numbers have the same length)
- `[{t}]`, which will be replaced by the total number of pages

When exporting to PNG, you have the following configuration options:

- Which resolution to render at by specifying `--ppi` followed by a number of
  pixels per inch. The default is `144`.

- Which pages to export by specifying `--pages` followed by a comma-separated
  list of numbers or dash-separated number ranges. Ranges can be half-open.
  Example: `2,3,7-9,11-`.

## Web App
Click "File" > "Export as" > "PNG" or click the downwards-facing arrow next to
the quick download button and select "Export as PNG". When exporting to PNG, you
have the following configuration options:

- The resolution at which the pages should be rendered, as a number of pixels
  per inch. The default is `144`.

- Which pages to export. Valid options are "All pages", "Current page", and
  "Custom ranges". Custom ranges are a comma-separated list of numbers or
  dash-separated number ranges. Ranges can be half-open. Example: `2,3,7-9,11-`.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
