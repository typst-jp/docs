---
description: |
<<<<<<< HEAD
   Typstの構文に関するコンパクトなリファレンスです。詳細については、言語のマークアップモード、数式モード、およびコードモードを参照してください。
---

# 構文

Typstはマークアップ言語です。
これは、シンプルな構文を使用して一般的なレイアウトタスクを簡単に行えるということです。
Typstの軽量なマークアップ構文は、文書を簡単かつ自動的にスタイリングできるsetルールとshowルールによって補完されています。
これら全ては、組み込み関数およびユーザー定義関数を備えた、緊密に統合されたスクリプト言語によって支えられています。

## モード { #modes }

Typstには3種類の構文モードがあります。マークアップモード、数式モード、そしてコードモードです。
Typst文書では、マークアップモードがデフォルトであり、数式モードでは数式を書くことができ、コードモードではTypstのスクリプト機能を利用できます。

以下の表を参照し、いつでも特定のモードに切り替えることができます。

| 新たなモード | 構文                         | 例                              |
| ------------ | ---------------------------- | ------------------------------- |
| コード       | コードの前に`#`を付ける      | `[Number: #(1 + 2)]`            |
| 数式         | 式を`[$..$]`で囲む           | `[$-x$ is the opposite of $x$]` |
| マークアップ | マークアップを`[[..]]`で囲む | `{let name = [*Typst!*]}`       |

一度`#`でコードモードに入ると、途中でマークアップモードや数式モードに切り替えない限り、さらにハッシュを使う必要はありません。

## マークアップ { #markup }

Typstは、最も一般的な文書要素に対する組み込みのマークアップを提供します。
ほとんどの構文要素は、対応する関数のショートカットに過ぎません。
以下の表は、利用可能な全てのマークアップと、その構文と使用法について詳しく学ぶための最適なページへのリンクを示しています。

