---
description: Typstチュートリアル
---

# テンプレートを作成する
このチュートリアルの前回の3つの章では、Typstでドキュメントを書く方法、基本的なスタイルを適用する方法、そして出版社のスタイルガイドに準拠するために外観を詳細にカスタマイズする方法を学びました。前章で作成した論文が大成功を収めたため、同じ会議のための続報論文を書くよう依頼されました。今回は、前章で作成したスタイルを再利用可能なテンプレートに変換したいと思います。この章では、あなたとあなたのチームが単一のshowルールで使用できるテンプレートの作成方法を学びます。始めましょう！

## Reusing data with variables { #variables }
In the past chapters, most of the content of the document was entered by hand.
In the third chapter, we used the `document` element and context to cut down on
repetition and only enter the title once. But in practice, there may be many
more things that occur multiple times in your document. There are multiple good
reasons to just define these repeated values once:

1. It makes changing them later easier
2. It allows you to quickly find all instances where you used something
3. It makes it easy to be consistent throughout
4. For long or hard-to-enter repeated segments, a shorter variable name is often
   more convenient to type

If you were using a conventional word processor, you might resort to using a
placeholder value that you can later search for. In Typst, however, you can
instead use variables to safely store content and reuse it across your whole
document through a variable name.

The technique of using context to reproduce an element's property we have
learned earlier is not always the most appropriate for this: Typst's built-in
elements focus on semantic properties like the title and description of a
document, or things that directly relate to typesetting, like the text size.

For our example, we want to take a look at Typst's pronunciation. One of the
best ways to transcribe pronunciation is the International Phonetic Alphabet
(IPA). But because it uses characters not found on common keyboards, typing IPA
repeatedly can become cumbersome. So let's instead define a variable that we can
reference multiple times.

```typ
#let ipa = [taɪpst]
```

Here, we use a new keyword, `{let}`, to indicate a variable definition. Then,
we put the name of our variable, in this case, `ipa`. Finally, we type an equals
sign and the value of our variable. It is enclosed in square brackets because
it is content, mirroring how you would call a function accepting content. In
other words, this syntax mirrors the phrase _"Let the variable `ipa` have the
value `{[taɪpst]}`."_

Now, we can use the variable in our document:

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

In the example, you can see that the variable can be used both in markup
(prefixed with a `#`) and in a function call (by just typing its name). Of
course, we can change the value of the variable and all its occurrences will
automatically change with it. Let's make it a bit clearer what is IPA and what
is normal prose by rendering IPA in italics. We are also using slashes which, by
convention, often enclose IPA.

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

Here, we called the text function and assigned its _return value_ to the
variable. When you call a function, it processes its arguments and then yields
another value (often content). So far in this tutorial, we called most
functions directly in markup, like this: `[#text(fill: red)[CRIMSON!]]`. This
call to the text function returns the red text as a return value. Because we
placed it in markup, its return value just immediately got inserted into the
content we wrote. With variables, we can instead store it to use it later or
compose it with other values.

Variables are not limited to storing content: they can store any data type Typst
knows about. Throughout this tutorial, you made use of many data types when you
passed them to Typst's built-in functions. Here is an example assigning each of
them to a variable:

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

In this chapter of the tutorial, you will leverage variables and your own
functions to build templates that can be reused across multiple documents.

## 簡易テンプレート { #toy-template }
Typstでは、テンプレートは文書全体をラップできる関数です。その方法を学ぶために、まずは独自の関数の書き方を復習しましょう。関数は何でもできるので、少し奇抜なものを作ってみませんか？

```example
#let amazed(term) = box[✨ #term ✨]

You are #amazed[beautiful]!
```

