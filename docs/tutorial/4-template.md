---
<<<<<<< HEAD
description: Typstチュートリアル
---

# テンプレートを作成する
このチュートリアルの前回の3つの章では、Typstでドキュメントを書く方法、基本的なスタイルを適用する方法、そして出版社のスタイルガイドに準拠するために外観を詳細にカスタマイズする方法を学びました。前章で作成した論文が大成功を収めたため、同じ会議のための続報論文を書くよう依頼されました。今回は、前章で作成したスタイルを再利用可能なテンプレートに変換したいと思います。この章では、あなたとあなたのチームが単一のshowルールで使用できるテンプレートの作成方法を学びます。始めましょう！

## 簡易テンプレート { #toy-template }
Typstでは、テンプレートは文書全体をラップできる関数です。その方法を学ぶために、まずは独自の関数の書き方を復習しましょう。関数は何でもできるので、少し奇抜なものを作ってみませんか？
=======
description: Typst's tutorial.
---

# Making a Template
In the previous three chapters of this tutorial, you have learned how to write a
document in Typst, apply basic styles, and customize its appearance in-depth to
comply with a publisher's style guide. Because the paper you wrote in the
previous chapter was a tremendous success, you have been asked to write a
follow-up article for the same conference. This time, you want to take the style
you created in the previous chapter and turn it into a reusable template. In
this chapter you will learn how to create a template that you and your team can
use with just one show rule. Let's get started!

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

## A toy template { #toy-template }
In Typst, templates are functions in which you can wrap your whole document. To
learn how to do that, let's first review how to write your very own functions.
They can do anything you want them to, so why not go a bit crazy?
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#let amazed(term) = box[✨ #term ✨]

You are #amazed[beautiful]!
```

<<<<<<< HEAD
この関数は単一の引数`term`を取り、`term`を✨で囲んだコンテンツブロックを返します。また、amazed対象の語が改行で✨と分離されないように、全体をボックスに入れています。

Typstに組み込まれている多くの関数には、オプションの名前付きパラメータがあります。私たちの関数にも名前付きパラメータを追加できます。テキストの色を選択できるパラメータを追加してみましょう。パラメータが指定されない場合のデフォルトの色を提供する必要があります。
=======
Comparing this against the previous section, you may have noticed that this
looks a lot like a variable definition using `{let}`. This instinct is correct:
Functions are just another data type. Here, we are defining the variable
`amazed`, assigning it a function that takes a single argument, `term`, and
returns content with the `term` surrounded by sparkles. We also put the whole
thing in a [`box`] so that the term we are amazed by cannot be separated from
its sparkles by a line break. The special function definition syntax makes the
definition shorter and more readable, but you can also use the regular variable
definition syntax (see [the scripting reference]($scripting/#bindings) for
details). After its definition, we are able to call the function just like all
built-in functions.

Many functions that come with Typst have optional named parameters. Our
functions can also have them. Let's add a parameter to our function that lets us
choose the color of the text. We need to provide a default color in case the
parameter isn't given.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#let amazed(term, color: blue) = {
  text(color, box[✨ #term ✨])
}

You are #amazed[beautiful]!
I am #amazed(color: purple)[amazed]!
```

<<<<<<< HEAD
テンプレートは`amazed`のようなカスタム関数でドキュメント全体をラップすることで機能します。しかし、文書全体を巨大な関数呼び出しでラップするのは面倒でしょう！代わりに、「everything」showルールを使用して、より洗練されたコードで同じことを実現できます。そのようなshowルールを書くには、showキーワードの直後にコロンを置き、関数を提供します。この関数にはドキュメントの残りの部分がパラメータとして渡されます。関数はこのコンテンツに対して何でも行うことができます。`amazed`関数は単一のコンテンツ引数で呼び出せるので、showルールに名前で渡すだけで良いのです。試してみましょう。
=======
Templates now work by wrapping our whole document in a custom function like
`amazed`. But wrapping a whole document in a giant function call would be
cumbersome! Instead, we can use an "everything" show rule to achieve the same
with cleaner code. To write such a show rule, put a colon directly after the
show keyword and then provide a function. This function is given the rest of the
document as a parameter. The function can then do anything with this content.
Since the `amazed` function can be called with a single content argument, we can
just pass it by name to the show rule. Let's try it:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

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

<<<<<<< HEAD
これで文書全体が`amazed`関数に渡され、文書をその関数でラップしたかのように機能します。もちろん、この特定の関数ではあまり有用ではありませんが、setルールと名前付き引数と組み合わせると、非常に強力になります。

