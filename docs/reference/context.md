---
description: |
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
このキーワードは式の前に置かれ、その式が環境の情報を元に計算されることを保証します。
その代わりに、コンテキスト式自体は不透明になります。コンテキスト式の結果にコード内で直接アクセスすることはできません。
なぜなら、コンテキスト式の結果はコンテキスト依存であるためです。したがって、正しい1つの結果が存在するのではなく、文書の異なる場所に複数の結果が存在する可能性があります。
そのため、コンテキスト依存データに基づいたすべてのものは、コンテキスト式の内部で行われる必要があります。

明示的なコンテキスト式以外にも、
文書内の位置を認識する場所では暗黙的にコンテキストが確立されます。
例えば、[showルール]($styling/#show-rules)はコンテキストを提供し[^1]、
アウトライン内の番号付けもカウンターを解決するための適切なコンテキストを提供します。

## 書式コンテキスト {#style-context}
setルールを使用すると、文書の一部または全体の書式のプロパティを調整できます。
これらは文書の進行に伴って変更される可能性があるため、既知のコンテスキトがなければこれらにアクセスすることはできません。コンテキストが利用可能な場合、
個別の要素関数のフィールドとして書式のプロパティにアクセスすることでこれらを簡単に取得できます。

```example
#set text(lang: "de")
#context text.lang
```

上記の説明の通り、コンテキスト式はそれが配置されるさまざまな環境に反応します。以下の例では、単一のコンテキスト式を作成し、それを `value` 変数に格納して複数回使用します。
それぞれのコンテキスト式は、現在の環境に適切に反応します。

```example
#let value = context text.lang
#value

#set text(lang: "de")
#value

#set text(lang: "fr")
#value
```

重要なのは、作成時に `value` は中身を覗くことができない不透明な [content]($content) になることです。それはどこかに配置されたときにのみ解決されます。なぜなら、そのときに初めてコンテキストが認識されるからです。コンテキスト式の本体は、それが配置される場所の数に応じて、0回、1回、または複数回評価される可能性があります。

## Location context
Context can not only give us access to set rule values. It can also let us know
_where_ in the document we currently are, relative to other elements, and
absolutely on the pages. We can use this information to create very flexible
interactions between different document parts. This underpins features like
heading numbering, the table of contents, or page headers dependant on section
headings.

[`counter.get`]($counter.get)のようないくつかの関数は、暗黙的に現在の位置にアクセスします。
以下の例では、見出しカウンターの値を取得したいとします。
これは文書全体で変化するため、まずコンテキスト式に入る必要があります。
次に、`get`を使用してカウンターの現在の値を取得します。
この関数は、カウンターの値を解決するためにコンテキストから現在の位置にアクセスします。
カウンターには複数のレベルがあり、`get`は解決された数値の配列を返します。
したがって、以下の結果が得られます。

```example
#set heading(numbering: "1.")

= Introduction
#lorem(5)

#context counter(heading).get()

= Background
#lorem(5)

#context counter(heading).get()
```

より柔軟性を持たせるために、[`here`]($here) 関数を使用してコンテキストから直接現在の[location]($location) を抽出することもできます。以下の例でこれを示します

- 最初に `{counter(heading).get()}` があり、これは先程のように `{(2,)}` に解決されます。
- 次に、より強力な [`counter.at`]($counter.at) と [`here`]($here) を組み合わせて使用します。これは `get` と同等であるため `{(2,)}` が得られます。
- 最後に、[label]($label) と組み合わせて `at` を使用して、文書内の _異なる_ 位置、つまり導入見出しの位置でカウンターの値を取得します。これにより `{(1,)}` が得られます。Typstのコンテキストシステムは、文書内の _任意の_ 位置でカウンターや状態の値を取得することができるタイムトラベル能力を提供します。

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

前述の通り、コンテキストを使用してページ上の要素の物理的位置を取得することもできます。
これは、`counter.at` と同様に機能する [`locate`]($locate) 関数を使用して行います。
この関数は一意の要素（ラベルでも可）に解決される位置または他の [selector]($selector) を取り、その要素のページ上の位置を返します。

```example
Background is at: \
#context locate(<back>).position()

= Introduction <intro>
#lorem(5)
#pagebreak()

= Background <back>
#lorem(5)
```

位置コンテキストを利用する関数は他にもありますが、最も顕著なのは [`query`]($query) です。
これらの詳細については、[introspection]($category/introspection) カテゴリを参照してください。

## ネストされたコンテキスト {#nested-contexts}
コンテキストは、コンテキストブロック内にネストされた関数呼び出しからもアクセス可能です。
以下の例では、`foo` 自体が [`to-absolute`]($length.to-absolute) と同様のコンテキスト依存の関数になります。

```example
#let foo() = 1em.to-absolute()
#context {
  foo() == text.size
}
```

コンテキストブロックはネストできます。
この場合、コンテキスト依存のコードは常に最も内側のコンテキストにアクセスします。
以下の例ではこれを示しています。最初の `text.lang` は外側のコンテキストブロックのスタイルにアクセスするため、
`{set text(lang: "fr")}` の効果は**見られません**。
しかし、2番目の `text.lang` の周りにあるネストされたコンテキストブロックはsetルールの後に始まるため、その効果が表示されます。

```example
#set text(lang: "de")
#context [
  #set text(lang: "fr")
  #text.lang \
  #context text.lang
]
```

なぜTypstが上記の例で最初の `text.lang` を計算する際にフランス語のsetルールを無視するのか疑問に思うかもしれません。その理由は、一般的な場合、setルールがコンテンツの構築後に適用される可能性があるため、Typstは適用されるすべてのスタイルを知ることができないからです。以下の例では、テンプレート関数が適用されるときに `text.lang` がすでに計算されています。そのため、Typstがテンプレート内のフランス語への言語変更に気づくことは不可能です。

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

しかし、2番目の `text.lang` は言語の変更に反応 _します_。なぜなら、その周囲のコンテキストブロックの評価が、それに対するスタイルがわかるまで遅延されるからです。これは、正確なスタイルにアクセスするために、コンテキストにとって適切な挿入ポイントを選択することの重要性を示しています。

同様のことが位置コンテキストにも当てはまります。
以下の例では、最初の `{c.display()}` 呼び出しは外側のコンテキストブロックにアクセスするため、 `{c.update(2)}` の効果を見ることはできません。
一方、2番目の `{c.display()}` は内部のコンテキストにアクセスするため、効果を見ることができます。

```example
#let c = counter("mycounter")
#c.update(1)
#context [
  #c.update(2)
  #c.display() \
  #context c.display()
]
```

## コンパイラの反復 {#compiler-iterations}
コンテキスト依存の相互作用を解決するため、Typstのコンパイラは文書を複数回処理します。
例えば、`locate` の呼び出しを解決するために、Typstはまずプレースホルダーの位置を提供し、文書をレイアウトし、レイアウトが完了した位置から既知の位置で再コンパイルします。
カウンターや状態、クエリを解決するためにも同じアプローチが取られます。
特定の場合には、Typstはすべてを解決するために2回以上の反復が必要になることもあります。
それが必要な場合もあれば、コンテキスト依存関数の誤用の兆候であることもあります（例えば[state]($state/#caution)の誤用）。
Typstが5回の試行ですべてを解決できない場合、警告 "layout did not converge within 5 attempts." が出力され、処理が停止します。

非常に注意深い読者の方は、上記で紹介した関数のうち、すべての関数が現在の位置を実際に使用しているわけではないことに気づいたかもしれません。`{counter(heading).get()}` は確かに現在の位置に依存していますが、例えば `{counter(heading).at(<intro>)}` はそうではありません。しかし、それでもコンテキストが必要です。その値は1つのコンパイラ反復内では常に同じですが、複数のコンパイラ反復の間に変化する可能性があります。もしこれをモジュールのトップレベルで直接呼び出すことができれば、モジュール全体とそのエクスポートは複数のコンパイラ反復の間に変化する可能性があり、それは望ましくありません。

[^1]: 現在、すべてのshowルールはスタイリングコンテキストを提供しますが、[locatable]($location/#locatable) 要素のshowルールのみが位置コンテキストを提供します。
