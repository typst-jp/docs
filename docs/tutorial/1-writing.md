---
<<<<<<< HEAD
description: Typstチュートリアル
---
# Typstで執筆するには

さあ始めましょう！あなたが大学で専門的なレポートを書くことになったとしましょう。そこには文章、数式、見出し、図が含まれています。
書き始めるには、まずTypst appで新しいプロジェクトを作成します。エディターに移動すると、2つのパネルが表示されます。
1つは文書を作成するソースパネル、もう1つはレンダリングされた文書が表示されるプレビューパネルです。

![Typst app screenshot](1-writing-app.png)

レポートの良い切り口はすでに考えてあるので、まずは導入を書いてみましょう。
エディターパネルにいくつかのテキストを入力してください。テキストがすぐにプレビューページに表示されるのがわかるでしょう。
=======
description: Typst's tutorial.
---

# Writing in Typst
Let's get started! Suppose you got assigned to write a technical report for
university. It will contain prose, maths, headings, and figures. To get started,
you create a new project on the Typst app. You'll be taken to the editor where
you see two panels: A source panel where you compose your document and a
preview panel where you see the rendered document.

![Typst app screenshot](1-writing-app.png)

You already have a good angle for your report in mind. So let's start by writing
the introduction. Enter some text in the editor panel. You'll notice that the
text immediately appears on the previewed page.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
In this report, we will explore the
various factors that influence fluid
dynamics in glaciers and how they
contribute to the formation and
behaviour of these natural structures.
```

<<<<<<< HEAD
_このチュートリアル全体を通して、このようなコード例を紹介します。アプリと同様に、最初のパネルにはマークアップが含まれ、2番目のパネルにはプレビューが表示されます。何が起こっているかわかりやすいように例にあわせてページを縮小しています。_

次のステップは、見出しを追加して、いくつかのテキストを強調することです。
Typstでは、頻繁に使う書式をシンプルなマークアップで表現するようになっています。見出しを追加するには `=` の文字を入力します。テキストを斜体で強調するには、テキストを `[_アンダースコア_]` で囲みます。
=======
_Throughout this tutorial, we'll show code examples like this one. Just like in the app, the first panel contains markup and the second panel shows a preview. We shrunk the page to fit the examples so you can see what's going on._

The next step is to add a heading and emphasize some text. Typst uses simple
markup for the most common formatting tasks. To add a heading, enter the `=`
character and to emphasize some text with italics, enclose it in
`[_underscores_]`.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
= Introduction
In this report, we will explore the
various factors that influence _fluid
dynamics_ in glaciers and how they
contribute to the formation and
behaviour of these natural structures.
```

<<<<<<< HEAD
簡単でしたね！新しい段落を追加するには、2行のテキストの間に空行を追加するだけです。
その段落に小見出しが必要な場合は、`=` の代わりに `==` を入力して作成します。
`=` の数が見出しのネストレベルを決定します。

次に、氷河の動態に影響を与える要因をいくつか列挙してみましょう。
そのために、ここでは番号付きリストを使いましょう。リストの各項目について、行の先頭に `+` 文字を入力します。
すると、Typstが自動的に項目を番号付けしてくれるのです。
=======
That was easy! To add a new paragraph, just add a blank line in between two
lines of text. If that paragraph needs a subheading, produce it by typing `==`
instead of `=`. The number of `=` characters determines the nesting level of the
heading.

Now we want to list a few of the circumstances that influence glacier dynamics.
To do that, we use a numbered list. For each item of the list, we type a `+`
character at the beginning of the line. Typst will automatically number the
items.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
+ The climate
+ The topography
+ The geology
```

<<<<<<< HEAD
箇条書きリストを追加したい場合は、`+` 文字の代わりに `-` 文字を使用します。
また、リストをネストすることもできます。
例えば、上記のリストの最初の項目にサブリストを追加するには、それをインデントします。
=======
If we wanted to add a bulleted list, we would use the `-` character instead of
the `+` character. We can also nest lists: For example, we can add a sub-list to
the first item of the list above by indenting it.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
+ The climate
  - Temperature
  - Precipitation
+ The topography
+ The geology
```

<<<<<<< HEAD
## 図表を追加する {#figure}