## setルールとshowルールの埋め込み { #set-and-show-rules }
テンプレートにいくつかのsetルールとshowルールを適用するには、関数内のコンテンツブロックで`set`と`show`を使用し、そのコンテンツブロックにドキュメントを挿入します。
=======
Our whole document will now be passed to the `amazed` function, as if we wrapped
it around it. Of course, this is not especially useful with this particular
function, but when combined with set rules and named arguments, it can be very
powerful.

## Embedding set and show rules { #set-and-show-rules }
To apply some set and show rules to our template, we can use `set` and `show`
within a content block in our function and then insert the document into
that content block.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

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

<<<<<<< HEAD
前章で発見したように、setルールはそのコンテンツブロック内の全てに適用されます。everythingのshowルールが文書全体を`template`関数に渡すため、テンプレート内のテキストのsetルールと文字列のshowルールが文書全体に適用されます。この知識を使って、前章で作成した論文の本文スタイルを再現するテンプレートを作成しましょう。
=======
Just like we already discovered in the previous chapter, set rules will apply to
everything within their content block. Since the everything show rule passes our
whole document to the `template` function, the text set rule and string show
rule in our template will apply to the whole document. Let's use this knowledge
to create a template that reproduces the body style of the paper we wrote in the
previous chapter.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#let conf(title, doc) = {
  set page(
    paper: "us-letter",
>>> margin: auto,
    header: align(
      right + horizon,
      title
    ),
<<<<<<< HEAD
=======
>>> numbering: "1",
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
>>>  show heading.where(
>>>    level: 1
>>>  ): it => block(
>>>    align(center,
>>>      text(
>>>        13pt,
>>>        weight: "regular",
>>>        smallcaps(it.body),
>>>      )
>>>    ),
>>>  )
>>>  show heading.where(
>>>    level: 2
>>>  ): it => box(
>>>    text(
>>>      11pt,
>>>      weight: "regular",
>>>      style: "italic",
>>>      it.body + [.],
>>>    )
>>>  )
=======
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
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

  doc
}

#show: doc => conf(
  [Paper title],
  doc,
)

= Introduction
<<<<<<< HEAD
#lorem(90)

<<< ...
=======
<<< ...
>>> #lorem(90)
>>>
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
>>> == Motivation
>>> #lorem(140)
>>>
>>> == Problem Statement
>>> #lorem(50)
>>>
>>> = Related Work
>>> #lorem(200)
```

<<<<<<< HEAD
コードの大部分は前章からコピーペーストしました。2つの違いがあります。

1. everythingのshowルールを使用して、全てを`conf`関数でラップしました。この関数はいくつかのsetルールとshowルールを適用し、最後に渡されたコンテンツをそのまま出力します。

2. さらに、コンテンツブロックの代わりに中括弧で囲まれたコードブロックを使用しました。この方法では、全てのsetルールや関数呼び出しの前に`#`を付ける必要がなくなります。代わりに、コードブロック内に直接マークアップを書くことはできなくなります。

また、タイトルがどこから来ているかに注目してください。以前は変数に格納しましたが、今はテンプレート関数の最初のパラメータとして受け取っています。そのために、everythingのshowルールにクロージャー（その場で使用される名前のない関数）を渡しました。`conf`関数は2つの引数（タイトルと本文）を期待しますが、showルールは本文のみを渡すからです。したがって、論文のタイトルを設定し、showルールからの単一パラメータを使用できる新しい関数定義を追加します。

## 名前付き引数を持つテンプレート { #named-arguments }
前章の論文にはタイトルと著者リストがありました。これらの要素をテンプレートに追加しましょう。タイトルに加えて、所属機関を含む著者リストと論文の要約をテンプレートに受け付けるようにします。可読性を保つために、これらを名前付き引数として追加します。最終的には、次のように機能させたいと思います。

```typ
#show: doc => conf(
  title: [Towards Improved Modelling],
=======
We copy-pasted most of that code from the previous chapter. The two differences
are this:

1. We wrapped everything in the function `conf` using an everything show rule.
   The function applies a few set and show rules and echoes the content it has
   been passed at the end.

2. Moreover, we used a curly-braced code block instead of a content block. This
   way, we don't need to prefix all set rules and function calls with a `#`. In
   exchange, we cannot write markup directly in the code block anymore.

Also note where the title comes from: We previously had it inside of a variable.
Now, we are receiving it as the first parameter of the template function. To do
so, we passed a closure (that's a function without a name that is used right
away) to the everything show rule. We did that because the `conf` function
expects two positional arguments, the title and the body, but the show rule will
only pass the body. Therefore, we add a new function definition that allows us
to set a paper title and use the single parameter from the show rule.

