---
description: |
<<<<<<< HEAD
  Typstを使ってアクセシブルな文書を作成する方法を学びます。このガイドでは、セマンティックマークアップ、読み上げ順序、代替テキスト、色のコントラスト、言語設定、PDF/UA準拠などをカバーし、全ての読者と支援技術に対応するファイルを作成する方法を説明します。
---

# アクセシビリティガイド

文書をアクセシブルにするということは、全ての人がその文書を使用し理解できるようにすることを意味します。それには、恒久的または一時的な障害を持つ人々だけでなく、さまざまなデバイスや好みを持つ人々も含まれます。アクセシビリティが重要な理由を強調するために、人々があなたの想定以上にさまざまな状況で文書を読む可能性があることを考えてみてください。

- ユーザーが文書を紙に印刷するかもしれない
- ユーザーがPDFリーダーのリフロー機能を有効にして、携帯電話で文書を読むかもしれない
- ユーザーが自身のコンピューターに文書を読み上げさせるかもしれない
- ユーザーが人工知能に文書の要約を依頼するかもしれない
- ユーザーが文書をHTMLなどの、よりアクセシブルなファイル形式に変換するかもしれない

これら全ての人々とシナリオに対応するため、**ユニバーサルアクセス**を考慮して文書を設計するべきです。ユニバーサルアクセスは、シンプルでありながら強力な原則です。後からアクセシビリティ対応を付け足すのではなく、最初から、できるだけ幅広いユーザーと利用状況に対応できるように設計します。これにより、全ての読者の体験が向上します！

Typstは、スクリーンリーダーで読み取りやすく、異なる画面サイズ向けにリフローされても見栄えがよく、自動アクセシビリティチェッカーにも合格しやすいアクセシブルなファイルの作成を支援できます。ただし、アクセシブルなファイルを作成するには、いくつかのルールを意識する必要があります。このガイドでは、アクセシビリティに影響する要因、ユニバーサルアクセスのための設計方法、そしてそれを実現するためにTypstが提供するツールを学べます。ここで述べる指針の多くは、全てのエクスポート形式に当てはまりますが、本ガイドはPDFエクスポートに焦点を当てています。HTMLエクスポートとの重要な違いも記載します。

## アクセシビリティの基礎 { #basics }

アクセシブルなファイルは、ソフトウェアが単にファイルを描画する以上のことを行えるようにします。すなわち、コンピューターが文書の各要素が何を表しているかを理解し、その情報を用いてユーザーに文書を提示できるようになるのです。

この情報はアクセスを提供するためにさまざまなソフトウェアによって利用されます。TypstからPDFをエクスポートすると、_PDFビューアー_（リーダーとも呼ばれます）が、Typstのプレビューでデザインした通りに文書のページを表示します。なかには、スクリーンリーダーや点字ディスプレイ、画面拡大鏡などの_支援技術_（Assistive Technology、AT）に頼ってPDFを利用する人もいます。その場合、ファイルに含まれるセマンティック情報が使われ、内容は音声や文字、あるいは別の視覚表現へと変換されます。また、PDFビューアーのリフロー機能を使い、Webページに似たレイアウトで閲覧するユーザーもいます。コンテンツは表示領域の幅にあわせて再配置され、連続的にスクロールできるようになります。さらに、別のユーザーはPDFを別の形式に再利用します。例えば大規模言語モデル（Large Language Model、LLM）に取り込むためのプレーンテキストやHTMLなどです。再利用の特殊な形としてコピー＆ペーストがあり、ユーザーはクリップボードを使ってファイルから内容を抽出し、別のアプリケーションで利用します。

アクセシビリティ対応はビューアーとATによって異なります。組み合わせによっては、他よりもうまく動作するものもあります。私たちのテストでは、Windowsでは[Adobe Acrobat][Acrobat]と[NVDA][NVDA]の組み合わせ、macOSでは[VoiceOver][VoiceOver]が最も充実したアクセシビリティ対応を提供しました。またHTMLエクスポートと組み合わせた場合、ブラウザーはPDFリーダーと比べて、より一貫したアクセシビリティの基準を提供します。

