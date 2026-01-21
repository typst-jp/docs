---
<<<<<<< HEAD
description: Typstのスクリプト記述機能を用いて、ドキュメントを自動化しましょう。
---

# スクリプト記述

Typstには強力なスクリプト言語が組み込まれています。これにより、ドキュメントを自動化し、コードを使用してより洗練されたスタイルを作成できます。
以下は、スクリプト記述の概念の概要です。

## 式 { #expressions }

Typstでは、マークアップとコードが一体となっています。
最も一般的な要素以外のものは　_関数_ を使って作成されます。
これを可能な限り便利にするため、Typstはコード式をマークアップに埋め込むためのコンパクトな構文を提供しています。式はハッシュ（`#`）で始まり、
この式が終了すると通常のマークアップの解析が再開されます。
式の直後に通常の文字列として解釈されるべき文字が続く場合、セミコロン（`;`）を使って式を強制的に終了できます。
=======
description: Automate your document with Typst's scripting capabilities.
---

# Scripting
Typst embeds a powerful scripting language. You can automate your documents and
create more sophisticated styles with code. Below is an overview over the
scripting concepts.

## Expressions
In Typst, markup and code are fused into one. All but the most common elements
are created with _functions._ To make this as convenient as possible, Typst
provides compact syntax to embed a code expression into markup: An expression is
introduced with a hash (`#`) and normal markup parsing resumes after the
expression is finished. If a character would continue the expression but should
be interpreted as text, the expression can forcibly be ended with a semicolon
(`;`). You can [escape a literal `#` or `;` with a backslash]($syntax/#escapes).
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#emph[Hello] \
#emoji.face \
#"hello".len()
```

<<<<<<< HEAD
上記の例では、[関数呼び出し]($function)、
[フィールドアクセス]($scripting/#fields)、
そして[メソッド呼び出し]($scripting/#methods)などのいくつかの使用可能な式を示しています。
この章の残りでは、より多くの種類の式について説明します。
いくつかの種類の式はハッシュ構文と互換性がありません（例えば二項演算子式）。
このような式をマークアップに埋め込むためには、`[#(1+2)]`のように丸括弧を使用します。

## ブロック { #blocks }
コードを構造化し、マークアップを埋め込むために、Typstは次の二種類の _ブロック_ を提供します。

- **コードブロック** `{{ let x = 1; x + 2 }}` \
  コードを書く際には、計算を複数のステートメントに分割したり、中間変数を作成したりすることがあるでしょう。
  コードブロックを使用すると、複数の式を1つのブロックにまとめられます。ブロック内の個々の式は改行またはセミコロンで区切られます。
  コードブロック内の個々の式の出力値は結合され、ブロックの値を決定します。
  `{let}`バインディングのように有用な出力のない式は`{none}`を生成し、これはどの値とも影響なく結合できます。

- **コンテンツブロック** `{[*Hey* there!]}` \
  コンテンツブロックを使用すると、マークアップやコンテンツをプログラム的な値として扱い、変数に保存したり、[関数]($function)に渡したりできます。
  コンテンツブロックは角括弧で囲まれ、任意のマークアップを含められます。
  コンテンツブロックは[content]($content)型の値を生成します。
  任意の数のコンテンツブロックを可変長引数として関数に渡せます。つまり、`{list[A][B]}`は`{list([A], [B])}`と等価です。

コンテンツブロックとコードブロックは任意にネストできます。以下の例では、`{[hello]}`が`{a + [ the ] + b}`の出力と結合され、`{[hello from the *world*]}`が生成されます。

=======
The example above shows a few of the available expressions, including
[function calls]($function), [field accesses]($scripting/#fields), and
[method calls]($scripting/#methods). More kinds of expressions are
discussed in the remainder of this chapter. A few kinds of expressions are not
compatible with the hash syntax (e.g. binary operator expressions). To embed
these into markup, you can use parentheses, as in `[#(1 + 2)]`.

## Blocks
To structure your code and embed markup into it, Typst provides two kinds of
_blocks:_

- **Code block:** `{{ let x = 1; x + 2 }}` \
  When writing code, you'll probably want to split up your computation into
  multiple statements, create some intermediate variables and so on. Code blocks
  let you write multiple expressions where one is expected. The individual
  expressions in a code block should be separated by line breaks or semicolons.
  The output values of the individual expressions in a code block are joined to
  determine the block's value. Expressions without useful output, like `{let}`
  bindings yield `{none}`, which can be joined with any value without effect.

- **Content block:** `{[*Hey* there!]}` \
  With content blocks, you can handle markup/content as a programmatic value,
  store it in variables and pass it to [functions]($function). Content
  blocks are delimited by square brackets and can contain arbitrary markup. A
  content block results in a value of type [content]. An arbitrary number of
  content blocks can be passed as trailing arguments to functions. That is,
  `{list([A], [B])}` is equivalent to `{list[A][B]}`.

Content and code blocks can be nested arbitrarily. In the example below,
`{[hello ]}` is joined with the output of  `{a + [ the ] + b}` yielding
`{[hello from the *world*]}`.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#{
  let a = [from]
  let b = [*world*]
  [hello ]
  a + [ the ] + b
}
```

<<<<<<< HEAD
## バインディングと分割 { #bindings }
前述のように、変数は`{let}`バインディングで定義できます。
変数には`=`記号のあとに続く式の値が代入されます。値の代入は任意であり、値が代入されなければ変数は`{none}`として初期化されます。
`{let}`キーワードは、[カスタム名前付き関数]($function/#defining-functions)を作成するためにも使用できます。
変数は、それが含まれるブロックの残りの部分（または、変数が含まれるブロックがない場合はファイル全体）の中でアクセスできます。
=======
## Bindings and Destructuring { #bindings }
As already demonstrated above, variables can be defined with `{let}` bindings.
The variable is assigned the value of the expression that follows the `=` sign.
A [valid variable name](#identifiers) may contain `-`, but cannot start with `-`.
The assignment of a value is optional, if no value is assigned, the variable
will be initialized as `{none}`. The `{let}` keyword can also be used to create
a [custom named function]($function/#defining-functions). Variables can be
accessed for the rest of the containing block (or the rest of the file if there
is no containing block).
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#let name = "Typst"
This is #name's documentation.
It explains #name.

<<<<<<< HEAD
#let add(x, y) = x + y
Sum is #add(2, 3).
```

letバインディングは[配列]($array)や[辞書]($dictionary)の分割にも使用できます。
この場合、代入の左辺は配列や辞書と同じ形式でなければなりません。
`..`演算子はパターン内で一度だけ、配列や辞書の残りの項目を受け取るために使用できます。
=======
#let my-add(x, y) = x + y
Sum is #my-add(2, 3).
```

Let bindings can also be used to destructure [arrays]($array) and
[dictionaries]($dictionary). In this case, the left-hand side of the
assignment should mirror an array or dictionary. The `..` operator can be used
once in the pattern to collect the remainder of the array's or dictionary's
items.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#let (x, y) = (1, 2)
The coordinates are #x, #y.

#let (a, .., b) = (1, 2, 3, 4)
The first element is #a.
The last element is #b.

#let books = (
  Shakespeare: "Hamlet",
  Homer: "The Odyssey",
  Austen: "Persuasion",
)

#let (Austen,) = books
Austen wrote #Austen.

#let (Homer: h) = books
Homer wrote #h.

#let (Homer, ..other) = books
#for (author, title) in other [
  #author wrote #title.
]
```

<<<<<<< HEAD
分割パターンでアンダースコアを使用して、要素を破棄できます。
=======
You can use the underscore to discard elements in a destructuring pattern:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#let (_, y, _) = (1, 2, 3)
The y coordinate is #y.
```

<<<<<<< HEAD
分割は関数の引数リスト内でも機能します。
=======
Destructuring also works in argument lists of functions ...
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#let left = (2, 4, 5)
#let right = (3, 2, 6)
#left.zip(right).map(
  ((a,b)) => a + b
)
```

<<<<<<< HEAD
そして通常の代入の左辺でも機能します。これは、変数の値を交換するなどの場合に便利です。
=======
... and on the left-hand side of normal assignments. This can be useful to
swap variables among other things.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#{
  let a = 1
  let b = 2
  (a, b) = (b, a)
  [a = #a, b = #b]
}
```

<<<<<<< HEAD
## 条件式 { #conditionals }

条件式を使用すると、ある条件が満たされているかどうかに応じて、異なるものを表示したり計算したりできます。
Typstは`{if}`式、`{else if}`式、および`{else}`式をサポートしています。
条件が`{true}`の場合、条件式は`if`の部分の結果が得られ、そうでない場合は`else`の部分の結果が得られます。
=======
## Conditionals
With a conditional, you can display or compute different things depending on
whether some condition is fulfilled. Typst supports `{if}`, `{else if}` and
`{else}` expressions. When the condition evaluates to `{true}`, the conditional
yields the value resulting from the if's body, otherwise yields the value
resulting from the else's body.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#if 1 < 2 [
  This is shown
] else [
  This is not.
]
```

<<<<<<< HEAD
各条件分岐は、その本文としてコードブロックまたはコンテンツブロックを含められます。
=======
Each branch can have a code or content block as its body.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

- `{if condition {..}}`
- `{if condition [..]}`
- `{if condition [..] else {..}}`
- `{if condition [..] else if condition {..} else [..]}`

<<<<<<< HEAD
## ループ { #loops }

ループを使用すると、コンテンツを繰り返したり、何かを反復的に計算したりできます。
Typstは、`{for}`ループと`{while}`ループの2つのループをサポートしています。
前者は指定されたコレクションを反復するのに対し、後者は条件が満たされている限り反復を続けます。
ブロックと同様に、ループは各反復からの結果を1つの値に _結合_ します。

以下の例では、forループによって作成された3つの文が1つのコンテンツ値に結合され、whileループの中の長さ1の配列が1つの大きな配列に結合されます。
=======
## Loops
With loops, you can repeat content or compute something iteratively. Typst
supports two types of loops: `{for}` and `{while}` loops. The former iterate
over a specified collection whereas the latter iterate as long as a condition
stays fulfilled. Just like blocks, loops _join_ the results from each iteration
into one value.

In the example below, the three sentences created by the for loop join together
into a single content value and the length-1 arrays in the while loop join
together into one larger array.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#for c in "ABC" [
  #c is a letter.
]

#let n = 2
#while n < 10 {
  n = (n * 2) - 1
  (n,)
}
```

<<<<<<< HEAD
forループはさまざまなコレクションを反復処理できます。

- `{for value in array {..}}` \
  [配列]($array)内の各項目を反復処理します。[letバインディング]($scripting/#bindings)で説明されている分割構文もここで使用できます。

- `{for pair in dict {..}}` \
  [辞書]($dictionary)のキーと値のペアを反復処理します。ペアは`{for (key, value) in dict {..}}`を使用しても分割できます。これは、全てのキーと値のペアの一時配列を作成しないため、`{for pair in dict.pairs() {..}}`よりも効率的です。

- `{for letter in "abc" {..}}` \
  [文字列]($str)の各文字を反復処理します。厳密に言うと、文字列の書記素クラスタの反復処理を行います。ほとんどの場合、書記素クラスタは単一のコードポイントに対応します。しかし、書記素クラスタは複数のコードポイントを含むことがあります。例えば、国旗の絵文字などです。

- `{for byte in bytes("😀") {..}}` \
  [文字列]($str)から変換されたり、エンコーディングなしでファイルから[読み取る]($read)ことができる[バイト]($bytes)を反復処理します。各バイト値は`{0}`から`{255}`までの[整数]($int)です。

ループの実行を制御するために、Typstは`{break}`と`{continue}`ステートメントを提供しています。
前者はループを早期終了し、後者はループの次の反復を開始します。
=======
For loops can iterate over a variety of collections:

- `{for value in array {..}}` \
  Iterates over the items in the [array]. The destructuring syntax described in
  [Let binding]($scripting/#bindings) can also be used here.

- `{for pair in dict {..}}` \
  Iterates over the key-value pairs of the [dictionary]. The pairs can also be
  destructured by using `{for (key, value) in dict {..}}`. It is more efficient
  than `{for pair in dict.pairs() {..}}` because it doesn't create a temporary
  array of all key-value pairs.

- `{for letter in "abc" {..}}` \
  Iterates over the characters of the [string]($str). Technically, it iterates
  over the grapheme clusters of the string. Most of the time, a grapheme cluster
  is just a single codepoint. However, a grapheme cluster could contain multiple
  codepoints, like a flag emoji.

- `{for byte in bytes("😀") {..}}` \
  Iterates over the [bytes], which can be converted from a [string]($str) or
  [read] from a file without encoding. Each byte value is an [integer]($int)
  between `{0}` and `{255}`.

To control the execution of the loop, Typst provides the `{break}` and
`{continue}` statements. The former performs an early exit from the loop while
the latter skips ahead to the next iteration of the loop.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#for letter in "abc nope" {
  if letter == " " {
    break
  }

  letter
}
```

<<<<<<< HEAD
ループの本体はコードブロックまたはコンテンツブロックにできます。
=======
The body of a loop can be a code or content block:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

- `{for .. in collection {..}}`
- `{for .. in collection [..]}`
- `{while condition {..}}`
- `{while condition [..]}`

<<<<<<< HEAD
## フィールド { #fields }
_ドット記法_ を使用することで値のフィールドにアクセスできます。[コンテンツ]($content)型の値については、[fields]($content.fields)関数を使用してフィールドを一覧表示できます。

<!-- textlint-disable jtf-style/1.1.3.箇条書き -->

対象の値は以下のいずれかです。
- 指定されたキーを持つ[辞書]($dictionary)、
- 指定された修飾子を持つ[記号]($symbol)、
- 指定された定義を含む[モジュール]($module)、
- 指定されたフィールドを持つ要素で構成された[コンテンツ]($content)。利用可能なフィールドは、その要素が構築された際に与えられた[要素関数]($function/#element-functions)の引数と一致します。

<!-- textlint-enable -->
=======
## Fields
You can use _dot notation_ to access fields on a value. For values of type
[`content`], you can also use the [`fields`]($content.fields) function to list
the fields.

The value in question can be either:
- a [dictionary] that has the specified key,
- a [symbol] that has the specified modifier,
- a [module] containing the specified definition,
- [content] consisting of an element that has the specified field. The
  available fields match the arguments of the
  [element function]($function/#element-functions) that were given when the
  element was constructed.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#let it = [= Heading]
#it.body \
#it.depth \
#it.fields()

#let dict = (greet: "Hello")
#dict.greet \
#emoji.face

```

<<<<<<< HEAD
## メソッド { #methods }
メソッド呼び出しは、値の[型]($type)で定義された関数を呼び出す便利な方法です。
例えば、[`str.len`]($str.len)関数は下記の2つの同等の方法で呼び出せます。
=======
## Methods
A _method call_ is a convenient way to call a function that is scoped to a
value's [type]. For example, we can call the [`str.len`]($str.len) function in
the following two equivalent ways:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#str.len("abc") is the same as
#"abc".len()
```

<<<<<<< HEAD
メソッド呼び出しの構造は`{value.method(..args)}`であり、これと等価である完全な関数呼び出しは`{type(value).method(value, ..args)}`です。各型のドキュメントには、その型に関連付けられた関数が一覧表示されています。現在、独自のメソッドは定義できません
=======
The structure of a method call is `{value.method(..args)}` and its equivalent
full function call is `{type(value).method(value, ..args)}`. The documentation
of each type lists its scoped functions. You cannot currently define your own
methods.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#let values = (1, 2, 3, 4)
#values.pop() \
#values.len() \

#("a, b, c"
    .split(", ")
    .join[ --- ])

#"abc".len() is the same as
#str.len("abc")
```

<<<<<<< HEAD
特定の値に対して呼び出された際に、その値を変更する特別な関数がいくつかあります（例： [`array.push`]($array.push)）。
これらの関数は、_必ず_ メソッド形式で呼び出す必要があります。
場合によっては、メソッドが副作用のためだけに呼び出されるとき、その戻り値は無視されるべきです（結合に加わるべきではありません）。
値を破棄する標準的な方法は、`{let _ = array.remove(1)}`のように
letバインディングを使用することです。

## モジュール { #modules }

Typstプロジェクトを、_モジュール_ と呼ばれる複数のファイルに分割できます。
モジュールは他のモジュールのコンテンツや定義を複数の方法で参照できます。

<!-- textlint-disable jtf-style/1.1.3.箇条書き -->

- **インクルード：** `{include "bar.typ"}` \
  パス`bar.typ`にあるファイルを評価し、その結果として得られる[コンテンツ]($content)を返します。

- **インポート：** `{import "bar.typ"}` \
  パス`bar.typ`にあるファイルを評価し、その結果として得られる
  [モジュール]($module)を現在のスコープに`bar`（拡張子なしのファイル名）として挿入します。次のように、`as`キーワードを使用してモジュール名を変更できます。
  `{import "bar.typ" as baz}`
  You can import nested items using dot notation: `{import "bar.typ": baz.a}`.

- **アイテムのインポート:** `{import "bar.typ": a, b}` \
  パス`bar.typ`にあるファイルを評価して、変数`a`と`b`の値
 （これらは`bar.typ`で、例えば`{let}`バインディングを通じて定義されている必要があります）
  を抽出し、現在のファイルで定義します。  `a, b`を`*`に置き換えると、モジュールで定義された全ての変数が読み込まれます。
  次のように、`as`キーワードを使用してモジュール名を変更できます。
  `{import "bar.typ": a as one, b as two}`

<!-- textlint-enable -->

以下の例に示すように、パスの代わりに[モジュール値]($module)を使用できます。
=======
There are a few special functions that modify the value they are called on (e.g.
[`array.push`]($array.push)). These functions _must_ be called in method form.
In some cases, when the method is only called for its side effect, its return
value should be ignored (and not participate in joining). The canonical way to
discard a value is with a let binding: `{let _ = array.remove(1)}`.

## Modules
You can split up your Typst projects into multiple files called _modules._ A
module can refer to the content and definitions of another module in multiple
ways:

- **Including:** `{include "bar.typ"}` \
  Evaluates the file at the path `bar.typ` and returns the resulting [content].

- **Import:** `{import "bar.typ"}` \
  Evaluates the file at the path `bar.typ` and inserts the resulting [module]
  into the current scope as `bar` (filename without extension). You can use the
  `as` keyword to rename the imported module: `{import "bar.typ" as baz}`. You
  can import nested items using dot notation: `{import "bar.typ": baz.a}`.

- **Import items:** `{import "bar.typ": a, b}` \
  Evaluates the file at the path `bar.typ`, extracts the values of the variables
  `a` and `b` (that need to be defined in `bar.typ`, e.g. through `{let}`
  bindings) and defines them in the current file. Replacing `a, b` with `*`
  loads all variables defined in a module. You can use the `as` keyword to
  rename the individual items: `{import "bar.typ": a as one, b as two}`

Instead of a path, you can also use a [module value]($module), as shown in the
following example:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#import emoji: face
#face.grin
```

<<<<<<< HEAD
## パッケージ { #packages }
プロジェクト間でビルド中のブロックを再利用するために、Typstの _パッケージ_ を作成してインポートできます。
パッケージのインポートは、名前空間、名前、バージョンの3つの組み合わせとして指定されます。
=======
## Packages
To reuse building blocks across projects, you can also create and import Typst
_packages._ A package import is specified as a triple of a namespace, a name,
and a version.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
>>> #let add(x, y) = x + y
<<< #import "@preview/example:0.1.0": add
#add(2, 7)
```

<<<<<<< HEAD
`preview`名前空間には、コミュニティで共有されているパッケージが含まれています。全ての利用可能なコミュニティパッケージは[Typst Universe]($universe)にあります。

Typstをローカルで使用している場合、独自のシステムローカルパッケージを作成できます。詳細については、[パッケージリポジトリ](https://github.com/typst/packages)を参照してください。

## 演算子 { #operators }

以下の表は、使用可能な全ての単項および二項演算子の効果、引数の個数（単項あるいは二項）、および優先度（高いほど強く結合）を示しています。
[剰余]($calc.rem-euclid)
などの一部の演算は特別な構文を持たず、
[`calc`]($category/foundations/calc)モジュールの関数を用いて実現できます。

|   演算子   | 効果                                 | 引数の個数 | 優先度 |
| :--------: | ------------------------------------ | :--------: | :----: |
|   `{-}`    | 負号                                 |    単項    |   7    |
|   `{+}`    | 効果なし (対称性のために存在)        |    単項    |   7    |
|   `{*}`    | 乗算                                 |    二項    |   6    |
|   `{/}`    | 除算                                 |    二項    |   6    |
|   `{+}`    | 加算                                 |    二項    |   5    |
|   `{-}`    | 減算                                 |    二項    |   5    |
|   `{==}`   | 等価性の評価                         |    二項    |   4    |
|   `{!=}`   | 非等価性の評価                       |    二項    |   4    |
|   `{<}`    | 小なりの評価                         |    二項    |   4    |
|   `{<=}`   | 以下の評価                           |    二項    |   4    |
|   `{>}`    | 大なりの評価                         |    二項    |   4    |
|   `{>=}`   | 以上の評価                           |    二項    |   4    |
|   `{in}`   | コレクション内に存在することの評価   |    二項    |   4    |
| `{not in}` | コレクション内に存在しないことの評価 |    二項    |   4    |
|  `{not}`   | 論理否定                             |    単項    |   3    |
|  `{and}`   | 短絡論理積                           |    二項    |   3    |
|   `{or}`   | 短絡論理和                           |    二項    |   2    |
|   `{=}`    | 代入                                 |    二項    |   1    |
|   `{+=}`   | 加算代入                             |    二項    |   1    |
|   `{-=}`   | 減算代入                             |    二項    |   1    |
|   `{*=}`   | 乗算代入                             |    二項    |   1    |
|   `{/=}`   | 除算代入                             |    二項    |   1    |
=======
The `preview` namespace contains packages shared by the community. You can find
all available community packages on [Typst Universe]($universe).

If you are using Typst locally, you can also create your own system-local
packages. For more details on this, see the
[package repository](https://github.com/typst/packages).

## Operators
The following table lists all available unary and binary operators with effect,
arity (unary, binary) and precedence level (higher binds stronger). Some
operations, such as [modulus]($calc.rem-euclid), do not have a special syntax
and can be achieved using functions from the
[`calc`]($category/foundations/calc) module.

| Operator   | Effect                          | Arity  | Precedence |
|:----------:|---------------------------------|:------:|:----------:|
|  `{-}`     | Negation                        | Unary  |     7      |
|  `{+}`     | No effect (exists for symmetry) | Unary  |     7      |
|  `{*}`     | Multiplication                  | Binary |     6      |
|  `{/}`     | Division                        | Binary |     6      |
|  `{+}`     | Addition                        | Binary |     5      |
|  `{-}`     | Subtraction                     | Binary |     5      |
|  `{==}`    | Check equality                  | Binary |     4      |
|  `{!=}`    | Check inequality                | Binary |     4      |
|  `{<}`     | Check less-than                 | Binary |     4      |
|  `{<=}`    | Check less-than or equal        | Binary |     4      |
|  `{>}`     | Check greater-than              | Binary |     4      |
|  `{>=}`    | Check greater-than or equal     | Binary |     4      |
|  `{in}`    | Check if in collection          | Binary |     4      |
| `{not in}` | Check if not in collection      | Binary |     4      |
|  `{not}`   | Logical "not"                   | Unary  |     3      |
|  `{and}`   | Short-circuiting logical "and"  | Binary |     3      |
|  `{or}`    | Short-circuiting logical "or"   | Binary |     2      |
|  `{=}`     | Assignment                      | Binary |     1      |
|  `{+=}`    | Add-Assignment                  | Binary |     1      |
|  `{-=}`    | Subtraction-Assignment          | Binary |     1      |
|  `{*=}`    | Multiplication-Assignment       | Binary |     1      |
|  `{/=}`    | Division-Assignment             | Binary |     1      |
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

[semver]: https://semver.org/
