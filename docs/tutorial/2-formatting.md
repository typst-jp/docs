---
description: Typstチュートリアル
---

# 書式を設定する
前章までで、あなたはいくつかのテキスト、少しの数式や画像を含むレポートを書いてきました。
しかし、その見た目はまだとても地味です。
ティーチングアシスタントはあなたが新しい組版システムを使っていることをまだ知らず、あなたは自身のレポートを他の学生の提出物に合わせたいと思うでしょう。
この章では、Typstの組版システムを使ってレポートの体裁を整える方法を示します。

## setルール { #set-rule }
前章で見たように、Typstにはコンテンツを挿入する関数（例：[`image`]関数）と、引数として受け取ったコンテンツを操作する関数（例：[`align`]関数）があります。
あなたが所望する書式設定をしたいとき最初に思いつくことは、それを行う関数を探して、その関数で文書全体を囲うことでしょう。
たとえば、レポートを両端揃えにしたい場合には以下のように記述します。

```example
#par(justify: true)[
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

ここで、関数のすべての引数は括弧の中で指定されるべきではないか？と思うでしょう。
なぜ括弧の _後_ にコンテンツを記述する2つ目の角括弧があるのでしょうか？
答えは、関数にコンテンツを渡すことはTypstではよくあるため、特別な構文があるからです。
コンテンツを引数リストの中に入れる代わりに、通常の引数の後に角括弧内でコンテンツを直接書くことが可能であり、これによりカンマ区切りを減らすことができます。

上で見たように、これは正しく動作し、[`par`]関数はその段落のすべて文章を両端揃えにします。
しかし、無数の関数で文書を囲み、選択的に各場所でスタイル適用をしようとするとすぐ面倒になります。

幸いなことに、Typstにはもっと簡潔な記法があります。
_setルール_ を使えば、以後現れるすべてのコンテンツに対してスタイル設定を適用可能です。
`{set}`キーワードを入力し、その後に設定したい関数の名前と引数のリストを括弧で囲んでsetルールを記述します。

```example
#set par(justify: true)

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

- [`text`] フォントの種類、大きさ、色などのテキストのプロパティを設定
- [`page`] ページサイズ、余白（マージン）、ヘッダー、段組み、フッターを設定
- [`par`] 段落の両端揃え、行間の幅など
- [`heading`] 見出しの見た目や番号付
- [`document`] タイトルや著者情報などPDF出力に含まれるメタデータ

関数のパラメーターはすべて設定できるわけではありません。
一般的に、関数のパラメーターを設定できるのは、関数に _どのように_ 実行させるかを指示するパラメーターだけであり、関数に _何を_ 実行させるかを指示するパラメーターは設定できません。
関数のリファレンスページには、設定可能なパラメーターが示されています。

文書にもう少しスタイルを追加してみましょう。
余白を大きくし、セリフ体のフォントを使用します。
この例では、ページサイズも設定します。

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

ここで注目していただきたい点を以下に記載します。

まず、[`page`] setルールです。
これはページサイズと余白の2つの引数を受け取ります。
ページサイズは文字列であり、Typstは[多くの標準ページサイズ]($page.paper)を受け付けますが、カスタムページサイズを指定することもできます。
余白は[辞書型]($dictionary)で指定します。辞書型とはキーと値のペアの集まりです。
この場合、キーは`x`と`y`で、値はそれぞれ水平マージンと垂直マージンです。
`{left}`, `{right}`, `{top}`, and `{bottom}`をキーとする辞書を渡すことで、各辺に別々の余白を指定することもできます。

つぎに[`text`] setルールです。
ここでは、フォントサイズを`{10pt}`、フォントの種類を `{"New Computer Modern"}`に設定します。
Typst appには多くのフォントが用意されております。
text関数の引数リストにいるとき、補完パネルで利用可能なフォントを探すことができます。

また、行間の幅（leading）も設定しました。
これは[length]の値として指定され、`em`という単位を使ってフォントのサイズに対する行間を指定しています。
`{1em}`は現在のフォントサイズ（デフォルトは`{11pt}`）に相当します。

最後に、中央揃えに垂直配置を追加して、画像をページの下部に配置しました。
垂直配置と水平配置を `{+}` 演算子で組み合わせることで2次元配置を指定できます。

## 洗練のヒント { #sophistication }
can do this by setting the `numbering` parameter of the [`heading`] function. -->
文書をより明確に構成するために、今度は見出しに番号を付けたいと思います。
これを行うには、[`heading`]関数の`numbering`パラメーターを設定します。

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

番号付けのパラメーターとして文字列 `{「1.」}` を指定しました。
これは、見出しにアラビア数字で番号を付け、各レベルの番号の間にドットを置くようにTypstに指示します。
見出しに[文字、ローマ数字、記号]($numbering)を使用することも可能です。

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

この例では、[`lorem`]関数を使って仮テキストを生成しています。
この関数は引数に数値を取り、その単語数の _Lorem Ipsum_ テキストを生成します。

<div class="info-box">

headingとtextのsetルールが、それぞれの関数で作成されていなくても、すべての見出しと文章に適用されることを不思議に思いませんでしたか？

Typstは内部的に`[= Conclusion]`と書くたびに`heading`関数を呼び出します。
実際に、関数呼び出し `[#heading[Conclusion]]` は上記の見出しマークアップと同等です。
他のマークアップ要素も同様に機能し、対応する関数呼び出しのための _シンタックスシュガー_ に過ぎません。

</div>

## showルール { #show-rule }
書式設定によりレポートの出来栄えにかなり満足してきましたが、最後にひとつだけ修正が必要な点があります。
あなたが書いているレポートは、より大きなプロジェクトのためのものであり、そのプロジェクト名には、たとえ散文であっても、常にロゴを添えるべきでしょう。

一つの方法として、検索と置換を使ってロゴを添えるすべての場所に`[#image("logo.svg")]`の呼び出しを追加することもできますが、それはとても面倒です。
別の方法として、[カスタム関数を定義する]($function/#defining-functions)ことで、常にロゴを画像として生成することもできます。
しかし、これらよりももっと簡単な方法があります。

showルールを使用すると、Typstが特定の要素をどのように表示するかを再定義できます。
これにより、Typstがどの要素をどのように表示するかを指定します。
Showルールはテキストのインスタンスや多くの関数、さらには文書全体にも適用可能です。

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

この例には新しい構文がたくさんあります。
ここでは、`{show}`キーワードを記述し、その後に表示させたいテキストの文字列とコロンを記述しています。
そして、表示したい内容を引数として受け取る関数を書いています。
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
