<<<<<<< HEAD
PDFファイルは文書を視覚的に正確な形で記述することに重点を置いていますが、文書構造に注釈をつける機能も備えています。
このハイブリッドなアプローチにより、
PDFファイルは文書交換に適した形式となっています。
どの端末でもまったく同じ見た目で表示される一方、
内容や構造も（少なくともある程度は）抽出できるためです。
PNGファイルとは異なり、PDFファイルは特定の解像度に縛られません。
これによって、品質を損なうことなく任意のサイズでファイルを閲覧できます。

# PDF形式でのエクスポート
## コマンドライン
PDFはTypstのデフォルトのエクスポート形式です。
`compile`または`watch`サブコマンドをフォーマットを指定せずに実行すると、PDFが作成されます。
PDF形式でエクスポートする際には、以下の設定オプションが指定可能です。

- `--pdf-standard`の後に1つまたは複数のカンマ区切りの規格を指定することで、
  Typstが準拠を強制するPDF規格を指定します。
  指定可能な規格は`1.4`, `1.5`, `1.6`, `1.7`, `2.0`, `a-1b`, `a-1a`, `a-2b`,
  `a-2u`, `a-2a`, `a-3b`, `a-3u`, `a-3a`, `a-4`, `a-4f`, `a-4e`, `ua-1`です。
  デフォルトではPDF 1.7に準拠したファイルが出力されます。

- `--no-pdf-tags`を指定すると、PDFのタグ付けを完全に無効化できます。
  デフォルトでは、Typstはベースラインレベルのアクセシビリティを提供するために常に_タグ付きPDF（Tagged PDF）_を出力します。
  このフラグを使用すると、タグ付けをオフにできます。
  これにより、ファイルはアクセシビリティを欠いた状態になり、PDF/Aのアクセシビリティ適合レベルやPDF/UAの全パートへの準拠が妨げられます。

- `--pages`の後に、カンマ区切りのページ番号またはダッシュによる番号範囲を指定して、エクスポートするページを指定します。
  範囲指定は半開区間にもできます。
  例：`2,3,7-9,11-`。

## Webアプリ
右上のクイックダウンロードボタンをクリックすると、デフォルト設定でPDFがエクスポートされます。
さらに設定する場合は、「File」>「Export as」>「PDF」を選択するか、
クイックダウンロードボタンの横にある下向き矢印をクリックして「Export as PDF」を選択します。
PDF形式でエクスポートする際には、以下の設定項目を指定できます。

- Typstが準拠を強制するPDF規格。
  デフォルトではPDF 1.7準拠のファイルが出力されます。
  指定可能な追加規格は`A-1b`, `A-1a`, `A-2b`, `A-2u`, `A-2a`, `A-3b`, `A-3u`,
  `A-3a`, `A-4`, `A-4f`, `A-4e`, `UA-1`です。

- エクスポートするページ。有効なオプションは「All pages（全てのページ）」、「Current page（現在のページ）」、および「Custom ranges（カスタム範囲）」です。
  カスタム範囲は、カンマ区切りの番号リストまたはダッシュで区切られた番号範囲です。
  範囲は半開区間にもできます。例：`2,3,7-9,11-`。

# PDF規格
国際標準化機構（ISO）は、基本となるPDF規格に加え、
特定の用途により適した形でPDFを利用できるようにする各種拡張規格を公開しています。
TypstはデフォルトでPDF 1.7形式のファイルをエクスポートします。
Adobe Acrobat 8以降および一般的に使用されているほとんどのPDFビューアーは
このPDFバージョンとの互換性があります。

一部のTypstの機能は、選択したPDF規格によっては無効化される場合があります。
現在は、PDF/AとPDF/UAを同時に選択できません。

## PDFバージョン
Typstは、1.4、1.5、1.6、1.7（デフォルト）、および2.0の5つの異なるPDFバージョンをサポートしています。
文書のエクスポート時には、これらそれぞれのバージョンを選択できます。
ただし、使用した機能によっては最小バージョンの制限を受けることがあります。
同様に、ターゲットとする規格によっても選択可能なバージョンの制限を受けることがあります（詳細は後述）。

