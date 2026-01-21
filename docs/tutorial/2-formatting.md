---
<<<<<<< HEAD
description: Typstチュートリアル
---

# 書式を設定する
前章までで、あなたはいくつかのテキスト、少しの数式や画像を含むレポートを書いてきました。
しかし、その見た目はまだとても地味です。
ティーチングアシスタントはあなたが新しい組版システムを使っていることをまだ知らず、あなたは自身のレポートを他の学生の提出物に合わせたいと思うでしょう。
この章では、Typstの組版システムを使ってレポートの体裁を整える方法を示します。

## setルール { #set-rule }
前章で見たように、Typstにはコンテンツを _挿入する_ 関数（例：[`image`]関数）と、引数として受け取ったコンテンツを*操作する*関数（例：[`align`]関数）があります。
例えば、フォントを変更したいとき、最初に思いつくことは、それを行う関数を探して、その関数で文書全体を囲むことでしょう。
=======
description: Typst's tutorial.
---

# Formatting
So far, you have written a report with some text, a few equations and images.
However, it still looks very plain. Your teaching assistant does not yet know
that you are using a new typesetting system, and you want your report to fit in
with the other student's submissions. In this chapter, we will see how to format
your report using Typst's styling system.

## Set rules
As we have seen in the previous chapter, Typst has functions that _insert_
content (e.g. the [`image`] function) and others that _manipulate_ content that
they received as arguments (e.g. the [`align`] function). The first impulse you
might have when you want, for example, to change the font, could be to look
for a function that does that and wrap the complete document in it.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#text(font: "New Computer Modern")[
  = Background
  In the case of glaciers, fluid
  dynamics principles can be used
  to understand how the movement
  and behaviour of the ice is
  influenced by factors such as
  temperature, pressure, and the
  presence of other fluids (such as
  water).
]
```

<<<<<<< HEAD
ここで、関数の全ての引数は括弧の中で指定されるべきではないか？と思うでしょう。
なぜ括弧の _後_ にコンテンツを記述する2つ目の角括弧があるのでしょうか？
答えは、関数にコンテンツを渡すことはTypstではよくあるため、特別な構文があるからです。
コンテンツを引数リストの中に入れる代わりに、通常の引数の後に角括弧内でコンテンツを直接書くことが可能であり、これによりカンマ区切りを減らすことができます。

上で見たように、これは正しく動作します。[`text`]関数を使えば、その範囲内の全てのテキストのフォントを調整できます。しかし、無数の関数で文書を囲み、選択的に各場所でスタイルを適用しようとするとすぐに面倒になります。

幸いなことに、Typstにはもっと簡潔な記法があります。
_setルール_ を使えば、以後現れる全てのコンテンツに対してスタイル設定を適用可能です。
`{set}`キーワードを入力し、その後に設定したい関数の名前と引数のリストを括弧で囲んでsetルールを記述します。
=======
Wait, shouldn't all arguments of a function be specified within parentheses? Why
is there a second set of square brackets with content _after_ the parentheses?
The answer is that, as passing content to a function is such a common thing to
do in Typst, there is special syntax for it: Instead of putting the content
inside of the argument list, you can write it in square brackets directly after
the normal arguments, saving on punctuation.

As seen above, that works. With the [`text`] function, we can adjust the font
for all text within it. However, wrapping the document in countless functions
and applying styles selectively and in-situ can quickly become cumbersome.

Fortunately, Typst has a more elegant solution. With _set rules,_ you can apply
style properties to all occurrences of some kind of content. You write a set
rule by entering the `{set}` keyword, followed by the name of the function whose
properties you want to set, and a list of arguments in parentheses.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#set text(
  font: "New Computer Modern"
)

= Background
In the case of glaciers, fluid
dynamics principles can be used
to understand how the movement
and behaviour of the ice is
influenced by factors such as
temperature, pressure, and the
presence of other fluids (such as
water).
```

<div class="info-box">

<<<<<<< HEAD
ここで起こっていることをより専門的な用語で説明すると、
setルールでは、ある関数のパラメーターに、それ以降にその関数を使うときのデフォルト値を設定しています。
</div>

## 補完パネル { #autocomplete }
Typst appを使用していると、`#`を入力した後に、使用可能な関数と引数リスト内で使用可能なパラメーターを表示するパネルがポップアップ表示されることに気づいたかもしれません。
これは補完パネルといい、文書を書いているときにとても便利な機能です。
矢印キーで入力したい補完項目に移動し、Returnキーを押せば補完入力されます。
パネルはEscapeキーを押すことで解除でき、`#`とタイプするか、<kbd>Ctrl</kbd> + <kbd>Space</kbd>キーを押すことで再び開くことができます。
補完パネルを使って関数の正しい引数を見つけましょう。
ほとんどの補完候補には、その項目が何をするかについての小さな説明がついています。

