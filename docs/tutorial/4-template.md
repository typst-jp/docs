---
description: Typstチュートリアル
---

# テンプレートを作成する

このチュートリアルの前回の3つの章では、Typstで文書を書く方法、基本的なスタイルを適用する方法、そして出版社のスタイルガイドに準拠するために外観を詳細にカスタマイズする方法を学びました。前章で作成した論文が大成功を収めたため、同じ会議のための続報論文を書くよう依頼されました。今回は、前章で作成したスタイルを再利用可能なテンプレートに変換したいと思います。この章では、あなたとあなたのチームが単一のshowルールで使用できるテンプレートの作成方法を学びます。始めましょう！

## 変数によるデータの再利用 { #variables }

これまでの章では、文書のコンテンツの大部分を手入力していました。
第3章では、`document`要素とコンテキストを使うことで重複を減らし、タイトルを1回だけ入力する方法を学びました。
しかし実際には、文書中に複数回現れるものは他にも多くあります。
こうした繰り返し現れる値を1度だけ定義することには、以下のような複数の利点があります。

1. 後からの変更が容易になる
2. 何かを使用した全ての箇所をすぐに見つけられる
3. 文書全体の一貫性を保ちやすい
4. 長いものや入力が難しい繰り返し要素に対して、短い変数名の方が入力しやすいことが多い

従来のワードプロセッサーを使っているなら、後で検索できるようなプレースホルダー値を使う手もあるでしょう。
しかしTypstでは、代わりに変数を使ってコンテンツを安全に保管し、変数名を通じて文書全体で再利用できます。

これまでに学んだ、コンテキストを使って要素のプロパティを再現するテクニックは、必ずしもこの用途には適していません。
Typstの組み込み要素は、文書のタイトルや説明のような意味的なプロパティ、あるいはテキストサイズのように直接組版にかかわるものに焦点を当てているためです。

例として、Typstの発音記号を見てみましょう。
発音を表記する最良の方法の1つは国際音声記号（IPA）です。
しかしIPAには一般的なキーボードに存在しない文字が含まれているため、繰り返し入力するのは煩雑になります。
そこで、複数回参照できる変数を定義してみましょう。

```typ
#let ipa = [taɪpst]
```

ここで新しいキーワード`{let}`を使って、変数定義であることを示しています。
続けて変数名（この場合は`ipa`）を書きます。
最後に、等号と変数の値を書きます。
値はコンテンツであるため角括弧で囲まれており、これはコンテンツを受け取る関数を呼び出すときと同じ書き方を反映しています。
言い換えると、この構文は_「変数`ipa`に値`{[taɪpst]}`を持たせる」_というフレーズを表現しています。

これで文書中で変数を使用できます。

```example
#let ipa = [taɪpst]

The canonical way to
pronounce Typst is #ipa.

#table(
  columns: (1fr, 1fr),
  [Name], [Typst],
  [Pronunciation], ipa,
)
```

この例では、変数がマークアップ中（`#`を前に付ける）と関数呼び出し中（変数名をそのまま記述する）の両方で使用できることがわかります。
もちろん、変数の値を変更すれば、全ての出現箇所も自動的に変わります。
IPAと通常の散文をより明確に区別するため、IPAを斜体で表示してみましょう。
慣習的にIPAを囲むのに使われるスラッシュも追加します。

```example
#let ipa = text(
  style: "italic",
<<< )[/taɪpst/]
>>> box[/taɪpst/])

The canonical way to
pronounce Typst is #ipa.

#table(
  columns: (1fr, 1fr),
  [Name], [Typst],
  [Pronunciation], ipa,
)
```

ここでは、text関数を呼び出してその_戻り値_を変数に代入しています。
関数を呼び出すと、引数を処理して別の値（多くの場合コンテンツ）を返します。
このチュートリアルではこれまで、`[#text(fill: red)[CRIMSON!]]`のように、ほとんどの関数をマークアップ中で直接呼び出してきました。
このtext関数の呼び出しは、戻り値として赤色のテキストを返します。
これをマークアップ中に置いたため、戻り値はそのまま私たちが書いたコンテンツに即座に挿入されていました。
変数を使うと、代わりに後で使用したり、他の値と組み合わせたりするために値を保存できます。

変数はコンテンツの保存に限られません。Typstが知っている任意のデータ型を保存できます。
このチュートリアル全体を通して、Typstの組み込み関数に渡す際にさまざまなデータ型を利用してきました。
以下は、それぞれの型を変数に代入する例です。