以下は、Typst文書において各新バージョンがPDF 1.4からどのように改善されているかを示すリストです。

- **PDF 1.5**（2003年）：カラーマネジメント、テキスト抽出、表におけるアクセシビリティ、リフロー、および絵文字フォントの改善
- **PDF 1.6**（2004年）：より柔軟なリンク
- **PDF 1.7**（2006年）：[添付ファイル]($pdf.attach)の許可、リフローの改善
- **PDF 2.0**（2017年）：アクセシビリティのためのメタデータとタグのセマンティクスの両方を改善

ファイルを表示するために使用するソフトウェアは、選択したPDFバージョンをサポートしている必要があります。
通常の状況ではこれは問題になりませんが、古いハードウェアを使用している場合はエラーの原因になる可能性があります。
一般的な文書交換においては、デフォルトのPDF 1.7の設定を維持するか、PDF 2.0を選択することをおすすめします。

文書内でPDFファイルを[画像]($image)として使用する場合、エクスポートするPDFバージョンは画像ファイルのバージョン以上である必要があります。

## PDF/UA
TypstはPDF/UA準拠ファイルの出力をサポートしています。
PDF/UAファイルは、_[ユニバーサルアクセス]($guides/accessibility/#basics)_を目的として設計されています。
このPDF規格を選択すると、Typstは文書をエクスポートする際に追加のチェックを実行します。
これらのチェックにより、アクセシビリティのベストプラクティスが遵守されているかが確認されます。
例えば、全ての画像に代替説明が付与されているかどうかが確認されます。

なお、PDF/UAにはアクセシビリティにとって重要でありながら自動的にチェックできないルールもいくつか存在することに注意してください。
そのため、PDF/UA-1文書をエクスポートする際には、以下の点を確認してください。

- 文書が英語以外の言語で書かれている場合、コンテンツの前に[テキストの言語を設定]($text.lang)していること。
- 独自の構造を定義するのではなく、必要に応じてTypstのセマンティック要素（[見出し]($heading)、[図表]($figure)、[リスト]($list)など）を使用していること。
  これにより、その構造が文書内で果たす役割をTypstが認識し、エクスポート時に反映できます。
  詳細については、[アクセシビリティガイド]($guides/accessibility)を参照してください。
- コントラスト、色、フォーマット、またはレイアウトのみを用いて情報を伝達していないこと。
  これらの要素の代わりに、あるいはこれらに加えて、テキストまたは代替説明を使用すること。
- セマンティックな意味を持たない装飾的な要素を全て[`pdf.artifact`]($pdf.artifact)でラップしていること。
- テキストの画像を使用しないこと。
  代わりに、テキストを直接マークアップ内に挿入すること。

Typstは現在、PDF 1.7（2006年）に基づくパート1（PDF/UA-1）のみをサポートしています。
PDF/UA-1にエクスポートする際には、自然言語による[数式の代替説明]($category/math/#accessibility)を手動で提供する必要があることに注意してください。

PDF 2.0（2017年）では、新しいアクセシビリティ機能が追加されました。
PDF 2.0でのエクスポートを設定すると、Typstはこれらの機能のいくつかを活用します。
しかし、PDF 2.0とPDF/UA-1は互換性がありません。
アクセシブルな文書を作成する場合、Typstは追加のチェック機能と幅広い互換性を考慮して、PDF 2.0の代わりにPDF/UA-1へのエクスポートを推奨しています。
PDF/UAのパート2はPDF 2.0用に設計されていますが、Typstではまだサポートされていません。

## PDF/A
TypstはオプションでPDF/A準拠ファイルの出力をサポートしています。
PDF/Aファイルは、現在および将来のPDF用ツールとの最大限の互換性を目的として設計された形式です。
この形式は実装が困難な機能や独自仕様に依存せず、網羅的なメタデータを含みます。
これにより、長期的なアーカイブに適した形式となっています。

PDF/A規格には複数のバージョン（ISOにおける用語では_パート_）があり、
ほとんどのパートにはファイルの適合レベルを示す複数のプロファイルが存在します。
現在、Typstは以下のPDF/A出力プロファイルをサポートしています。

- **PDF/A-1b：** ISO 19005-1における_基本（basic）_適合レベル。
  このPDF/AバージョンはPDF 1.4（2001年）を基盤としており、自己完結型でアーカイブ可能なPDFファイルを生成します。
  PDF/A規格の以降のパートとは対照的に、PDF/A-1ファイルでは透明度が許可されていません。

- **PDF/A-1a：** 基本レベルであるPDF/A-1bを基盤とした、_アクセシブル（accessible）_な適合レベル。
  このレベルに準拠するには、ファイルがアクセシブルな_タグ付きPDF（Tagged PDF）_ファイルである必要があります。
  PDF 1.4のファイルでは全てのPDFアクセシビリティ機能が利用できるわけではないため、アクセシビリティは後のパートで改善されることに注意してください。
  さらに、ファイル内の全てのテキストは既知のUnicodeコードポイントで構成されている必要があります。

- **PDF/A-2b：** ISO 19005-2における基本適合レベル。
  このPDF/AバージョンはPDF 1.7を基盤としており、自己完結型でアーカイブ可能なPDFファイルを生成します。

- **PDF/A-2u：** 基本レベルであるA-2bを基盤とした、_Unicodeマッピング可能（Unicode-mappable）_な適合レベル。
  これには、文書内の全てのテキストが既知のUnicodeコードポイントで構成されている必要があるというルールも追加されています。
  可能であれば、常にPDF/A-2bよりもこの規格を優先してください。

- **PDF/A-2a：** Unicodeマッピング可能なレベルであるA-2uを基盤とした、_アクセシブル（accessible）_な適合レベル。
  この適合レベルには、さらに2つの要件が追加されます。
  まず、ファイルがアクセシブルな_タグ付きPDF（Tagged PDF）_ファイルである必要があります。
  Typstは、この適合レベルに到達できるよう自動的にタグを追加します。
  この適合レベルをターゲットにする場合は、リファレンス全体のアクセシビリティに関するセクションや[アクセシビリティガイド]($guides/accessibility)にも注意を払ってください。
  最後に、PDF/A-2aでは[Unicodeの私用領域（Private Use area）](https://en.wikipedia.org/wiki/Private_Use_Areas)のコードポイントを使用することが禁止されています。
  アクセシブルなファイルを構築したい場合は、より多くの自動アクセシビリティチェックを有効にできるPDF/UA-1も同時にターゲットにすることを検討してください。

- **PDF/A-3b：** ISO 19005-3における基本適合レベル。
  このPDF/AバージョンはPDF 1.7を基盤としており、
  任意の関連ファイルを[添付ファイル]($pdf.attach)として含められるアーカイブ可能なPDFファイルを生成します。
  PDF/A-2bとの違いは、
  PDF/A非準拠のファイルを埋め込む機能がある点のみです。

- **PDF/A-3u：** 基本レベルであるA-3bを基盤とした、_Unicodeマッピング可能（Unicode-mappable）_な適合レベル。
  PDF/A-2bと同様に、全てのテキストが既知のUnicodeコードポイントで構成されている必要があります。
  これらのルールは添付ファイルには適用されません。
  可能であれば、常にPDF/A-3bよりもこの規格を優先してください。

- **PDF/A-3a：** Unicodeマッピング可能なレベルであるA-3uを基盤とした、_アクセシブル（accessible）_な適合レベル。
  PDF/A-2aと同様に、ファイルがアクセシブルな_タグ付きPDF（Tagged PDF）_であること、およびUnicodeの私用領域の文字を使用していないことが求められます。
  前述の通り、これらのルールは添付ファイルには適用されません。

- **PDF/A-4：** ISO 19005-4における基本適合レベル。
  このPDF/AバージョンはPDF 2.0（2017年）を基盤としており、自己完結型でアーカイブ可能なPDFファイルを生成します。
  PDF/A-4にはアクセシビリティに関連するパートはありません。
  代わりに、専用のPDF/UA規格でアクセシビリティについてより詳しく規定されています。
  PDF/A-4ファイルはPDF/UA-2に準拠できます（Typstでは現在未サポート）。

- **PDF/A-4f：** 基本レベルであるA-4を基盤とした、_埋め込みファイル（embedded files）_の適合レベル。
  ISO 19005のパート3に準拠するファイルと同様に、このレベルに準拠するファイルは、関連する任意のファイルを[添付ファイル]($pdf.attach)として含められます。
  PDF/A-4との唯一の違いは、PDF/Aに準拠していないファイルを添付する機能がある点です。

- **PDF/A-4e：** 埋め込みファイルのレベルであるA-4fを基盤とした、_エンジニアリング（engineering）_の適合レベル。
  このレベルに準拠したファイルは、3Dオブジェクトを含められます。
  Typstは3Dコンテンツをサポートしていないため、Typstの観点からは機能的にPDF/A-4fと同等です。

PDF/Aをターゲットにしたいが、どの設定を使用すべきか迷う場合は、いくつか役立つ経験則があります。
まず、標準規格の「バージョン番号」にあたる**パート（part）**を決定しなければなりません。
以下の質問に答えてみてください。

1. **2006年より前のソフトウェアやハードウェアでファイルを読み込める必要がありますか？**
   もしそうなら、パート1（PDF/A-1）を選択してください。

2. **その必要がなく、[添付ファイル]($pdf.attach)を含める必要がありますか？**
   もしそうなら、パート3（PDF/A-3）またはパート4の埋め込みファイルレベル（PDF/A-4f）を選択しなければなりません。

3. **PDF 2.0で導入された機能を使用する必要がありますか？**
   現在、TypstにおけるPDF 2.0機能の利用は、[title要素]($title)などにおけるアクセシビリティの微妙な改善に限定されています。
   これらの改善がなくても問題ない場合は、（添付ファイルが不要なら）パート2、または（添付ファイルが必要なら）パート3を選択することで、互換性を最大化できます。
   PDF 2.0の機能に依存する場合は、パート4を使用してください。

あとは、**適合レベル（conformance level）**（末尾の小文字）を選択するだけです。

- **パート1、2、または3を選択した場合：**
  通常は、末尾に小文字の`a`が付く、アクセシブルな適合レベルを選択すべきです。
  代替説明で表現しきれないアーティストのポートフォリオのように、文書に本質的にアクセシビリティがない場合は、代わりに適合レベル`u`を選択してください。
  Unicode私用領域からコードポイントを使用したことなどによりコンパイルエラーになった場合にのみ、基本レベル`b`を使用してください。

- **パート4を選択した場合：**
  ファイルを埋め込む必要がある場合を除き、基本適合レベル（PDF/A-4）を選択すべきです。

PDF/Aと通常のPDFのどちらをエクスポートするかを選択する際には、
PDF/Aファイルには追加のメタデータが含まれ、
また一部のリーダーはユーザーによるPDF/Aファイルの修正を阻止することに留意してください。
一部のTypstの機能は、選択したPDF規格によっては無効化される場合があります。

# PDF固有の機能
Typstでは、グローバルな`pdf`モジュールを通じてPDFに特化した機能を提供しています。
そのモジュールに含まれる定義については、以下を参照してください。

このモジュールには、最終的なAPIが定まっていない関数がいくつか含まれています。
これらは、複雑な表を含む文書のアクセシビリティを強化するために設計されています。
これには、[`table-summary`]($pdf.table-summary)、[`header-cell`]($pdf.header-cell)、および[`data-cell`]($pdf.data-cell)が含まれます。
これらの関数は全て、将来のTypstのリリースにおいて、表関数に統合されるか、または完全に削除される予定です。
これらの関数を有効にするには、`--features a11y-extras`を渡すか、環境変数`TYPST_FEATURES`を`a11y-extras`に設定します。
現在、Webアプリではこれらの機能は利用できません。
=======
PDF files focus on accurately describing documents visually, but also have
facilities for annotating their structure. This hybrid approach makes
them a good fit for document exchange: They render exactly the same on every
device, but also support extraction of a document's content and structure (at
least to an extent). Unlike PNG files, PDFs are not bound to a specific
resolution. Hence, you can view them at any size without incurring a loss of
quality.

# Exporting as PDF
## Command Line
PDF is Typst's default export format. Running the `compile` or `watch`
subcommand without specifying a format will create a PDF. When exporting to PDF,
you have the following configuration options:

- Which [PDF standards](#pdf-standards) Typst should enforce conformance with by
  specifying `--pdf-standard` followed by one or multiple comma-separated
  standards. Valid standards are `1.4`, `1.5`, `1.6`, `1.7`, `2.0`, `a-1b`,
  `a-1a`, `a-2b`, `a-2u`, `a-2a`, `a-3b`, `a-3u`, `a-3a`, `a-4`, `a-4f`, `a-4e`,
  and `ua-1`. By default, Typst outputs PDF-1.7-compliant files.

- You can disable PDF tagging completely with `--no-pdf-tags`. By default, Typst
  will always write _Tagged PDF_ to provide a baseline level of accessibility.
  Using this flag, you can turn tags off. This will make your file inaccessible
  and prevent conformance with accessible conformance levels of PDF/A and all
  parts of PDF/UA.

- Which pages to export by specifying `--pages` followed by a comma-separated
  list of numbers or dash-separated number ranges. Ranges can be half-open.
  Example: `2,3,7-9,11-`.

## Web App
Click the quick download button at the top right to export a PDF with default
settings. For further configuration, click "File" > "Export as" > "PDF" or click
the downwards-facing arrow next to the quick download button and select "Export
as PDF". When exporting to PDF, you have the following configuration options:

- Which PDF standards Typst should enforce conformance with. By default, Typst
  outputs PDF-1.7-compliant files. You can choose the PDF version freely between
  1.4 and 2.0. Valid additional standards are `A-1b`, `A-1a`, `A-2b`, `A-2u`,
  `A-2a`, `A-3b`. `A-3u`, `A-3a`, `A-4`, `A-4f`, `A-4e`, and `UA-1`.

- Which pages to export. Valid options are "All pages", "Current page", and
  "Custom ranges". Custom ranges are a comma-separated list of numbers or
  dash-separated number ranges. Ranges can be half-open. Example: `2,3,7-9,11-`.

# PDF standards
The International Standards Organization (ISO) has published the base PDF
standard and various standards that extend it to make PDFs more suitable for
specific use-cases. By default, Typst exports PDF 1.7 files. Adobe Acrobat 8 and
later as well as all other commonly used PDF viewers are compatible with this
PDF version.

Some features of Typst may not be available depending on the PDF standard you
choose. You currently cannot choose both PDF/A and PDF/UA at the same time.

## PDF versions
Typst supports five different PDF versions: 1.4, 1.5, 1.6, 1.7 (default), and
2.0. You can choose each of these versions for your document export. However,
based on the features you used there may be a minimum version. Likewise, the
standards you target can limit which versions you can choose (see below for
more).

Here is a list on how each new version improves over PDF 1.4 for Typst
documents:

- **PDF 1.5** (2003): Improved color management, text extraction, table
  accessibility, reflow, and emoji fonts
- **PDF 1.6** (2004): More flexible links
- **PDF 1.7** (2006): Allows [attachments]($pdf.attach), improved reflow
- **PDF 2.0** (2017): Improved both metadata and tag semantics for accessibility

The software used to read your file must support your PDF version. Under normal
circumstances, this poses no problem, but it can be a source of errors when
working with older hardware. For general exchange, we recommend keeping the
default PDF 1.7 setting or choosing PDF 2.0.

When using PDF files as [images]($image) in your document, the export PDF
version must equal or exceed the image file versions.

## PDF/UA
Typst supports writing PDF/UA-conformant files. PDF/UA files are designed
for _[Universal Access]($guides/accessibility/#basics)._ When you choose this PDF
standard, Typst will run additional checks when exporting your document. These
checks will make sure that you are following accessibility best practices. For
example, it will make sure that all your images come with alternative
descriptions.

Note that there are some rules in PDF/UA that are crucial for accessibility but
cannot be automatically checked. Hence, when exporting a PDF/UA-1 document, make
sure you did the following:

- If your document is written in a different language than English, make sure
  [set the text language]($text.lang) before any content.
- Make sure to use Typst's semantic elements (like [headings]($heading),
  [figures]($figure), and [lists]($list)) when appropriate instead of defining
  custom constructs. This lets Typst know (and export) the role a construct
  plays in the document. See [the Accessibility guide]($guides/accessibility)
  for more details.
- Do not exclusively use contrast, color, format, or layout to communicate an
  idea. Use text or alternative descriptions instead of or in addition to these
  elements.
- Wrap all decorative elements without a semantic meaning in [`pdf.artifact`].
- Do not use images of text. Instead, insert the text directly into your markup.

Typst currently only supports part one (PDF/UA-1) which is based on PDF 1.7
(2006). When exporting to PDF/UA-1, be aware that you will need to manually
provide [alternative descriptions of mathematics]($category/math/#accessibility)
in natural language.

New accessibility features were added to PDF 2.0 (2017). When set to PDF 2.0
export, Typst will leverage some of these features. PDF 2.0 and PDF/UA-1,
however, are mutually incompatible. For accessible documents, we currently
recommend exporting to PDF/UA-1 instead of PDF 2.0 for the additional checks and
greater compatibility. The second part of PDF/UA is designed for PDF 2.0, but
not yet supported by Typst.

## PDF/A
Typst optionally supports emitting PDF/A-conformant files. PDF/A files are
geared towards maximum compatibility with current and future PDF tooling. They
do not rely on difficult-to-implement or proprietary features and contain
exhaustive metadata. This makes them suitable for long-term archival.

The PDF/A Standard has multiple versions (_parts_ in ISO terminology) and most
parts have multiple profiles that indicate the file's conformance level. You can
target one part and conformance level at a time. Currently, Typst supports these
PDF/A output profiles:

- **PDF/A-1b:** The _basic_ conformance level of ISO 19005-1. This version of
  PDF/A is based on PDF 1.4 (2001) and results in self-contained, archivable PDF
  files. As opposed to later parts of the PDF/A standard, transparency is not
  allowed in PDF/A-1 files.

- **PDF/A-1a:** This is the _accessible_ conformance level that builds on the
  basic level PDF/A-1b. To conform to this level, your file must be an
  accessible _Tagged PDF_ file. Note that accessibility is improved with later
  parts as not all PDF accessibility features are available for PDF 1.4 files.
  Furthermore, all text in the file must consist of known Unicode code points.

- **PDF/A-2b:** The _basic_ conformance level of ISO 19005-2. This version of
  PDF/A is based on PDF 1.7 (2006) and results in self-contained, archivable PDF
  files.

- **PDF/A-2u:** This is the _Unicode-mappable_ conformance level that builds on
  the basic level A-2b. It also adds rules that all text in the document must
  consist of known Unicode code points. If possible, always prefer this standard
  over PDF/A-2b.

- **PDF/A-2a:** This is the _accessible_ conformance level that builds on the
  Unicode-mappable level A-2u. This conformance level also adds two
  requirements: Your file must be an accessible _Tagged PDF_ file. Typst
  automatically adds tags to help you reach this conformance level. Also pay
  attention to the Accessibility Sections throughout the reference and the
  [Accessibility Guide]($guides/accessibility) when targeting this conformance
  level. Finally, PDF/A-2a forbids you from using code points in the [Unicode
  Private Use area](https://en.wikipedia.org/wiki/Private_Use_Areas). If you
  want to build an accessible file, also consider additionally targeting
  PDF/UA-1, which enables more automatic accessibility checks.

- **PDF/A-3b:** The _basic_ conformance level of ISO 19005-3. This version of
  PDF/A is based on PDF 1.7 (2006) and results in archivable PDF files that can
  contain arbitrary other related files as [attachments]($pdf.attach). The only
  difference between it and PDF/A-2b is the capability to attach
  non-PDF/A-conformant files.

- **PDF/A-3u:** This is the _Unicode-mappable_ conformance level that builds on
  the basic level A-3b. Just like PDF/A-2b, this requires all text to consist
  of known Unicode code points. These rules do not apply to attachments. If
  possible, always prefer this standard over PDF/A-3b.

- **PDF/A-3a:** This is the _accessible_ conformance level that builds on the
  Unicode-mappable level A-3u. Just like PDF/A-2a, this requires files to be
  accessible _Tagged PDF_ and to not use characters from the Unicode Private Use
  area. Just like before, these rules do not apply to attachments.

- **PDF/A-4:** The basic conformance level of ISO 19005-4. This version of PDF/A
  is based on PDF 2.0 (2017) and results in self-contained, archivable PDF
  files. PDF/A-4 has no parts relating to accessibility. Instead, the topic has
  been elaborated on more in the dedicated PDF/UA standard. PDF/A-4 files can
  conform to PDF/UA-2 (currently not supported in Typst).

- **PDF/A-4f:** The _embedded files_ conformance level that builds on the basic
  level A-4. Files conforming to this level can contain arbitrary other related
  files as [attachments]($pdf.attach), just as files conforming to part 3 of ISO
  19005. The only difference between it and PDF/A-4 is the capability to attach
  non-PDF/A-conformant files.

- **PDF/A-4e:** The _engineering_ conformance level that builds on the embedded
  files level A-4f. Files conforming to this level can contain 3D objects. Typst
  does not support 3D content, so this is functionally equivalent to PDF/A-4f
  from a Typst perspective.

If you want to target PDF/A but are unsure about which particular setting to
use, there are some good rules of thumb. First, you must determine your
**part,** that's the "version number" of the standard. Ask yourself these
questions:

1. **Does pre-2006 software or equipment need to be able to read my file?** If
   so, choose part one (PDF/A-1).

2. **If not, does my file need [attachments]($pdf.attach)?** If so, you must
   choose part three (PDF/A-3) or the embedded files level of part four
   (PDF/A-4f).

3. **Does your file need to use features introduced in PDF 2.0?** Currently, use
   of PDF 2.0 features in Typst is limited to minor improvements in
   accessibility, e.g. for the [title element]($title). If you can do without
   these improvements, maximize compatibility by choosing part two (when you
   don't need attachments) or part three (when you do need attachments). If you
   rely on PDF 2.0 features, use part four.

Now, you only need to choose a **conformance level** (the lowercase letter at
the end).

- **If you decided on part one, two, or three,** you should typically choose the
  accessible conformance level of your part, indicated by a lowercase `a` at the
  end. If your document is inherently inaccessible, e.g. an artist's portfolio
  that cannot be boiled down to alternative descriptions, choose conformance
  level `u` instead. Only if this results in a compiler error, e.g. because you
  used code points from the Unicode Private Use Area, use the basic level `b`.

- **If you have chosen part four,** you should choose the basic conformance
  level (PDF/A-4) except when needing to embed files.

When choosing between exporting PDF/A and regular PDF, keep in mind that PDF/A
files contain additional metadata, and that some readers will prevent the user
from modifying a PDF/A file.

# PDF-specific functionality
Typst exposes PDF-specific functionality in the global `pdf` module. See below
for the definitions it contains.

This module contains some functions without a final API. They are designed to
enhance accessibility for documents with complex tables. This includes
[`table-summary`]($pdf.table-summary), [`header-cell`]($pdf.header-cell), and
[`data-cell`]($pdf.data-cell). All of these functions will be removed in a
future Typst release, either through integration into table functions or through
full removal. You can enable these functions by passing `--features a11y-extras`
or setting the `TYPST_FEATURES` environment variable to `a11y-extras`. In the
web app, these features are not available at this time.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