## Templates with named arguments { #named-arguments }
Our paper in the previous chapter had a title and an author list. Let's add
these things to our template. In addition to the title, we want our template to
accept a list of authors with their affiliations and the paper's abstract. To
keep things readable, we'll add those as named arguments. In the end, we want it
to work like this:

```typ
#show: doc => conf(
  title: [
    A Fluid Dynamic Model for
    Glacier Flow
  ],
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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

<<<<<<< HEAD
この新しいテンプレート関数を構築しましょう。まず、`title`引数にデフォルト値を追加します。これにより、タイトルを指定せずにテンプレートを呼び出すことができます。また、空のデフォルト値を持つ名前付き引数として`authors`および`abstract`パラメータを追加します。次に、前章からタイトル、要約、著者を生成するコードをテンプレートにコピーし、固定の詳細をパラメータに置き換えます。

新しい`authors`パラメータは、`name`、`affiliation`、`email`というキーを持つ[辞書]($dictionary)の[配列]($array)を想定しています。任意の数の著者を持つことができるため、著者リストに1列、2列、または3列が必要かどうかを動的に決定します。まず、`authors`配列の[`.len()`]($array.len)メソッドを使用して著者の数を決定します。次に、列数を著者数と3の最小値に設定し、3列以上作成しないようにします。3人以上の著者がいる場合は、代わりに新しい行が挿入されます。この目的のために、`grid`関数に`row-gutter`パラメータも追加しました。そうしないと、行同士が近すぎてしまいます。辞書から著者の詳細を抽出するには、[フィールドアクセス構文]($scripting/#fields)を使用します。

各著者についてグリッドに引数を提供する必要があります。ここで配列の[`map`メソッド]($array.map)が便利です。これは引数として関数を取り、その関数が配列の各アイテムで呼び出されます。各著者の詳細をフォーマットし、コンテンツ値を含む新しい配列を返す関数を渡します。これで、グリッドの複数の引数として使用したい値の配列ができました。[`spread`演算子]($arguments)を使用してこれを実現できます。これは配列を取り、その各アイテムを関数の個別の引数として適用します。

結果のテンプレート関数は次のようになります。

```typ
#let conf(
  title: none,
=======
Let's build this new template function. First, we add a default value to the
`title` argument. This way, we can call the template without specifying a title.
We also add the named `authors` and `abstract` parameters with empty defaults.
Next, we copy the code that generates title, abstract and authors from the
previous chapter into the template, replacing the fixed details with the
parameters.

