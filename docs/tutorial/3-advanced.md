---
description: Typstチュートリアル
---

# 高度なスタイリング
このチュートリアルの前の2つの章では、Typstで文書を書く方法とその書式を変更する方法を学びました。
それらの章を通して書いたレポートが優れた評価を得たため、指導教員はそれをもとに学会論文を書いてほしいと考えています！
もちろん、論文は学会のスタイルガイドに従わなければなりません。
どうすればそれを達成できるか見てみましょう。

始める前に、チームを作成して、そのチームに教員を招待して追加しましょう。
まず、エディターの左上にある戻るアイコンでアプリのダッシュボードに戻ります。
次に、左のツールバーのプラスアイコンを選択し、チームを作成します。
最後に、新しいチームをクリックし、チーム名の横にあるmanage teamをクリックして設定に進みます。
これで教員をメールで招待することができます。

![The team settings](3-advanced-team-settings.png)

次に、プロジェクトをチームに移動します。
プロジェクトを開き、左のツールバーの歯車アイコンを選んで設定に行き、Ownerのドロップダウンから新しいチームを選択します。
変更を保存するのを忘れないでください！

あなたの教員もプロジェクトを編集することができ、お互いにリアルタイムで変更を確認できます。
公式の[Discordサーバー](https://discord.gg/2uDybryKPe)に参加して他のユーザーを見つけ、一緒にチームを組んでみることも可能です！

## 学会ガイドライン { #guidelines }
レイアウトのガイドラインは学会ウェブサイトに掲載されております。
ここでは以下の条件であった場合を考えましょう。

- フォントは11ptのセリフ体
- タイトルは17ptで太字
- アブストラクトは1段組みで本文は2段組み
- アブストラクトは中央揃え
- 本文は両端揃え
- 第1レベルのセクションの見出しは13ptで中央に配置し、小さな大文字で表示
- 第2レベルの見出しは斜体で、本文と同じ大きさ
- ページはUSレターサイズとし、下中央にページ番号を付け、各ページの右上に論文のタイトルを記載

これらのうち、多くの項目については既に対応方法を知っていますが、いくつかについては新しい記法を学ぶ必要があります。

## setルール { #set-rules }
まず、文書のsetルールを書くことから始めましょう。

```example
#set page(
>>> margin: auto,
  paper: "us-letter",
  header: align(right)[
    A Fluid Dynamic Model for
    Glacier Flow
  ],
  numbering: "1",
)
#set par(justify: true)
#set text(
  font: "Libertinus Serif",
  size: 11pt,
)

#lorem(600)
```

ここで行われていることの大半は、すでに分かりでしょう。
テキストサイズを`{11pt}`に、フォントをLibertinus Serifに設定しています。
また、段落の両端揃えを有効にし、ページサイズをUSレターとしています。

ここで、`header`は新しい引数で、各ページの上部の余白に置くコンテンツを設定できます。
ヘッダーには、学会のスタイルガイドで要求されているように、論文のタイトルを指定します。
`align`関数を用いてそのテキストを右寄せにします。

最後に `numbering` 引数について説明します。
ここでは、ページ番号の付け方を定義する[numbering pattern]($numbering)を指定できます。
例えば`{"1"}`と設定すると、Typstは素のページ番号のみを表示します。
また`{"(1/1)"}`と設定すると、カッコで囲まれた現在のページと総ページ数が表示されるでしょう。
さらに、カスタム関数を用意して完全に好みの書式にすることも可能です。

## タイトルとアブストラクトの作成 { #title-and-abstract }
それでは、タイトルとアブストラクトを追加しましょう。
Typstには[`title`]($title)関数があります。この関数にタイトルを引数として渡してみましょう。

```example
>>> #set page(width: 300pt, margin: 30pt)
>>> #set text(font: "Libertinus Serif", 11pt)
#title[
  A Fluid Dynamic Model
  for Glacier Flow
]
```

タイトルはすでに太字で、周囲には余白もあります。
ただし、左揃えで、文字サイズも17ptではありません。
そこで、見た目を調整しましょう。
`title`関数には、フォントや文字サイズを設定する引数はありません。
代わりに、これらのプロパティは`text`関数と`align`関数で設定します。

<div class="info-box">

`title`関数で挿入したタイトルと、等号で作成した見出しは何が違うのでしょうか？

第1レベルの見出しを含め、見出しは文書内に複数回出現できます。
一方、タイトルは通常、文書の冒頭に一度だけ出現します。
両者を区別しておくと、スクリーンリーダーなどの支援技術を利用する読者にとってアクセシブルな文書を、Typstが生成しやすくなります。
</div>

ある要素の中にある別の種類の要素のプロパティをカスタマイズするには、show-setルールを使用できます。
まず、`show`に続けてカスタマイズする要素を指定します。
この指定を_セレクター_と呼びます。
次にコロンを入力し、セレクターにマッチする要素に適用するsetルールを記述します。
まとめると、構文は次のようになります。

```typ
#show your-selector: set some-element(/* ... */)
```

ここで確認しましょう。タイトルを中央揃えにし、文字サイズを17ptにしたいのでした。
そのため、次の2つのshow-setルールが必要です。

- セレクターが`title`、ルールが`{set text(size: 17pt)}`のshow-setルール
- セレクターが`title`、ルールが`{set align(center)}`のshow-setルール

例は次のようになります。

```example
>>> #set page(width: 300pt, margin: 30pt)
>>> #set text(font: "Libertinus Serif", 11pt)
#show title: set text(size: 17pt)
#show title: set align(center)

#title[
  A Fluid Dynamic Model
  for Glacier Flow
]
```

これでよさそうです。著者一覧も追加しましょう。
この論文は指導教員と共同で執筆しているので、自分の名前と指導教員の名前を追加します。

```example
>>> #set page(width: 300pt, margin: 30pt)
>>> #set text(font: "Libertinus Serif", 11pt)
>>>
>>> #show title: set text(size: 17pt)
>>> #show title: set align(center)
>>>
>>> #title[
>>>   A Fluid Dynamic Model
>>>   for Glacier Flow
>>> ]

#grid(
  columns: (1fr, 1fr),
  align(center)[
    Therese Tungsten \
    Artos Institute \
    #link("mailto:tung@artos.edu")
  ],
  align(center)[
    Dr. John Doe \
    Artos Institute \
    #link("mailto:doe@artos.edu")
  ]
)
```

著者情報が記載された2つのブロックが隣り合わせにレイアウトされています。
このレイアウトを作るために[`grid`]($grid)関数を使っています。
これにより、各列の大きさや、どのコンテンツをどのセルに入れるかを正確に制御することができます。
`columns`引数には、[相対長さ]($relative)または[割合]($fraction)の配列を渡します。
この場合、2つの等しい割合のサイズを渡し、使用可能なスペースを2つの等しい列に分割するように指示します。
次に、grid関数に2つのコンテンツ引数を渡しました。
ひとつは主著者であるあなたの情報で、もうひとつは指導教員の情報です。
ここでも `align` 関数を使用して、コンテンツを列の中央に配置しています。
grid関数はセルを指定するコンテンツ引数を任意の数で受け取れます。
行は自動的に追加されますが、`rows`引数で手動でサイズを指定することも可能です。

タイトルと著者一覧を見ると、少し近すぎます。
この問題は、別のshow-setルールを使ってタイトルの下の間隔を設定することで解決できます。
タイトルやグリッド、段落など、Typstがページの上から下へ配置する全ての要素を_ブロック_と呼びます。
各ブロックは[`block`]($block)関数によって制御されます。
この関数は、ブロック同士の間隔や、ブロック内で改ページできるかどうかなどを制御します。
つまり、タイトルを選択してブロックの間隔を設定するshow-setルールをさらに記述できます。

```example
>>> #set page(width: 300pt, margin: 30pt)
>>> #set text(font: "Libertinus Serif", 11pt)
>>>
#show title: set text(size: 17pt)
#show title: set align(center)
#show title: set block(below: 1.2em)

#title[
  A Fluid Dynamic Model
  for Glacier Flow
]

#grid(
<<<   // ...
>>>   columns: (1fr, 1fr),
>>>   align(center)[
>>>     Therese Tungsten \
>>>     Artos Institute \
>>>     #link("mailto:tung@artos.edu")
>>>   ],
>>>   align(center)[
>>>     Dr. John Doe \
>>>     Artos Institute \
>>>     #link("mailto:doe@artos.edu")
>>>   ]
)
```

このshow-setルールで、タイトルの下の間隔を上書きしました。
ここでは`em`単位を使用しています。
`em`を使うと、フォントサイズの倍数で長さを表現できます。
この例では、タイトルと著者一覧の間隔をフォントサイズのちょうど1.2倍に設定しました。

それでは、アブストラクトを追加しましょう。
学会では、アブストラクトを両端揃えにせず、中央に配置することが求められている点を思い出してください。

```example:0,0,612,317.5
>>> #set page(
>>>   "us-letter",
>>>   margin: auto,
>>>   header: align(right + horizon)[
>>>     A Fluid Dynamic Model for
>>>     Glacier Flow
>>>   ],
>>>   numbering: "1",
>>> )
>>> #set par(justify: true)
>>> #set text(font: "Libertinus Serif", 11pt)
>>>
>>> #show title: set text(size: 17pt)
>>> #show title: set align(center)
>>> #show title: set block(below: 1.2em)
>>>
>>> #title[
>>>   A Fluid Dynamic Model
>>>   for Glacier Flow
>>> ]
>>>
>>> #grid(
>>>   columns: (1fr, 1fr),
>>>   align(center)[
>>>     Therese Tungsten \
>>>     Artos Institute \
>>>     #link("mailto:tung@artos.edu")
>>>   ],
>>>   align(center)[
>>>     Dr. John Doe \
>>>     Artos Institute \
>>>     #link("mailto:doe@artos.edu")
>>>   ]
>>> )
>>>
<<< ...

#align(center)[
  #set par(justify: false)
  *Abstract* \
  #lorem(80)
]
>>> #lorem(600)
```
できました！特筆すべき点は、`align`のコンテンツ引数の中にあるsetルールを使って、アブストラクトの両端揃えをオフにしたことです。
これは、最初のsetルールの後に指定されたにもかかわらず、文書の残りの部分には影響しません。
コンテンツ・ブロック内で設定されたものは、そのブロック内のコンテンツにのみ影響します。

さらに、ヘッダーと`title`要素の引数にタイトルを重複して記述している点も改善できます。
両方で同じタイトルを使うため、文書メタデータを保持するために用意された場所にタイトルを保存できると便利です。
その場合、両方の場所からタイトルを取得する方法も必要になります。
タイトルの保存には`document`要素が役立ちます。
`document`要素をsetルールで使用すると、タイトル、説明、キーワードなどの文書メタデータを保存できます。

```typ
#set document(title: [A Fluid Dynamic Model for Glacier Flow])
```

ここで設定したタイトルは、PDFをエクスポートしたときにPDFリーダーのタイトルバーへ表示されます。
また、オペレーティングシステムの検索でも、このタイトルを使ってファイルを見つけられます。
さらに、タイトルの設定は文書のアクセシビリティ向上にも役立ち、アクセシビリティを重視したPDF規格であるPDF/UAに準拠する場合には必須です。

次に、設定した値をページ上のタイトルとヘッダーの両方で取得する方法が必要です。
`title`関数は`document`要素と連携するように設計されているため、引数なしで呼び出すとタイトルが表示されます。
一方、ヘッダーでは、より明示的に指定する必要があります。
ヘッダーにタイトルを挿入したいという意図をTypstは判断できないため、手動で指示する必要があります。

_コンテキスト_を使うと、これまでに要素へ設定した任意の値を取得できます。
`{context}`キーワードを使用すれば、`document`要素の`title`プロパティを含め、どの要素のどのプロパティにもアクセスできます。
次のように使用します。

```example:single
#set document(title: [
  A Fluid Dynamic Model
  for Glacier Flow
])

<<< ...

#set page(
>>> "us-letter",
>>> margin: auto,
  header: align(
    right + horizon,
    // Retrieve the document
    // element's title property.
    context document.title,
  ),
<<<   ...
>>> numbering: "1",
)
>>> #set par(justify: true)
>>> #set text(font: "Libertinus Serif", 11pt)

>>> #show title: set text(size: 17pt)
>>>
>>> #show title: set align(center)
>>> #show title: set block(below: 1.2em)
#title()

<<< ...

>>> #grid(
>>>   columns: (1fr, 1fr),
>>>   align(center)[
>>>     Therese Tungsten \
>>>     Artos Institute \
>>>     #link("mailto:tung@artos.edu")
>>>   ],
>>>   align(center)[
>>>     Dr. John Doe \
>>>     Artos Institute \
>>>     #link("mailto:doe@artos.edu")
>>>   ]
>>> )
>>>
>>> #align(center)[
>>>   #set par(justify: false)
>>>   *Abstract* \
>>>   #lorem(80)
>>> ]
>>>
>>> #lorem(600)
```

まず、空の丸括弧で`title`関数を呼び出している点に注目してください。
引数を渡していないため、上で`document`要素に設定した値がデフォルトで使用されます。
空の丸括弧と空の角括弧の違いは重要です。
空の丸括弧は何も渡していないことを示しますが、空の角括弧は1つの引数、つまり空のコンテンツブロックを渡していることを示します。
空の角括弧で呼び出すと、タイトルには何も表示されません。

次に、ヘッダーを見てみましょう。
角括弧内にタイトルを直接記述する代わりに、`context`キーワードを使って文書のタイトルにアクセスしています。
これにより、上で設定した内容がそのまま挿入されます。
コンテキストの役割は、プロパティへのアクセスだけではありません。
文書内に特定の要素が存在するかを調べたり、別の要素の物理的な寸法を測定したりできます。
コンテキストを使えば、利用者の設定に応じて変化する強力なテンプレートを構築できます。

<div class="info-box">

<details>
<summary>
要素のプロパティにアクセスするには、なぜ`context`キーワードが必要なのでしょうか？
</summary>

通常、変数にアクセスするときは、その値がどのような値になるか正確に分かっています。

- `[#sym.pi]`のように、変数がTypstの組み込み定数である場合
- 変数が引数によって定義されている場合
- 変数が現在のスコープで定義または上書きされている場合

しかし、それだけでは不十分な場合があります。
この章では、タイトルを含むページヘッダーを挿入しました。
ヘッダーには1つのコンテンツしか渡していませんが、ページごとに異なるヘッダーを表示したいこともあります。
例えば、章の名前を表示したり、ページ番号を使ったりしたい場合です。
コンテキストブロックを1つ記述しておくと、Typstはそれが挿入された場所を起点に、直前の見出しや現在のページ番号などを調べて処理します。
そのため、同じコンテキストブロックでも、挿入するページによって異なる出力を生成できます。

詳しくは、このチュートリアルを終えた後に[コンテキストのドキュメント]($context)を参照してください。
</details>
</div>

## 段組みと見出しの追加 { #columns-and-headings }
上の論文は、残念ながら文字が単調にぎっしり詰まっていて読みにくい見た目をしています。
これを修正するために、見出しを追加し、2段組のレイアウトに変更してみましょう。
幸いなことに、setルールで`page`に`column`引数を追加することで簡単に行えます。

引数リストに`{columns: 2}`を加えることで、文書全体を2段組みとなります。
しかし、これではタイトルと著者、アブストラクトにも影響が出てしまいます。
それらを1段組みのままに維持するためには、[`{place}`]($place)関数を呼び出して囲みましょう。
place関数は引数として配置とコンテンツを受け取ります。
オプション引数である`{scope}`引数を使えば、現在の段組みとその親（ページ）のどちらに対して配置するかを決めることが可能です。
これらに加えて、もうひとつ設定することがあります。
オプション引数がない場合、`{place}`はそのコンテンツを文書の流れから外し、他のレイアウトに影響を与えることなく、他のコンテンツの上に配置します。

```example
#place(
  top + center,
  rect(fill: black),
)
#lorem(30)
```

もしここで`{place}`を使わなければ、黒塗りの長方形は独立した行になるはずですが、
`{place}`を使うと、それに続く数行のテキストの上に重なります。
同様に、テキスト側もこの長方形がないかのように振る舞います。
この動作を変更するには、引数`{float: true}`を渡してください。
これにより`{place}`でページの上部または下部に配置されたアイテムが、他のコンテンツと重ならないように設定できます。

```example:single
>>> #set document(title: [
>>>   A Fluid Dynamic Model
>>>   for Glacier Flow
>>> ])
>>>
#set page(
>>> margin: auto,
  paper: "us-letter",
  header: align(
    right + horizon,
    context document.title,
  ),
  numbering: "1",
  columns: 2,
)
>>> #set par(justify: true)
>>> #set text(font: "Libertinus Serif", 11pt)

#place(
  top + center,
  float: true,
  scope: "parent",
  clearance: 2em,
)[
>>> #show title: set text(size: 17pt)
>>> #show title: set align(center)
>>> #show title: set block(below: 1.2em)
>>>
>>> #title()
>>>
>>> #grid(
>>>   columns: (1fr, 1fr),
>>>   [
>>>     Therese Tungsten \
>>>     Artos Institute \
>>>     #link("mailto:tung@artos.edu")
>>>   ],
>>>   [
>>>     Dr. John Doe \
>>>     Artos Institute \
>>>     #link("mailto:doe@artos.edu")
>>>   ]
>>> )
<<<   ...

  #par(justify: false)[
    *Abstract* \
    #lorem(80)
  ]
]

= Introduction
#lorem(300)

= Related Work
#lorem(200)
```

この例では、`{place}` 関数の `clearance` 引数も使用しています。
これにより、[`{v}`]($v)関数を使用する代わりに、本文との間にスペースを設けています。
また、コンテンツはcenter引数を継承しているため、各パーツごとに行っていた明示的な `{align(center, ...)}` 呼び出しも削除できます。

最後に見出しのスタイルの設定をしましょう。
ガイドラインに従うために、見出しは中央揃えにして、小さな大文字を使わなければなりません。
`heading`関数はそのような設定を提供していないため、独自の見出しshowルールを書く必要があります。

- 見出しを中央揃えにするshow-setルール
- 見出しを13ptにし、太さを標準にするshow-setルール
- 見出し全体を`smallcaps`関数で囲むshowルール

```example:50,250,265,270
>>> #set document(title: [
>>>   A Fluid Dynamic Model
>>>   for Glacier Flow
>>> ])
>>>
>>> #set page(
>>>   "us-letter",
>>>   margin: auto,
>>>   header: align(
>>>     right + horizon,
>>>     context document.title,
>>>   ),
>>>   numbering: "1",
>>>   columns: 2,
>>> )
>>> #set par(justify: true)
>>> #set text(font: "Libertinus Serif", 11pt)
#show heading: set align(center)
#show heading: set text(
  size: 13pt,
  weight: "regular",
)
#show heading: smallcaps

<<< ...
>>> #place(
>>>   top + center,
>>>   float: true,
>>>   scope: "parent",
>>>   clearance: 2em,
>>> )[
>>>   #show title: set text(size: 17pt)
>>>   #show title: set align(center)
>>>   #show title: set block(below: 1.2em)
>>>
>>>   #title()
>>>
>>>   #grid(
>>>     columns: (1fr, 1fr),
>>>     [
>>>       Therese Tungsten \
>>>       Artos Institute \
>>>       #link("mailto:tung@artos.edu")
>>>     ],
>>>     [
>>>       Dr. John Doe \
>>>       Artos Institute \
>>>       #link("mailto:doe@artos.edu")
>>>     ]
>>>   )
>>>
>>>   #par(justify: false)[
>>>     *Abstract* \
>>>     #lorem(80)
>>>   ]
>>> ]

= Introduction
<<< ...
>>> #lorem(35)

== Motivation
<<< ...
>>> #lorem(45)
```

これで見た目が整いました！
ここでは、全ての見出しに適用されるshowルールを使用しました。
最後のshowルールでは、見出し全体に`smallcaps`関数を適用しています。
次の例で示すように、独自のshowルールで見出しのデフォルトの見た目を完全に上書きすることもできます。

ただし、全ての見出しが同じように見えるという問題が残っています。
「Motivation」と「Problem Statement」のサブセクションは斜体の追い込み見出しにする必要がありますが、現状ではセクションの見出しと区別できません。
この問題は、`where`セレクターを使用すると解決できます。
`where`は、見出しなどの要素に対して呼び出せる[メソッド]($scripting/#methods)で、プロパティに基づいて要素を絞り込めます。
これにより、セクションとサブセクションの見出しを区別できます。

```example:50,250,265,245
>>> #set document(title: [
>>>   A Fluid Dynamic Model
>>>   for Glacier Flow
>>> ])
>>>
>>> #set page(
>>>   "us-letter",
>>>   margin: auto,
>>>   header: align(
>>>     right + horizon,
>>>     context document.title,
>>>   ),
>>>   numbering: "1",
>>>   columns: 2,
>>> )
>>> #set par(justify: true)
>>> #set text(font: "Libertinus Serif", 11pt)
>>>
#show heading.where(level: 1): set align(center)
#show heading.where(level: 1): set text(size: 13pt, weight: "regular")
#show heading.where(level: 1): smallcaps

#show heading.where(level: 2): set text(
  size: 11pt,
  weight: "regular",
  style: "italic",
)
#show heading.where(level: 2): it => {
  it.body + [.]
}
>>>
>>> #place(
>>>   top + center,
>>>   float: true,
>>>   scope: "parent",
>>>   clearance: 2em,
>>> )[
>>>   #show title: set text(size: 17pt)
>>>   #show title: set align(center)
>>>   #show title: set block(below: 1.2em)
>>>
>>>   #title()
>>>
>>>   #grid(
>>>     columns: (1fr, 1fr),
>>>     [
>>>       Therese Tungsten \
>>>       Artos Institute \
>>>       #link("mailto:tung@artos.edu")
>>>     ],
>>>     [
>>>       Dr. John Doe \
>>>       Artos Institute \
>>>       #link("mailto:doe@artos.edu")
>>>     ]
>>>   )
>>>
>>>   #par(justify: false)[
>>>     *Abstract* \
>>>     #lorem(80)
>>>   ]
>>> ]
>>>
>>> = Introduction
>>> #lorem(35)
>>>
>>> == Motivation
>>> #lorem(45)
```

この例では、まず`{.where(level: 1)}`でセレクターをより具体的にし、先ほどのルールの適用範囲を第1レベルの見出しに限定しています。
次に、第2レベルの見出しに適用するshow-setルールを追加します。
最後に、独自の関数を使ったshowルールが必要です。
デフォルトでは、見出しのコンテンツはブロックで囲まれます。
そのため、見出しは独立した行になります。
しかし、ここでは見出しを本文へ追い込みたいので、独自のshowルールを指定してこのブロックを取り除きます。

このルールには、見出しを引数として受け取る関数を指定します。
この引数は慣例として`it`と呼ばれますが、別の名前でも構いません。
この引数はコンテンツとして使用でき、その場合はデフォルトの見出し全体がそのまま表示されます。
一方、独自の見出しを作る場合は、`body`、`numbering`、`level`などのフィールドを使って見た目を組み立てられます。
ここでは、見出しの本文と末尾のピリオドだけを表示し、組み込みのshowルールが生成するブロックを省いています。
showルールで`it.numbering`を明示的に使用していないため、この見出しには見出しの番号付けなどに関するsetルールが反映されなくなることに注意してください。
このようなshowルールを記述し、文書を引き続きカスタマイズできるようにするには、これらのフィールドを考慮する必要があります。

これは素晴らしい！
第1レベルと第2レベルの見出しにそれぞれ選択的に適用される2つのshowルールを書きました。
`where`セレクターを使用して、見出しをレベルでフィルタリングしました。
そして、サブセクションの見出しを本文と改行せずにレンダリングしました。
また、サブセクションの見出しの最後にピリオドを自動的に追加してます。

ここで、学会のスタイルガイドを確認しましょう。

- フォントは11ptのセリフ体 ✓
- タイトルは17ptで太字 ✓
- アブストラクトは1段組みで本文は2段組み ✓
- アブストラクトは中央揃え ✓
- 本文は両端揃え ✓
- 第1レベルのセクションの見出しは13ptで中央に配置し、小さな大文字で表示 ✓
- 第2レベルの見出しは斜体で、本文と同じ大きさ ✓
- ページはUSレターサイズとし、下中央にページ番号を付け、各ページの右上に論文のタイトルを記載 ✓

これで、全てのスタイルに準拠し、論文を学会に提出できます！完成した論文は次のようになっています。
<img
  src="3-advanced-paper.png"
  alt="The finished paper"
  style="box-shadow: 0 4px 12px rgb(89 85 101 / 20%); width: 500px; max-width: 100%; display: block; margin: 24px auto;"
>

## まとめ
このセクションでは、ヘッダーとフッターの作成方法、関数とスコープを使用してローカルにスタイルをオーバーライドする方法、[`grid`]関数を使用してより複雑なレイアウトを作成する方法、個々の関数と文書全体のshowルールを記述する方法を学びました。
コンテキストを使用して要素のプロパティにアクセスする方法も学びました。
また、[`where`セレクター]($styling/#show-rules)を使用して、見出しをそのレベルによってフィルタリングする方法も学びました。

結果として論文は大成功でした！
あなたはその学会にて同じ志を持つ研究者にたくさん出会い、来年同じ学会で発表したいプロジェクトを計画しています。
その際に、同じスタイルガイドを使って新しい論文を書く必要があるため、あなたやあなたのチームのために、時間を節約できるテンプレートを作りたいと思うのではないでしょうか？

次のセクションでは、複数の文書で再利用できるテンプレートの作成方法を学びます。
これはより高度なトピックですので、今すぐには手が出せないという方は、後ほどお気軽にお越しください。
