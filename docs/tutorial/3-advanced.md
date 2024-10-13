---
description: Typst's tutorial.
---

# 高度なスタイリング
このチュートリアルの前の2つの章では、Typstで文書を書く方法とその書式を変更する方法を学びました。
それらの章を通して書いたレポートが優れた評価を得たため、指導教員はそれを基に学会論文を書いてほしいと考えています！
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
>>>  margin: auto,
  paper: "us-letter",
  header: align(right)[
    A fluid dynamic model for
    glacier flow
  ],
  numbering: "1",
)
#set par(justify: true)
#set text(
  font: "Linux Libertine",
  size: 11pt,
)

#lorem(600)
```

ここで行われていることの大半は、すでに分かりでしょう。
テキストサイズを`{11pt}`に、フォントをLinux Libertineに設定しています。
また、段落の両端揃えを有効にし、ページサイズをUSレターとしています。

ここで、`header`は新しい引数で、各ページの上部の余白に置くコンテンツを設定できます。
ヘッダーには、学会のスタイルガイドで要求されているように、論文のタイトルを指定します。
`align`関数を用いてそのテキストを右寄せにします。

最後に `numbering` 引数について説明します。
ここでは、ページ番号の付け方を定義する[numbering pattern]($numbering)を指定できます。
例えば`{"1"}`と設定すると、Typstは素のページ番号のみを表示します。
また`{"(1/1)"}`と設定すると、カッコで囲まれた現在のページと総ページ数が表示されるでしょう。
さらに、カスタム関数を用意して完全に好みの書式にすることも可能です。

## タイトルと要旨 { #title-and-abstract }
それでは、タイトルとアブストラクトを追加しましょう。
まずはタイトルを中央揃えにし、`[*stars*]`で囲んでフォントを太文字にします。

```example
>>> #set page(width: 300pt, margin: 30pt)
>>> #set text(font: "Linux Libertine", 11pt)
#align(center, text(17pt)[
  *A fluid dynamic model
  for glacier flow*
])
```

正しく動作していることが確認できます。
`text`関数を使って、前のテキストのsetルールをローカルで上書きし、関数の引数で文字サイズを17ptに大きくしました。
次に、著者リストも追加しましょう。
指導教員と一緒にこの論文を書いているため、自分の名前と教員の名前を追加します。

```example
>>> #set page(width: 300pt, margin: 30pt)
>>> #set text(font: "Linux Libertine", 11pt)
>>>
>>> #align(center, text(17pt)[
>>>   *A fluid dynamic model
>>>   for glacier flow*
>>> ])
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
このレイアウトを作るために[`grid`]関数を使っています。
これにより、各列の大きさや、どのコンテンツをどのセルに入れるかを正確に制御することができます。
`columns`引数には、[相対長さ]($relative)または[割合]($fraction)の配列を渡します。
この場合、2つの等しい割合のサイズを渡し、使用可能なスペースを2つの等しい列に分割するように指示します。
次に、grid関数に2つの内容引数を渡しました。
ひとつは主著者であるあなたの情報で、もうひとつは指導教員の情報です。
ここでも `align` 関数を使用して、コンテンツを列の中央に配置しています。
grid関数はセルを指定するcontent引数を任意の数で受け取れます。
行は自動的に追加されますが、`rows`引数で手動でサイズを指定することも可能です。

それでは、アブストラクトを追加しましょう。
学会は、アブストラクトを中央に配置することを望んでいることを忘れないでください。