![Autocomplete panel](2-formatting-autocomplete.png)

## ページの設定 { #page-setup }
setルールの説明に戻ります。
setルールを書くときは、スタイルを設定したい要素の種類に応じて関数を選択します。
以下は、setルールでよく使われる関数のリストです。

- [`text`]($text) フォントの種類、大きさ、色などのテキストのプロパティを設定
- [`page`]($page) ページサイズ、余白（マージン）、ヘッダー、段組み、フッターを設定
- [`par`]($par) 段落の両端揃え、行間の幅など
- [`heading`]($heading) 見出しの見た目や番号付
- [`document`]($document) タイトルや著者情報などPDF出力に含まれるメタデータ

関数のパラメーターは全て設定できるわけではありません。
一般的に、関数のパラメーターを設定できるのは、関数に _どのように_ 実行させるかを指示するパラメーターだけであり、関数に _何を_ 実行させるかを指示するパラメーターは設定できません。
関数のリファレンスページには、設定可能なパラメーターが示されています。

文書にもう少しスタイルを追加してみましょう。
余白を大きくし、セリフ体のフォントを使用します。
この例では、ページサイズも設定します。
=======
Want to know in more technical terms what is happening here?

Set rules can be conceptualized as setting default values
for some of the parameters of a function for all future
uses of that function.
</div>

## The autocomplete panel { #autocomplete }
If you followed along and tried a few things in the app, you might have noticed
that always after you enter a `#` character, a panel pops up to show you the
available functions, and, within an argument list, the available parameters.
That's the autocomplete panel. It can be very useful while you are writing your
document: You can apply its suggestions by hitting the Return key or navigate to
the desired completion with the arrow keys. The panel can be dismissed by
hitting the Escape key and opened again by typing `#` or hitting
<kbd>Ctrl</kbd> + <kbd>Space</kbd>. Use the autocomplete panel to discover the
right arguments for functions. Most suggestions come with a small description of
what they do.

![Autocomplete panel](2-formatting-autocomplete.png)

## Set up the page { #page-setup }
Back to set rules: When writing a rule, you choose the function depending on
what type of element you want to style. Here is a list of some functions that
are commonly used in set rules:

- [`text`] to set font family, size, color, and other properties of text
- [`page`] to set the page size, margins, headers, enable columns, and footers
- [`par`] to justify paragraphs, set line spacing, and more
- [`heading`] to set the appearance of headings and enable numbering
- [`document`] to set the metadata contained in the PDF output, such as title
  and author

Not all function parameters can be set. In general, only parameters that tell
a function _how_ to do something can be set, not those that tell it _what_ to
do it with. The function reference pages indicate which parameters are settable.

Let's add a few more styles to our document. We want larger margins and a serif
font. For the purposes of the example, we'll also set another page size.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#set page(
  paper: "a6",
  margin: (x: 1.8cm, y: 1.5cm),
)
#set text(
  font: "New Computer Modern",
  size: 10pt
)
#set par(
  justify: true,
  leading: 0.52em,
)

= Introduction
In this report, we will explore the
various factors that influence fluid
dynamics in glaciers and how they
contribute to the formation and
behaviour of these natural structures.

>>> Glacier displacement is influenced
>>> by a number of factors, including
>>> + The climate
>>> + The topography
>>> + The geology
>>>
>>> This report will present a physical
>>> model of glacier displacement and
>>> dynamics, and will explore the
>>> influence of these factors on the
>>> movement of large bodies of ice.
<<< ...

