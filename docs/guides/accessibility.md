---
description: |
  Typst を使ってアクセシブルな文書を作成する方法を学びます。
  このガイドでは、セマンティックマークアップ、読み上げ順序、代替テキスト、色のコントラスト、言語設定、PDF/UA準拠などをカバーし、全ての読者と支援技術に対応するファイルを作成する方法を説明します。
---

# アクセシビリティガイド

文書をアクセシブルにするということは、全ての人がその文書を使用し理解できるようにすることを意味します。それには、恒久的または一時的な障害を持つ人々だけでなく、さまざまなデバイスや好みを持つ人々も含まれます。アクセシビリティが重要な理由を強調するために、人々があなたの想定以上にさまざまな状況で文書を読む可能性があることを考えてみてください。

- ユーザーが文書を紙に印刷するかもしれない
- ユーザーがPDFリーダーのリフロー機能を有効にして、携帯電話で文書を読むかもしれない
- ユーザーが自身のコンピューターに文書を読み上げさせるかもしれない
- ユーザーが人工知能に文書の要約を依頼するかもしれない
- ユーザーが文書をHTMLなどの、よりアクセシブルなファイル形式に変換するかもしれない

これら全ての人々とシナリオに対応するために、**ユニバーサルアクセス**のために文書を設計するべきです。ユニバーサルアクセスは、単純ですが強力な原則です。後からアクセシビリティ対応を付け足すのではなく、最初から、できるだけ幅広いユーザーと利用状況に対応できるように設計します。これにより、全ての読者の体験が向上します！

Typstは、スクリーンリーダーで読み取りやすく、異なる画面サイズ向けにリフローされても見栄えがよく、自動アクセシビリティチェッカーにも合格しやすいアクセシブルなファイルの作成を支援できます。ただし、アクセシブルなファイルを作成するには、いくつかのルールを意識する必要があります。このガイドでは、アクセシビリティに影響する要因、ユニバーサルアクセスのための設計方法、そしてそれを実現するためにTypstが提供するツールを学べます。ここで述べる指針の多くは、全てのエクスポート形式に当てはまりますが、本ガイドはPDFエクスポートに焦点を当てています。HTMLエクスポートとの重要な違いも記載します。

## アクセシビリティの基礎 { #basics }

アクセシブルなファイルは、ソフトウェアが単にファイルを描画する以上のことを行えるようにします。すなわち、コンピュータが文書の各要素が何を表しているかを理解し、その情報を用いてユーザーに文書を提示できるようになるのです。

この情報はアクセスを提供するためにさまざまなソフトウェアによって利用されます。TypstからPDFをエクスポートすると、_PDFビューアー_（リーダーとも呼ばれます）が、Typstのプレビューでデザインした通りに文書のページを表示します。なかには、スクリーンリーダーや点字ディスプレイ、画面拡大鏡などの_支援技術_（AT：Assistive Technology）に頼ってPDFを利用する人もいます。その場合、ファイルに含まれるセマンティック情報が使われ、内容は音声や文字、あるいは別の視覚表現へと変換されます。一方で、別のユーザーはPDFビューアーにファイルをリフローさせて、Webページに似たレイアウトを作ります。内容はビューポートの幅に収まり、連続的にスクロールできるようになります。さらに、別のユーザーはPDFを別の形式に再利用します。たとえば大規模言語モデル（LLM：Large Language Model）に取り込むためのプレーンテキストやHTMLなどです。再利用の特殊な形としてコピー＆ペーストがあり、ユーザーはクリップボードを使ってファイルから内容を抽出し、別のアプリケーションで利用します。

アクセシビリティ対応はビューアーとATによって異なります。組み合わせによっては、他よりもうまく動作するものもあります。私たちのテストでは、Windowsでは[Adobe Acrobat][Acrobat]と[NVDA][NVDA]の組み合わせ、macOSでは[VoiceOver][VoiceOver]が最も充実したアクセシビリティ対応を提供しました。またHTMLエクスポートと組み合わせた場合、ブラウザはPDFリーダーと比べて、より一貫したアクセシビリティの基準を提供します。

