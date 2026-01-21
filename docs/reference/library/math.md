<<<<<<< HEAD
Typstには数式を組版するための特別な[構文]($syntax/#math)とライブラリ関数があります。
数式は、テキスト中にインラインで、あるいは独立したブロックのいずれかで表示できます。
始まりと終わりに少なくとも1つの空白がある場合（例えば、`[$ x^2 $]`）はブロックとして組版されます。

# 変数
数式モードでは1文字は常にそのまま表示されます。
しかし、複数の文字は変数か関数として解釈されます。
複数の文字を文字通りに表示するには引用符で囲んでください。
1文字の変数にアクセスしたい場合は[ハッシュ構文]($scripting/#expressions)を使用してください。
=======
Typst has special [syntax]($syntax/#math) and library functions to typeset
mathematical formulas. Math formulas can be displayed inline with text or as
separate blocks. They will be typeset into their own block if they start and end
with at least one space (e.g. `[$ x^2 $]`).

# Variables
In math, single letters are always displayed as is. Multiple letters, however,
are interpreted as variables and functions. To display multiple letters
verbatim, you can place them into quotes and to access single letter variables,
you can use the [hash syntax]($scripting/#expressions).
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
$ A = pi r^2 $
$ "area" = pi dot "radius"^2 $
$ cal(A) :=
    { x in RR | x "is natural" } $
#let x = 5
$ #x < 17 $
```

<<<<<<< HEAD
# 記号
数式モードでは`pi`、`dot`、`RR`などのさまざまな[記号]($category/symbols/sym)が利用可能です。
多くの数式記号ではバリアントが利用可能です。
記号に[修飾子]($symbol)を適用することで異なるバリアントを選択できます。
Typstはさらに`=>`のような、記号を近似するいくつかの省略記法を認識します。
そのような省略記法が存在する場合、記号ドキュメントのリストに記載されています。
=======
# Symbols
Math mode makes a wide selection of [symbols]($category/symbols/sym) like `pi`,
`dot`, or `RR` available. Many mathematical symbols are available in different
variants. You can select between different variants by applying
[modifiers]($symbol) to the symbol. Typst further recognizes a number of
shorthand sequences like `=>` that approximate a symbol. When such a shorthand
exists, the symbol's documentation lists it.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
$ x < y => x gt.eq.not y $
```

<<<<<<< HEAD
# 改行
数式には改行を含めることもできます。
各行には、その場所で配置することを指定する、1つまたは複数の _配置点_（`&`）を含めることができます。
=======
# Line Breaks
Formulas can also contain line breaks. Each line can contain one or multiple
_alignment points_ (`&`) which are then aligned.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
$ sum_(k=0)^n k
    &= 1 + ... + n \
    &= (n(n+1)) / 2 $
```

<<<<<<< HEAD
# 関数呼び出し
数式モードはハッシュプレフィックスを用いない特別な関数呼び出しをサポートしています。
このような「数式呼び出し」内では、引数リストは以下の通りコード中とは少し異なる動作をします。

- その中では、Typstは依然として「数式モード」です。
したがって、その中で数式を直接書くことができますが、コード式を渡すには（数式構文で利用可能なstringを除いて）ハッシュ構文を使う必要があります。
- 位置引数、名前付き引数、引数展開をサポートしています。
- 後続のコンテンツブロックはサポートされていません。
- 2次元引数リストのための追加の構文があります。
セミコロン（`;`）はその前のカンマ区切りの引数を配列引数へとマージします。
=======
# Function calls
Math mode supports special function calls without the hash prefix. In these
"math calls", the argument list works a little differently than in code:

- Within them, Typst is still in "math mode". Thus, you can write math directly
  into them, but need to use hash syntax to pass code expressions (except for
  strings, which are available in the math syntax).
- They support positional and named arguments, as well as argument spreading.
- They don't support trailing content blocks.
- They provide additional syntax for 2-dimensional argument lists. The semicolon
  (`;`) merges preceding arguments separated by commas into an array argument.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
$ frac(a^2, 2) $
$ vec(1, 2, delim: "[") $
$ mat(1, 2; 3, 4) $
$ mat(..#range(1, 5).chunks(2)) $
$ lim_x =
    op("lim", limits: #true)_x $
```

<<<<<<< HEAD
数式呼び出しでカンマやセミコロンをそのまま書きたい場合は、バックスラッシュを用いてエスケープしてください。
一方、コロンは識別子の直後にあるときにのみ特別な方法で認識されるため、そのまま表示したい場合は単にその前に空白を挿入してください。

前にハッシュがある関数呼び出しは通常のコードの関数呼び出しで、これらの規則に影響されません。

# 配置
数式が複数の _配置点_（`&`）を含む場合、右揃えと左揃えが交互に行われる列のブロックが作成されます。
以下の例では、`(3x + y) / 7`という式は右揃えで、`= 9`が左揃えです。
"given"という単語も左揃えです。これは`&&`が行内に2つの配置点を作成し、2回配置方法が切り替わるためです。
`& &`と`&&`は全く同じように振る舞います。
一方、"multiply by 7"は、その前に`&`がただ1つあるため右揃えです。
各配置点は単に右揃えと左揃えを交互に切り替えます。
=======
To write a verbatim comma or semicolon in a math call, escape it with a
backslash. The colon on the other hand is only recognized in a special way if
directly preceded by an identifier, so to display it verbatim in those cases,
you can just insert a space before it.

Functions calls preceded by a hash are normal code function calls and not
affected by these rules.

# Alignment
When equations include multiple _alignment points_ (`&`), this creates blocks of
alternatingly right- and left-aligned columns. In the example below, the
expression `(3x + y) / 7` is right-aligned and `= 9` is left-aligned. The word
"given" is also left-aligned because `&&` creates two alignment points in a row,
alternating the alignment twice. `& &` and `&&` behave exactly the same way.
Meanwhile, "multiply by 7" is right-aligned because just one `&` precedes it.
Each alignment point simply alternates between right-aligned/left-aligned.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
$ (3x + y) / 7 &= 9 && "given" \
  3x + y &= 63 & "multiply by 7" \
  3x &= 63 - y && "subtract y" \
  x &= 21 - y/3 & "divide by 3" $
```

<<<<<<< HEAD
# 数式フォント
以下に示すように、[show-setルール]($styling/#show-rules)を用いて数式フォントの設定が可能です。
数式組版に適しているのは一部の特別なOpenType数式フォントのみであることに注意してください。
=======
# Math fonts
You can set the math font by with a [show-set rule]($styling/#show-rules) as
demonstrated below. Note that only special OpenType math fonts are suitable for
typesetting maths.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#show math.equation: set text(font: "Fira Math")
$ sum_(i in NN) 1 + i $
```

<<<<<<< HEAD
# mathモジュール
全ての数学関数は、数式中ではデフォルトで利用できる`math`[モジュール]($scripting/#modules)に属しています。
数式外では、`math.`プレフィックスを付けるとアクセスできます。
=======
# Math module
All math functions are part of the `math` [module]($scripting/#modules), which
is available by default in equations. Outside of equations, they can be accessed
with the `math.` prefix.

# Accessibility
To make math accessible, you must provide alternative descriptions of equations
in natural language using the [`alt` parameter of
`math.equation`]($math.equation.alt). For more information, see the [Textual
Representations section of the Accessibility
Guide]($guides/accessibility/#textual-representations).

```example
#math.equation(
  alt: "d S equals delta q divided by T",
  $ d "S" = (delta q) / T $,
)
```

In the future, Typst will automatically make equations without alternative
descriptions accessible in HTML and PDF 2.0 export.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