```typ
// Content with markup inside
#let blind-text = [_Lorem ipsum_ dolor sit amet]

// Unformatted strings
#let funny-font = "MS Comic Sans"

// Absolute lengths (see also pt, in, ...)
#let mile = 160934cm

// Lengths relative to the font size
#let double-space = 2em

// Ratios
#let progress = 80%

// Integer numbers
#let answer = 42

// Booleans
#let truth = false

// Horizontal and vertical alignment
#let focus = center
```

このチュートリアルの本章では、変数と独自の関数を活用して、複数の文書で再利用できるテンプレートを構築します。

## 簡易テンプレート { #toy-template }

Typstでは、テンプレートは文書全体をラップできる関数です。その方法を学ぶために、まずは独自の関数の書き方を復習しましょう。関数は何でもできるので、少し奇抜なものを作ってみませんか？

```example
#let amazed(term) = box[✨ #term ✨]

You are #amazed[beautiful]!
```

前のセクションと比較すると、これが`{let}`を使った変数定義によく似ていることに気付くかもしれません。
この直感は正しく、関数は単なるもう1つのデータ型です。
ここでは変数`amazed`を定義し、`term`という単一の引数を取り、`term`の両側にきらめきを付けたコンテンツを返す関数を割り当てています。
さらに、感心している語が改行できらめきから分離されないように、全体を[`box`]($box)で囲んでいます。
この特別な関数定義の構文は、定義をより短く読みやすくしますが、通常の変数定義の構文を使うこともできます（詳細は[スクリプト記述]($scripting/#bindings)を参照）。
定義後は、組み込み関数と同じように関数を呼び出せます。

Typstの組み込み関数の多くにはオプションの名前付き引数があります。
私たちの関数にもそれを持たせられます。
テキストの色を選べるパラメーターを関数に追加してみましょう。
パラメーターが与えられなかった場合に備えて、デフォルトの色を指定する必要があります。

```example
#let amazed(term, color: blue) = {
  text(color, box[✨ #term ✨])
}

You are #amazed[beautiful]!
I am #amazed(color: purple)[amazed]!
```

テンプレートは`amazed`のようなカスタム関数で文書全体をラップすることで機能します。しかし、文書全体を巨大な関数呼び出しでラップするのは面倒でしょう！代わりに、"everything" showルールを使用して、より洗練されたコードで同じことが実現できます。そのようなshowルールを書くには、showキーワードの直後にコロンを置き、関数を提供します。この関数には文書の残りの部分がパラメーターとして渡されます。関数はこのコンテンツに対して任意の処理が可能です。`amazed`関数は単一のコンテンツ引数で呼び出せるので、showルールに名前で渡すだけで良いのです。試してみましょう。

```example
>>> #let amazed(term, color: blue) = {
>>>   text(color, box[✨ #term ✨])
>>> }
#show: amazed
I choose to focus on the good
in my life and let go of any
negative thoughts or beliefs.
In fact, I am amazing!
```

これで文書全体が`amazed`関数に渡され、文書をその関数でラップしたかのように機能します。もちろん、この特定の関数ではあまり有用ではありませんが、setルールと名前付き引数と組み合わせると、非常に強力になります。

## setルールとshowルールの埋め込み { #set-and-show-rules }

テンプレートにいくつかのsetルールとshowルールを適用するには、関数内のコンテンツブロックで`set`と`show`を使用し、そのコンテンツブロックに文書を挿入します。

```example
#let template(doc) = [
  #set text(font: "Inria Serif")
  #show "something cool": [Typst]
  #doc
]

#show: template
I am learning something cool today.
It's going great so far!
```

前章で発見したように、setルールはそのコンテンツブロック内の全てに適用されます。everything showルールが文書全体を`template`関数に渡すため、テンプレート内のテキストのsetルールと文字列のshowルールが文書全体に適用されます。この知識を使って、前章で作成した論文の本文スタイルを再現するテンプレートを作成しましょう。

```example
#let conf(title, doc) = {
  set page(
    paper: "us-letter",
>>> margin: auto,
    header: align(
      right + horizon,
      title
    ),
>>> numbering: "1",
    columns: 2,
<<<     ...
  )
  set par(justify: true)
  set text(
    font: "Libertinus Serif",
    size: 11pt,
  )

  // Heading show rules.
<<<   ...
>>> show heading.where(level: 1): set align(center)
>>> show heading.where(level: 1): set text(size: 13pt, weight: "regular")
>>> show heading.where(level: 1): smallcaps
>>>
>>> show heading.where(level: 2): set text(
>>>   size: 11pt,
>>>   weight: "regular",
>>>   style: "italic",
>>> )
>>> show heading.where(
>>>   level: 2
>>> ): it => {
>>>   it.body + [.]
>>> }

  doc
}

#show: doc => conf(
  [Paper title],
  doc,
)

= Introduction
<<< ...
>>> #lorem(90)
>>>
>>> == Motivation
>>> #lorem(140)
>>>
>>> == Problem Statement
>>> #lorem(50)
>>>
>>> = Related Work
>>> #lorem(200)
```

コードの大部分は前章からコピーペーストしました。2つの違いがあります。

1. everything showルールを使用して、全てを`conf`関数でラップしました。この関数はいくつかのsetルールとshowルールを適用し、最後に渡されたコンテンツをそのまま出力します。

2. さらに、コンテンツブロックの代わりに中括弧で囲まれたコードブロックを使用しました。この方法では、全てのsetルールや関数呼び出しの前に`#`を付ける必要がなくなります。代わりに、コードブロック内に直接マークアップを書けなくなります。

また、タイトルがどこから来ているかに注目してください。以前は変数に格納しましたが、今はテンプレート関数の最初のパラメーターとして受け取っています。そのために、everything showルールにクロージャー（その場で使用される名前のない関数）を渡しました。`conf`関数は2つの引数（タイトルと本文）を期待しますが、showルールは本文のみを渡すからです。したがって、論文のタイトルを設定し、showルールからの単一パラメーターを使用できる新しい関数定義を追加します。

## 名前付き引数を持つテンプレート { #named-arguments }

前章の論文にはタイトルと著者リストがありました。これらの要素をテンプレートに追加しましょう。タイトルに加えて、所属機関を含む著者リストと論文の要約をテンプレートに受け付けるようにします。可読性を保つために、これらを名前付き引数として追加します。最終的には、次のように機能させたいと思います。

```typ
#show: doc => conf(
  title: [
    A Fluid Dynamic Model for
    Glacier Flow
  ],
  authors: (
    (
      name: "Theresa Tungsten",
      affiliation: "Artos Institute",
      email: "tung@artos.edu",
    ),
    (
      name: "Eugene Deklan",
      affiliation: "Honduras State",
      email: "e.deklan@hstate.hn",
    ),
  ),
  abstract: lorem(80),
  doc,
)

...
```

この新しいテンプレート関数を構築しましょう。まず、`title`引数にデフォルト値を追加します。これにより、タイトルを指定せずにテンプレートを呼び出せます。また、空のデフォルト値を持つ名前付き引数として`authors`および`abstract`パラメーターを追加します。次に、前章からタイトル、要約、著者を生成するコードをテンプレートにコピーし、固定の詳細をパラメーターに置き換えます。

新しい`authors`パラメーターは、`name`、`affiliation`、`email`というキーを持つ[辞書]($dictionary)の[配列]($array)を想定しています。任意の数の著者を持てるため、著者リストに1列、2列、または3列が必要かどうかを動的に決定します。まず、`authors`配列の[`.len()`]($array.len)メソッドを使用して著者の数を決定します。次に、列数を著者数と3の最小値に設定し、3列以上作成しないようにします。4人以上の著者がいる場合は、代わりに新しい行が挿入されます。この目的のために、`grid`関数に`row-gutter`パラメーターも追加しました。そうしないと、行同士が近すぎてしまいます。辞書から著者の詳細を抽出するには、[フィールドアクセス構文]($scripting/#fields)を使用します。

各著者についてグリッドに引数を提供する必要があります。ここで配列の[`map`メソッド]($array.map)が便利です。これは引数として関数を取り、その関数が配列の各アイテムで呼び出されます。各著者の詳細をフォーマットし、コンテンツ値を含む新しい配列を返す関数を渡します。これで、グリッドの複数の引数として使用したい値の配列ができました。[`spread`演算子]($arguments)を使用してこれを実現できます。これは配列を取り、その各アイテムを関数の個別の引数として適用します。

結果のテンプレート関数は次のようになります。

```typ
#let conf(
  title: none,
  authors: (),
  abstract: [],
  doc,
) = {
  // Set and show rules from before.
>>> // (skipped)
<<<   ...

  place(
    top + center,
    float: true,
    scope: "parent",
    clearance: 2em,
    {
      title()

      let count = authors.len()
      let ncols = calc.min(count, 3)
      grid(
        columns: (1fr,) * ncols,
        row-gutter: 24pt,
        ..authors.map(author => [
          #author.name \
          #author.affiliation \
          #link("mailto:" + author.email)
        ]),
      )

      par(justify: false)[
        *Abstract* \
        #abstract
      ]

    }
  )

  doc
}
```

## 別ファイル { #separate-file }

多くの場合、テンプレートは別のファイルで指定され、それから文書にインポートされます。この方法では、メインファイルはすっきりとし、テンプレートを簡単に再利用できます。ファイルパネルでプラスボタンをクリックして新しいテキストファイルを作成し、`conf.typ`という名前を付けます。`conf`関数定義をその新しいファイルに移動します。これで、showルールの前にインポートを追加することで、メインファイルからアクセスできます。`{import}`キーワードとコロンの間にファイルのパスを指定し、インポートしたい関数に名前を付けます。

テンプレートの適用をより洗練させるためにできるもう1つのことは、関数の[`.with`]($function.with)メソッドを使用して、全ての名前付き引数を事前に設定することです。これにより、クロージャーを記述してテンプレートリストの最後にコンテンツ引数を追加する必要がなくなります。[Typst Universe]($universe)のテンプレートは、この関数呼び出しのスタイルで動作するように設計されています。

```example:single
>>> #let conf(
>>>   authors: (),
>>>   abstract: [],
>>>   doc,
>>> ) = {
>>>   set page(
>>>     "us-letter",
>>>     margin: auto,
>>>     header: align(
>>>       right + horizon,
>>>       context document.title,
>>>     ),
>>>     numbering: "1",
>>>     columns: 2,
>>>   )
>>>   set par(justify: true)
>>>   set text(font: "Libertinus Serif", 11pt)
>>>   show title: set text(size: 17pt)
>>>   show title: set align(center)
>>>   show title: set block(below: 1.2em)
>>>
>>>   show heading.where(level: 1): set align(center)
>>>   show heading.where(level: 1): set text(size: 13pt, weight: "regular")
>>>   show heading.where(level: 1): smallcaps
>>>
>>>   show heading.where(level: 2): set text(
>>>     size: 11pt,
>>>     weight: "regular",
>>>     style: "italic",
>>>   )
>>>   show heading.where(
>>>     level: 2
>>>   ): it => {
>>>     it.body + [.]
>>>   }
>>>
>>>   show heading.where(
>>>     level: 2
>>>   ): it => text(
>>>     size: 11pt,
>>>     weight: "regular",
>>>     style: "italic",
>>>     it.body + [.],
>>>   )
>>>
>>>   place(
>>>     top + center,
>>>     float: true,
>>>     scope: "parent",
>>>     clearance: 2em,
>>>     {
>>>       title()
>>>
>>>       let count = authors.len()
>>>       let ncols = calc.min(count, 3)
>>>       grid(
>>>         columns: (1fr,) * ncols,
>>>         row-gutter: 24pt,
>>>         ..authors.map(author => [
>>>           #author.name \
>>>           #author.affiliation \
>>>           #link("mailto:" + author.email)
>>>         ]),
>>>       )
>>>
>>>       par(justify: false)[
>>>         *Abstract* \
>>>         #abstract
>>>       ]
>>>     }
>>>   )
>>>
>>>   doc
>>> }
<<< #import "conf.typ": conf

#set document(title: [
  A Fluid Dynamic Model for
  Glacier Flow
])

#show: conf.with(
  authors: (
    (
      name: "Theresa Tungsten",
      affiliation: "Artos Institute",
      email: "tung@artos.edu",
    ),
    (
      name: "Eugene Deklan",
      affiliation: "Honduras State",
      email: "e.deklan@hstate.hn",
    ),
  ),
  abstract: lorem(80),
)

= Introduction
#lorem(90)

== Motivation
#lorem(140)

== Problem Statement
#lorem(50)

= Related Work
#lorem(200)
```

これで会議論文を、その会議用の再利用可能なテンプレートに変換しました！[フォーラム](https://forum.typst.app/)や[TypstのDiscordサーバー](https://discord.gg/2uDybryKPe)で共有して、他の人も使えるようにしてみてはいかがでしょうか？

## まとめ { #review }

おめでとうございます！Typstのチュートリアルを完了しました。このセクションでは、独自の関数を定義する方法と、再利用可能な文書スタイルを定義するテンプレートを作成・適用する方法を学びました。あなたは多くを学び、ここまで来ました。これでTypstを使用して独自の文書を作成し、他の人と共有できます。

私たちはまだ非常に若いプロジェクトであり、フィードバックを求めています。質問、提案、またはバグを発見した場合は、[フォーラム](https://forum.typst.app/)、[Discordサーバー](https://discord.gg/2uDybryKPe)、[GitHub](https://github.com/typst/typst/)、またはウェブアプリのフィードバックフォーム（ヘルプメニューからいつでも利用可能）でお知らせください。

さっそく[サインアップ](https://typst.app)して何か書いてみましょう！