アクセシブルなファイルを生成できるのは、PDFとHTMLのエクスポートだけです。PNGとSVGは、どちらも単体ではアクセシブルではありません。どちらの形式も、[テキストによる表現](#textual-representations)を用意することで、アクセシブルなより大きな成果物の中で利用できます。

## セマンティクスの保持 { #maintaining-semantics }

ATによる利用や再利用のために正しいセマンティック情報をファイルに付与するためには、ファイル中の各部分がどのようなセマンティック上の役割を果たすのかをTypstが把握する必要があります。例えば、これはコンパイルされたPDFの見出しが単なる大きく太い文字のテキストであってはならないということを意味します。代わりに、その特定のテキストが見出しを構成することを示す明示的な情報（_タグ_として知られます）を含めるべきです。そうするとスクリーンリーダーはそれを見出しとして読み上げ、ユーザーが見出し間を移動できるようになります。

Typstの慣用的な使い方、すなわち組み込みのマークアップや要素を用いている場合、Typstは自動的にリッチなセマンティック情報を持つタグをファイルへ付与します。次の2つのコード例を見てみましょう。

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

どちらの例も、見た目は同じです。どちらも"Heading"というテキストが太字で、サイズは16ptになっています。しかしながら、アクセシブルなのは例のうち2つ目のみです。見出し用のマークアップを使うことで、Typstはこのテキストのセマンティックな意味が「見出し」であることを理解し、その情報を最終的なPDFに反映できます。1つ目の例では、Typstは通常のテキストに対して太字と大きめの文字を適用すべきだということしか分かりません。そのため、それが見出しを意図したものなのか、スタイル上の選択なのか、あるいは引用のような別の要素なのかを推測することはできません。

セマンティクスの利用は見出しに限りません。以下はセマンティクスを利用すべき要素のさらなる例です。

- テキストを強調する場合は[`text`]関数の代わりにアンダースコア / [`emph`]を使用する
- テキストに強い強調を与える場合は[`text`]関数の代わりにアスタリスク / [`strong`]を使用する
- アイテム化された内容や順序のある内容を扱う場合は、改行を伴う通常のテキストの代わりにリスト（[`list`]、[`enum`]、[`terms`]）を使用する
- インライン引用やブロック引用には[`quote`]を使用する
- 文献を手動で印刷するのではなく、組み込みの[`bibliography`]と[`cite`]関数を使用する
- 文書の他の部分を参照する場合は、単に参照をタイプするのではなく、ラベルと[`ref`]または`@references`を使用する
- キャプションを提供する場合は、テキストを関数呼び出しの下に追加するのではなく、[`figure`要素の`caption`引数]($figure.caption)を使用する

要素のデフォルトのスタイルを調整したい時であっても、独自のカスタム関数で置き換えるのではなく、[setルール]($styling/#set-rules)、show-setルール、そして[showルール]($styling/#show-rules)を使用して外観をカスタマイズしてください。以下は、ドキュメント内の強い強調の見た目を変更する方法の例です。

```example
// Change how text inside of strong emphasis looks
#show strong: set text(tracking: 0.2em, fill: blue, weight: "black")

When setting up your tents, *never forget* to secure the pegs.
```

show-setルールは[`strong`]要素のデフォルトの見た目を完全に変更しますが、そのセマンティックな意味は保持します。さらにカスタマイズが必要な場合は、完全に独自のレイアウトコードを持つshowルールを指定できますが、それでもTypstはその要素のセマンティックな目的を保持します。

## 読み上げ順序 { #reading-order }

ATが文書中のコンテンツを正しい順序で読み上げられるようにするため、また再利用アプリケーションのためにも、アクセシブルなファイルは読み上げ順序を明示しなければなりません。これは、論理的な読み上げ順序がレイアウト順序とは異なる可能性があるためです。こうした差異の典型的な例がフロート図表です。図表がページ中央の段落に関連するものであっても、ページの上端や下端に配置されることがあります。アクセシブルでないファイルでは、PDFリーダーやATはレイアウト順序と論理的な読み上げ順序が同一であると推測せざるを得ず、その結果ATユーザーに混乱を招くことがよくあります。読み上げ順序が適切に定義されていれば、スクリーンリーダーは脚注やフロート図表を、意味が通る位置で直ちに読み上げます。

幸い、Typstのマークアップは既に単一の読み上げ順序を暗黙的に含んでいます。Typst文書は、マークアップ内で内容を配置した順に読み上げられると考えてよいでしょう。ほとんどの文章ではそれで十分です。ただし、[`place`]関数や[`move`]関数、あるいは[フロート図表]($figure.placement)を使用する場合は、たとえレイアウトに影響しなくても、マークアップ上の論理的な読み上げ順序として適切な位置に関数呼び出しを置くよう、特に注意が必要です。配置しようとしている内容をスクリーンリーダーにどの位置で読み上げてほしいかを自問してみてください。

## レイアウトコンテナ { #layout-containers }

Typstはコンテンツを視覚的に配置するための、[`grid`]、[`stack`]、[`box`]、[`columns`]、[`block`]などのレイアウトコンテナを提供します。これらのコンテナにはセマンティックな意味は付与されません。TypstはPDFリフローの際にこれらのコンテナの一部を保持しますが、他のコンテナは破棄されます。

ユニバーサルアクセスのための設計の際、ATユーザーはコンテナが作り出す視覚的なレイアウトを閲覧できないことが多い、という点を認識しておく必要があります。代わりに、ATはその内容をただ読み上げるだけなので、アクセシビリティの観点ではこれらのコンテナは透過的なものと考えるのが最善です。例えば、グリッドの内容は、ソースコード内でセルを追加した順番通りに、ただ平坦に読み上げられるだけです。あなたが作成したレイアウトが単に視覚的・装飾的なものであれば、それで問題ありません。しかし、もしそのレイアウトが通常のPDFリーダーを用いてファイルを閲覧する、目の見える人にとっては明らかなセマンティックな意味を持つ場合は、それはアクセシブルではありません。その代わりに、テキストを活用した代替表現を作成するか、あるいは代替となるテキストによる表現を提供するために、[`figure`]要素で包んでください。

グリッドコンテナを表形式データの表現に使用してはいけません。代わりに[`table`]を使用してください。表はATユーザーにとってアクセシブルであり、ユーザーはATによって表を二次元的に移動して参照できます。表はリフローや再利用の際にも保持されます。表を作成する際には[`table.header`]($table.header)と[`table.footer`]($table.footer)要素を使用して、個々の行のセマンティックな役割をマークアップしてください。表のドキュメントには、表をアクセシブルにする方法について詳しく説明した[アクセシビリティセクション]($table/#accessibility)があります。また、ATユーザーが表にアクセスできるとはいえ、それがしばしば負担になることにも留意してください。表は視覚的な閲覧に最適化されているためです。セル群の内容を読み上げられながら、その行と列を思い出し続けなければならない状況は、追加の認知負荷を生みます。表の要点を、別の箇所でテキストやキャプションとしてアクセシブルに提供することを検討してください。

同様に、[`rotate`]、[`scale`]、[`skew`]のような関数を使用する場合は、この変換がセマンティックな意味を持たないか、あるいは意味がATユーザーに他の場所、つまり図の[代替テキスト](#textual-representations)やキャプションなどで利用可能であることに注意してください。

## アーティファクト { #artifacts }

ページ上の一部のものには、セマンティックな意味がなく、文書の内容にも関係しないものがあります。これらの項目を_アーティファクト_と呼びます。アーティファクトはATや再利用からは隠され、リフローの際には消失します。以下はアーティファクトの例です。

- 行末に自動ハイフネーションによって挿入されるハイフン
- 各ページのヘッダーおよびフッター
- 純粋に装飾目的のページ背景画像

一般に文書がアクセシブルと見なされるためには、ページ上のすべての要素が、ATが読み上げられる何らかの方法を持つか、またはアーティファクトである必要があります。

Typstは、ヘッダー、フッター、ページの背景・前景、自動ハイフネーションなど、多くのレイアウト上のアーティファクトを自動的にアーティファクトとしてタグ付けします。ただし、文書に純粋に装飾的な内容を追加したい場合は、[`pdf.artifact`]関数を使用して、コンテンツの一部をアーティファクトとしてマークできます。要素をアーティファクトとしてマークすべきかどうか迷った場合は、自問してみてください。スクリーンリーダーがその要素を読み上げるとしたら、単に邪魔になるだけでしょうか？それならば、それはアーティファクトかもしれません。逆に、読み上げられると便利なものであれば、それはアーティファクトではありません。

技術的な理由により、いったんアーティファクトの中に入ると、コンテンツは再びセマンティックになることはできません。アーティファクトとセマンティックな内容を重ねるには、[`place`]を使用して内容を互いの上に移動してください。

Typstは、[`square`]や[`circle`]のような形状やパスをアーティファクトとしてマークしますが、その内容はセマンティックに関連し、ATにアクセス可能なままになることに留意してください。形状にセマンティックな意味がある場合は、代替のテキスト説明を提供するために、[`figure`]要素でそれらを包んでください。

## 色の使用とコントラスト { #color-use-and-contrast }

ユニバーサルアクセスとは、文書がAT、リフロー、再利用に対応していることのみを意味するのではありません。視力が低下している人も含め、すべての人が視覚的にアクセシブルであることを意味します。加齢にはしばしば視力の低下が伴うだけでなく、かなりの割合の人が色の識別に困難を抱えています。具体的には、男性の約8%、女性の約0.5%が色覚異常です。

<div style="display:flex; gap: 8px 16px; width: 100%; flex-wrap: wrap; margin: 24px auto; ">
<img src="chart-bad-regular.png" alt="Bar chart showing Energy production in Germany by kind in terawatt-hours on the X axis and the year on the y-axis. Each bar has up to four segments, for Nuclear (violet), Renewables (green), Fossil Fuels (red), and Other (blue). There is a legend in the top right corner associating the segment colors with their labels" width="958" height="637" style="box-shadow: 0 4px 12px rgb(89 85 101 / 20%); width: 200px; max-width: 100%; height: auto; display: block; border-radius: 6px; flex-grow: 1">
<img src="chart-bad-deuteranopia.png" alt="The same bar chart with changed colors, with the segments for Nuclear and Other in a very similar dark blue, and the neighboring segments of Renewables and Fossil Fuels in two almost indistinguishable shades of sickly yellow" width="958" height="637" style="box-shadow: 0 4px 12px rgb(89 85 101 / 20%); width: 200px; max-width: 100%; height: auto; display: block; border-radius: 6px; flex-grow: 1">
</div>

これは、文書内の情報を目の見える人にアクセシブルにする方法として、色を唯一の手段にしてはならない、ということを意味します。例として、1本の棒が複数の色分けされた区画で構成される積み上げ棒グラフを考えてみましょう。この例では、ドイツ国内のエネルギー生産量を種類別に示したグラフ[^1]を扱います。この図には、グラフの通常の見え方と、2型色覚の色覚異常がある人にはどのように見えるかをシミュレートした画像が示されています。最初と最後の区画の2組はどちらも青っぽく見え、中央の2区画は黄色っぽく見えることが分かります。したがって、色覚異常の利用者にとって最初の課題は、「Renewable」と「Fossil Fuels」の区画の境界を見分けることです。さらに、どの区画がどれに対応するかを順序だけで追跡しなければならず、認知負荷が増します。このグラフをさらにアクセシブルでなくする方法としては、区画の順序を凡例の順序と一致させないことが挙げられます。

では、どうすればこのグラフを改善できるでしょうか。まず、どの情報も色の使用だけで伝えられないようにしてください。その方法のひとつは、各区画にパターンを追加することです。さらに、各区画の境界を見分けやすくするために、高コントラストの境界線を追加できます。すると、グラフはたとえば次のようになります。

<div>
<img src="chart-good.png" alt="The same bar chart with the original colors. This time, black outlines around each segment are added. Additionally, each segment has a unique pattern." width="958" height="637" style="box-shadow: 0 4px 12px rgb(89 85 101 / 20%); width: 500px; max-width: 100%; height: auto; display: block; margin: 24px auto; border-radius: 6px">
</div>

一般的な色覚異常の人にも識別しやすい色を選ぶことで、このグラフはさらに改善できます。また、2色パターンを選ぶ、棒に合わせてそれらを整列させる、あるいはフォントの使い方を変えることで、デザインをさらに調整していくこともできます。

Webアプリでは、内蔵の色覚異常シミュレーターを使ってデザインを確認できます。利用するには、「View」メニューを開き、「Simulate color blindness」メニューで目的のモードを選択してください。Webアプリを使用していない場合でも、Web上の他のツールを使って、[さまざまな種類の色覚異常における色の見え方をシミュレーション][color-blind-simulator]できます。

背景と前景の間の色のコントラストにも注意してください。たとえば、脚注に薄いグレーの文字を使うと、読みにくくなることがあります。低コントラストを招きやすいもう一つの状況として、画像の上に文字を重ねる場合があります。

<div>
<img src="color-contrast.png" alt="Two callout boxes with the text 'Caution: Keep hands away from active stapler' with different designs. Each box has a contrast gauge for its text and graphical elements below it. The left box is shaded in a light red and the text is a regular shade of red. It has a text contrast of 2.8:1 and a graphics contrast of 1.4:1. The right box is white with a red outline and dark red text. It has a text contrast of 5.9:1 and a graphics contrast of 3.9:1." width="1536" height="708" style="box-shadow: 0 4px 12px rgb(89 85 101 / 20%); width: 512px; max-width: 100%; height: auto; display: block; margin: 24px auto; border-radius: 6px">
</div>

この例では、注意喚起ボックスの2つのデザインを比較できます。これらのボックスはユーザーが危険を回避するのを助けることを目的としているため、ユーザーが実際にその内容を読めることが極めて重要です。しかし、最初のボックスでは背景がかなり明るく、ボックス自体の輪郭を見分けにくくなっています。さらに悪いことに、薄い赤の背景上では赤い文字が読みにくくなっています。文字のコントラスト比は2.8:1で、Web Content Accessibility Guidelines（WCAG）が定める4.5:1の基準を満たしていません。同様に、ボックスの白いページ背景に対するコントラスト比も1.4:1で、グラフィカルオブジェクトに対する3:1の閾値を下回っています。

2つ目の例では、WCAG AA の色コントラスト閾値を満たすように色が調整されています。視力に問題がない場合でも、ボックス内の文字は明らかに読みやすくなっているはずです!

前景色と背景色として使う[2色の組み合わせがどの程度のコントラストを持つかを比較するためのツール][wcag-contrast]があります。最も一般的なのは、WCAGの色コントラスト比です。フォントサイズが決まると、色の組み合わせは、基準を満たさないか、AAレベルに達するか、あるいはより高いAAAレベルに達するかのいずれかになります。すべての色の組み合わせについて、少なくともAAコントラストを目指してください。

| コンテンツ                                   | AA比率 | AAA比率 |
| -------------------------------------------- | -----: | ------: |
| 大きい文字（18pt以上、または太字で14pt以上） |    3:1 |   4.5:1 |
| 小さい文字                                   |  4.5:1 |     7:1 |
| 非テキストコンテンツ                         |    3:1 |     3:1 |

WCAGのような一般的なアクセシビリティフレームワークでは、純粋に装飾目的の文字やロゴは例外扱いとなる点に注意してください。これらはグラフィックとしての性質を持つため、AA のコントラスト比基準を満たさないコントラスト比であっても許容される場合があります。

## テキストによる表現 { #textual-representations }

ATの利用や一部の再利用ワークフローをサポートするためには、セマンティックな意味を持つ全ての要素にテキストによる表現が必要です。これをユニバーサルアクセスの観点で考えてみてください。ある項目が[アーティファクト](#artifacts)でないなら、それはセマンティックな意味を持っています。しかし、ATがその項目を取り込めない場合、文書の持つセマンティックな意味をATユーザーは完全には受け取れません。したがって、ユニバーサルアクセスを実現するために、代替表現を提供するためのTypstの組み込み機能を利用してください。

画像を追加する際は、画像内で見えている内容を説明するために、必ず[image関数の`alt`引数]($image.alt)を使用してください。この代替説明（altテキストとも呼ばれます）は、画像の要点を説明するものにすべきです。電話で友人にその画像を説明するとしたら、どのように説明するかを考えてみてください。良い代替説明を書くためには、その画像が現れる文脈を考慮してください。

```example
#image("heron.jpg", alt: "?")

Herons have feet with interdigital
webbing, allowing for good mobility
when swimming, and wings that span
up to 2.3 m.
```

[この画像][heron]には、どのような代替説明が適切でしょうか？_避けるべき_例をいくつか見てみましょう。

- `{"サギの画像"}` \
  ❌ スクリーンリーダーは画像であることをすでに自動で読み上げるため、「画像」と言うのは冗長です。この例では、ATユーザーには「画像、サギの画像」と聞こえます。

- `{"鳥"}` \
  ❌ この代替説明は十分に具体的ではありません。たとえば、この画像がサギを描いていること、そして足と翼の両方が見えていることは、ユーザーにとって重要な情報です。

- `{"飛行中のアオサギ。Wikimedia Commonsの Makasch1966による写真。CC Attribution 4.0 Internationalライセンス"}` \
  ❌ 代替説明には、帰属情報・ジョーク・メタデータのような、画像に見えていない情報を含めるべきではありません。その情報は目の見えるユーザーにはアクセシブルでないことを念頭に置いてください。そうした情報は別の場所に記載すべきです。

- `{"低空を右から左へ飛ぶアオサギ。足は伸びており、やや下向きで、暗い森が見え始めるぼやけた地平線に触れている。鳥の翼は広がって上向きに弧を描いている。画像の左下には、ピントの合っていない枝が見える。"}` \
  ❌ この代替説明は冗長すぎます。画像が内容にとってどの程度重要かを判断してください。目の見えるユーザーが現実的にどのくらいその画像を見るかを考えてください。altテキストも、読むのにかかる負担がだいたい同程度になるようにするべきです。たとえば、上の説明に含まれる解剖学的な記述は、動物学の教科書でより長い説明をする場合には適切かもしれません。一方、構図に関する情報は、写真について書く場合に有用です。この例の画像に付随する文脈は比較的短いため、より簡潔な説明にしてください。

代わりに、この例では次のような代替テキストを使うことができます。

- `{"足と翼を広げて飛ぶサギ"}` \
  ✅ この代替説明は画像を説明しており、文脈にも関連していて、求められる簡潔さにも合っています。

良い代替説明の書き方をさらに学ぶための [Web 上のリソース][alt-text-tips] があります。画像に代替テキストを追加するという要件は、すべての画像形式に適用されます。Typstは現在、PDF画像ファイル単体がアクセシブルであっても、コンパイル後の文書内ではそのPDF画像のタグを保持しません。

文字を画像にしたものは使わないでください。同様に、パス操作を使って文字を手動で描画しないでください。Typstは、画像内の文字をネイティブなテキストと同じようにアクセシブルにするために処理できません。このルールには1つだけ例外があります。文字の見た目が文書の意味にとって本質的であり、かつTypstのネイティブ機能では再現できない場合に限って、文字の画像を使用してください。その場合は、代替説明の中で、文字としての内容と本質的な視覚的特徴の両方を記述しなければなりません。

image関数と同様に、figure関数も[alt属性]($figure.alt)を持ちます。この属性を使用すると、多くのスクリーンリーダーやその他のATはfigure内部の内容を読み上げず、代わりに代替説明のみを読み上げます。代替説明は、AT利用者がfigure本体にアクセスする必要がないよう、十分に包括的でなければなりません。代替説明は、figureの内容にほかの方法ではアクセスできない場合にのみ使用してください。たとえば、figureに`table`要素が含まれている場合は、そのfigureの`alt`属性を使用しないでください。一方、セマンティックな意味を持つ図形をfigure内で使用している場合は、`alt`属性を使用してください。`alt`と`caption`の両方を指定すると、その両方がATによって読み上げられます。figureに画像が含まれている場合、代替説明はfigureではなく[画像自体]($image.alt)に設定してください。両方への設定は行わないでください。画像の説明がfigureの説明によって上書きされてしまうためです。

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

最後に、[`math.equation`]を使って数式に代替説明を指定できます。数式は、自然言語で声に出して読み上げる場合を想定して記述してください。現時点では、すべてのエクスポート形式で数式をアクセシブルにするために、代替説明の追加が必要です。数式に代替説明を追加しなかった場合、PDF/UA-1におけるエクスポートは失敗します。将来的には、TypstはMathML技術を活用し、HTMLおよびPDF 2.0における数式を自動的にアクセシブルにする予定です。

```typ
#math.equation(
  alt: "a squared plus b squared equals c squared",
  $ a^2 + b^2 = c^2 $,
)
```

テキストとして表現される要素のもう一つが、リンクです。_here_ や _go_ のような、説明的でないリンクテキストは避けるのが望ましいです。こうしたリンクテキストは、文書において検索エンジン最適化（SEO：Search Engine Optimization）を考慮する場合、SEOにも悪影響を及ぼします。代わりに、リンクがどこを指しているのかをわかるテキストを、リンク自体に含めるようにしてください。なお、最高レベルのアクセシビリティを目指しているのでなければ、リンク自体の文言が説明的でなくても、その目的が直近の周辺の文脈から理解できるのであれば問題ありません。

## 自然言語 { #natural-language }

スクリーンリーダーが文書を正しく読み上げ、翻訳ソフトウェアが適切に動作するためには、文書がどの自然言語で書かれているかを明示する必要があります。文書の冒頭で[`[#set text(lang: "..")]`]($text.lang)ルールを使用するか、テンプレート側の言語設定機能を使用してください。そうしない場合、Typstはその内容を英語で書かれているものとみなします。選択した自然言語は、アクセシビリティだけでなく、Typstがどのようにハイフネーションを適用するか、どの組版規則が適用されるか、図や参照のラベル、そしてWebアプリではスペルチェックにどの言語が使われるかにも影響します。

中国語や英語のように地域差が大きい言語を使用する場合は、[`region`引数]($text.region)も使用してください。たとえば、香港で話される中国語は次のように指定します。

```typ
#set text(lang: "zh", region: "HK")
```

言語を指定するには、ISO 639コードを使用します。地域については、[ISO 3166-1 alpha-2][iso-3166-1-alpha-2] コードを使用します。ISO 639には3種類の規格があり、ドイツ語の"de"のような2文字の言語コード用のものが1つ（[ISO 639-1][iso-639-1-list]）、"deu" のような3文字の言語コード用のものが2つ（[ISO 639-2][iso-639-2-list] と [ISO 639-3][iso-639-3-list]）あります。使用する言語に2文字のISO 639-1コードがある場合は、常にそちらを使用してください。ISO 639-2と639-3はほとんどのコードを共有していますが、いくつかの違いがあります。言語コードが両規格で異なる場合は、PDF 1.7（Typstのデフォルト）およびそれ以下へのエクスポートにはISO 639-2を、PDF 2.0とHTMLへのエクスポートにはISO 639-3を使用してください。

通常の言語コードを提供するのが難しい場合に使用できる、ISO 639-2とISO 639-3の両方で定義されている3つの特別な言語コードがあります。

- `zxx` は自然言語でないテキスト用
- `und` は自然言語を特定できないテキスト用
- `mis` は言語コードが割り当てられていない言語のテキスト用

文書に複数言語のテキストが含まれている場合は、text関数やスコープ付きのtext setルールを使用して、他の言語のインスタンスを囲むことができます。

```example
This is #text(lang: "fr")[français].

#[
  #set text(lang: "es")
  Este es un fragmento más largo
  del texto en español.
]
```

## 文書タイトルと見出し { #document-title-and-headings }

文書にタイトルを付けると、ATユーザーにとっても、通常のPDFビューアーのユーザーにとっても、その文書を見つけたり、ほかの文書との間を移動したりしやすくなります。このため、WCAGやPDF/UAなどのアクセシビリティ標準では、文書に機械可読なタイトルを設定することが求められています。

Typstでこれを行うには、以下のsetルールを、文書内のどのコンテンツよりも前に配置してください。

```typ
#set document(title: "GlorboCorp Q1 2023 Revenue Report")
```

これにより、[文書メタデータ内のタイトル]($document.title)と、PDFビューアーやWebブラウザのタイトルバーのタイトルが設定されます。テンプレートを使用していてこれがエラーになる場合は、テンプレート側で文書タイトルを設定する別の方法が用意されていないか確認してください。

おそらく、文書内にもタイトルを目に見える形で示したくなるでしょう。そのためには、[`title`]要素を使用します。title要素を引数なしで呼び出すと、文書タイトルとして設定したコンテンツがそのまま出力されます。あるいは、コンテンツを位置引数としてのbody引数として渡して、タイトルをカスタマイズすることもできます。文書内でtitle要素を複数回使用しないでください。

文書タイトルに見出しを使ってはいけません。代わりにtitle要素を使用してください。HTMLの経験がある場合は、Typstにおけるheading要素のセマンティクスがHTMLの見出しとは異なることを覚えておくことが重要です。Typst文書では、セクション見出しとして複数の第1レベルの見出しを使用することが推奨されます。HTMLにエクスポートする際、[title]は`h1`タグとしてシリアル化される一方、[第1レベルの見出し]($heading.level)は`h2`タグとしてシリアル化されます。PDFエクスポートでは、対象とするPDFバージョンに基づいて、タイトルと見出しが正しくタグ付けされます。

使用する見出しの階層は、順番通りであることが重要です。より深い階層に進むときに、見出しレベルを飛ばしてはいけません。つまり、第3レベルの見出しの次には、レベル4以下の見出しが続く必要がありますが、レベル5以上の見出しが続いてはいけません。

```typ
// ❌ Don't do this:
= First level heading
=== Third level heading
```

[Adobe Acrobat の自動アクセシビリティチェック][acro-check-outline]を通過するためには、21ページ以上の文書にはアウトライン化された見出しが必要であることに注意してください。

## Accessibility Standards and Legislation { #accessibility-standards-and-legislation }

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

## Testing for Accessibility { #testing-for-accessibility }

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

## Limits and considerations for export formats { #limits-and-considerations-for-export-formats }

Even when you design your document with accessibility in mind, you should be aware of the limitations of your export format. Fundamentally, AT support for PDF files is more difficult to implement than for other formats such as HTML. PDF was conceived in 1993 to accurately render print documents on a computer. Accessibility features were first added with PDF 1.4 in 2001, and improved in PDF 1.5 (2003) and PDF 2.0 (2017). By contrast, HTML offers a richer semantic model and more flexibility, so AT support in browsers generally surpasses what is possible in PDF viewers.

Also keep in mind that PDF files are mostly static. This allows you to disregard many WCAG and EN 301 549 rules designed for interactive content and multimedia. However, the lack of interactivity also makes it more difficult for users to customize a document's layout to their needs.

For example, [WCAG Success Criterion 1.4.12][wcag-sg-1412-us] (codified in Clause 10.1.4.12 of EN 301 549) prescribes that a user must be able to increase character, letter, line, and paragraph spacing to very wide values. This benefits users with reduced vision or dyslexia. The Success Criterion does not require you to design your document with these layout parameters. Instead, it only requires a mechanism through which users can increase these parameters when reading the document. For HTML files, it is easy to comply with this Success Criterion because the browser lets the user override these spacing parameters on a page. For PDF, the situation is more nuanced: Theoretically, Typst adds tags and attributes designed for reflow to a file. A PDF reader, when reflowing, could allow its user to increase spacings beyond what is codified in these tags. In practice, we are not aware of a PDF viewer with this feature. Instead, this Success Criterion can be satisfied by repurposing the PDF into a HTML file and opening it in a browser.

In practice, even if your file is technically compliant, you cannot expect your users to know about these workarounds. Therefore, if you are aiming to meet the highest standards of Universal Access, consider distributing an HTML version of your document alongside your PDF. Export this file directly using Typst's [HTML export]($html) (in preview). Even though HTML export will not conserve many aspects of your visual layout, it will produce a file that leverages semantic HTML and technologies like [Digital Publishing ARIA][dpub-aria] to provide Universal Access. It will be of a higher quality than a PDF file repurposed to HTML.

Finally, keep in mind that PDFs are designed for print. Hence, you should not assume that interactive features like links are available to users who chose to print your document.

As mentioned above, files created by PNG and SVG export are not accessible.

[^1]: Dataset from the German Federal Statistics Authority (Statistisches Bundesamt, Destatis). ["Bruttostromerzeugung nach Energieträgern in Deutschland ab 1990"](https://www.destatis.de/DE/Themen/Branchen-Unternehmen/Energie/Erzeugung/bar-chart-race.html), 2025, available under the _Data licence Germany – attribution – version 2.0._

[^2]: For example, when using footnotes, the check "Lbl and LBody must be children of LI" in the "List" section is expected to fail.

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