アクセシブルなファイルを生成できるのは、PDFとHTMLのエクスポートだけです。PNGとSVGはいずれも、単体でアクセシブルではありません。ただし、どちらの形式も[テキストによる表現](#textual-representations)を用意すれば、アクセシブルなより大きな成果物の中で利用できます。

## セマンティクスの保持 { #maintaining-semantics }

ATによる利用や再利用のために正しいセマンティック情報をファイルに付与するためには、ファイル中の各部分がどのようなセマンティック上の役割を果たすのかをTypstは把握する必要があります。例えば、これはコンパイルされたPDFの見出しが単なる大きく太い文字のテキストであってはならないということを意味します。代わりに、その特定のテキストが見出しを構成することを示す明示的な情報（_タグ_として知られます）を含めるべきです。そうするとスクリーンリーダーはそれを見出しとして読み上げ、ユーザーが見出し間を移動できるようになります。

Typstの慣用的な使い方、すなわち組み込みのマークアップや要素を用いている場合、Typstは自動的にリッチなセマンティック情報を持つタグをファイルへ付与します。次の2つのコード例を見てみましょう。
=======
  Learn how to create accessible documents with Typst. This guide covers semantic markup, reading order, alt text, color contrast, language settings, and PDF/UA compliance to ensure your files work for all readers and Assistive Technology.
---

# Accessibility Guide

Making a document accessible means that it can be used and understood by everyone. That not only includes people with permanent or temporary disabilities, but also those with different devices or preferences. To underscore why accessibility is important, consider that people might read your document in more contexts than you expected:

- A user may print the document on paper
- A user may read your document on a phone, with reflow in their PDF reader enabled
- A user may have their computer read the document back to them
- A user may ask artificial intelligence to summarize your document for them
- A user may convert your document to another file format like HTML that is more accessible to them

To accommodate all of these people and scenarios, you should design your document for **Universal Access.** Universal Access is a simple but powerful principle: instead of retrofitting a project for accessibility after the fact, design from the beginning to work for the broadest possible range of people and situations. This will improve the experience for all readers!

Typst can help you to create accessible files that read well on screen readers, look good even when reflowed for a different screen size, and pass automated accessibility checkers. However, to create accessible files, you will have to keep some rules in mind. This guide will help you learn what issues impact accessibility, how to design for Universal Access, and what tools Typst gives you to accomplish this. Much of the guidance here applies to all export targets, but the guide focuses on PDF export. Notable differences to HTML export are called out.

## Basics of Accessibility { #basics }

Accessible files allow software to do more with them than to just render them. Instead, your computer can understand what each part of the document is supposed to represent and use this information to present the document to the user.

This information is consumed by different software to provide access. When exporting a PDF from Typst, the _PDF viewer_ (sometimes also called a reader) will display the document's pages just as you designed them with Typst's preview. Some people rely on _Assistive Technology_ (AT) such as screen readers, braille displays, screen magnifiers, and more for consuming PDF files. In that case, the semantic information in the file is used to adapt the contents of a file into spoken or written text, or into a different visual representation. Other users will make the PDF viewer reflow the file to create a layout similar to a web page: The content will fit the viewport's width and scroll continuously. Finally, some users will repurpose the PDF into another format, for example plain text for ingestion into a Large Language Model (LLM) or HTML. A special form of repurposing is copy and paste where users use the clipboard to extract content from a file to use it in another application.

Accessibility support differs based on viewer and AT. Some combinations work better than others. In our testing, [Adobe Acrobat][Acrobat] paired with [NVDA][NVDA] on Windows and [VoiceOver][VoiceOver] on macOS provided the richest accessibility support. Paired with HTML export, browsers provide a more consistent baseline of accessibility when compared to PDF readers.

Only PDF and HTML export produce accessible files. Neither PNGs nor SVGs are accessible on their own. Both formats can be used in an accessible larger work by providing a [textual representation](#textual-representations).

## Maintaining semantics

To add correct semantic information for AT and repurposing to a file, Typst needs to know what semantic role each part of the file plays. For example, this means that a heading in a compiled PDF should not just be text that is large and bold. Instead, the file should contain the explicit information (known as a _tag_) that a particular text makes up a heading. A screen reader will then announce it as a heading and allow the user to navigate between headings.

If you are using Typst idiomatically, using the built-in markup and elements, Typst automatically adds tags with rich semantic information to your files. Let's take a look at two code examples:
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```example
// ❌ Don't do this
#text(
  size: 16pt,
  weight: "bold",
)[Heading]
```

```example
// ✅ Do this
#show heading: set text(size: 16pt)
= Heading
```

<<<<<<< HEAD
どちらの例も、見た目は同じです。どちらも"Heading"というテキストが太字で、サイズは16ptになっています。しかしながら、アクセシブルなのは例のうち2つ目のみです。見出し用のマークアップを使うことで、Typstはこのテキストのセマンティックな意味が「見出し」であることを理解し、その情報を最終的なPDFに反映できます。1つ目の例では、Typstは通常のテキストに対して太字と大きめの文字を適用すべきだということしかわかりません。そのため、それが見出しを意図したものなのか、スタイル上の選択なのか、あるいは引用のような別の要素なのかを推測できません。

セマンティクスの利用は見出しに限りません。以下はセマンティクスを利用すべき要素のさらなる例です。

- テキストを強調する場合は[`text`]($text)関数の代わりにアンダースコア/[`emph`]($emph)を使用する
- テキストに強い強調を与える場合は[`text`]($text)関数の代わりにアスタリスク/[`strong`]($strong)を使用する
- アイテム化された内容や順序のある内容を扱う場合は、改行を伴う通常のテキストの代わりにリスト（[`list`]($list)、[`enum`]($enum)、[`terms`]($terms)）を使用する
- インライン引用やブロック引用には[`quote`]($quote)を使用する
- 文献を手動で記述するのではなく、組み込みの[`bibliography`]($bibliography)と[`cite`]($cite)関数を使用する
- 文書の他の部分を参照する場合は、単に参照をタイプするのではなく、ラベルと[`ref`]($ref)または`@references`を使用する
- キャプションを提供する場合は、テキストを関数呼び出しの下に追加するのではなく、[`figure`要素の`caption`引数]($figure.caption)を使用する

要素のデフォルトのスタイルを調整したい時であっても、独自のカスタム関数で置き換えるのではなく、[setルール]($styling/#set-rules)、show-setルール、そして[showルール]($styling/#show-rules)を使用して外観をカスタマイズしてください。以下は、文書内の強い強調の見た目を変更する方法の例です。
=======
Both of these examples look the same. They both contain the text "Heading" in boldface, sized at 16 point. However, only the second example is accessible. By using the heading markup, Typst knows that the semantic meaning of this text is that of a heading and can propagate that information to the final PDF. In the first example, it just knows that it should use boldface and larger type on otherwise normal text and cannot make the assumption that you meant that to be a heading and not a stylistic choice or some other element like a quote.

Using semantics is not limited to headings. Here are a few more examples for elements you should use:

- Use underscores / [`emph`] instead of the [`text`] function to make text emphasized
- Use stars / [`strong`] instead of the text function to make text carry strong emphasis
- Use lists ([`list`], [`enum`], [`terms`]) instead of normal text with newlines when working with itemized or ordered content
- Use [`quote`] for inline and block quotes
- Use the built-in [`bibliography`] and [`cite`] functions instead of manually printing a bibliography
- Use labels and [`ref`] or `@references` to reference other parts of your documents instead of just typing out a reference
- Use the [`caption` argument of the `figure` element]($figure.caption) to provide captions instead of adding them as text below the function call

If you want to style the default appearance of an element, do not replace it with your own custom function. Instead, use [set]($styling/#set-rules), show-set, and [show rules]($styling/#show-rules) to customize its appearance. Here is an example on how you can change how strong emphasis looks in your document:
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```example
// Change how text inside of strong emphasis looks
#show strong: set text(tracking: 0.2em, fill: blue, weight: "black")

When setting up your tents, *never forget* to secure the pegs.
```

<<<<<<< HEAD
show-setルールは[`strong`]($strong)要素のデフォルトの見た目を完全に変更しますが、そのセマンティックな意味は保持します。さらにカスタマイズが必要な場合は、完全に独自のレイアウトコードを持つshowルールを指定できますが、それでもTypstはその要素のセマンティックな目的を保持します。

## 読み上げ順序 { #reading-order }

ATが文書中のコンテンツを正しい順序で読み上げられるようにするため、また再利用アプリケーションのためにも、アクセシブルなファイルは読み上げ順序を明示しなければなりません。これは、論理的な読み上げ順序がレイアウト順序とは異なる可能性があるためです。こうした差異の典型的な例がフロート図表です。図表がページ中央の段落に関連するものであっても、ページの上端や下端に配置されることがあります。アクセシブルでないファイルでは、PDFリーダーやATはレイアウト順序と論理的な読み上げ順序が同一であると推測せざるを得ず、その結果ATユーザーに混乱を招くことがよくあります。読み上げ順序が適切に定義されていれば、スクリーンリーダーは脚注やフロート図表を、意味が通る位置で直ちに読み上げます。

幸い、Typstのマークアップは既に単一の読み上げ順序を暗黙的に含んでいます。Typst文書は、マークアップ内で内容を配置した順に読み上げられると考えてよいでしょう。ほとんどの文章ではそれで十分です。ただし、[`place`]($place)関数や[`move`]($move)関数、あるいは[フロート図表]($figure.placement)を使用する場合は、たとえレイアウトに影響しなくても、マークアップ上の論理的な読み上げ順序として適切な位置に関数呼び出しを置くよう、特に注意が必要です。配置しようとしている内容をスクリーンリーダーにどの位置で読み上げてほしいかを自問してみてください。

## レイアウトコンテナ { #layout-containers }

Typstはコンテンツを視覚的に配置するための、[`grid`]($grid)、[`stack`]($stack)、[`box`]($box)、[`columns`]($columns)、[`block`]($block)などのレイアウトコンテナを提供します。これらのコンテナにはセマンティックな意味は付与されません。TypstはPDFリフローの際にこれらのコンテナの一部を保持しますが、他のコンテナは破棄されます。

ユニバーサルアクセスのための設計の際、ATユーザーはコンテナが作り出す視覚的なレイアウトを閲覧できないことが多い、という点を認識しておく必要があります。代わりに、ATはその内容をただ読み上げるだけなので、アクセシビリティの観点ではこれらのコンテナは透過的なものと考えるのが最善です。例えば、グリッドの内容は、ソースコード内でセルを追加した順番通りに、ただ平坦に読み上げられるだけです。あなたの作成したレイアウトが単に視覚的・装飾的なものであれば、それで問題ありません。しかし、もしそのレイアウトが通常のPDFリーダーを用いてファイルを閲覧する、目の見える人にとっては明らかなセマンティックな意味を持つ場合は、それはアクセシブルではありません。その代わりに、テキストを活用した代替表現を作成するか、あるいは代替となるテキストによる表現を提供するために、[`figure`]($figure)要素で包んでください。

グリッドコンテナを表形式データの表現に使用してはいけません。代わりに[`table`]($table)を使用してください。表はATユーザーにとってアクセシブルであり、ユーザーはATによって表を2次元的に移動して参照できます。表はリフローや再利用の際にも保持されます。表を作成する際には[`table.header`]($table.header)と[`table.footer`]($table.footer)要素を使用して、個々の行のセマンティックな役割をマークアップしてください。表のドキュメントには、表をアクセシブルにする方法について詳しく説明した[アクセシビリティセクション]($table/#accessibility)があります。また、ATユーザーが表にアクセスできるとはいえ、それがしばしば負担になることにも留意してください。表は視覚的な閲覧に最適化されているためです。セル群の内容を読み上げられながら、その行と列を思い出し続けなければならない状況は、追加の認知負荷を生みます。表の要点を、別の箇所でテキストやキャプションとしてアクセシブルに提供することを検討してください。

同様に、[`rotate`]($rotate)、[`scale`]($scale)、[`skew`]($skew)のような関数を使用する場合は、この変換がセマンティックな意味を持たないか、あるいは意味がATユーザーに他の場所、つまり図の[代替テキスト](#textual-representations)やキャプションなどで利用可能であることに注意してください。

## アーティファクト { #artifacts }

ページ上の一部のものには、セマンティックな意味がなく、文書の内容にも関係しないものがあります。これらの項目を_アーティファクト_と呼びます。アーティファクトはATや再利用からは隠され、リフローの際には消失します。以下はアーティファクトの例です。

- 行末に自動ハイフネーションによって挿入されるハイフン
- 各ページのヘッダーおよびフッター
- 純粋に装飾目的のページ背景画像

一般に文書がアクセシブルと見なされるためには、ページ上の全ての要素が、ATが読み上げられる何らかの方法を持つか、またはアーティファクトである必要があります。

Typstは、ヘッダー、フッター、ページの背景・前景、自動ハイフネーションなど、多くのレイアウト上のアーティファクトを自動的にアーティファクトとしてタグ付けします。ただし、文書に純粋に装飾的な内容を追加したい場合は、[`pdf.artifact`]($pdf.artifact)関数を使用して、コンテンツの一部をアーティファクトとしてマークできます。要素をアーティファクトとしてマークすべきかどうか迷った場合は、自問してみてください。スクリーンリーダーがその要素を読み上げるとしたら、単に邪魔になるだけでしょうか？それならば、それはアーティファクトかもしれません。逆に、読み上げられると便利なものであれば、それはアーティファクトではありません。

技術的な理由により、いったんアーティファクトの中に入ると、コンテンツは再びセマンティックになることはできません。アーティファクトとセマンティックな内容を重ねるには、[`place`]($place)を使用してそれぞれを独立した要素として重ねあわせてください。

Typstは、[`square`]($square)や[`circle`]($circle)のような形状やパスをアーティファクトとしてマークしますが、その内容はセマンティックに関連し、ATからアクセス可能なままであることに留意してください。形状にセマンティックな意味がある場合は、代替のテキスト説明を提供するために、[`figure`]($figure)要素でそれらを包んでください。

## 色の使用とコントラスト { #color-use-and-contrast }

ユニバーサルアクセスとは、文書がAT、リフロー、再利用に対応していることのみを意味するのではありません。視力が低下している人も含め、全ての人が視覚的にアクセス可能であることを意味します。加齢にはしばしば視力の低下が伴うだけでなく、かなりの割合の人が色の識別に困難を抱えています。具体的には、男性の約8%、女性の約0.5%が色覚異常です。

<div style="display:flex; gap: 8px 16px; width: 100%; flex-wrap: wrap; margin: 24px auto; ">
<img src="chart-bad-regular.png" alt="種類別のドイツのエネルギー生産量（テラワット時）を横軸、年を縦軸に取った棒グラフ。各棒は最大4区画で、原子力（紫）、再生可能エネルギー（緑）、化石燃料（赤）、その他（青）を表す。右上に各色とラベルを対応付けた凡例がある。" width="958" height="637" style="box-shadow: 0 4px 12px rgb(89 85 101 / 20%); width: 200px; max-width: 100%; height: auto; display: block; border-radius: 6px; flex-grow: 1">
<img src="chart-bad-deuteranopia.png" alt="同じ棒グラフを色を変えて示したもの。原子力とその他の区画は非常に近い濃い青で、隣接する再生可能エネルギーと化石燃料の区画も、ほとんど見分けがつかない黄系の2色になっている。" width="958" height="637" style="box-shadow: 0 4px 12px rgb(89 85 101 / 20%); width: 200px; max-width: 100%; height: auto; display: block; border-radius: 6px; flex-grow: 1">
</div>

これは、文書内の情報を目の見える人へアクセシブルにする方法として、色を唯一の手段にしてはならない、ということを意味します。例として、1本の棒が複数の色分けされた区画で構成される積み上げ棒グラフを考えてみましょう。この例では、ドイツ国内のエネルギー生産量を種類別に示したグラフ[^1]を扱います。この図には、グラフの通常の見え方と、2型色覚の人にはどのように見えるかをシミュレートした画像が示されています。最初と最後の区画の2組はどちらも青っぽく見え、中央の2区画は黄色っぽく見えることがわかります。したがって、色覚異常の利用者にとって最初の課題は、「Renewable」と「Fossil Fuels」の区画の境界を見分けることです。さらに、どの区画がどれに対応するかを順序だけで追跡しなければならず、認知負荷が増します。このグラフをさらにアクセシブルでなくする方法としては、区画の順序を凡例の順序と一致させないことが挙げられます。

では、どうすればこのグラフを改善できるでしょうか。まず、どの情報も色の使用だけで伝えられないようにしてください。その方法のひとつは、各区画にパターンを追加することです。さらに、各区画の境界を見分けやすくするために、高コントラストの境界線を追加できます。すると、グラフは例えば次のようになります。

<div>
<img src="chart-good.png" alt="同じ棒グラフを元の配色で示したもの。今回は各区画の周囲に黒い輪郭線が追加され、さらに各区画に固有のパターンが付けられている。" width="958" height="637" style="box-shadow: 0 4px 12px rgb(89 85 101 / 20%); width: 500px; max-width: 100%; height: auto; display: block; margin: 24px auto; border-radius: 6px">
</div>

一般的な色覚異常の人にも識別しやすい色を選ぶことで、このグラフはさらに改善できます。また、2色パターンを選ぶ、棒にあわせてそれらを整列させる、あるいはフォントの使い方を変えることで、デザインをさらに調整していくこともできます。

Webアプリでは、内蔵の色覚異常シミュレーターを使ってデザインを確認できます。利用するには、「View」メニューを開き、「Simulate color blindness」メニューで目的のモードを選択してください。Webアプリを使用していない場合でも、Web上の他のツールを使って、[さまざまな種類の色覚異常における色の見え方をシミュレーション][color-blind-simulator]できます。

背景と前景の間の色のコントラストにも注意してください。例えば、脚注に薄いグレーの文字を使うと、読みにくくなることがあります。低コントラストを招きやすいもう1つの状況として、画像の上に文字を重ねる場合があります。

<div>
<img src="color-contrast.png" alt="「注意。作動中のホチキスに手を近づけないでください」という文言を含む、デザインの異なる2つの注意喚起ボックス。各ボックスの下には文字と図形要素のコントラスト指標がある。左は薄い赤の背景に通常の赤文字で、文字コントラストは2.8:1、図形コントラストは1.4:1。右は白地に赤い枠線と濃い赤文字で、文字コントラストは5.9:1、図形コントラストは3.9:1。" width="1536" height="708" style="box-shadow: 0 4px 12px rgb(89 85 101 / 20%); width: 512px; max-width: 100%; height: auto; display: block; margin: 24px auto; border-radius: 6px">
</div>

この例では、注意喚起ボックスの2つのデザインを比較できます。これらのボックスはユーザーが危険を回避するのを助けることを目的としているため、ユーザーが実際にその内容を読めることがきわめて重要です。しかし、最初のボックスでは背景がかなり明るく、ボックス自体の輪郭を見分けにくくなっています。さらに悪いことに、薄い赤の背景上では赤い文字が読みにくくなっています。文字のコントラスト比は2.8:1で、Web Content Accessibility Guidelines（WCAG）が定める4.5:1の基準を満たしていません。同様に、ボックスの白いページ背景に対するコントラスト比も1.4:1で、グラフィカルオブジェクトに対する3:1の閾値を下回っています。

2つ目の例では、WCAG AAの色コントラスト閾値を満たすように色が調整されています。視力に問題がない場合でも、ボックス内の文字は明らかに読みやすくなっているはずです！

前景色と背景色として使う[2色の組み合わせがどの程度のコントラストを持つかを比較するためのツール][wcag-contrast]があります。最も一般的なのは、WCAGの色コントラスト比です。フォントサイズが決まると、色の組み合わせは、基準を満たさないか、AAレベルに達するか、あるいはより高いAAAレベルに達するかのいずれかになります。全ての色の組み合わせについて、少なくともAAコントラストを目指してください。

| コンテンツ                                   | AA比率 | AAA比率 |
| -------------------------------------------- | -----: | ------: |
| 大きい文字（18pt以上、または太字で14pt以上） |    3:1 |   4.5:1 |
| 小さい文字                                   |  4.5:1 |     7:1 |
| 非テキストコンテンツ                         |    3:1 |     3:1 |

WCAGのような一般的なアクセシビリティフレームワークでは、純粋に装飾目的の文字やロゴは例外扱いとなる点に注意してください。これらはグラフィックとしての性質を持つため、AAのコントラスト比基準を満たさないコントラスト比であっても許容される場合があります。

## テキストによる表現 { #textual-representations }

ATの利用や一部の再利用ワークフローをサポートするためには、セマンティックな意味を持つ全ての要素にテキストによる表現が必要です。これをユニバーサルアクセスの観点で考えてみてください。ある項目が[アーティファクト](#artifacts)でないなら、それはセマンティックな意味を持っています。しかし、ATがその項目を取り込めない場合、文書の持つセマンティックな意味をATユーザーは完全には受け取れません。したがって、ユニバーサルアクセスを実現するために、代替表現を提供するためのTypstの組み込み機能を利用してください。

画像を追加する際は、画像内で見えている内容を説明するために、必ず[image関数の`alt`引数]($image.alt)を使用してください。この代替説明（altテキストとも呼ばれます）は、画像の要点を説明するものにすべきです。電話で友人にその画像を説明するとしたら、どのように説明するかを考えてみてください。良い代替説明を書くためには、その画像が現れる文脈を考慮してください。
=======
The show-set rule completely changes the default appearance of the [`strong`] element, but its semantic meaning will be preserved. If you need even more customization, you can provide show rules with fully custom layout code and Typst will still retain the semantic purpose of the element.

## Reading order

For AT to read the contents of a document in the right order and for repurposing applications, accessible files must make their reading order explicit. This is because the logical reading order can differ from layout order. Floating figures are a common example for such a difference: A figure may be relevant to a paragraph in the center of a page but appear at the top or bottom edge. In non-accessible files, PDF readers and AT have to assume that layout order equals the logical reading order, often leading to confusion for AT users. When the reading order is well-defined, screen readers read a footnote or a floating figure immediately where it makes sense.

Fortunately, Typst markup already implies a single reading order. You can assume that Typst documents will read in the order that content has been placed in the markup. For most documents, this is good enough. However, when using the [`place`] and [`move`] function or [floating figures]($figure.placement), you must pay special attention to place the function call at an appropriate spot in the logical reading order in markup, even if this has no consequence on the layout. Just ask yourself where you would want a screen reader to announce the content that you are placing.

## Layout containers

Typst provides some layout containers like [`grid`], [`stack`], [`box`], [`columns`], and [`block`] to visually arrange your content. None of these containers come with any semantic meaning attached. Typst will conserve some of these containers during PDF reflow while other containers will be discarded.

When designing for Universal Access, you need to be aware that AT users often cannot view the visual layout that the container creates. Instead, AT will just read its contents, so it is best to think about these containers as transparent in terms of accessibility. For example, a grid's contents will just be read out flatly, in the order that you have added the cells in the source code. If the layout you created is merely visual and decorative, this is fine. If, however, the layout carries semantic meaning that is apparent to a sighted user viewing the file in a regular PDF reader, it is not accessible. Instead, create an alternative representation of your content that leverages text or wrap your container in the [`figure`] element to provide an alternative textual description.

Do not use the grid container to represent tabular data. Instead, use [`table`]. Tables are accessible to AT users: their AT will allow them to navigate the table two-dimensionally. Tables are conserved during reflow and repurposing. When creating tables, use the [`table.header`]($table.header) and [`table.footer`]($table.footer) elements to mark up the semantic roles of individual rows. The table documentation contains an [accessibility section]($table/#accessibility) with more information on how to make your tables accessible. Keep in mind that while AT users can access tables, it is often cumbersome to them: Tables are optimized for visual consumption. Being read the contents of a set of cells while having to recall their row and column creates additional mental load. Consider making the core takeaway of the table accessible as text or a caption elsewhere.

Likewise, if you use functions like [`rotate`], [`scale`], and [`skew`], take care that this transformation either has no semantic meaning or that the meaning is available to AT users elsewhere, i.e. in figure [alt text](#textual-representations) or a caption.

## Artifacts

Some things on a page have no semantic meaning and are irrelevant to the content of a document. We call these items _artifacts._ Artifacts are hidden from AT and repurposing and will vanish during reflow. Here are some examples for artifacts:

- The hyphens inserted by automatic hyphenation at the end of a line
- The headers and footers on each page
- A purely decorative page background image

In general, every element on a page must either have some way for AT to announce it or be an artifact for a document to be considered accessible.

Typst automatically tags many layout artifacts such as headers, footers, page back- and foregrounds, and automatic hyphenation as artifacts. However, if you'd like to add purely decorative content to your document, you can use the [`pdf.artifact`] function to mark a piece of content as an artifact. If you are unsure if you should mark an element as an artifact, ask yourself this: Would it be purely annoying if a screen reader announced the element to you? Then, it may be an artifact. If, instead, it could be useful to have it announced, then it is not an artifact.

For technical reasons, once in an artifact, content cannot become semantic again. To stack artifacts and semantic contents, use [`place`] to move the contents on top of one another.

Please note that Typst will mark shapes and paths like [`square`] and [`circle`] as artifacts while their content will remain semantically relevant and accessible to AT. If your shapes have a semantic meaning, please wrap them in the [`figure`] element to provide an alternative textual description.

## Color use and contrast

Universal Access not only means that your documents works with AT, reflow, and repurposing, but also that visual access is possible to everyone, including people with impaired eyesight. Not only does aging often come with worse sight, a significant portion of people have problems differentiating color: About 8% of men and 0.5% of women are color blind.

<div style="display:flex; gap: 8px 16px; width: 100%; flex-wrap: wrap; margin: 24px auto; ">
<img src="chart-bad-regular.png" alt="Bar chart showing Energy production in Germany by kind in terawatt-hours on the X axis and the year on the y-axis. Each bar has up to four segments, for Nuclear (violet), Renewables (green), Fossil Fuels (red), and Other (blue). There is a legend in the top right corner associating the segment colors with their labels" width="958" height="637" style="box-shadow: 0 4px 12px rgb(89 85 101 / 20%); width: 200px; max-width: 100%; height: auto; display: block; border-radius: 6px; flex-grow: 1">
<img src="chart-bad-deuteranopia.png" alt="The same bar chart with changed colors, with the segments for Nuclear and Other in a very similar dark blue, and the neighboring segments of Renewables and Fossil Fuels in two almost indistinguishable shades of sickly yellow" width="958" height="637" style="box-shadow: 0 4px 12px rgb(89 85 101 / 20%); width: 200px; max-width: 100%; height: auto; display: block; border-radius: 6px; flex-grow: 1">
</div>

This means that color must not be the only way you make information accessible to sighted users in your documents. As an example, consider a stacked bar chart with multiple colored segments per bar. Our example shows a chart of the domestic energy production in Germany by kind[^1]. In the picture, you can see the chart as it would normally appear and a simulation of how it would appear to people with deuteranopia-type color blindness. You can see that the two pairs of the first and last segment both look blue and the center pair looks yellow-ish. The first challenge for the colorblind user is thus to make out the boundary of the "Renewable" and "Fossil Fuels" bar. Then, they must keep track of which bar is which by only their order, adding to their mental load. A way to make this chart even less accessible would be to make the order of segments not match their order in the legend.

How can we improve the chart? First, make sure that no information is solely communicated through color use. One possible way to do this by adding a pattern to each bar. Then, we can help the user make out the boundaries of each segment by adding a high-contrast border. Then, our chart could look something like this:

<div>
<img src="chart-good.png" alt="The same bar chart with the original colors. This time, black outlines around each segment are added. Additionally, each segment has a unique pattern." width="958" height="637" style="box-shadow: 0 4px 12px rgb(89 85 101 / 20%); width: 500px; max-width: 100%; height: auto; display: block; margin: 24px auto; border-radius: 6px">
</div>

This could be further improved by choosing colors that are differentiable to people afflicted by common colorblindness types. You could also iterate on the design by choosing two-tone patterns, aligning them to the bars, or changing font use.

You can check your design in the web app by using the built-in color blindness simulator. To use it, open the "View" menu and select the desired mode in the "Simulate color blindness" menu. You can also use other tools on the web to [simulate the color perception of various color blindnesses][color-blind-simulator] if you are not using our web app.

Also consider the color contrast between background and foreground. For example, when you are using light gray text for footnotes, they could become hard to read. Another situation that often leads to low contrast is superimposing text on an image.

<div>
<img src="color-contrast.png" alt="Two callout boxes with the text 'Caution: Keep hands away from active stapler' with different designs. Each box has a contrast gauge for its text and graphical elements below it. The left box is shaded in a light red and the text is a regular shade of red. It has a text contrast of 2.8:1 and a graphics contrast of 1.4:1. The right box is white with a red outline and dark red text. It has a text contrast of 5.9:1 and a graphics contrast of 3.9:1." width="1536" height="708" style="box-shadow: 0 4px 12px rgb(89 85 101 / 20%); width: 512px; max-width: 100%; height: auto; display: block; margin: 24px auto; border-radius: 6px">
</div>

In our example, we can see two designs for callout boxes. Because these boxes aim to help the user avoid a hazard, it is paramount that they can actually read them. However, in the first box, the background is fairly light, making it hard to make out the box. Worse, the red text is difficult to read on the light red background. The text has a 2.8:1 contrast ratio, failing the bar of 4.5:1 contrast the Web Content Accessibility Guidelines (WCAG) set. Likewise, the box has an 1.4:1 contrast ratio with the white page background, falling short of the 3:1 threshold for graphical objects.

Colors in the second example have been adjusted to meet WCAG AA color contrast thresholds. It should be markedly easier to read the text in the box, even if you have good vision!

There are [tools to compare how much contrast a pair of colors has][wcag-contrast] as foreground and background. The most common one is the WCAG color contrast ratio. For a given font size, a color pair may either fail the test, get to the AA level, or reach the higher AAA level. Aim for at least AA contrast for all your color pairings.

| Content                                | AA Ratio | AAA Ratio |
|----------------------------------------|---------:|----------:|
| Large text (≥18pt, or bold and ≥14pt)  | 3:1      | 4.5:1     |
| Small text                             | 4.5:1    | 7:1       |
| Non-text content                       | 3:1      | 3:1       |

Note that common accessibility frameworks like WCAG make an exception for purely decorative text and logos: Due to their graphic character, they can have contrast ratios that fail to achieve AA contrast ratio.

## Textual representations

To support AT use and some repurposing workflows, all elements with a semantic meaning must have a textual representation. Think about it in terms of Universal Access: If an item is not an [artifact](#artifacts), it has a semantic meaning. If, however, AT cannot ingest the item, the full semantic meaning of a document is not available to AT users. Hence, to provide Universal Access, use the mechanisms built into Typst to provide alternative representations.

When you add an image, be sure to use the [`alt` argument of the image function]($image.alt) to describe what's visible in the image. This alternative description (sometimes known as alt text) should describe the gist of the image: Think about how you would describe the image to a friend if you called them on the phone. To write good alternative descriptions, consider the context in which the image appears:
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```example
#image("heron.jpg", alt: "?")

Herons have feet with interdigital
webbing, allowing for good mobility
when swimming, and wings that span
up to 2.3 m.
```

<<<<<<< HEAD
[この画像][heron]には、どのような代替説明が適切でしょうか？_避けるべき_例をいくつか見てみましょう。

- `{"サギの画像"}` \
  ❌ スクリーンリーダーは画像であることをすでに自動で読み上げるため、「画像」と言うのは冗長です。この例では、ATユーザーには「画像、サギの画像」と聞こえます。

- `{"鳥"}` \
  ❌ この代替説明は十分に具体的ではありません。例えば、この画像がサギを描いていること、そして足と翼の両方が見えていることは、ユーザーにとって重要な情報です。

- `{"飛行中のアオサギ。Wikimedia CommonsのMakasch1966による写真。CC Attribution 4.0 Internationalライセンス"}` \
  ❌ 代替説明には、帰属情報・ジョーク・メタデータのような、画像に見えていない情報を含めるべきではありません。その情報は目の見えるユーザーにはアクセシブルでないことを念頭に置いてください。そうした情報は別の場所に記載すべきです。

- `{"低空を右から左へ飛ぶアオサギ。足は伸びており、やや下向きで、暗い森が見え始めるぼやけた地平線に触れている。鳥の翼は広がって上向きに弧を描いている。画像の左下には、ピントの合っていない枝が見える。"}` \
  ❌ この代替説明は冗長すぎます。画像が内容にとってどの程度重要かを判断してください。目の見えるユーザーが現実的にどのくらいその画像を見るかを考えてください。altテキストも、読むのにかかる負担がだいたい同程度となるようにするべきです。例えば、上の説明内の解剖学的な記述は、動物学の教科書でより長い説明をする場合には適切かもしれません。一方、構図に関する情報は、写真について書く場合に有用です。この例の画像に付随する文脈は比較的短いため、より簡潔な説明にしてください。

代わりに、この例では次のような代替テキストを使うことができます。

- `{"足と翼を広げて飛ぶサギ"}` \
  ✅ この代替説明は画像を説明しており、文脈にも関連していて、求められる簡潔さにも合っています。

良い代替説明の書き方をさらに学ぶための[Web上のリソース][alt-text-tips]があります。画像に代替テキストを追加するという要件は、全ての画像形式に適用されます。Typstは現在、PDF画像ファイル単体がアクセシブルであっても、コンパイル後の文書内ではそのPDF画像のタグを保持しません。

文字を画像にしたものは使わないでください。同様に、パス操作を使って文字を手動で描画しないでください。Typstは、画像内の文字をネイティブなテキストと同じようにアクセシブルなものとして処理できません。このルールには1つだけ例外があります。文字の見た目が文書の意味にとって本質的であり、かつTypstのネイティブ機能では再現できない場合だけ、文字の画像を使用してください。その場合は、代替説明の中で、文字としての内容と本質的な視覚的特徴の両方を記述しなければなりません。

image関数と同様に、figure関数も[alt属性]($figure.alt)を持ちます。この属性を使用すると、多くのスクリーンリーダーやその他のATはfigure内部の内容を読み上げず、代わりに代替説明のみを読み上げます。代替説明は、AT利用者がfigure本体にアクセスする必要がないよう、十分に包括的でなければなりません。代替説明は、figureの内容にほかの方法ではアクセスできない場合にのみ使用してください。例えば、figureに`table`要素が含まれている場合は、そのfigureの`alt`属性を使用しないでください。一方、セマンティックな意味を持つ図形をfigure内で使用している場合は、`alt`属性を使用してください。`alt`と`caption`の両方を指定すると、その両方がATによって読み上げられます。figureに画像が含まれている場合、代替説明はfigureではなく[画像自体]($image.alt)に設定してください。両方への設定は行わないでください。画像の説明がfigureの説明によって上書きされてしまうためです。
=======
What could be a good alternative description for [this image][heron]? Let's consider a few examples for what _not_ to do:

- `{"Image of a heron"}` \
  ❌ The screen reader will already announce the image on its own, so saying this is an image is redundant. In this example, the AT user would hear "Image, Image of a heron".

- `{"A bird"}` \
  ❌ The alternative description is not specific enough. For example, it is relevant to a user that the image depicts a heron and both its feet and wings are visible.

- `{"Gray heron in flight. Picture by Makasch1966 on Wikimedia Commons, CC Attribution 4.0 International license"}` \
  ❌ The alternative description should not include details not visible in the image, such as attribution, jokes, or metadata. Keep in mind that it is not accessible to sighted users. That information belongs elsewhere.

- `{"Gray heron flying low, heading from the right to left. Its feet are extended and slightly point downwards, touching a blurred horizon where a dark forest becomes visible. The bird's wings are extended and arc upwards. There are out-of-focus branches visible in the lower left corner of the image."}` \
  ❌ The alternative description is too verbose. Use your discretion and determine how important the image is to the content. Think about how long a sighted user would realistically look at the image; your alt text should take about the same effort to 'consume.' For example, the anatomic description contained above could be appropriate for a longer discussion in a zoology textbook while the compositional information is useful when writing about photography. The context the example image comes with is relatively short, so write a more brief description.

Instead, in the given example, you could use this alternative text:

- `{"Heron in flight with feet and wings spread"}` \
  ✅ This alternative description describes the image, is relevant to the context, and matches its brevity.

There are resources available on the web [to learn more about writing good alternative descriptions][alt-text-tips]. The requirement to add alternative text to images applies to all image formats. Typst does not currently retain the tags of a PDF image in the compiled document, even if the PDF image file on its own was accessible.

Do not use images of text; likewise, do not use the path operations to draw text manually. Typst will not be able to process text in any images to make it accessible in the same way that native text is. There is one exception to this rule: Use an image of text when the appearance of the text is essential to the semantic meaning of the document and cannot be reproduced with Typst natively. In that case, you must describe both the textual content and the essential visual characteristics in the alternative description.

Like the image function, the figure function has a [`alt` attribute]($figure.alt). When you use this attribute, many screen readers and other AT will not announce the content inside of the figure and instead just read the alternative description. Your alternative description must be comprehensive enough so that the AT user does not need to access the body of the figure. Only use the alternative description if the content of the figure are not otherwise accessible. For example, do not use the `alt` attribute of a figure if it contains a `table` element, but do use it if you used shapes within that come with a semantic meaning. If you specify both `alt` and `caption`, both will be read by AT. When your figure contains an image, set the alternative description on the [image itself]($image.alt), not on the figure. Do not set both, as the image description would be overridden by the figure description.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```typ
#figure(
  alt: "Star with a blue outline",
  curve(
    stroke: blue,
    curve.move((25pt, 0pt)),
    curve.line((10pt, 50pt)),
    curve.line((50pt, 20pt)),
    curve.line((0pt, 20pt)),
    curve.line((40pt, 50pt)),
    curve.close(),
  ),
)
```

<<<<<<< HEAD
最後に、[`math.equation`]($math.equation)を使って数式に代替説明を指定できます。数式は、自然言語で声に出して読み上げる場合を想定して記述してください。現時点では、全てのエクスポート形式で数式をアクセシブルにするには、代替説明の追加が必要です。数式に代替説明を追加しなかった場合、PDF/UA-1としてのエクスポートは失敗します。将来的には、TypstはMathML技術を活用し、HTMLおよびPDF 2.0における数式を自動的にアクセシブルにする予定です。
=======
Finally, you can specify an alternative description on math using [`math.equation`]. Describe your formula as if read out loud in natural language. Currently, adding an alternative description is required for accessible math for all export formats. Not adding an alternative description for your formula will result in a failure of PDF/UA-1 export. In the future, Typst will automatically make math accessible in HTML and PDF 2.0 by leveraging MathML technology.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```typ
#math.equation(
  alt: "a squared plus b squared equals c squared",
<<<<<<< HEAD
=======
  block: true,
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
  $ a^2 + b^2 = c^2 $,
)
```

<<<<<<< HEAD
テキストとして表現されるもう1つの要素はリンクです。_here_や_go_のような、説明的でないリンクテキストは避けるのが望ましいです。こうしたリンクテキストは、文書において検索エンジン最適化（Search Engine Optimization、SEO）を考慮する場合にも悪影響を及ぼします。代わりに、リンク先がわかるテキストを、リンク自体に含めてください。なお、最高レベルのアクセシビリティを目指しているのでなければ、リンク自体の文言が説明的でなくても、その目的が直近の周辺の文脈から理解できるのであれば問題ありません。

## 自然言語 { #natural-language }

スクリーンリーダーが文書を正しく読み上げ、翻訳ソフトウェアが適切に動作するためには、文書がどの自然言語で書かれているかを明示する必要があります。文書の冒頭で[`[#set text(lang: "..")]`]($text.lang)ルールを使用するか、テンプレート側の言語設定機能を使用してください。そうしない場合、Typstはその内容を英語で書かれているものとみなします。選択した自然言語は、アクセシビリティだけでなく、Typstがどのようにハイフネーションを適用するか、どの組版規則が適用されるか、図や参照のラベル、そしてWebアプリではスペルチェックにどの言語が使われるかにも影響します。

中国語や英語のように地域差が大きい言語を使用する場合は、[`region`引数]($text.region)も使用してください。例えば、香港で話される中国語は次のように指定します。
=======
Another element that represents itself as text are links. It is best to avoid non-descriptive link texts such as _here_ or _go._ These link texts also hurt Search Engine Optimization (SEO) if that is a consideration for your document. Instead, try to have the link contain text about where it is pointing to. Note that, unless you are aiming for the highest level of accessibility, it is also okay if the link itself is not descriptive but its purpose can be understood from the content immediately surrounding it.

## Natural Language

In order for screen readers to pronounce your document correctly and translation software to work properly, you must indicate in which natural language your document is written. Use the rule [`[#set text(lang: "..")]`]($text.lang) at the very start of your document or your template's capability to set a language. If you do not do so, Typst will assume that your content is in English. The natural language you choose not only impacts accessibility, but also how Typst will apply hyphenation, what typesetting conventions are applied, the labels of figures and references, and, in the web app, what language is used for spellcheck.

If you are using a language with significant variation between regions, such as Chinese or English, also use [the `region` argument]($text.region). For example, Chinese as it is spoken in Hong Kong would look like this:
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```typ
#set text(lang: "zh", region: "HK")
```

<<<<<<< HEAD
言語を指定するには、ISO 639コードを使用します。地域については、[ISO 3166-1 alpha-2][iso-3166-1-alpha-2]コードを使用します。ISO 639には3種類の規格があり、ドイツ語の"de"のような2文字の言語コード用のものが1つ（[ISO 639-1][iso-639-1-list]）、"deu"のような3文字の言語コード用のものが2つ（[ISO 639-2][iso-639-2-list]と[ISO 639-3][iso-639-3-list]）あります。使用する言語に2文字のISO 639-1コードがある場合は、常にそちらを使用してください。ISO 639-2と639-3はほとんどのコードを共有していますが、いくつかの違いがあります。言語コードが両規格で異なる場合は、PDF 1.7（Typstのデフォルト）およびそれ以下へのエクスポートにはISO 639-2を、PDF 2.0とHTMLへのエクスポートにはISO 639-3を使用してください。

通常の言語コードを提供するのが難しい場合に使用できる、ISO 639-2とISO 639-3の両方で定義されている3つの特別な言語コードがあります。

- `zxx`は自然言語でないテキスト用
- `und`は自然言語を特定できないテキスト用
- `mis`は言語コードが割り当てられていない言語のテキスト用

文書に複数言語のテキストが含まれている場合は、text関数やスコープ付きのtext setルールを使用して、他の言語のインスタンスを囲むことができます。
=======
To specify your language, use ISO 639 codes. For regions, use the [ISO 3166-1 alpha-2][iso-3166-1-alpha-2] code. ISO 639 contains three variants, one for two-letter language codes like "de" for German [(ISO 639-1)][iso-639-1-list] and two for three-letter language codes like "deu" ([ISO 639-2][iso-639-2-list] and [ISO 639-3][iso-639-3-list]). If your language has a two-letter ISO 639-1 code, always prefer using that. ISO 639-2 and 639-3 share most codes, but there are some differences. When your language code differs between the two standards, use ISO 639-2 when exporting to PDF 1.7 (Typst's default) and below and ISO 639-3 for export to PDF 2.0 and HTML.

There are three special language codes defined by both ISO 639-2 and ISO 639-3 that you can use when providing a normal language code is difficult:

- `zxx` for text that is not in a natural language
- `und` for text for which you cannot determine the natural language
- `mis` for text in languages that have not been assigned a language code

If your document contains text in multiple languages, you can use the text function or a scoped text set rule to enclose instances of other languages:
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```example
This is #text(lang: "fr")[français].

#[
  #set text(lang: "es")
  Este es un fragmento más largo
  del texto en español.
]
```

<<<<<<< HEAD
## 文書タイトルと見出し { #document-title-and-headings }

文書にタイトルを付けると、ATユーザーにとっても、通常のPDFビューアーのユーザーにとっても、その文書を見つけたり、ほかの文書との間を移動したりしやすくなります。このため、WCAGやPDF/UAなどのアクセシビリティ規格では、文書に機械可読なタイトルを設定することが求められています。

Typstでこれを行うには、以下のsetルールを、文書内のどのコンテンツよりも前に配置してください。
=======
## Document Title and Headings

Titling your document makes it easier to retrieve it and to navigate between it and other documents, both for AT users and regular users of PDF viewers. This is why accessibility standards such as WCAG and PDF/UA require you to set a machine-readable title for your document.

To do so in Typst, place this set rule in your document before any content:
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```typ
#set document(title: "GlorboCorp Q1 2023 Revenue Report")
```

<<<<<<< HEAD
これにより、[文書メタデータ内のタイトル]($document.title)と、PDFビューアーやWebブラウザーのタイトルバーのタイトルが設定されます。テンプレートを使用していてこれがエラーになる場合は、テンプレート側で文書タイトルを設定する別の方法が用意されていないか確認してください。

おそらく、文書内にもタイトルを目に見える形で示したくなるでしょう。そのためには、[`title`]($title)要素を使用します。title要素を引数なしで呼び出すと、文書タイトルとして設定したコンテンツがそのまま出力されます。あるいは、コンテンツを位置引数のbodyとして渡して、タイトルをカスタマイズもできます。文書内でtitle要素を複数回使用しないでください。

文書タイトルに見出しを使ってはいけません。代わりにtitle要素を使用してください。HTMLの経験がある場合は、Typstにおけるheading要素のセマンティクスがHTMLの見出しとは異なることを覚えておくことが重要です。Typst文書では、セクション見出しとして複数の第1レベルの見出しを使用することが推奨されます。HTMLにエクスポートする際、[`title`]($title)は`h1`タグとしてシリアル化される一方、[第1レベルの見出し]($heading.level)は`h2`タグとしてシリアル化されます。PDFエクスポートでは、対象とするPDFバージョンに基づいて、タイトルと見出しが正しくタグ付けされます。

使用する見出しの階層は、順番通りであることが重要です。より深い階層へ進むとき、見出しレベルを飛ばしてはいけません。つまり、第3レベルの見出しの後は、レベル4以下の見出しを続ける必要があります。ただし、レベル5以上を続けてはいけません。
=======
This will set the [title in the document's metadata]($document.title) and in the title bar of the PDF viewer or web browser. If this results in an error when using a template, consider whether your template may provide an alternative way to set the document title.

Most likely, you will also want to include the title visibly in your document. To do so, use the [`title`] element. When you add a call to the title element without any arguments, it will print the contents of what you set as the document's title. Alternatively, you can customize the title by passing content as the positional body argument. Do not use the title element more than once in your document.

Never use a heading for your document title; instead, use the title element. Should you have experience with HTML, it is important to remember that the semantics of the heading element in Typst differ from HTML headings. It is encouraged to use multiple first-level headings for section headings in Typst documents. When exporting to HTML, a [title] will be serialized as a `h1` tag while a [first-level heading]($heading.level) will be serialized as a `h2` tag. In PDF export, the title and headings will be correctly tagged based on the PDF version targeted.

It is important that the sequence of headings you use is sequential: Never skip a heading level when going deeper. This means that a third-level heading must be followed by a heading of level four or lower, but never a heading of level five or higher.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```typ
// ❌ Don't do this:
= First level heading
=== Third level heading
```

<<<<<<< HEAD
[Adobe Acrobatの自動アクセシビリティチェック][acro-check-outline]を通過するためには、21ページ以上の文書にはアウトライン化された見出しが必要であることに注意してください。

## アクセシビリティ規格と法令 { #accessibility-standards-and-legislation }

Typstは、あなたの文書を国際規格に照らしてチェックすることで、その文書がアクセシブルであると主張することを支援できます。PDFエクスポートに関しては、アクセシブルなファイルのための複数の規格があり、特にPDF/UA規格が有名です。その第1部（PDF/UA-1）はすでにTypstでサポートされており、第2部（PDF/UA-2）のサポートは将来に向けて計画されています。以下に、関連する全ての規格についての説明を示します。

- **Tagged PDF:** Tagged PDFには、ATが解析できる、文書のセマンティックな構造に関する機械可読なデータが含まれています。TypstはデフォルトでTagged PDFを書き出しますが、Typstは文書のセマンティックな構造について知っている場合にのみ適切なタグを付けることができることに注意してください。Typstの要素を使用してセマンティクスを伝える方法については、[_セマンティクスの保持_](#maintaining-semantics)セクションを参照してください。ユニバーサルアクセスを提供するためには、非テキストコンテンツのテキスト表現も自分で提供する責任があります。

- **PDF/UA-1:** PDF/UA-1規格は、ユニバーサルアクセスに最適化されたPDF 1.7ファイルの書き方を説明しています。Tagged PDFを前提とし、画像や数式の代替説明を強制し、文書タイトルを要求し、表などの文書内容の構造に関するルールを導入します。このガイドに従っていれば、PDF/UA-1エクスポート中に発生する可能性のあるほとんどのコンパイルエラーをすでに回避しています。

- **PDF/UA-2:** PDF 2.0ファイルを対象とした、より新しいPDF/UA-2という規格もあります。これは、数式やいくつかのセマンティックな要素のアクセシビリティを改善します。PDF/UA-2のサポートはまだTypstでは利用できませんが、計画されています。

- **Well Tagged PDF (WTPDF):** これは、PDF/UA-2に非常によく似た業界規格です。PDF/UA-2と同様に、現在Typstではサポートされていません。もともとは、PDF/UA仕様の両方の部分が国際標準化機構から高額でしか入手できなかったために策定されました。そのため、[WTPDF][WTPDF]は全ての適合するファイルがPDF/UA-2への適合も宣言できるように設計されました。現在では、[PDF/UA仕様の両方の部分が無料で利用可能になっている][pdf-ua-free]ため、WTPDFの関連性は低下しています。

- **PDF/A-1a:** PDF/A規格は、アーカイブに適したPDFファイルの作成方法を説明しています。PDF/A規格の第1部から第3部には複数の適合レベルがあります。最も厳しい適合レベルAにはアクセシビリティのルールが含まれており、そのルールを満たすファイルだけが将来の幅広い人々にとって利用可能な状態を保てます。レベルAはTagged PDFへの適合を前提とし、画像の代替説明を提供することを強制します。アクセシビリティに関連しない他のPDF/Aルール（例：透明性、色など）も適用されます。PDF/A規格のこの部分は、古いPDF 1.4仕様に基づいています。提出先がそれを要求する場合や、非常に互換性の高いファイルが必要な場合にのみ使用してください。それ以外の場合は、PDF/UA-1とPDF/Aの第2部と第3部がより良い代替手段を提供します。

- **PDF/A-2a**と**PDF/A-3a:** PDF/Aの第1部と同様に、これらの規格はアーカイブや長期保存に適したファイルの作成に焦点を当てています。これらの両方の規格は、PDF 1.4ではなく新しいPDFバージョン1.7を対象としています。ここでも、最も厳しい適合レベルAにはアクセシビリティのルールが含まれています。PDF/A-1aのルールに加えて、これらの規格では、意味が普遍的に定義されていない[Unicode Private Use Area][unic-pua]の文字の使用が禁止されています。PDF/A-1に対する改善点には、透明性の使用やより良いリフローが可能になることが含まれます。PDF/A規格のこの2つの部のどちらを選ぶかについては、他のファイルを[添付]($pdf.attach)する必要がない限り、PDF/A-2aを選択してください。適合レベルAは専用のPDF/UA規格を優先するため、PDF/A-4からは削除されたことに注意してください。

[PDFリファレンスページ]($pdf/#pdf-standards)には、サポートされている各規格に関する詳細な情報が含まれています。PDF/UA、PDF/A-2a、またはPDF/A-3aのいずれかを有効にするには、[CLIの適切なフラグ]($pdf/#command-line)を使用するか、Webアプリでエクスポートのドロップダウンを使用してPDFをクリックしてください。現時点では、PDF/AとPDF/UAのどちらかを選択する必要があります。アクセシビリティに焦点を当てた文書の場合は、後者をおすすめします。

PDFエクスポート用にこれらの規格のいずれかを選択すると、Typstはあなたがそのルールに違反しているかどうかを検出し、違反している場合は説明付きのエラーメッセージを表示してエクスポートを失敗させます。現時点で利用可能な最も厳格なアクセシビリティチェックを行うには、PDF/UA-1を選んでください。タグは、エクスポートする全ての文書に対してアクセシビリティのベースラインを提供するため、正当な理由がない限りタグ付けを無効にしないでください。

ユニバーサルアクセスを構成する要素の中には、自動チェックの難しいものがあることにすでにお気づきかもしれません。例えば、Typstは現時点では、色のコントラストが十分かどうか、あるいは設定された自然言語が実際の自然言語と一致しているかを自動的にはチェックしません（ただし、Webアプリを使っている場合はスペルチェックエラーの数が手掛かりになるはずです）。これらの人間的な要素のいくつかをより詳細に扱う国際規格が2つあります。

- **[Web Content Accessibility Guidelines (WCAG)][WCAG]**: インターネットを支える技術の背後にある大規模な国際コンソーシアムであるW3Cによって策定されたWCAGは、Webサイトをアクセシブルにする方法を示しています。これらのルールは全てTypstのHTML出力に適用可能であり、その多くはTypstのPDF出力にも適用されます。WCAGはそのルールをA、AA、AAAの3つのレベルに分けています。通常の文書はAAを目指すことが推奨されます。ユニバーサルアクセスに高い基準を求めるのであれば、AAAの達成基準も検討できます。ただし、TypstはまだAAA準拠に必要な全てのPDF機能を公開しているわけではありません。例えば、略語の展開をATがアクセスできる方法で定義することなどです。
- **[European Norm EN 301 549][EN301549]**: 第9節は、アクセシブルなWebサイトの作成方法を説明し、第10節は、Typstで作成されたPDFを含む非Web文書に適用されるルールを説明しています。この規格は、どのWCAG条項がPDFにも適用されるかを指摘しています。この規格への適合は、EUおよび各国のアクセシビリティ法に準拠するための良い出発点です。

EN 301 549および関連するWCAGの規定に適合するためには、文書がタグ付けされていなければならないことに留意してください。適合を目指すのであれば、それらに含まれる達成基準に関する多くのチェックを自動化するために、PDF/UA-1をエクスポートに使用することを強くおすすめします。

多くの地域には、一定の状況下でアクセシブルなファイルの作成を要求するアクセシビリティ法があります。以下はその一部です。

- **[European Accessibility Act (EAA, EU 2019/882)][EAA]**: この規則は、電子書籍、消費者向け銀行サービス、電子商取引サービスなどに適用されます。これらのアプリケーションで配布されるファイルがアクセシブルであることを要求します。
- **Americans with Disabilities Act (ADA)**: 米国司法省は、2026年までにADA第II編の下で、[公的機関に対してWCAGに準拠したファイルの提供を要求する][ADA-2]予定です。同様に、[民間組織もADAおよび州法の下でアクセシブルでないデジタルサービスに対して責任を問われる可能性][ADA-dominos]があります。

このガイドの使用は、どちらの規則への準拠に近づくのにも役立ちます。

## アクセシビリティのテスト { #testing-for-accessibility }

PDF文書がアクセシブルかどうかをテストするには、自動ツールと手動テストを利用できます。PDF/UAやPDF/Aのような一部の規格は自動ツールだけでチェックできますが、WCAGやその他の規格のいくつかのルールは手動チェックが必要です。自動化可能なチェックの多くは、Tagged PDFが有効になっていればTypstによって自動的に通過します。さらに多くの自動化可能なチェックについては、PDF/UA-1エクスポートを有効にすると、Typstが代わりにそれらを実行します。自動化ツールが提供できるのはアクセシビリティの基準に限られます。真のユニバーサルアクセスのためには、ATを使って自分で文書を試すのが最善です。

適合性の確認に使える自動チェッカーの一覧は以下の通りです。

- **[veraPDF][veraPDF]:** このオープンソースツールは、あなたのPDFファイルが、PDF/AおよびPDF/UA規格のうち、そのファイルが適合を宣言している部分に準拠しているかどうかを確認できます。エクスポート時にこれらの規格のいずれかを選択した場合は、このツールを使用してください。失敗はTypstのバグと見なされるため、[GitHubで報告](https://github.com/typst/typst/issues)してください。

- **[PDF Accessibility Checker (PAC)][PAC]:** フリーウェアのPACは、あなたの文書がPDF/UAやWCAGのルールに準拠しているかどうかをチェックします。PDF/UAタブで重大なエラーが表示された場合、これはTypstのバグと見なされるため、[GitHubで報告](https://github.com/typst/typst/issues)してください。PDF/UAとQualityタブの警告は、バグ、文書の問題、またはいずれでもない可能性があります。わからない場合は、[フォーラム][Typst Forum]や[Discord][Discord]で確認してください。WCAGタブのエラーと警告は、あなたの文書に問題があることを示しています。

- **[Accessibility Check in Adobe Acrobat Pro][acro-check]:** 有償版Adobe Acrobatに搭載されているアクセシビリティチェッカーは、全てのPDF文書について問題がないかを確認します。よく知られた国際規格や業界規格への準拠を確認するのではなく、Adobeは独自のテスト群を作成しています。これらのテストの判定基準が、PDF/UAのような国際規格と矛盾することがあるため、Acrobatの一部のチェックはTypst文書では失敗することが想定されます[^2]。コントラストチェックなどの他のチェックは有用であり、あなたの文書に問題があることを示します。

手動チェックを行う際は、まずチェックリストを使って始めることができます。あなたの組織がアクセシビリティを重視している場合、独自のチェックリストを持っていることがあります。そのようなリストがない場合は、[ブレーメン大学（英語）][checklist-unib]や[カナダ政府][checklist-canada]、[米国社会保障局][checklist-us-ssa]などの大学や政府のリストを試してみることができます。これらのチェックリストは記述の詳しさが異なりますが、全て最も重要な手動チェックをカバーしています。TypstでPDF/UA-1エクスポートを選択していれば、それらに含まれる技術的なチェックの多くはスキップできます。どのチェックリストを使用するか迷った場合は、自分の組織と文化的に近い組織のものを選んでください。

しかしながら、広く流通する文書で最高水準のアクセシビリティを目指すのであれば、ATを用いて文書をチェックすることを検討してください。AT製品やPDFビューアーは多数ありますが、通常は1つの組み合わせをテストするだけで十分です。どれが最適かは、使用しているオペレーティングシステムによって異なります。

- Windows: [Adobe Acrobat][Acrobat]と[NVDA][NVDA]でテストしてください。NVDAは無料のオープンソースソフトウェアです。Acrobatの無料版も利用可能です。
- macOS: [Adobe Acrobat][Acrobat]と[VoiceOver][VoiceOver]でテストしてください。VoiceOverはmacOSやその他のAppleプラットフォームに組み込まれているスクリーンリーダーです。
- Linux: [Evince][Evince]または[Okular][Okular]と[Orca][Orca]でテストしてください。これらの3つのツールは全て無料のオープンソースソフトウェアです。ただし、Linuxプラットフォーム全体でのATサポートは、WindowsやmacOSで利用可能なものよりも遅れています。同様に、EvinceやOkularはAcrobatほどアクセシビリティサポートが充実していません。代わりにAcrobatでテストすることを強くおすすめします。

テストを始めたばかりのときは、使っているスクリーンリーダーに対話型のトレーニングプログラムがある場合は、それを完了することを検討してください。スクリーンリーダーに慣れることは、常時スクリーンリーダーを使うユーザーのように文書を体験する助けになります。文書を確認する際は、目の見えるユーザーが利用できるものと同じ情報の全てにアクセスできることだけでなく、ナビゲーションしやすいことも確認してください。ユーザーが得る体験は、使用するPDFビューアーとATの組み合わせによって異なります。

## エクスポート形式に関する制限事項と留意点 { #limits-and-considerations-for-export-formats }

アクセシビリティを意識して文書を設計した場合でも、エクスポート形式の制限を理解しておく必要があります。基本的に、PDFファイルに対するATのサポートは、HTMLのようなほかの形式に比べて実装が難しいものです。PDFは1993年に、印刷文書をコンピューター上で正確に表示するために考案されました。アクセシビリティ機能は2001年のPDF 1.4で初めて追加され、その後PDF 1.5（2003年）とPDF 2.0（2017年）で改善されました。対照的に、HTMLはより豊富なセマンティックモデルと柔軟性を提供するため、ブラウザーにおけるATのサポートは一般にPDFビューアーで可能なものを上回っています。

また、PDFファイルは基本的に静的なものであることも覚えておいてください。このため、インタラクティブなコンテンツやマルチメディア向けに設計されたWCAGやEN 301 549のルールの多くは無視できます。ただし、インタラクティブ性がないことによって、ユーザーが自分のニーズにあわせて文書のレイアウトを調整することは、かえって難しくなります。

例えば、[WCAG達成基準1.4.12][wcag-sg-1412-us]（EN 301 549の第10.1.4.12項に明文化されています）は、ユーザーが文字・字間・行間・段落間隔を非常に大きい値まで拡大できなければならないと規定しています。これは、弱視のあるユーザーやディスレクシアのあるユーザーに有益です。この達成基準は、文書をこれらのレイアウトパラメーターで設計することまでは要求していません。その代わりに、この達成基準が求めているのは、ユーザーが文書を読む際にこれらのパラメーターを増やせる仕組みだけです。HTMLファイルでは、ブラウザーがページ上のこれらの間隔パラメーターをユーザー側から上書きできるようにしているため、この達成基準に準拠するには容易です。PDFの場合は、事情がもう少し複雑です。理論上は、Typstはリフロー用に設計されたタグと属性をファイルに追加します。PDFリーダーはリフロー時に、これらのタグで規定されている範囲を超えて間隔を広げられるよう、ユーザーに許可できる可能性があります。しかし実際には、私たちはこの機能を備えたPDFビューアーを把握していません。その代わりに、この達成基準はPDFをHTMLとして再利用し、ブラウザーで開けば満たすことができます。

実際には、たとえあなたのファイルが技術的には準拠していても、ユーザーがこうした回避策を知っていることを期待できません。したがって、ユニバーサルアクセスの最高水準を満たすことを目指すのであれば、PDFとあわせて文書のHTMLバージョンを配布することを検討してください。Typstの[HTMLエクスポート]($html)（プレビュー中）を直接使用してこのファイルをエクスポートしてください。HTMLエクスポートは視覚的なレイアウトの多くの側面を保持しませんが、セマンティックなHTMLや[Digital Publishing ARIA][dpub-aria]などの技術を活用してユニバーサルアクセスを提供するファイルを生成します。PDFファイルをHTMLに再利用したものよりも高品質になります。

最後に、PDFはもともと印刷を目的として設計された形式であることを忘れないでください。したがって、あなたの文書を印刷して読むことを選んだユーザーに対して、リンクのようなインタラクティブ機能が利用できるものだと想定してはいけません。

前述の通り、PNGおよびSVGエクスポートで作成されたファイルはアクセシブルではありません。

[^1]: ドイツ連邦統計局（Statistisches Bundesamt, Destatis）のデータセット。["Bruttostromerzeugung nach Energieträgern in Deutschland ab 1990"](https://www.destatis.de/DE/Themen/Branchen-Unternehmen/Energie/Erzeugung/bar-chart-race.html)。2025年。_Data licence Germany – attribution – version 2.0._の下で利用可能です。

[^2]: 例えば脚注を使用する場合、「List」セクションにある「Lbl and LBody must be children of LI」というチェック項目が失敗（不合格）になることが想定されます。
=======
Note that in order to pass the [automated accessibility check in Adobe Acrobat][acro-check-outline], documents with 21 pages or more must contain outlined headings.

## Accessibility Standards and Legislation

Typst can help you to assert that your document is accessible by checking it against international standards. For PDF export, there are multiple standards for accessible files, most notably the PDF/UA standard. Its first part (PDF/UA-1) is already supported by Typst while support for the second part (PDF/UA-2) is planned for the future. Below, you can find an explanation of all relevant standards:

- **Tagged PDF:** Tagged PDFs contain machine-readable data about the semantic structure of a document that AT can parse. Typst will write Tagged PDFs by default, but keep in mind that Typst can only write appropriate tags if it knows about the semantic structure of your document. Refer to the Section [_Maintaining semantics_](#maintaining-semantics) to learn how to use Typst's elements to communicate semantics. To provide Universal Access, you are also responsible to provide textual representation of non-text content yourself.

- **PDF/UA-1:** The PDF/UA standard explains how to write a PDF 1.7 file optimized for Universal Access. It implies Tagged PDF, enforces alternative descriptions for images and mathematics, requires a document title, and introduces rules how document contents like tables should be structured. If you are following this guide, you are already avoiding most of the compiler errors that can occur during PDF/UA-1 export.

- **PDF/UA-2:** There is also the more recent part PDF/UA-2 that targets PDF 2.0 files. It improves accessibility for mathematics and some semantic elements. Support for PDF/UA-2 not yet available in Typst, but planned.

- **Well Tagged PDF (WTPDF):** This is an industry standard that is very similar to PDF/UA-2. Like PDF/UA-2, it is not currently supported by Typst. Originally, it was drafted because both parts of the PDF/UA specification were only available at a high cost from the International Standards Organization. Hence, [WTPDF][WTPDF] was designed so that all conforming files can also declare conformance with PDF/UA-2. By now, [both parts of the PDF/UA specification are available free of charge][pdf-ua-free], decreasing the relevance of WTPDF.

- **PDF/A-1a:** The PDF/A standard describes how to produce PDF files that are well-suited for archival. Parts one to three of the PDF/A standard feature multiple conformance levels. The strictest conformance level A contains rules for accessibility as only files meeting those rules remain usable to the broadest range of people in the far future. Level A implies conformance with Tagged PDF and forces you to provide alternative descriptions for images. Other PDF/A rules not relating to accessibility, e.g. about transparency, colors, and more also apply. This part of the PDF/A standard is based on the outdated PDF 1.4 specification. Only use it if your venue requires it or if you need a very compatible file. Otherwise, PDF/UA-1 and the second and third part of PDF/A provide better alternatives.

- **PDF/A-2a** and **PDF/A-3a:** Like the first part of PDF/A, these standards focus on creating files suitable for archival and long-term storage. Both of these standards target the newer PDF version 1.7 instead of PDF 1.4. Here too, the strictest conformance level A contains rules for accessibility. In addition to the rules in PDF/A-1a, these standards disallow the use of characters in the [Unicode Private Use Area][unic-pua] whose meaning is not universally defined. Improvements over PDF/A-1 include the ability to use transparency and better reflow. When choosing between these two parts of the PDF/A standard, choose PDF/A-2a unless you need to [attach]($pdf.attach) other files. Note that conformance level A has been removed from PDF/A-4 in favor of the dedicated PDF/UA standard.

The [PDF reference page]($pdf/#pdf-standards) contains more information about each supported standard. To enable either PDF/UA, PDF/A-2a, or PDF/A-3a, use the [appropriate flag in the CLI]($pdf/#command-line) or use the export dropdown and click on PDF in the web app. At the moment, you must choose between PDF/A and PDF/UA. For accessibility-focused documents, we recommend the latter.

When you select one of these standards for PDF export, Typst will detect if you are in violation of their rules and fail the export with a descriptive error message. For the strictest accessibility check currently available, choose PDF/UA-1. Do not disable tagging unless you have a good reason, as tags provide a baseline of accessibility across all documents you export.

Maybe you already noticed that some of the factors that go into Universal Access are hard to check automatically. For example, Typst will currently not automatically check that your color contrasts are sufficient or whether the configured natural language matches the actual natural language (although the amount of spellcheck errors should provide a hint if you are using the web app). There are two international standards that address some of these human factors in more detail:

- The **[Web Content Accessibility Guidelines (WCAG)][WCAG]**: Designed by the W3C, a big international consortium behind the technologies that power the internet, WCAG describes how to make a web site accessible. All of these rules are applicable to Typst's HTML output, and many of them apply to its PDF output. WCAG separates its rules into the three levels A, AA, and AAA. It is recommended that normal documents aim for AA. If you have high standards for Universal Access, you can also consider AAA Success Criteria. However, Typst does not yet expose all PDF features needed for AAA compliance, e.g. an AT-accessible way to define expansions for abbreviations.
- The **[European Norm EN 301 549][EN301549]**: Its Section 9 describes how to create accessible websites and its Section 10 describes what rules apply to non-web documents, including PDFs created by Typst. It points out which WCAG clauses are also applicable to PDFs. Conformance with this standard is a good start for complying with EU and national accessibility laws.

Keep in mind that in order to conform with EN 301 549 and the relevant WCAG provisions, your document must be tagged. If you aim for conformance, we strongly suggest using PDF/UA-1 for export to automate many of the checks for the success criteria within.

Many territories have accessibility legislation that requires you to create accessible files under some circumstances. Here are only some of them:

- **[European Accessibility Act (EAA, EU 2019/882)][EAA]**: This regulation applies to e-books, consumer banking services, e-commerce services, and more. It requires the files distributed in these applications to be accessible.
- **Americans with Disabilities Act (ADA)**: The Department of Justice will [require public sector organizations to provide files][ADA-2] in accordance to WCAG under Title II of the ADA by 2026. Likewise, [private organizations can be held liable][ADA-dominos] for inaccessible digital services under the ADA and state law.

Using this guide can help you reach compliance with either regulation.

## Testing for Accessibility

In order to test whether your PDF document is accessible, you can use automated tools and manual testing. Some standards like PDF/UA and PDF/A can be checked exclusively through automated tools, while some rules in WCAG and other standards require manual checks. Many of the automatable checks are automatically passed by Typst when Tagged PDF is enabled. For many other automatable checks, you can enable PDF/UA-1 export so that Typst will run them instead. Automated tools can only provide a baseline of accessibility. For truly Universal Access, it is best if you try the document yourself with AT.

Here is a list of automated checkers to try to test for conformance:

- **[veraPDF][veraPDF]:** This open-source tool can check if your PDF file conforms to the parts of the PDF/A and PDF/UA standards it declared conformance with. Use this tool if you have chosen one of these standards during export. Failures are considered bugs in Typst and should be [reported on GitHub](https://github.com/typst/typst/issues).

- **[PDF Accessibility Checker (PAC)][PAC]:** The freeware PAC checks whether your document complies with PDF/UA and WCAG rules. When you receive a hard error in the PDF/UA tab, this is considered a bug in Typst and should be [reported on GitHub](https://github.com/typst/typst/issues). Warnings in the PDF/UA and Quality tabs may either be bugs, problems in your document, or neither. Check on the [Forum][Typst Forum] or on [Discord][Discord] if you are unsure. Errors and warnings in the WCAG tab indicate problems with your document.

- **[Accessibility Check in Adobe Acrobat Pro][acro-check]:** The accessibility checker in the paid version of Adobe Acrobat checks all PDF documents for problems. Instead of checking compliance with a well-known international or industry standard, Adobe has created their own suite of tests. Because the rules behind these tests sometimes contradict international standards like PDF/UA, some of Acrobat's checks are expected to fail for Typst documents[^2]. Other checks, such as the contrast check are useful and indicate problems with your document.

When doing manual checking, you can start with a checklist. If your organization places emphasis on accessibility, they will sometimes have their own list. In absence of one, you can try lists by universities such as [Universität Bremen (in English)][checklist-unib] or governments such as in [Canada][checklist-canada] or by the [US Social Security Administration][checklist-us-ssa]. Although these checklists differ in verbosity, they all cover the most essential manual checks. Many of the technical checks in them can be skipped if you choose PDF/UA-1 export in Typst. If unsure which checklist to use, choose one from an organization culturally similar to yours.

However, to reach the highest standard of accessibility for widely circulated documents, consider checking your document with AT. Although there are many AT products and PDF viewers, it is typically sufficient to test a single combination. Which is best differs depending on your operating system:

- Windows: Test with [Adobe Acrobat][Acrobat] and [NVDA][NVDA]. NVDA is free, open-source software. A free version of Acrobat is available.
- macOS: Test with [Adobe Acrobat][Acrobat] and [VoiceOver][VoiceOver]. VoiceOver is the screen reader that is built into macOS and other Apple platforms.
- Linux: Test with [Evince][Evince] or [Okular][Okular] and [Orca][Orca]. All three tools are free, open-source software. However, AT support across Linux platforms lags behind what is available on Windows and macOS. Likewise, Evince and Okular have less accessibility support than Acrobat. We strongly suggest testing with Acrobat instead.

When first getting into testing, consider completing the interactive training program your screen reader offers, if any. Building confidence with a screen reader helps you experience your document like a full-time screen reader user. When checking your document, check that it not only makes all the same information accessible that is available to a sighted user, but also that it is easy to navigate. The experience your users will have will vary based on the pairing of PDF viewer and AT they use.

## Limits and considerations for export formats

Even when you design your document with accessibility in mind, you should be aware of the limitations of your export format. Fundamentally, AT support for PDF files is more difficult to implement than for other formats such as HTML. PDF was conceived in 1993 to accurately render print documents on a computer. Accessibility features were first added with PDF 1.4 in 2001, and improved in PDF 1.5 (2003) and PDF 2.0 (2017). By contrast, HTML offers a richer semantic model and more flexibility, so AT support in browsers generally surpasses what is possible in PDF viewers.

Also keep in mind that PDF files are mostly static. This allows you to disregard many WCAG and EN 301 549 rules designed for interactive content and multimedia. However, the lack of interactivity also makes it more difficult for users to customize a document's layout to their needs.

For example, [WCAG Success Criterion 1.4.12][wcag-sg-1412-us] (codified in Clause 10.1.4.12 of EN 301 549) prescribes that a user must be able to increase character, letter, line, and paragraph spacing to very wide values. This benefits users with reduced vision or dyslexia. The Success Criterion does not require you to design your document with these layout parameters. Instead, it only requires a mechanism through which users can increase these parameters when reading the document. For HTML files, it is easy to comply with this Success Criterion because the browser lets the user override these spacing parameters on a page. For PDF, the situation is more nuanced: Theoretically, Typst adds tags and attributes designed for reflow to a file. A PDF reader, when reflowing, could allow its user to increase spacings beyond what is codified in these tags. In practice, we are not aware of a PDF viewer with this feature. Instead, this Success Criterion can be satisfied by repurposing the PDF into a HTML file and opening it in a browser.

In practice, even if your file is technically compliant, you cannot expect your users to know about these workarounds. Therefore, if you are aiming to meet the highest standards of Universal Access, consider distributing an HTML version of your document alongside your PDF. Export this file directly using Typst's [HTML export]($html) (in preview). Even though HTML export will not conserve many aspects of your visual layout, it will produce a file that leverages semantic HTML and technologies like [Digital Publishing ARIA][dpub-aria] to provide Universal Access. It will be of a higher quality than a PDF file repurposed to HTML.

Finally, keep in mind that PDFs are designed for print. Hence, you should not assume that interactive features like links are available to users who chose to print your document.

As mentioned above, files created by PNG and SVG export are not accessible.

[^1]: Dataset from the German Federal Statistics Authority (Statistisches Bundesamt, Destatis). ["Bruttostromerzeugung nach Energieträgern in Deutschland ab 1990"](https://www.destatis.de/DE/Themen/Branchen-Unternehmen/Energie/Erzeugung/bar-chart-race.html), 2025, available under the _Data licence Germany – attribution – version 2.0._

[^2]: For example, when using footnotes, the check "Lbl and LBody must be children of LI" in the "List" section is expected to fail.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

[NVDA]: https://www.nvaccess.org/download/
[Acrobat]: https://www.adobe.com/acrobat.html
[VoiceOver]: https://support.apple.com/guide/voiceover/welcome/mac
[wcag-contrast]: https://webaim.org/resources/contrastchecker/ "WebAIM Contrast Checker"
[wcag-sg-1412-us]: https://www.w3.org/WAI/WCAG21/Understanding/text-spacing.html "Understanding SC 1.4.12: Text Spacing (Level AA)"
[alt-text-tips]: https://webaim.org/techniques/alttext/
[iso-3166-1-alpha-2]: https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2 "ISO 3166-1 alpha-2"
[iso-639-1-list]: https://en.wikipedia.org/wiki/List_of_ISO_639_language_codes "List of ISO 639 language codes"
[iso-639-2-list]: https://en.wikipedia.org/wiki/List_of_ISO_639-2_codes "List of ISO 639-2 codes"
[iso-639-3-list]: https://en.wikipedia.org/wiki/List_of_ISO_639-3_codes "List of ISO 639-3 codes"
[WCAG]: https://www.w3.org/TR/WCAG21/
[EN301549]: https://www.etsi.org/deliver/etsi_en/301500_301599/301549/03.02.01_60/en_301549v030201p.pdf
[EAA]: https://eur-lex.europa.eu/eli/dir/2019/882/oj "Directive (EU) 2019/882 of the European Parliament and of the Council of 17 April 2019 on the accessibility requirements for products and services (Text with EEA relevance)"
[ADA-2]: https://www.ada.gov/law-and-regs/regulations/title-ii-2010-regulations/ "Americans with Disabilities Act Title II Regulations"
[ADA-dominos]: https://www.boia.org/blog/the-robles-v.-dominos-settlement-and-why-it-matters "The Robles v. Domino’s Settlement (And Why It Matters)"
[color-blind-simulator]: https://daltonlens.org/colorblindness-simulator "Online Color Blindness Simulators"
[unic-pua]: https://en.wikipedia.org/wiki/Private_Use_Areas "Private Use Areas"
[pdf-ua-free]: https://pdfa.org/sponsored-standards/ "Sponsored ISO standards for PDF technology"
[WTPDF]: https://pdfa.org/wtpdf/ "Well-Tagged PDF (WTPDF)"
[acro-check]: https://helpx.adobe.com/acrobat/using/create-verify-pdf-accessibility.html "Create and verify PDF accessibility (Acrobat Pro)"
[acro-check-outline]: https://helpx.adobe.com/acrobat/using/create-verify-pdf-accessibility.html#Bookmarks "Create and verify PDF accessibility (Acrobat Pro) - Bookmarks"
[veraPDF]: https://verapdf.org "Industry Supported PDF/A Validation"
[PAC]: https://pac.pdf-accessibility.org/en "PDF Accessibility Checker"
[Typst Forum]: https://forum.typst.app/
[Discord]: https://discord.gg/2uDybryKPe
[checklist-unib]: https://www.uni-bremen.de/fileadmin/user_upload/universitaet/Digitale_Transformation/Projekt_BALLON/Checklisten/2._Auflage_englisch/Checklist_for_accessible_PDF_ENG-US_ver2.pdf "Accessible E-Learning and Teaching - Checklist for Creating and Reviewing Accessible PDF Documents"
[checklist-canada]: https://a11y.canada.ca/en/pdf-accessibility-checklist/ "PDF accessibility checklist"
[checklist-us-ssa]: https://www.ssa.gov/accessibility/checklists/PDF_508_Compliance_Checklist.pdf
"Portable Document Format (PDF) Basic Testing Guide"
[Evince]: https://wiki.gnome.org/Apps/Evince/
[Okular]: https://okular.kde.org/ "Okular - The Universal Document Viewer"
[Orca]: https://orca.gnome.org "Orca - A free and open source screen reader"
[heron]: https://commons.wikimedia.org/wiki/File:Reiher_im_Flug.jpg
[dpub-aria]: https://www.w3.org/TR/dpub-aria-1.1/ "Specification for Digital Publishing WAI-ARIA Module 1.1"