Comparing this against the previous section, you may have noticed that this
looks a lot like a variable definition using `{let}`. This instinct is correct:
Functions are just another data type. Here, we are defining the variable
`amazed`, assigning it a function that takes a single argument, `term`, and
returns content with the `term` surrounded by sparkles. We also put the whole
thing in a [`box`]($box) so that the term we are amazed by cannot be separated from
its sparkles by a line break. The special function definition syntax makes the
definition shorter and more readable, but you can also use the regular variable
definition syntax (see [the scripting reference]($scripting/#bindings) for
details). After its definition, we are able to call the function just like all
built-in functions.

Many functions that come with Typst have optional named parameters. Our
functions can also have them. Let's add a parameter to our function that lets us
choose the color of the text. We need to provide a default color in case the
parameter isn't given.

```example
#let amazed(term, color: blue) = {
  text(color, box[✨ #term ✨])
}

You are #amazed[beautiful]!
I am #amazed(color: purple)[amazed]!
```

テンプレートは`amazed`のようなカスタム関数でドキュメント全体をラップすることで機能します。しかし、文書全体を巨大な関数呼び出しでラップするのは面倒でしょう！代わりに、「everything」showルールを使用して、より洗練されたコードで同じことを実現できます。そのようなshowルールを書くには、showキーワードの直後にコロンを置き、関数を提供します。この関数にはドキュメントの残りの部分がパラメータとして渡されます。関数はこのコンテンツに対して何でも行うことができます。`amazed`関数は単一のコンテンツ引数で呼び出せるので、showルールに名前で渡すだけで良いのです。試してみましょう。

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
テンプレートにいくつかのsetルールとshowルールを適用するには、関数内のコンテンツブロックで`set`と`show`を使用し、そのコンテンツブロックにドキュメントを挿入します。

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

前章で発見したように、setルールはそのコンテンツブロック内の全てに適用されます。everythingのshowルールが文書全体を`template`関数に渡すため、テンプレート内のテキストのsetルールと文字列のshowルールが文書全体に適用されます。この知識を使って、前章で作成した論文の本文スタイルを再現するテンプレートを作成しましょう。

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

1. everythingのshowルールを使用して、全てを`conf`関数でラップしました。この関数はいくつかのsetルールとshowルールを適用し、最後に渡されたコンテンツをそのまま出力します。

2. さらに、コンテンツブロックの代わりに中括弧で囲まれたコードブロックを使用しました。この方法では、全てのsetルールや関数呼び出しの前に`#`を付ける必要がなくなります。代わりに、コードブロック内に直接マークアップを書くことはできなくなります。

また、タイトルがどこから来ているかに注目してください。以前は変数に格納しましたが、今はテンプレート関数の最初のパラメータとして受け取っています。そのために、everythingのshowルールにクロージャー（その場で使用される名前のない関数）を渡しました。`conf`関数は2つの引数（タイトルと本文）を期待しますが、showルールは本文のみを渡すからです。したがって、論文のタイトルを設定し、showルールからの単一パラメータを使用できる新しい関数定義を追加します。

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

この新しいテンプレート関数を構築しましょう。まず、`title`引数にデフォルト値を追加します。これにより、タイトルを指定せずにテンプレートを呼び出すことができます。また、空のデフォルト値を持つ名前付き引数として`authors`および`abstract`パラメータを追加します。次に、前章からタイトル、要約、著者を生成するコードをテンプレートにコピーし、固定の詳細をパラメータに置き換えます。

新しい`authors`パラメータは、`name`、`affiliation`、`email`というキーを持つ[辞書]($dictionary)の[配列]($array)を想定しています。任意の数の著者を持つことができるため、著者リストに1列、2列、または3列が必要かどうかを動的に決定します。まず、`authors`配列の[`.len()`]($array.len)メソッドを使用して著者の数を決定します。次に、列数を著者数と3の最小値に設定し、3列以上作成しないようにします。3人以上の著者がいる場合は、代わりに新しい行が挿入されます。この目的のために、`grid`関数に`row-gutter`パラメータも追加しました。そうしないと、行同士が近すぎてしまいます。辞書から著者の詳細を抽出するには、[フィールドアクセス構文]($scripting/#fields)を使用します。

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
多くの場合、テンプレートは別のファイルで指定され、それからドキュメントにインポートされます。この方法では、メインファイルはすっきりとし、テンプレートを簡単に再利用できます。ファイルパネルでプラスボタンをクリックして新しいテキストファイルを作成し、`conf.typ`という名前を付けます。`conf`関数定義をその新しいファイルに移動します。これで、showルールの前にインポートを追加することで、メインファイルからアクセスできます。`{import}`キーワードとコロンの間にファイルのパスを指定し、インポートしたい関数に名前を付けます。

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
おめでとうございます！Typstのチュートリアルを完了しました。このセクションでは、独自の関数を定義する方法と、再利用可能なドキュメントスタイルを定義するテンプレートを作成・適用する方法を学びました。あなたは多くを学び、ここまで来ました。これでTypstを使用して独自の文書を作成し、他の人と共有することができます。

私たちはまだ非常に若いプロジェクトであり、フィードバックを求めています。質問、提案、またはバグを発見した場合は、[フォーラム](https://forum.typst.app/)、[Discordサーバー](https://discord.gg/2uDybryKPe)、[GitHub](https://github.com/typst/typst/)、またはウェブアプリのフィードバックフォーム（ヘルプメニューからいつでも利用可能）でお知らせください。

さっそく[サインアップ](https://typst.app)して何か書いてみましょう！