あなたは「レポートに図表を入れるともっとよくなる」と考えているとします。やりましょう。
Typstでは、PNG、JPEG、GIF、SVGの形式の画像をサポートしています。
プロジェクトに画像ファイルを追加するには、まず左サイドバーのボックスアイコンをクリックして _ファイルパネル_ を開きます。
ここにはプロジェクト内の全てのファイルのリストが表示されます。
現在、ここにあるのはあなたが書いているメインのTypstファイルだけです。
別のファイルをアップロードするには、右上隅の矢印のボタンをクリックします。
これによりアップロードダイアログが開き、コンピュータからアップロードするファイルを選択できます。
レポートに用いる画像ファイルを選んでください。

![Upload dialog](1-writing-upload.png)

以前にも見てきたように、Typstでは特定の記号（_マークアップ_ と呼ばれる）が特有の意味を持ちます。
`=`、`-`、`+`、`_` をそれぞれ見出し、リスト、強調テキストを作成するために使用することができます。
しかし、文書に挿入したいもの全てに特別な記号を割り当てると、すぐに分かりづらく、そして扱いづらくなってしまいます。
そのため、Typstでは一般的な書式にのみマークアップ記号を用意し、それ以外は全て _関数_ を使って挿入します。

ページに画像を表示させるためには、Typstの[`image`]($image)関数を使用します。
=======
## Adding a figure { #figure }
You think that your report would benefit from a figure. Let's add one. Typst
supports images in the formats PNG, JPEG, GIF, SVG, PDF, and WebP. To add an
image file to your project, first open the _file panel_ by clicking the box icon
in the left sidebar. Here, you can see a list of all files in your project.
Currently, there is only one: The main Typst file you are writing in. To upload
another file, click the button with the arrow in the top-right corner. This
opens the upload dialog, in which you can pick files to upload from your
computer. Select an image file for your report.

![Upload dialog](1-writing-upload.png)

We have seen before that specific symbols (called _markup_) have specific
meaning in Typst. We can use `=`, `-`, `+`, and `_` to create headings, lists
and emphasized text, respectively. However, having a special symbol for
everything we want to insert into our document would soon become cryptic and
unwieldy. For this reason, Typst reserves markup symbols only for the most
common things. Everything else is inserted with _functions._ For our image to
show up on the page, we use Typst's [`image`] function.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#image("glacier.jpg")
```

<<<<<<< HEAD
一般的に、関数は一連の _引数_ に対して何らかの出力を生成します。
マークアップ内で関数を _呼び出す_ 時は、あなたが関数の引数を指定すると、Typstがその結果（関数の _戻り値_）を文書に挿入してくれます。
今回の場合、`image` 関数は1つの引数として画像ファイルへのパスを受け取ります。
マークアップで関数を呼び出すには、まず `#` 文字を入力し、直後に関数の名前を記述します。
その後、引数を丸括弧で囲みます。
Typstは引数リスト内でさまざまなデータ型を認識します。
私たちの画像のファイルパスは短い [文字列]($str) ですので、二重引用符で囲む必要があります。

