---
description: |
<<<<<<< HEAD
   文書内の位置に応じて反応するコンテンツの扱い方を説明します。
---

# コンテキスト
時には、文書内の位置に応じて反応するコンテンツを作成したいことがあります。
これは、設定されたテキストの言語に依存するローカライズされたフレーズや、
前にいくつの見出しがあったかに基づいて正しい値を表示する、
見出し番号のような単純なものかもしれません。
しかし、Typstコードは直接的に文書内の位置を認識しているわけではありません。
ソーステキストの冒頭にあるコードが、文書の最後に配置されるコンテンツを生成する可能性があります。

そのため、周囲の環境に反応するコンテンツを生成するためには、Typstへの明示的な指示が必要です。
これを行うには、`{context}` キーワードを使用します。
このキーワードは式の前に置かれ、その式が環境の情報をもとに計算されることを保証します。
その代わりに、コンテキスト式自体は不透明になります。コンテキスト式の結果にコード内で直接アクセスすることはできません。
なぜなら、コンテキスト式の結果はコンテキスト依存であるためです。したがって、正しい1つの結果が存在するのではなく、文書の異なる場所に複数の結果が存在する可能性があります。
そのため、コンテキスト依存データに基づいた全てのものは、コンテキスト式の内部で行われる必要があります。