| 名称             | 例                          | 参照                                 |
| ---------------- | --------------------------- | ------------------------------------ |
| 段落区切り       | 空行                        | [`parbreak`]($parbreak)              |
| 強調(太字)       | `[*strong*]`                | [`strong`]($strong)                  |
| 強調(イタリック) | `[_emphasis_]`              | [`emph`]($emph)                      |
| rawテキスト      | ``[`print(1)`]``            | [`raw`]($raw)                        |
| リンク           | `[https://typst.app/]`      | [`link`]($link)                      |
| ラベル           | `[<intro>]`                 | [`label`]($label)                    |
| 参照             | `[@intro]`                  | [`ref`]($ref)                        |
| 見出し           | `[= Heading]`               | [`heading`]($heading)                |
| 箇条書きリスト   | `[- item]`                  | [`list`]($list)                      |
| 番号付きリスト   | `[+ item]`                  | [`enum`]($enum)                      |
| 用語リスト       | `[/ Term: description]`     | [`terms`]($terms)                    |
| 数式             | `[$x^2$]`                   | [Math]($category/math)               |
| 改行             | `[\]`                       | [`linebreak`]($linebreak)            |
| スマートクオート | `['single' or "double"]`    | [`smartquote`]($smartquote)          |
| 短縮記号         | `[~]`, `[---]`              | [Symbols]($category/symbols/sym)     |
| コード構文       | `[#rect(width: 1cm)]`       | [Scripting]($scripting/#expressions) |
| 文字エスケープ   | `[Tweet at us \#ad]`        | [Below](#escapes)                    |
| コメント         | `[/* block */]`, `[// line]` | [Below](#comments)                   |

## 数式モード { #math }

数式モードは、数式を組版するために使用される特別なマークアップモードです。
数式を `[$]` の文字で囲むことによって、数式モードに入ることができます。
これはマークアップモードとコードモードの両方で機能します。
数式が少なくとも1つのスペースで始まり終わる場合、その数式は独自のブロックに組版されます（例：`[$ x^2 $]`）。
インライン数式は、スペースを省略することで作成できます（例：`[$x^2$]`）。
以下に、数式モードに特有の構文の概要を示します。

| 名称                    | 例                       | 参照                                 |
| ----------------------- | ------------------------ | ------------------------------------ |
| インライン数式          | `[$x^2$]`                | [Math]($category/math)               |
| ブロック数式            | `[$ x^2 $]`              | [Math]($category/math)               |
| 下付き文字              | `[$x_1$]`                | [`attach`]($category/math/attach)    |
| 上付き文字              | `[$x^2$]`                | [`attach`]($category/math/attach)    |
| 分数                    | `[$1 + (a+b)/5$]`        | [`frac`]($math.frac)                 |
| 改行                    | `[$x \ y$]`              | [`linebreak`]($linebreak)            |
| 揃え位置                | `[$x &= 2 \ &= 3$]`      | [Math]($category/math)               |
| 変数アクセス            | `[$#x$, $pi$]`           | [Math]($category/math)               |
| フィールドアクセス      | `[$arrow.r.long$]`       | [Scripting]($scripting/#fields)      |
| 暗黙の乗算              | `[$x y$]`                | [Math]($category/math)               |
| 短縮記号                | `[$->$]`, `[$!=$]`       | [Symbols]($category/symbols/sym)     |
| 数式内のテキスト/文字列 | `[$a "is natural"$]`     | [Math]($category/math)               |
| 数式関数呼び出し        | `[$floor(x)$]`           | [Math]($category/math)               |
| コード構文              | `[$#rect(width: 1cm)$]`  | [Scripting]($scripting/#expressions) |
| 文字エスケープ          | `[$x\^2$]`               | [Below](#escapes)                    |
| コメント                | `[$/* comment */$]`      | [Below](#comments)                   |

## コードモード { #code }

コードブロックや式の中では、新しい式は先頭に`#`を付けずに始めることができます。
多くの構文要素は式に特有のものです。
以下に、コードモードで利用可能な全ての構文の一覧表を示します。

| 名称                       | 例                             | 参照                                  |
| -------------------------- | ------------------------------ | ------------------------------------- |
| none                       | `{none}`                       | [`none`]($reference/foundations/none) |
| 自動                       | `{auto}`                       | [`auto`]($reference/foundations/auto) |
| ブール値                   | `{false}`, `{true}`            | [`bool`]($reference/foundations/bool) |
| 整数                       | `{10}`, `{0xff}`               | [`int`]($reference/foundations/int)   |
| 浮動小数点数               | `{3.14}`, `{1e5}`              | [`float`]($reference/foundations/float) |
| 長さ                       | `{2pt}`, `{3mm}`, `{1em}`, ..  | [`length`]($reference/layout/length)  |
| 角度                       | `{90deg}`, `{1rad}`            | [`angle`]($reference/layout/angle)    |
| 比率                       | `{2fr}`                        | [`fraction`]($reference/layout/fraction) |
| 割合                       | `{50%}`                        | [`ratio`]($reference/layout/ratio)    |
| 文字列                     | `{"hello"}`                    | [`str`]($reference/foundations/str)   |
| ラベル                     | `{<intro>}`                    | [`label`]($reference/foundations/label) |
| 数式                       | `[$x^2$]`                      | [Math]($category/math)                |
| rawテキスト                | ``[`print(1)`]``               | [`raw`]($reference/text/raw)          |
| 変数アクセス               | `{x}`                          | [Scripting]($scripting/#blocks)       |
| コードブロック             | `{{ let x = 1; x + 2 }}`       | [Scripting]($scripting/#blocks)       |
| コンテンツブロック         | `{[*Hello*]}`                  | [Scripting]($scripting/#blocks)       |
| 括弧付き式                 | `{(1 + 2)}`                    | [Scripting]($scripting/#blocks)       |
| 配列                       | `{(1, 2, 3)}`                  | [Array]($array)                       |
| 辞書                       | `{(a: "hi", b: 2)}`            | [Dictionary]($dictionary)             |
| 単項演算子                 | `{-x}`                         | [Scripting]($scripting/#operators)    |
| 二項演算子                 | `{x + y}`                      | [Scripting]($scripting/#operators)    |
| 代入                       | `{x = 1}`                      | [Scripting]($scripting/#operators)    |
| フィールドアクセス         | `{x.y}`                        | [Scripting]($scripting/#fields)       |
| メソッド呼び出し           | `{x.flatten()}`                | [Scripting]($scripting/#methods)      |
| 関数呼び出し               | `{min(x, y)}`                  | [Function]($function)                 |
| 引数展開                   | `{min(..nums)}`                | [Arguments]($arguments)               |
| 無名関数                   | `{(x, y) => x + y}`            | [Function]($function)                 |
| letバインディング          | `{let x = 1}`                  | [Scripting]($scripting/#bindings)     |
| 名前付き関数               | `{let f(x) = 2 * x}`           | [Function]($function)                 |
| setルール                  | `{set text(14pt)}`             | [Styling]($styling/#set-rules)        |
| set-ifルール               | `{set text(..) if .. }`        | [Styling]($styling/#set-rules)        |
| show-setルール             | `{show heading: set block(..)}` | [Styling]($styling/#show-rules)     |
| 関数付きshowルール         | `{show raw: it => {..}}`       | [Styling]($styling/#show-rules)       |
| everything showルール      | `{show: template}`             | [Styling]($styling/#show-rules)       |
| コンテキスト式             | `{context text.lang}`          | [Context]($context)                   |
| 条件式                     | `{if x == 1 {..} else {..}}`   | [Scripting]($scripting/#conditionals) |
| forループ                  | `{for x in (1, 2, 3) {..}}`    | [Scripting]($scripting/#loops)        |
| whileループ                | `{while x < 10 {..}}`          | [Scripting]($scripting/#loops)        |
| ループ制御フロー           | `{break, continue}`            | [Scripting]($scripting/#loops)        |
| 関数からのリターン         | `{return x}`                   | [Function]($function)                 |
| モジュールをインクルード   | `{include "bar.typ"}`          | [Scripting]($scripting/#modules)      |
| モジュールをインポート     | `{import "bar.typ"}`           | [Scripting]($scripting/#modules)      |
| モジュールからのインポート | `{import "bar.typ": a, b, c}`  | [Scripting]($scripting/#modules)      |
| コメント                   | `{/* block */}`, `{// line}`   | [Below](#comments)                    |

## コメント { #comments }

コメントはTypstによって無視され、出力には含まれません。
これは古いバージョンを除外したり、注釈を追加したりするのに便利です。
単一行をコメントアウトするには、行の先頭に`//`を付けます。

=======
   A compact reference for Typst's syntax. Learn more about the language within
   markup, math, and code mode.
---

# Syntax
Typst is a markup language. This means that you can use simple syntax to
accomplish common layout tasks. The lightweight markup syntax is complemented by
set and show rules, which let you style your document easily and automatically.
All this is backed by a tightly integrated scripting language with built-in and
user-defined functions.

## Modes
Typst has three syntactical modes: Markup, math, and code. Markup mode is the
default in a Typst document, math mode lets you write mathematical formulas, and
code mode lets you use Typst's scripting features.

You can switch to a specific mode at any point by referring to the following
table:

| New mode | Syntax                          | Example                         |
|----------|---------------------------------|---------------------------------|
| Code     | Prefix the code with `#`        | `[Number: #(1 + 2)]`            |
| Math     | Surround equation with `[$..$]` | `[$-x$ is the opposite of $x$]` |
| Markup   | Surround markup with `[[..]]`   | `{let name = [*Typst!*]}`       |

Once you have entered code mode with `#`, you don't need to use further hashes
unless you switched back to markup or math mode in between.

## Markup
Typst provides built-in markup for the most common document elements. Most of
the syntax elements are just shortcuts for a corresponding function. The table
below lists all markup that is available and links to the  best place to learn
more about their syntax and usage.

| Name               | Example                      | See                      |
| ------------------ | ---------------------------- | ------------------------ |
| Paragraph break    | Blank line                   | [`parbreak`]             |
| Strong emphasis    | `[*strong*]`                 | [`strong`]               |
| Emphasis           | `[_emphasis_]`               | [`emph`]                 |
| Raw text           | ``[`print(1)`]``             | [`raw`]                  |
| Link               | `[https://typst.app/]`       | [`link`]                 |
| Label              | `[<intro>]`                  | [`label`]                |
| Reference          | `[@intro]`                   | [`ref`]                  |
| Heading            | `[= Heading]`                | [`heading`]              |
| Bullet list        | `[- item]`                   | [`list`]                 |
| Numbered list      | `[+ item]`                   | [`enum`]                 |
| Term list          | `[/ Term: description]`      | [`terms`]                |
| Math               | `[$x^2$]`                    | [Math]($category/math)   |
| Line break         | `[\]`                        | [`linebreak`]            |
| Smart quote        | `['single' or "double"]`     | [`smartquote`]           |
| Symbol shorthand   | `[~]`, `[---]`               | [Symbols]($category/symbols/sym) |
| Code expression    | `[#rect(width: 1cm)]`        | [Scripting]($scripting/#expressions) |
| Character escape   | `[Tweet at us \#ad]`         | [Below](#escapes)        |
| Comment            | `[/* block */]`, `[// line]` | [Below](#comments)       |

## Math mode { #math }
Math mode is a special markup mode that is used to typeset mathematical
formulas. It is entered by wrapping an equation in `[$]` characters. This works
both in markup and code. The equation will be typeset into its own block if it
starts and ends with at least one space (e.g. `[$ x^2 $]`). Inline math can be
produced by omitting the whitespace (e.g. `[$x^2$]`). An overview over the
syntax specific to math mode follows:

| Name                   | Example                  | See                      |
| ---------------------- | ------------------------ | ------------------------ |
| Inline math            | `[$x^2$]`                | [Math]($category/math)   |
| Block-level math       | `[$ x^2 $]`              | [Math]($category/math)   |
| Bottom attachment      | `[$x_1$]`                | [`attach`]($category/math/attach) |
| Top attachment         | `[$x^2$]`                | [`attach`]($category/math/attach) |
| Fraction               | `[$1 + (a+b)/5$]`        | [`frac`]($math.frac)     |
| Line break             | `[$x \ y$]`              | [`linebreak`]            |
| Alignment point        | `[$x &= 2 \ &= 3$]`      | [Math]($category/math)   |
| Variable access        | `[$#x$, $pi$]`           | [Math]($category/math)   |
| Field access           | `[$arrow.r.long$]`       | [Scripting]($scripting/#fields) |
| Implied multiplication | `[$x y$]`                | [Math]($category/math)   |
| Symbol shorthand       | `[$->$]`, `[$!=$]`       | [Symbols]($category/symbols/sym) |
| Text/string in math    | `[$a "is natural"$]`     | [Math]($category/math)   |
| Math function call     | `[$floor(x)$]`           | [Math]($category/math)   |
| Code expression        | `[$#rect(width: 1cm)$]`  | [Scripting]($scripting/#expressions) |
| Character escape       | `[$x\^2$]`               | [Below](#escapes)        |
| Comment                | `[$/* comment */$]`      | [Below](#comments)       |

## Code mode { #code }
Within code blocks and expressions, new expressions can start without a leading
`#` character. Many syntactic elements are specific to expressions. Below is
a table listing all syntax that is available in code mode:

| Name                     | Example                       | See                                |
| ------------------------ | ----------------------------- | ---------------------------------- |
| None                     | `{none}`                      | [`none`]                           |
| Auto                     | `{auto}`                      | [`auto`]                           |
| Boolean                  | `{false}`, `{true}`           | [`bool`]                           |
| Integer                  | `{10}`, `{0xff}`              | [`int`]                            |
| Floating-point number    | `{3.14}`, `{1e5}`             | [`float`]                          |
| Length                   | `{2pt}`, `{3mm}`, `{1em}`, .. | [`length`]                         |
| Angle                    | `{90deg}`, `{1rad}`           | [`angle`]                          |
| Fraction                 | `{2fr}`                       | [`fraction`]                       |
| Ratio                    | `{50%}`                       | [`ratio`]                          |
| String                   | `{"hello"}`                   | [`str`]                            |
| Label                    | `{<intro>}`                   | [`label`]                          |
| Math                     | `[$x^2$]`                     | [Math]($category/math)             |
| Raw text                 | ``[`print(1)`]``              | [`raw`]                            |
| Variable access          | `{x}`                         | [Scripting]($scripting/#blocks)    |
| Code block               | `{{ let x = 1; x + 2 }}`      | [Scripting]($scripting/#blocks)    |
| Content block            | `{[*Hello*]}`                 | [Scripting]($scripting/#blocks)    |
| Parenthesized expression | `{(1 + 2)}`                   | [Scripting]($scripting/#blocks)    |
| Array                    | `{(1, 2, 3)}`                 | [Array]($array)                    |
| Dictionary               | `{(a: "hi", b: 2)}`           | [Dictionary]($dictionary)          |
| Unary operator           | `{-x}`                        | [Scripting]($scripting/#operators) |
| Binary operator          | `{x + y}`                     | [Scripting]($scripting/#operators) |
| Assignment               | `{x = 1}`                     | [Scripting]($scripting/#operators) |
| Field access             | `{x.y}`                       | [Scripting]($scripting/#fields)    |
| Method call              | `{x.flatten()}`               | [Scripting]($scripting/#methods)   |
| Function call            | `{min(x, y)}`                 | [Function]($function)              |
| Argument spreading       | `{min(..nums)}`               | [Arguments]($arguments)            |
| Unnamed function         | `{(x, y) => x + y}`           | [Function]($function)              |
| Let binding              | `{let x = 1}`                 | [Scripting]($scripting/#bindings)  |
| Named function           | `{let f(x) = 2 * x}`          | [Function]($function)              |
| Set rule                 | `{set text(14pt)}`            | [Styling]($styling/#set-rules)     |
| Set-if rule              | `{set text(..) if .. }`       | [Styling]($styling/#set-rules)     |
| Show-set rule            | `{show heading: set block(..)}` | [Styling]($styling/#show-rules)  |
| Show rule with function  | `{show raw: it => {..}}`      | [Styling]($styling/#show-rules)    |
| Show-everything rule     | `{show: template}`            | [Styling]($styling/#show-rules)    |
| Context expression       | `{context text.lang}`         | [Context]($context)                |
| Conditional              | `{if x == 1 {..} else {..}}`  | [Scripting]($scripting/#conditionals) |
| For loop                 | `{for x in (1, 2, 3) {..}}`   | [Scripting]($scripting/#loops)     |
| While loop               | `{while x < 10 {..}}`         | [Scripting]($scripting/#loops)     |
| Loop control flow        | `{break, continue}`           | [Scripting]($scripting/#loops)     |
| Return from function     | `{return x}`                  | [Function]($function)              |
| Include module           | `{include "bar.typ"}`         | [Scripting]($scripting/#modules)   |
| Import module            | `{import "bar.typ"}`          | [Scripting]($scripting/#modules)   |
| Import items from module | `{import "bar.typ": a, b, c}` | [Scripting]($scripting/#modules)   |
| Comment                  | `{/* block */}`, `{// line}`  | [Below](#comments)                 |

## Comments
Comments are ignored by Typst and will not be included in the output. This is
useful to exclude old versions or to add annotations. To comment out a single
line, start it with `//`:
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
```example
// our data barely supports
// this claim

We show with $p < 0.05$
that the difference is
significant.
```

<<<<<<< HEAD
コメントは `/*` と `*/` で囲むこともできます。この場合、コメントを複数行にわたって書くことができます。

=======
Comments can also be wrapped between `/*` and `*/`. In this case, the comment
can span over multiple lines:
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
```example
Our study design is as follows:
/* Somebody write this up:
   - 1000 participants.
   - 2x2 data design. */
```

<<<<<<< HEAD
## エスケープシーケンス { #escapes }

エスケープシーケンスは、Typstで入力が難しい特殊文字や他に特別な意味を持つ文字を挿入するために使用されます。
文字をエスケープするには、バックスラッシュをその前に置きます。
任意のUnicodeコードポイントを挿入するためには、16進エスケープシーケンス（`[\u{1f600}]`）を使用できます。
このエスケープシーケンスは[文字列]($str)でも機能します。
=======
## Escape sequences { #escapes }
Escape sequences are used to insert special characters that are hard to type or
otherwise have special meaning in Typst. To escape a character, precede it with
a backslash. To insert any Unicode codepoint, you can write a hexadecimal escape
sequence: `[\u{1f600}]`. The same kind of escape sequences also work in
[strings]($str).
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```example
I got an ice cream for
\$1.50! \u{1f600}
```

<<<<<<< HEAD
## 識別子 { #identifiers }
変数名や関数名など(_識別子_)には、
文字、数字、ハイフン（`-`）、アンダースコア（`_`）を使用できます。
先頭は文字またはアンダースコアでなければなりません。

より厳密には、Typstにおける識別子の構文は[Unicode Standard Annex #31](https://www.unicode.org/reports/tr31/)に基づいていますが、
2つの拡張を加えています。
開始文字として`_`を許可すること、
継続文字として`_`と`-`の両方を許可することです。

複数の単語からなる識別子の場合、
推奨される表記規約は[ケバブケース](https://en.wikipedia.org/wiki/Letter_case#Kebab_case)です。
ケバブケースでは、単語は小文字で書かれ、ハイフンで区切られます（例： `top-edge`）。
これは、予測可能性を維持できるため、
他の人が利用するモジュールやパッケージを開発する際に特に重要です。
=======
## Identifiers
Names of variables, functions, and so on (_identifiers_) can contain letters,
numbers, hyphens (`-`), and underscores (`_`). They must start with a letter or
an underscore.

More specifically, the identifier syntax in Typst is based on the
[Unicode Standard Annex #31](https://www.unicode.org/reports/tr31/), with two
extensions: Allowing `_` as a starting character, and allowing both `_` and `-`
as continuing characters.

For multi-word identifiers, the recommended case convention is
[Kebab case](https://en.wikipedia.org/wiki/Letter_case#Kebab_case). In Kebab
case, words are written in lowercase and separated by hyphens (as in
`top-edge`). This is especially relevant when developing modules and packages
for others to use, as it keeps things predictable.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```example
#let kebab-case = [Using hyphen]
#let _schön = "😊"
#let 始料不及 = "😱"
#let π = calc.pi

#kebab-case
#if -π < 0 { _schön } else { 始料不及 }
// -π means -1 * π,
// so it's not a valid identifier
```

<<<<<<< HEAD
## パス { #paths }

Typstは、画像、Typstファイル、データファイルなどのリソース（他ファイル）を参照するためにファイルパスを必要とするさまざまな機能を持ちます。
パスは[文字列型]($str)で指定し、相対パスと絶対パスの2種類があります。

- **相対パス**は、その機能を呼び出すTypstファイルの場所からのパスを指定します。デフォルトはこれです。

=======
## Paths
Typst has various features that require a file path to reference external
resources such as images, Typst files, or data files. Paths are represented as
[strings]($str). There are two kinds of paths: Relative and absolute.

- A **relative path** searches from the location of the Typst file where the
  feature is invoked. It is the default:
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
  ```typ
  #image("images/logo.png")
  ```

<<<<<<< HEAD
- **絶対パス**はプロジェクトの_ルートディレクトリ_からのパスを指定します。先頭を`/`から始めます。

=======
- An **absolute path** searches from the _root_ of the project. It starts with a
  leading `/`:
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
  ```typ
  #image("/assets/logo.png")
  ```

<<<<<<< HEAD
### プロジェクトのルートディレクトリ

デフォルトでは、プロジェクトのルートディレクトリはメインのTypstファイルの親ディレクトリです。
セキュリティ上の理由により、ルートディレクトリの外にあるファイルは読み取れません。

プロジェクトのルートディレクトリとして特定のフォルダーを設定したい場合は、CLIの`--root`フラグを使用できます。
対象のファイルがそのフォルダーのサブディレクトリ内に含まれていることを確認してください。

=======
### Project root
By default, the project root is the parent directory of the main Typst file.
For security reasons, you cannot read any files outside of the root directory.

If you want to set a specific folder as the root of your project, you can use
the CLI's `--root` flag. Make sure that the main file is contained in the
folder's subtree!
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
```bash
typst compile --root .. file.typ
```

<<<<<<< HEAD
Webアプリでは、プロジェクト自体がルートディレクトリとして扱われます。
どのファイルをプレビューしていても（プレビューはファイルパネル内の各Typstファイルの横にある目のアイコンで切り替え可能）、
その中にある全てのファイルを常に読み取れます。

### パスとパッケージ

パッケージは、それ自身のディレクトリ内にあるファイルしか読み込むことができません。
その中では、絶対パスはプロジェクトのルートディレクトリではなく、パッケージのルートディレクトリを指します。
このため、パッケージはプロジェクトディレクトリ内のファイルを直接読み込むことができません。
パッケージがプロジェクトのリソース（例えばロゴ画像など）を必要とする場合は、
あらかじめ読み込んだ画像を`{logo: image("mylogo.svg")}`のように名前付きパラメーターとして渡す必要があります。
その後でも、パッケージ内でsetルールを記述することで画像の見た目をカスタマイズできます。

将来的には、パスが[文字列とは異なるパス専用の型](https://github.com/typst/typst/issues/971)になるかもしれません。
そうすることで、そのパスがどこで生成されたかという情報を保持できるようになります。
これにより、異なるルートからリソースを読み込むことも可能になるでしょう。
=======
In the web app, the project itself is the root directory. You can always read
all files within it, no matter which one is previewed (via the eye toggle next
to each Typst file in the file panel).

### Paths and packages
A package can only load files from its own directory. Within it, absolute paths
point to the package root, rather than the project root. For this reason, it
cannot directly load files from the project directory. If a package needs
resources from the project (such as a logo image), you must pass the already
loaded image, e.g. as a named parameter `{logo: image("mylogo.svg")}`. Note that
you can then still customize the image's appearance with a set rule within the
package.

In the future, paths might become a
[distinct type from strings](https://github.com/typst/typst/issues/971), so that
they can retain knowledge of where they were constructed. This way, resources
could be loaded from a different root.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