挿入された画像はページ全体の幅を使います。これを変更するには、`image` 関数に `width `引数を渡します。
これは _名前付き_ 引数であり、`引数の名前: 引数の値` という形式で指定されます。
複数の引数がある場合はカンマで区切ります。そのため、ここでは先ほど指定したファイルパスの後ろにカンマを付ける必要があります。
=======
In general, a function produces some output for a set of _arguments_. When you
_call_ a function within markup, you provide the arguments and Typst inserts the
result (the function's _return value_) into the document. In our case, the
`image` function takes one argument: The path to the image file. To call a
function in markup, we first need to type the `#` character, immediately
followed by the name of the function. Then, we enclose the arguments in
parentheses. Typst recognizes many different data types within argument lists.
Our file path is a short [string of text]($str), so we need to enclose it in
double quotes.

The inserted image uses the whole width of the page. To change that, pass the
`width` argument to the `image` function. This is a _named_ argument and
therefore specified as a `name: value` pair. If there are multiple arguments,
they are separated by commas, so we first need to put a comma behind the path.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#image("glacier.jpg", width: 70%)
```

<<<<<<< HEAD
`width` 引数は [相対的な長さ]($relative) です。
上の例では、画像がページの幅の `{70%}` を占めるようにパーセンテージを指定しています。
また、`{1cm}` や `{0.7in}` のような絶対値を指定することもできます。

テキストと同様に、画像もデフォルトではページの左側に配置されます。さらに、図表の説明（キャプション）も欠けています。
これらを修正するために、[figure]($figure) 関数を使用しましょう。
この関数には、名前付きでない通常の引数（位置引数）として、図表を指定する必要があります。さらにオプションとして、図表に付ける説明文（`caption`）を名前付き引数で指定できます。

`figure` 関数の引数リスト内では、Typstは既にコードモードになっています。
これは、 `image` 関数の呼び出し前にある `#` 記号を削除する必要があることを意味します。
`#` 記号は、マークアップ内でテキストと関数呼び出しを区別するために書くものなのです。

キャプションの中には、任意のマークアップを含めることが出来ます。
ある関数の引数としてマークアップを指定するためには、それを角括弧 `[ ]` で囲みます。この「マークアップが角括弧で囲まれている構造」のことを、_コンテンツブロック_ と呼ばれます
=======
The `width` argument is a [relative length]($relative). In our case, we
specified a percentage, determining that the image shall take up `{70%}` of the
page's width. We also could have specified an absolute value like `{1cm}` or
`{0.7in}`.

Just like text, the image is now aligned at the left side of the page by
default. It's also lacking a caption. Let's fix that by using the [figure]
function. This function takes the figure's contents as a positional argument and
an optional caption as a named argument.

Within the argument list of the `figure` function, Typst is already in code
mode. This means, you now have to remove the hash before the image function call.
The hash is only needed directly in markup (to disambiguate text from function
calls).

The caption consists of arbitrary markup. To give markup to a function, we
enclose it in square brackets. This construct is called a _content block._
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#figure(
  image("glacier.jpg", width: 70%),
  caption: [
    _Glaciers_ form an important part
    of the earth's climate system.
  ],
)
```

<<<<<<< HEAD
あなたはレポートの執筆を続けるうちに、今度は先ほど挿入した図を文中から参照したくなったとします。
その場合、まず図にラベルを付けます。
ラベルとは、文書内の要素を一意に識別するための名前のことです。先ほど挿入した図の後ろに、その図のラベルを山括弧 `< >` で囲んで書き加えます。
これで、テキスト内で `[@]` 記号を書いた後ろにラベル名を指定すると、その図を参照できるようになりました。
見出しや方程式もラベルを付けて参照可能にすることができます。
=======
You continue to write your report and now want to reference the figure. To do
that, first attach a label to figure. A label uniquely identifies an element in
your document. Add one after the figure by enclosing some name in angle
brackets. You can then reference the figure in your text by writing an `[@]`
symbol followed by that name. Headings and equations can also be labelled to
make them referenceable.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
Glaciers as the one shown in
@glaciers will cease to exist if
we don't take action soon!

#figure(
  image("glacier.jpg", width: 70%),
  caption: [
    _Glaciers_ form an important part
    of the earth's climate system.
  ],
) <glaciers>
```

<div class="info-box">

<<<<<<< HEAD
これまでに、コンテンツブロック（角括弧 `[ ]` 内のマークアップ）と文字列（二重引用符 `" "` 内のテキスト）を関数に渡してきました。
どちらもテキストを含んでいるように見えますが、違いは何でしょうか？

コンテンツブロックはテキストを含むことができますが、それ以外にもさまざまなマークアップ、関数呼び出しなどを含むことができます。
一方、文字列は本当に _文字の並び_ に過ぎません。

例えば、image関数は、引数として画像ファイルへのパスが渡されることを想定しています。ここに文章の段落や他の画像を渡しても意味がありません。
image関数の引数としてマークアップがダメで文字列が許可されるのは、そういうわけなのです。
それとは反対に、文字列はコンテンツブロックが期待される場所であればどこにでも書くことが出来ます。なぜなら、文字列は単なる文字の並びであり、文字の並びは有効なコンテンツの一種だからです。

</div>

## 参考文献の追加 {#bibliography}

レポートを作成する際には、その主張を裏付ける必要がありますよね。
参考文献を文書に追加するには、[`bibliography`]($bibliography) 関数を使用できます。
この関数は、引数として参考文献ファイルへのパスが渡されることを想定しています。