The new `authors` parameter expects an [array] of [dictionaries]($dictionary)
with the keys `name`, `affiliation` and `email`. Because we can have an
arbitrary number of authors, we dynamically determine if we need one, two or
three columns for the author list. First, we determine the number of authors
using the [`.len()`]($array.len) method on the `authors` array. Then, we set the
number of columns as the minimum of this count and three, so that we never
create more than three columns. If there are more than three authors, a new row
will be inserted instead. For this purpose, we have also added a `row-gutter`
parameter to the `grid` function. Otherwise, the rows would be too close
together. To extract the details about the authors from the dictionary, we use
the [field access syntax]($scripting/#fields).

We still have to provide an argument to the grid for each author: Here is where
the array's [`map` method]($array.map) comes in handy. It takes a function as an
argument that gets called with each item of the array. We pass it a function
that formats the details for each author and returns a new array containing
content values. We've now got one array of values that we'd like to use as
multiple arguments for the grid. We can do that by using the
[`spread` operator]($arguments). It takes an array and applies each of its items
as a separate argument to the function.

The resulting template function looks like this:

```typ
#let conf(
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
  authors: (),
  abstract: [],
  doc,
) = {
  // Set and show rules from before.
<<<<<<< HEAD
>>> #set page(columns: 2)
<<<   ...

  set align(center)
  text(17pt, title)

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

  set align(left)
=======
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

>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
  doc
}
```

<<<<<<< HEAD
## 別ファイル { #separate-file }
多くの場合、テンプレートは別のファイルで指定され、それからドキュメントにインポートされます。この方法では、メインファイルはすっきりとし、テンプレートを簡単に再利用できます。ファイルパネルでプラスボタンをクリックして新しいテキストファイルを作成し、`conf.typ`という名前を付けます。`conf`関数定義をその新しいファイルに移動します。これで、showルールの前にインポートを追加することで、メインファイルからアクセスできます。`{import}`キーワードとコロンの間にファイルのパスを指定し、インポートしたい関数に名前を付けます。

テンプレートの適用をより洗練させるためにできるもう1つのことは、関数の[`.with`]($function.with)メソッドを使用して、全ての名前付き引数を事前に設定することです。これにより、クロージャーを記述してテンプレートリストの最後にコンテンツ引数を追加する必要がなくなります。[Typst Universe]($universe)のテンプレートは、この関数呼び出しのスタイルで動作するように設計されています。

```example:single
>>> #let conf(
>>>   title: none,
=======
## A separate file { #separate-file }
Most of the time, a template is specified in a different file and then imported
into the document. This way, the main file you write in is kept clutter free and
your template is easily reused. Create a new text file in the file panel by
clicking the plus button and name it `conf.typ`. Move the `conf` function
definition inside of that new file. Now you can access it from your main file by
adding an import before the show rule. Specify the path of the file between the
`{import}` keyword and a colon, then name the function that you want to import.

Another thing that you can do to make applying templates just a bit more elegant
is to use the [`.with`]($function.with) method on functions to pre-populate all
the named arguments. This way, you can avoid spelling out a closure and
appending the content argument at the bottom of your template list. Templates on
[Typst Universe]($universe) are designed to work with this style of function
call.

```example:single
>>> #let conf(
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
>>>   authors: (),
>>>   abstract: [],
>>>   doc,
>>> ) = {
<<<<<<< HEAD
>>>  set text(font: "Libertinus Serif", 11pt)
>>>  set par(justify: true)
>>>  set page(
>>>    "us-letter",
>>>    margin: auto,
>>>    header: align(
>>>      right + horizon,
>>>      title
>>>    ),
>>>    numbering: "1",
>>>    columns: 2,
>>>  )
>>>
>>>  show heading.where(
>>>    level: 1
>>>  ): it => block(
>>>    align(center,
>>>      text(
>>>        13pt,
>>>        weight: "regular",
>>>        smallcaps(it.body),
>>>      )
>>>    ),
>>>  )
>>>  show heading.where(
>>>    level: 2
>>>  ): it => box(
>>>    text(
>>>      11pt,
>>>      weight: "regular",
>>>      style: "italic",
>>>      it.body + [.],
>>>    )
>>>  )
>>>
>>>  place(
>>>    top,
>>>    float: true,
>>>    scope: "parent",
>>>    clearance: 2em,
>>>    {
>>>      set align(center)
>>>      text(17pt, title)
>>>      let count = calc.min(authors.len(), 3)
>>>      grid(
>>>        columns: (1fr,) * count,
>>>        row-gutter: 24pt,
>>>        ..authors.map(author => [
>>>          #author.name \
>>>          #author.affiliation \
>>>          #link("mailto:" + author.email)
>>>        ]),
>>>      )
>>>      par(justify: false)[
>>>        *Abstract* \
>>>        #abstract
>>>      ]
>>>    },
>>>  )
>>>  doc
>>>}
<<< #import "conf.typ": conf
#show: conf.with(
  title: [
    Towards Improved Modelling
  ],
=======
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
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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

<<<<<<< HEAD
これで会議論文を、その会議用の再利用可能なテンプレートに変換しました！[フォーラム](https://forum.typst.app/)や[TypstのDiscordサーバー](https://discord.gg/2uDybryKPe)で共有して、他の人も使えるようにしてみてはいかがでしょうか？

## まとめ { #review }
おめでとうございます！Typstのチュートリアルを完了しました。このセクションでは、独自の関数を定義する方法と、再利用可能なドキュメントスタイルを定義するテンプレートを作成・適用する方法を学びました。あなたは多くを学び、ここまで来ました。これでTypstを使用して独自の文書を作成し、他の人と共有することができます。

私たちはまだ非常に若いプロジェクトであり、フィードバックを求めています。質問、提案、またはバグを発見した場合は、[フォーラム](https://forum.typst.app/)、[Discordサーバー](https://discord.gg/2uDybryKPe)、[GitHub](https://github.com/typst/typst/)、またはウェブアプリのフィードバックフォーム（ヘルプメニューからいつでも利用可能）でお知らせください。

さっそく[サインアップ](https://typst.app)して何か書いてみましょう！
=======
We have now converted the conference paper into a reusable template for that
conference! Why not share it in the [Forum](https://forum.typst.app/) or on
[Typst's Discord server](https://discord.gg/2uDybryKPe) so that others can use
it too?

## Review
Congratulations, you have completed Typst's Tutorial! In this section, you have
learned how to define your own functions and how to create and apply templates
that define reusable document styles. You've made it far and learned a lot. You
can now use Typst to write your own documents and share them with others.

We are still a super young project and are looking for feedback. If you have any
questions, suggestions or you found a bug, please let us know
in the [Forum](https://forum.typst.app/),
on our [Discord server](https://discord.gg/2uDybryKPe),
on [GitHub](https://github.com/typst/typst/),
or via the web app's feedback form (always available in the Help menu).

So what are you waiting for? [Sign up](https://typst.app) and write something!
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