#align(center + bottom)[
  #image("glacier.jpg", width: 70%)

  *Glaciers form an important
  part of the earth's climate
  system.*
]
```

<<<<<<< HEAD
ここで注目していただきたい点を以下に記載します。

まず、[`page`]($page) setルールです。
これはページサイズと余白の2つの引数を受け取ります。
ページサイズは文字列であり、Typstは[多くの標準ページサイズ]($page.paper)を受け付けますが、カスタムページサイズを指定することもできます。
余白は[辞書型]($dictionary)で指定します。辞書型とはキーと値のペアの集まりです。
この場合、キーは`x`と`y`で、値はそれぞれ水平マージンと垂直マージンです。
`{left}`, `{right}`, `{top}`, and `{bottom}`をキーとする辞書を渡すことで、各辺に別々の余白を指定することもできます。

つぎに[`text`]($text) setルールです。
ここでは、フォントサイズを`{10pt}`、フォントの種類を `{"New Computer Modern"}`に設定します。
Typst appには多くのフォントが用意されております。
text関数の引数リストにいるとき、補完パネルで利用可能なフォントを探すことができます。

また、行間の幅（leading）も設定しました。
これは[length]($length)の値として指定され、`em`という単位を使ってフォントのサイズに対する行間を指定しています。
`{1em}`は現在のフォントサイズ（デフォルトは`{11pt}`）に相当します。

最後に、中央揃えに垂直配置を追加して、画像をページの下部に配置しました。
垂直配置と水平配置を `{+}` 演算子で組み合わせることで2次元配置を指定できます。

## 洗練のヒント { #sophistication }
文書をより明確に構成するために、今度は見出しに番号を付けたいと思います。
これを行うには、[`heading`]関数の`numbering`パラメーターを設定します。
=======
There are a few things of note here.

First is the [`page`] set rule. It receives two arguments: the page size and
margins for the page. The page size is a string. Typst accepts [many standard
page sizes,]($page.paper) but you can also specify a custom page size. The
margins are specified as a [dictionary.]($dictionary) Dictionaries are a
collection of key-value pairs. In this case, the keys are `x` and `y`, and the
values are the horizontal and vertical margins, respectively. We could also have
specified separate margins for each side by passing a dictionary with the keys
`{left}`, `{right}`, `{top}`, and `{bottom}`.

Next is the set [`text`] set rule. Here, we set the font size to `{10pt}` and
font family to `{"New Computer Modern"}`. The Typst app comes with many fonts
that you can try for your document. When you are in the text function's argument
list, you can discover the available fonts in the autocomplete panel.

We have also set the spacing between lines (a.k.a. leading): It is specified as
a [length] value, and we used the `em` unit to specify the leading relative to
the size of the font: `{1em}` is equivalent to the current font size (which
defaults to `{11pt}`).

Finally, we have bottom aligned our image by adding a vertical alignment to our
center alignment. Vertical and horizontal alignments can be combined with the
`{+}` operator to yield a 2D alignment.

## A hint of sophistication { #sophistication }
To structure our document more clearly, we now want to number our headings. We
can do this by setting the `numbering` parameter of the [`heading`] function.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
>>> #set text(font: "New Computer Modern")
#set heading(numbering: "1.")

= Introduction
#lorem(10)

== Background
#lorem(12)

== Methods
#lorem(15)
```

<<<<<<< HEAD
番号付けのパラメーターとして文字列 `{「1.」}` を指定しました。
これは、見出しにアラビア数字で番号を付け、各レベルの番号の間にドットを置くようにTypstに指示します。
見出しに[文字、ローマ数字、記号]($numbering)を使用することも可能です。
=======
We specified the string `{"1."}` as the numbering parameter. This tells Typst to
number the headings with arabic numerals and to put a dot between the number of
each level. We can also use [letters, roman numerals, and symbols]($numbering)
for our headings:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
>>> #set text(font: "New Computer Modern")
#set heading(numbering: "1.a")

= Introduction
#lorem(10)

== Background
#lorem(12)