Typstでは、ネイティブな参考文献の形式として[Hayagriva](https://github.com/typst/hayagriva/blob/main/docs/file-format.md)を使用していますが、
互換性のためにBibLaTeXファイルも使用できます。
クラスメートが既に文献調査を行い、`.bib` ファイルを送ってくれたので、それを使用しましょう。
ファイルパネルを通じてファイルをアップロードし、Typst appでアクセスできるようにします。

文書に参考文献が追加されている場合、参考文献欄にある文献を文中で引用することができます。
引用はラベルへの参照と同じ構文を使用します。デフォルトでは、文中に文献の引用を記述した時点で初めて、その文献がTypstの参考文献セクションに表示されるようになっています。
Typstはさまざまな引用および参考文献のスタイルをサポートしています。詳細については [リファレンス]($bibliography.style)を参照してください。
=======
So far, we've passed content blocks (markup in square brackets) and strings
(text in double quotes) to our functions. Both seem to contain text. What's the
difference?

A content block can contain text, but also any other kind of markup, function
calls, and more, whereas a string is really just a _sequence of characters_ and
nothing else.

For example, the image function expects a path to an image file.
It would not make sense to pass, e.g., a paragraph of text or another image as
the image's path parameter. That's why only strings are allowed here.
In contrast, strings work wherever content is expected because text is a
valid kind of content.
</div>

## Adding a bibliography { #bibliography }
As you write up your report, you need to back up some of your claims. You can
add a bibliography to your document with the [`bibliography`] function. This
function expects a path to a bibliography file.

Typst's native bibliography format is
[Hayagriva](https://github.com/typst/hayagriva/blob/main/docs/file-format.md),
but for compatibility you can also use BibLaTeX files. As your classmate has
already done a literature survey and sent you a `.bib` file, you'll use that
one. Upload the file through the file panel to access it in Typst.

Once the document contains a bibliography, you can start citing from it.
Citations use the same syntax as references to a label. As soon as you cite a
source for the first time, it will appear in the bibliography section of your
document. Typst supports different citation and bibliography styles. Consult the
[reference]($bibliography.style) for more details.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
= Methods
We follow the glacier melting models
established in @glacier-melt.

#bibliography("works.bib")
```

<<<<<<< HEAD
## 数式 {#maths}

方法に関する節を肉付けした後、文書の主要な部分である方程式に進みます。
Typstには組み込みの数学記法があり、独自の数学表記を使用します。
簡単な方程式から始めましょう。Typstに数学的な表現を期待することを知らせるために、`[$]` 記号で囲みます。
=======
## Maths
After fleshing out the methods section, you move on to the meat of the document:
Your equations. Typst has built-in mathematical typesetting and uses its own
math notation. Let's start with a simple equation. We wrap it in `[$]` signs
to let Typst know it should expect a mathematical expression:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
The equation $Q = rho A v + C$
defines the glacial flow rate.
```

<<<<<<< HEAD
方程式はインラインで表示され、周囲のテキストと同じ行に配置されます。
それを独立した行にしたい場合は、方程式の最初と最後にそれぞれ1つずつスペースを挿入する必要があります。
=======
The equation is typeset inline, on the same line as the surrounding text. If you
want to have it on its own line instead, you should insert a single space at its
start and end:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
The flow rate of a glacier is
defined by the following equation:

$ Q = rho A v + C $
```

<<<<<<< HEAD
Typstでは、単一の文字 `Q`, `A`, `v`, `C` はそのまま表示され、一方で `rho` はギリシャ文字に変換されているのがわかります。
数式モードでは、単一の文字は常にそのまま表示されます。しかし、複数個が連なっている文字は記号、変数、または関数名として扱われます。
異なる種類の文字どうしの乗算を（乗算記号を省略して）示すためには、文字と文字の間にスペースを挿入してください。

複数の文字からなる変数を表したい場合は、変数の名前を引用符で囲みます。
=======
We can see that Typst displayed the single letters `Q`, `A`, `v`, and `C` as-is,
while it translated `rho` into a Greek letter. Math mode will always show single
letters verbatim. Multiple letters, however, are interpreted as symbols,
variables, or function names. To imply a multiplication between single letters,
put spaces between them.

If you want to have a variable that consists of multiple letters, you can
enclose it in quotes:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
The flow rate of a glacier is given
by the following equation:

$ Q = rho A v + "time offset" $
```

<<<<<<< HEAD
レポートには総和の式も必要です。
`sum` 記号を使用して、総和の範囲を下付き文字と上付き文字で指定することができます。
=======
You'll also need a sum formula in your paper. We can use the `sum` symbol and
then specify the range of the summation in sub- and superscripts:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
Total displaced soil by glacial flow:

$ 7.32 beta +
  sum_(i=0)^nabla Q_i / 2 $
```

<<<<<<< HEAD
シンボルや変数に下付き文字を追加するには、`_` の文字を入力してから下付き文字を入力します。
同様に、上付き文字を追加するには `^` の文字を使用します。
もし下付き文字や上付き文字が複数の要素からなる場合は、それらを丸括弧で囲む必要があります。

上記の例から分数の挿入方法もわかると思います。
分子と分母の間に `/` の文字を置くだけで、Typstは自動的にそれを分数に変換します。
Typstでは、丸括弧のネストをスマートに解決するようになっています。プログラミング言語や関数電卓のように、丸括弧を入れ子にした式を入力すると、
Typstは丸括弧で囲まれた部分式を適切に解釈して自動的に置き換えます。
=======
To add a subscript to a symbol or variable, type a `_` character and then the
subscript. Similarly, use the `^` character for a superscript. If your
sub- or superscript consists of multiple things, you must enclose them
in round parentheses.

The above example also showed us how to insert fractions: Simply put a `/`
character between the numerator and the denominator and Typst will automatically
turn it into a fraction. Parentheses are smartly resolved, so you can enter your
expression as you would into a calculator and Typst will replace parenthesized
sub-expressions with the appropriate notation.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
Total displaced soil by glacial flow:

$ 7.32 beta +
  sum_(i=0)^nabla
    (Q_i (a_i - epsilon)) / 2 $
```

<<<<<<< HEAD
数学の全ての概念に特別な構文があるわけではありません。
代わりに、先程の `image` 関数のように関数を使用します。
例えば、列ベクトルを挿入するには、`vec` 関数を使用できます。
数式モード内では、関数呼び出しは `#` で始める必要はありません。
=======
Not all math constructs have special syntax. Instead, we use functions, just
like the `image` function we have seen before. For example, to insert a column
vector, we can use the [`vec`]($math.vec) function. Within math mode, function
calls don't need to start with the `#` character.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
$ v := vec(x_1, x_2, x_3) $
```

<<<<<<< HEAD
数式モード内でのみ使用可能な関数もあります。
例えば、[`cal`]($math.cal) 関数は集合などに一般的に使用されるカリグラフィ文字を表示するために使われます。
数式モードが提供する全ての関数の完全なリストについては、[リファレンスの数式セクション]($category/math)を参照してください。

もう1つ、矢印などの多くの記号には多くのバリエーションがあります。
こうしたさまざまなバリエーションの中から特定の記号を選択するには、その記号のカテゴリ名の後に、ドットと具体的な記号の種類を示す修飾子を追加します。
=======
Some functions are only available within math mode. For example, the
[`cal`]($math.cal) function is used to typeset calligraphic letters commonly
used for sets. The [math section of the reference]($category/math) provides a
complete list of all functions that math mode makes available.

One more thing: Many symbols, such as the arrow, have a lot of variants. You can
select among these variants by appending a dot and a modifier name to a symbol's
name:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
$ a arrow.squiggly b $
```

<<<<<<< HEAD
この表記法はマークアップモードでも利用可能ですが、そこでは記号名の前に `#sym.` を付ける必要があります。
利用可能な全ての記号については[記号セクション]($category/symbols/sym)を参照してください。

## まとめ {#review}

あなたはTypstで基本的な文書を書く方法を学びました。テキストの強調やリストの書き方、画像の挿入、コンテンツの配置、Typstにおける数学的な式の組版などを学びました。また、Typstの関数についても学びました。
Typstでは文書に挿入できるさまざまなコンテンツがあります。例えば、[表]($table)や[図形]($category/visualize)、[コードブロック]($raw)などです。さらにこれらや他の機能について詳しく学ぶには[リファレンス]($reference)を参照してください。

ここまでで、レポートの執筆は完了しました。
あなたは右上のダウンロードボタンをクリックしてPDFを保存したはずです。
しかし、あなたはレポートがあまりにも素朴に見えると感じるかもしれません。
次のセクションでは、文書の外観をカスタマイズする方法を学びます。
=======
This notation is also available in markup mode, but the symbol name must be
preceded with `#sym.` there. See the [symbols section]($category/symbols/sym)
for a list of all available symbols.

## Review
You have now seen how to write a basic document in Typst. You learned how to
emphasize text, write lists, insert images, align content, and typeset
mathematical expressions. You also learned about Typst's functions. There are
many more kinds of content that Typst lets you insert into your document, such
as [tables]($table), [shapes]($category/visualize), and [code blocks]($raw). You
can peruse the [reference] to learn more about these and other features.

For the moment, you have completed writing your report. You have already saved a
PDF by clicking on the download button in the top right corner. However, you
think the report could look a bit less plain. In the next section, we'll learn
how to customize the look of our document.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