明示的なコンテキスト式以外にも、
文書内の位置を認識する場所では暗黙的にコンテキストが確立されます。
例えば、[showルール]($styling/#show-rules)はコンテキストを提供し[^1]、
アウトライン内の番号付けもカウンターを解決するための適切なコンテキストを提供します。

## 書式コンテキスト {#style-context}
setルールを使用すると、文書の一部または全体の書式のプロパティを調整できます。
これらは文書の進行に伴って変更される可能性があるため、既知のコンテスキトがなければこれらにアクセスすることはできません。コンテキストが利用可能な場合、
個別の要素関数のフィールドとして書式のプロパティにアクセスすることでこれらを簡単に取得できます。
=======
   How to deal with content that reacts to its location in the document.
---

# Context
Sometimes, we want to create content that reacts to its location in the
document. This could be a localized phrase that depends on the configured text
language or something as simple as a heading number which prints the right
value based on how many headings came before it. However, Typst code isn't
directly aware of its location in the document. Some code at the beginning of
the source text could yield content that ends up at the back of the document.

To produce content that is reactive to its surroundings, we must thus
specifically instruct Typst: We do this with the `{context}` keyword, which
precedes an expression and ensures that it is computed with knowledge of its
environment. In return, the context expression itself ends up opaque. We cannot
directly access whatever results from it in our code, precisely because it is
contextual: There is no one correct result, there may be multiple results in
different places of the document. For this reason, everything that depends on
the contextual data must happen inside of the context expression.

Aside from explicit context expressions, context is also established implicitly
in some places that are also aware of their location in the document:
[Show rules]($styling/#show-rules) provide context[^1] and numberings in the
outline, for instance, also provide the proper context to resolve counters.

## Style context
With set rules, we can adjust style properties for parts or the whole of our
document. We cannot access these without a known context, as they may change
throughout the course of the document. When context is available, we can
retrieve them simply by accessing them as fields on the respective element
function.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#set text(lang: "de")
#context text.lang
```

<<<<<<< HEAD
上記の説明の通り、コンテキスト式はそれが配置されるさまざまな環境に反応します。以下の例では、単一のコンテキスト式を作成し、それを `value` 変数に格納して複数回使用します。
それぞれのコンテキスト式は、現在の環境に適切に反応します。
=======
As explained above, a context expression is reactive to the different
environments it is placed into. In the example below, we create a single context
expression, store it in the `value` variable and use it multiple times. Each use
properly reacts to the current surroundings.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#let value = context text.lang
#value

#set text(lang: "de")
#value

#set text(lang: "fr")
#value
```

<<<<<<< HEAD
重要なのは、作成時に `value` は中身を覗くことができない不透明な [content]($content) になることです。それはどこかに配置されたときにのみ解決されます。なぜなら、そのときに初めてコンテキストが認識されるからです。コンテキスト式の本体は、それが配置される場所の数に応じて、0回、1回、または複数回評価される可能性があります。

## 位置コンテキスト
コンテキストでアクセスできるのはsetルールの値だけではありません。
そのコンテキストが現在の文書内の _どこにあるのか_ を、他の要素との相対位置やページの絶対位置として知ることも可能です。
この情報を活用することで、文書内のさまざまなパーツ同士を柔軟に連携させることができます。
位置コンテキストは、見出し番号や目次、章ごとに変わるページヘッダーを扱う機能の基盤となります。

[`counter.get`]($counter.get)のようないくつかの関数は、暗黙的に現在の位置にアクセスします。
以下の例では、見出しカウンターの値を取得したいとします。
これは文書全体で変化するため、まずコンテキスト式に入る必要があります。
次に、`get`を使用してカウンターの現在の値を取得します。
この関数は、カウンターの値を解決するためにコンテキストから現在の位置にアクセスします。
カウンターには複数のレベルがあり、`get`は解決された数値の配列を返します。
したがって、以下の結果が得られます。
=======
Crucially, upon creation, `value` becomes opaque [content] that we cannot peek
into. It can only be resolved when placed somewhere because only then the
context is known. The body of a context expression may be evaluated zero, one,
or multiple times, depending on how many different places it is put into.

## Location context
We've already seen that context gives us access to set rule values. But it can
do more: It also lets us know _where_ in the document we currently are, relative
to other elements, and absolutely on the pages. We can use this information to
create very flexible interactions between different document parts. This
underpins features like heading numbering, the table of contents, or page
headers dependent on section headings.

Some functions like [`counter.get`]($counter.get) implicitly access the current
location. In the example below, we want to retrieve the value of the heading
counter. Since it changes throughout the document, we need to first enter a
context expression. Then, we use `get` to retrieve the counter's current value.
This function accesses the current location from the context to resolve the
counter value. Counters have multiple levels and `get` returns an array with the
resolved numbers. Thus, we get the following result:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#set heading(numbering: "1.")

= Introduction
#lorem(5)

#context counter(heading).get()

= Background
#lorem(5)

#context counter(heading).get()
```

<<<<<<< HEAD
より柔軟性を持たせるために、[`here`]($here) 関数を使用してコンテキストから直接現在の[location]($location) を抽出することもできます。以下の例でこれを示します

- 最初に `{counter(heading).get()}` があり、これは先程のように `{(2,)}` に解決されます。
- 次に、より強力な [`counter.at`]($counter.at) と [`here`]($here) を組み合わせて使用します。これは `get` と同等であるため `{(2,)}` が得られます。
- 最後に、[label]($label) と組み合わせて `at` を使用して、文書内の _異なる_ 位置、つまり導入見出しの位置でカウンターの値を取得します。これにより `{(1,)}` が得られます。Typstのコンテキストシステムは、文書内の _任意の_ 位置でカウンターや状態の値を取得することができるタイムトラベル能力を提供します。
=======
For more flexibility, we can also use the [`here`] function to directly extract
the current [location] from the context. The example below
demonstrates this:

- We first have `{counter(heading).get()}`, which resolves to `{(2,)}` as
  before.
- We then use the more powerful  [`counter.at`] with [`here`], which in
  combination is equivalent to `get`, and thus get `{(2,)}`.
- Finally, we use `at` with a [label] to retrieve the value of the counter at a
  _different_ location in the document, in our case that of the introduction
  heading. This yields `{(1,)}`. Typst's context system gives us time travel
  abilities and lets us retrieve the values of any counters and states at _any_
  location in the document.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#set heading(numbering: "1.")

= Introduction <intro>
#lorem(5)

= Background <back>
#lorem(5)

#context [
  #counter(heading).get() \
  #counter(heading).at(here()) \
  #counter(heading).at(<intro>)
]
```

<<<<<<< HEAD
前述の通り、コンテキストを使用してページ上の要素の物理的位置を取得することもできます。
これは、`counter.at` と同様に機能する [`locate`]($locate) 関数を使用して行います。
この関数は一意の要素（ラベルでも可）に解決される位置または他の [selector]($selector) を取り、その要素のページ上の位置を返します。
=======
As mentioned before, we can also use context to get the physical position of
elements on the pages. We do this with the [`locate`] function, which works
similarly to `counter.at`: It takes a location or other [selector] that resolves
to a unique element (could also be a label) and returns the position on the
pages for that element.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
Background is at: \
#context locate(<back>).position()

= Introduction <intro>
#lorem(5)
#pagebreak()

= Background <back>
#lorem(5)
```

<<<<<<< HEAD
位置コンテキストを利用する関数は他にもありますが、最も顕著なのは [`query`]($query) です。
これらの詳細については、[introspection]($category/introspection) カテゴリを参照してください。

## ネストされたコンテキスト {#nested-contexts}
コンテキストは、コンテキストブロック内にネストされた関数呼び出しからもアクセス可能です。
以下の例では、`foo` 自体が [`to-absolute`]($length.to-absolute) と同様のコンテキスト依存の関数になります。
=======
There are other functions that make use of the location context, most
prominently [`query`]. Take a look at the
[introspection]($category/introspection) category for more details on those.

## Nested contexts
Context is also accessible from within function calls nested in context blocks.
In the example below, `foo` itself becomes a contextual function, just like
[`to-absolute`]($length.to-absolute) is.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#let foo() = 1em.to-absolute()
#context {
  foo() == text.size
}
```

<<<<<<< HEAD
コンテキストブロックはネストできます。
この場合、コンテキスト依存のコードは常に最も内側のコンテキストにアクセスします。
以下の例ではこれを示しています。最初の `text.lang` は外側のコンテキストブロックのスタイルにアクセスするため、
`{set text(lang: "fr")}` の効果は**見られません**。
しかし、2番目の `text.lang` の周りにあるネストされたコンテキストブロックはsetルールの後に始まるため、その効果が表示されます。
=======
Context blocks can be nested. Contextual code will then always access the
innermost context. The example below demonstrates this: The first `text.lang`
will access the outer context block's styles and as such, it will **not**
see the effect of `{set text(lang: "fr")}`. The nested context block around the
second `text.lang`, however, starts after the set rule and will thus show
its effect.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#set text(lang: "de")
#context [
  #set text(lang: "fr")
  #text.lang \
  #context text.lang
]
```

<<<<<<< HEAD
なぜTypstが上記の例で最初の `text.lang` を計算する際にフランス語のsetルールを無視するのか疑問に思うかもしれません。その理由は、一般的な場合、setルールがコンテンツの構築後に適用される可能性があるため、Typstは適用される全てのスタイルを知ることができないからです。以下の例では、テンプレート関数が適用されるときに `text.lang` がすでに計算されています。そのため、Typstがテンプレート内のフランス語への言語変更に気づくことは不可能です。
=======
You might wonder why Typst ignores the French set rule when computing the first
`text.lang` in the example above. The reason is that, in the general case, Typst
cannot know all the styles that will apply as set rules can be applied to
content after it has been constructed. Below, `text.lang` is already computed
when the template function is applied. As such, it cannot possibly be aware of
the language change to French in the template.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#let template(body) = {
  set text(lang: "fr")
  upper(body)
}

#set text(lang: "de")
#context [
  #show: template
  #text.lang \
  #context text.lang
]
```

<<<<<<< HEAD
しかし、2番目の `text.lang` は言語の変更に反応 _します_。なぜなら、その周囲のコンテキストブロックの評価が、それに対するスタイルがわかるまで遅延されるからです。これは、正確なスタイルにアクセスするために、コンテキストにとって適切な挿入ポイントを選択することの重要性を示しています。

同様のことが位置コンテキストにも当てはまります。
以下の例では、最初の `{c.display()}` 呼び出しは外側のコンテキストブロックにアクセスするため、 `{c.update(2)}` の効果を見ることはできません。
一方、2番目の `{c.display()}` は内部のコンテキストにアクセスするため、効果を見ることができます。
=======
The second `text.lang`, however, _does_ react to the language change because
evaluation of its surrounding context block is deferred until the styles for it
are known. This illustrates the importance of picking the right insertion point for a context to get access to precisely the right styles.

The same also holds true for the location context. Below, the first
`{c.display()}` call will access the outer context block and will thus not see
the effect of `{c.update(2)}` while the second `{c.display()}` accesses the inner context and will thus see it.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#let c = counter("mycounter")
#c.update(1)
#context [
  #c.update(2)
  #c.display() \
  #context c.display()
]
```

<<<<<<< HEAD
## コンパイラの反復 {#compiler-iterations}
コンテキスト依存の相互作用を解決するため、Typstのコンパイラは文書を複数回処理します。
例えば、`locate` の呼び出しを解決するために、Typstはまずプレースホルダーの位置を提供し、文書をレイアウトし、レイアウトが完了した位置から既知の位置で再コンパイルします。
カウンターや状態、クエリを解決するためにも同じアプローチが取られます。
特定の場合には、Typstは全てを解決するために2回以上の反復が必要になることもあります。
それが必要な場合もあれば、コンテキスト依存関数の誤用の兆候であることもあります（例えば[state]($state/#caution)の誤用）。
Typstが5回の試行で全てを解決できない場合、警告 "layout did not converge within 5 attempts." が出力され、処理が停止します。

非常に注意深い読者の方は、上記で紹介した関数のうち、全ての関数が現在の位置を実際に使用しているわけではないことに気づいたかもしれません。`{counter(heading).get()}` は確かに現在の位置に依存していますが、例えば `{counter(heading).at(<intro>)}` はそうではありません。しかし、それでもコンテキストが必要です。その値は1つのコンパイラ反復内では常に同じですが、複数のコンパイラ反復の間に変化する可能性があります。もしこれをモジュールのトップレベルで直接呼び出すことができれば、モジュール全体とそのエクスポートは複数のコンパイラ反復の間に変化する可能性があり、それは望ましくありません。

[^1]: 現在、全てのshowルールはスタイリングコンテキストを提供しますが、[locatable]($location/#locatable) 要素のshowルールのみが位置コンテキストを提供します。
=======
## Compiler iterations
To resolve contextual interactions, the Typst compiler processes your document
multiple times. For instance, to resolve a `locate` call, Typst first provides a
placeholder position, layouts your document and then recompiles with the known
position from the finished layout. The same approach is taken to resolve
counters, states, and queries. In certain cases, Typst may even need more than
two iterations to resolve everything. While that's sometimes a necessity, it may
also be a sign of misuse of contextual functions (e.g. of
[state]($state/#caution)). If Typst cannot resolve everything within five
attempts, it will stop and output the warning "layout did not converge within 5
attempts."

A very careful reader might have noticed that not all of the functions presented
above actually make use of the current location. While
`{counter(heading).get()}` definitely depends on it,
`{counter(heading).at(<intro>)}`, for instance, does not. However, it still
requires context. While its value is always the same _within_ one compilation
iteration, it may change over the course of multiple compiler iterations. If one
could call it directly at the top level of a module, the whole module and its
exports could change over the course of multiple compiler iterations, which
would not be desirable.

[^1]: Currently, all show rules provide [style context](#style-context), but
      only show rules on [locatable]($location/#locatable) elements provide a
      [location context](#location-context).
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