== Methods
#lorem(15)
```

<<<<<<< HEAD
この例では、[`lorem`]関数を使って仮テキストを生成しています。
この関数は引数に数値を取り、その単語数の _Lorem Ipsum_ テキストを生成します。

<div class="info-box">

headingとtextのsetルールが、それぞれの関数で作成されていなくても、全ての見出しと文章に適用されることを不思議に思いませんでしたか？

Typstは内部的に`[= Conclusion]`と書くたびに`heading`関数を呼び出します。
実際に、関数呼び出し `[#heading[Conclusion]]` は上記の見出しマークアップと同等です。
他のマークアップ要素も同様に機能し、対応する関数呼び出しのための _シンタックスシュガー_ に過ぎません。

</div>

## showルール { #show-rule }
書式設定によりレポートの出来栄えにかなり満足してきましたが、最後にひとつだけ修正が必要な点があります。
あなたが書いているレポートは、より大きなプロジェクトのためのものであり、そのプロジェクト名には、たとえ散文であっても、常にロゴを添えるべきでしょう。

1つの方法として、検索と置換を使ってロゴを添える全ての場所に`[#image("logo.svg")]`の呼び出しを追加することもできますが、それはとても面倒です。
別の方法として、[カスタム関数を定義する]($function/#defining-functions)ことで、常にロゴを画像として生成することもできます。
しかし、これらよりももっと簡単な方法があります。

showルールを使用すると、Typstが特定の要素をどのように表示するかを再定義できます。
これにより、Typstがどの要素をどのように表示するかを指定します。
Showルールはテキストのインスタンスや多くの関数、さらには文書全体にも適用可能です。
=======
This example also uses the [`lorem`] function to generate some placeholder text.
This function takes a number as an argument and generates that many words of
_Lorem Ipsum_ text.

<div class="info-box">

Did you wonder why the headings and text set rules apply to all text and headings,
even if they are not produced with the respective functions?

Typst internally calls the `heading` function every time you write
`[= Conclusion]`. In fact, the function call `[#heading[Conclusion]]` is
equivalent to the heading markup above. Other markup elements work similarly,
they are only _syntax sugar_ for the corresponding function calls.
</div>

## Show rules
You are already pretty happy with how this turned out. But one last thing needs
to be fixed: The report you are writing is intended for a larger project and
that project's name should always be accompanied by a logo, even in prose.

You consider your options. You could add an `[#image("logo.svg")]` call before
every instance of the logo using search and replace. That sounds very tedious.
Instead, you could maybe
[define a custom function]($function/#defining-functions) that always yields the
logo with its image. However, there is an even easier way:

With show rules, you can redefine how Typst displays certain elements. You
specify which elements Typst should show differently and how they should look.
Show rules can be applied to instances of text, many functions, and even the
whole document.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

```example
#show "ArtosFlow": name => box[
  #box(image(
    "logo.svg",
    height: 0.7em,
  ))
  #name
]

This report is embedded in the
ArtosFlow project. ArtosFlow is a
project of the Artos Institute.
```

<<<<<<< HEAD
この例には新しい構文がたくさんあります。
ここでは、`{show}`キーワードを記述し、その後に表示させたいテキストの文字列とコロンを記述しています。
そして、表示したいコンテンツを引数として受け取る関数を書いています。
ここでは、その引数を`name`と定義しました。
これで、ArtosFlowの名前を表示するために、関数本体で変数`name`を使えます。
このshowルールでは、名前の前にロゴ画像を追加し、ロゴと名前の間に改行が入らないように、結果をboxの中に入れます。
画像もboxの中に入れることで、画像が段落として表示されないようにしています。

最初のbox関数とimage関数の呼び出しは、マークアップに直接埋め込まれていないため、先頭の`#`は必要ありませんでした。
Typstがマークアップの代わりにコードを期待している場合、関数、キーワード、変数を使用する際に、先頭の`#`は必要ありません。
この事象は、パラメーターリスト、関数定義、[コードブロック]($scripting)で見られます。

## まとめ { #review }
Typst文書に基本的な書式を適用する方法を分かりいただけたと思います。
setルールを用いて、フォントを設定し、段落の両端を揃え、ページ寸法を変更し、見出しに番号を追加する方法を学びました。
また、基本的なshowルールを使用して、文書全体のテキストの表示方法を変更する方法も学びました。

ここで学んだ方法で作成したレポートを提出すると、あなたの指導教員はそれをとても気に入り、学会用の論文に仕立てたいと言うでしょう！
次章では、より高度なshowルールと関数を使用して、文書を論文としてフォーマットする方法を学びます。
=======
There is a lot of new syntax in this example: We write the `{show}` keyword,
followed by a string of text we want to show differently and a colon. Then, we
write a function that takes the content that shall be shown as an argument.
Here, we called that argument `name`. We can now use the `name` variable in the
function's body to print the ArtosFlow name. Our show rule adds the logo image
in front of the name and puts the result into a box to prevent linebreaks from
occurring between logo and name. The image is also put inside of a box, so that
it does not appear in its own paragraph.

The calls to the first box function and the image function did not require a
leading `#` because they were not embedded directly in markup. When Typst
expects code instead of markup, the leading `#` is not needed to access
functions, keywords, and variables. This can be observed in parameter lists,
function definitions, and [code blocks]($scripting).

## Review
You now know how to apply basic formatting to your Typst documents. You learned
how to set the font, justify your paragraphs, change the page dimensions, and
add numbering to your headings with set rules. You also learned how to use a
basic show rule to change how text appears throughout your document.

You have handed in your report. Your supervisor was so happy with it that they
want to adapt it into a conference paper! In the next section, we will learn how
to format your document as a paper using more advanced show rules and functions.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