```example:0,0,612,317.5
>>> #set text(font: "Linux Libertine", 11pt)
>>> #set par(justify: true)
>>> #set page(
>>>   "us-letter",
>>>   margin: auto,
>>>   header: align(right + horizon)[
>>>     A fluid dynamic model for
>>>     glacier flow
>>>   ],
>>>   numbering: "1",
>>> )
>>>
>>> #align(center, text(17pt)[
>>>   *A fluid dynamic model
>>>   for glacier flow*
>>> ])
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

できました！特筆すべき点は、`align`のcontent引数の中にあるsetルールを使って、アブストラクトの両端揃えをオフにしたことです。
これは、最初のsetルールの後に指定されたにもかかわらず、文書の残りの部分には影響しません。
コンテンツ・ブロック内で設定されたものは、そのブロック内のコンテンツにのみ影響します。

ヘッダーとタイトルの2回入力する必要がないように、論文タイトルを変数に保存することも可能です。
変数の宣言には`{let}`を使用します。

```example:single
#let title = [
  A fluid dynamic model
  for glacier flow
]

<<< ...

>>> #set text(font: "Linux Libertine", 11pt)
>>> #set par(justify: true)
#set page(
>>>   "us-letter",
>>>   margin: auto,
  header: align(
    right + horizon,
    title
  ),
<<<   ...
>>>   numbering: "1",
)

#align(center, text(17pt)[
  *#title*
])

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

`title`変数にコンテンツを設定した後は、関数内やマークアップ内（関数のように接頭辞に`#`をつける）で使用できます。
こうすることで、別のタイトルに決めた場合、一箇所で簡単に変更することができます。

## 段組みと見出し { #columns-and-headings }
上の論文は、残念ながら文字が単調にぎっしり詰まっていて読みにくい見た目をしています。
これを修正するために、見出しを追加し、2段組のレイアウトに変更してみましょう。
[`columns`]関数は数値と内容を受け取り、指定された列数に内容を段組みします。
アブストラクト以降はすべて2段組みにしたいため、文書全体にcolumn関数を適用する必要があります。

文書全体を巨大な関数呼び出しで囲う代わりに、「以降すべて」に対してshowルールを使うことができます。
このようなshowルールを書くには、showキーワードの直後にコロンを置き、それから関数を指定します。
この関数は、文書の残りの部分をパラメータとして与えます。
ここではパラメータを `rest` と宣言していますが、どのような名前を選んでもかまいません。
この関数は、このコンテンツに対して様々な処理を適用できます。
この例では、`columns`関数に渡します。

```example:single
>>> #let title = [
>>>   A fluid dynamic model
>>>   for glacier flow
>>> ]
>>>
>>> #set text(font: "Linux Libertine", 11pt)
>>> #set par(justify: true)
>>> #set page(
>>>   "us-letter",
>>>   margin: auto,
>>>   header: align(
>>>     right + horizon,
>>>     title
>>>   ),
>>>   numbering: "1",
>>> )
>>>
>>> #align(center, text(
>>>   17pt,
>>>   weight: "bold",
>>>   title,
>>> ))
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
>>> #align(center)[
>>>   #set par(justify: false)
>>>   *Abstract* \
>>>   #lorem(80)
>>> ]
>>> #v(4mm)
<<< ...

#show: rest => columns(2, rest)

= Introduction
#lorem(300)

= Related Work
#lorem(200)
```

最後に見出しのスタイルの設定をしましょう。
ガイドラインに従うために、見出しは中央揃えにして、小さな大文字を使わなければなりません。
`heading`関数はそのような設定を提供していないため、独自の見出しshowルールを書く必要があります。

```example:50,250,265,270
>>> #let title = [
>>>   A fluid dynamic model
>>>   for glacier flow
>>> ]
>>>
>>> #set text(font: "Linux Libertine", 11pt)
>>> #set par(justify: true)
>>> #set page(
>>>   "us-letter",
>>>   margin: auto,
>>>   header: align(
>>>     right + horizon,
>>>     title
>>>   ),
>>>   numbering: "1",
>>> )
#show heading: it => [
  #set align(center)
  #set text(12pt, weight: "regular")
  #block(smallcaps(it.body))
]

<<< ...
>>>
>>> #align(center, text(
>>>   17pt,
>>>   weight: "bold",
>>>   title,
>>> ))
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
>>> #align(center)[
>>>   #set par(justify: false)
>>>   *Abstract* \
>>>   #lorem(80)
>>> ]
>>>
>>> #v(4mm)
>>> #show: rest => columns(2, rest)
>>>
>>> = Introduction
>>> #lorem(35)
>>>
>>> == Motivation
>>> #lorem(45)
```

うまくできました！
すべての見出しに適用されるshowルールを使用しました。
この関数にパラメータとして見出しを渡します。
このパラメータはコンテンツとして使用することもできますが、`title`、`numbers`、`level`といったフィールドも持っているため、そこから独自の書式を構成することも可能です。
ここではセンター揃えにし、見出しはデフォルトで太字なのでフォントのウェイトを `{"regular"}` に設定し、[`smallcaps`] 関数を使って見出しのタイトルを小さな大文字でレンダリングしています。

残る唯一の問題は、すべての見出しが同じように見えることです。
MotivationとProblem Statementはサブセクションであり、イタリック体であるべきですが、今はセクションの見出しと見分けがつきません。
この問題は、setルールに`where`セレクターを使うことで解決できます。
これは、見出し（および他の要素）に対して呼び出せる[メソッド]($scripting/#methods)で、レベルごとにフィルタリングすることが可能です。
これによりセクションとサブセクションの見出しを区別できます。

```example:50,250,265,245
>>> #let title = [
>>>   A fluid dynamic model
>>>   for glacier flow
>>> ]
>>>
>>> #set text(font: "Linux Libertine", 11pt)
>>> #set par(justify: true)
>>> #set page(
>>>   "us-letter",
>>>   margin: auto,
>>>   header: align(
>>>     right + horizon,
>>>     title
>>>   ),
>>>   numbering: "1",
>>> )
>>>
#show heading.where(
  level: 1
): it => block(width: 100%)[
  #set align(center)
  #set text(12pt, weight: "regular")
  #smallcaps(it.body)
]

#show heading.where(
  level: 2
): it => text(
  size: 11pt,
  weight: "regular",
  style: "italic",
  it.body + [.],
)
>>>
>>> #align(center, text(
>>>   17pt,
>>>   weight: "bold",
>>>   title,
>>> ))
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
>>> #align(center)[
>>>   #set par(justify: false)
>>>   *Abstract* \
>>>   #lorem(80)
>>> ]
>>>
>>> #v(4mm)
>>> #show: rest => columns(2, rest)
>>>
>>> = Introduction
>>> #lorem(35)
>>>
>>> == Motivation
>>> #lorem(45)
```

これは素晴らしい！
第1レベルと第2レベルの見出しにそれぞれ選択的に適用される2つのshowルールを書きました。
`where`セレクタを使用して、見出しをレベルでフィルタリングしました。
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

これで、すべてのスタイルに準拠し、論文を学会に提出できます！完成した論文は次のようになっています。
<img
  src="3-advanced-paper.png"
  alt="The finished paper"
  style="box-shadow: 0 4px 12px rgb(89 85 101 / 20%); width: 500px; max-width: 100%; display: block; margin: 24px auto;"
>

## まとめ
このセクションでは、ヘッダーとフッターの作成方法、関数とスコープを使用してローカルにスタイルをオーバーライドする方法、[`grid`]関数を使用してより複雑なレイアウトを作成する方法、個々の関数と文書全体のshowルールを記述する方法を学びました。
また、[`where`セレクタ]($styling/#show-rules)を使用して、見出しをそのレベルによってフィルタリングする方法も学びました。

結果として論文は大成功でした！
あなたはその学会にて同じ志を持つ研究者にたくさん出会い、来年同じ学会で発表したいプロジェクトを計画しています。
その際に、同じスタイルガイドを使って新しい論文を書く必要があるため、あなたやあなたのチームのために、時間を節約できるテンプレートを作りたいと思うのではないでしょうか？

次のセクションでは、複数の文書で再利用できるテンプレートの作成方法を学びます。
これはより高度なトピックですので、今すぐには手が出せないという方は、後ほどお気軽にお越しください。
