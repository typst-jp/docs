---
<<<<<<< HEAD
description: Typstチュートリアル
---

# 高度なスタイリング
このチュートリアルの前の2つの章では、Typstで文書を書く方法とその書式を変更する方法を学びました。
それらの章を通して書いたレポートが優れた評価を得たため、指導教員はそれをもとに学会論文を書いてほしいと考えています！
もちろん、論文は学会のスタイルガイドに従わなければなりません。
どうすればそれを達成できるか見てみましょう。

始める前に、チームを作成して、そのチームに教員を招待して追加しましょう。
まず、エディターの左上にある戻るアイコンでアプリのダッシュボードに戻ります。
次に、左のツールバーのプラスアイコンを選択し、チームを作成します。
最後に、新しいチームをクリックし、チーム名の横にあるmanage teamをクリックして設定に進みます。
これで教員をメールで招待できます。

![The team settings](3-advanced-team-settings.png)

次に、プロジェクトをチームに移動します。
プロジェクトを開き、左のツールバーの歯車アイコンを選んで設定に行き、Ownerのドロップダウンから新しいチームを選択します。
変更を保存するのを忘れないでください！

あなたの教員もプロジェクトを編集でき、お互いにリアルタイムで変更を確認できます。
公式の[Discordサーバー](https://discord.gg/2uDybryKPe)に参加して他のユーザーを見つけ、一緒にチームを組んでみることも可能です！

## 学会ガイドライン { #guidelines }
レイアウトのガイドラインは学会ウェブサイトに掲載されております。
ここでは以下の条件であった場合を考えましょう。

- フォントは11ptのセリフ体
- タイトルは17ptで太字
- アブストラクトは1段組みで本文は2段組み
- アブストラクトは中央揃え
- 本文は両端揃え
- 第1レベルのセクションの見出しは13ptで中央に配置し、小さな大文字で表示
- 第2レベルの見出しは斜体で、本文と同じ大きさ
- ページはUSレターサイズとし、下中央にページ番号を付け、各ページの右上に論文のタイトルを記載

これらのうち、多くの項目については既に対応方法を知っていますが、いくつかについては新しい記法を学ぶ必要があります。

## setルール { #set-rules }
まず、文書のsetルールを書くことから始めましょう。
=======
description: Typst's tutorial.
---

# Advanced Styling
In the previous two chapters of this tutorial, you have learned how to write a
document in Typst and how to change its formatting. The report you wrote
throughout the last two chapters got a straight A and your supervisor wants to
base a conference paper on it! The report will of course have to comply with the
conference's style guide. Let's see how we can achieve that.

Before we start, let's create a team, invite your supervisor and add them to the
team. You can do this by going back to the app dashboard with the back icon in
the top left corner of the editor. Then, choose the plus icon in the left
toolbar and create a team. Finally, click on the new team and go to its settings
by clicking 'manage team' next to the team name. Now you can invite your
supervisor by email.

![The team settings](3-advanced-team-settings.png)

Next, move your project into the team: Open it, going to its settings by
choosing the gear icon in the left toolbar and selecting your new team from the
owners dropdown. Don't forget to save your changes!

Now, your supervisor can also edit the project and you can both see the changes
in real time. You can join our [Discord server](https://discord.gg/2uDybryKPe)
to find other users and try teams with them!

## The conference guidelines { #guidelines }
The layout guidelines are available on the conference website. Let's take a look
at them:

- The font should be an 11pt serif font
- The title should be in 17pt and bold
- The paper contains a single-column abstract and two-column main text
- The abstract should be centered
- The main text should be justified
- First level section headings should be 13pt, centered, and rendered in small
  capitals
- Second level headings are run-ins, italicized and have the same size as the
  body text
- Finally, the pages should be US letter sized, numbered in the center of the
  footer and the top right corner of each page should contain the title of the
  paper

We already know how to do many of these things, but for some of them, we'll need
to learn some new tricks.

## Writing the right set rules { #set-rules }
Let's start by writing some set rules for the document.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```example
#set page(
>>> margin: auto,
  paper: "us-letter",
  header: align(right)[
    A Fluid Dynamic Model for
    Glacier Flow
  ],
  numbering: "1",
)
#set par(justify: true)
#set text(
  font: "Libertinus Serif",
  size: 11pt,
)

#lorem(600)
```

<<<<<<< HEAD
ここで行われていることの大半は、すでに分かりでしょう。
テキストサイズを`{11pt}`に、フォントをLibertinus Serifに設定しています。
また、段落の両端揃えを有効にし、ページサイズをUSレターとしています。

ここで、`header`は新しい引数で、各ページの上部の余白に置くコンテンツを設定できます。
ヘッダーには、学会のスタイルガイドで要求されているように、論文のタイトルを指定します。
`align`関数を用いてそのテキストを右寄せにします。

最後に `numbering` 引数について説明します。
ここでは、ページ番号の付け方を定義する[numbering pattern]($numbering)を指定できます。
例えば`{"1"}`と設定すると、Typstは素のページ番号のみを表示します。
また`{"(1/1)"}`と設定すると、カッコで囲まれた現在のページと総ページ数が表示されるでしょう。
さらに、カスタム関数を用意して完全に好みの書式にできます。

## タイトルとアブストラクトの作成 { #title-and-abstract }
それでは、タイトルとアブストラクトを追加しましょう。
Typstには[`title`]($title)関数があります。この関数にタイトルを引数として渡してみましょう。
=======
You are already familiar with most of what is going on here. We set the text
size to `{11pt}` and the font to Libertinus Serif. We also enable paragraph
justification and set the page size to US letter.

The `header` argument is new: With it, we can provide content to fill the top
margin of every page. In the header, we specify our paper's title as requested
by the conference style guide. We use the `align` function to align the text to
the right.

Last but not least is the `numbering` argument. Here, we can provide a
[numbering pattern]($numbering) that defines how to number the pages. By
setting it to `{"1"}`, Typst only displays the bare page number. Setting it to
`{"(1/1)"}` would have displayed the current page and total number of pages
surrounded by parentheses. And we could even have provided a completely custom
function here to format things to our liking.

## Creating a title and abstract { #title-and-abstract }
Now, let's add a title and an abstract. We'll start with the title. Typst comes
with a [`title`] function. Let's start by providing our title as an argument:
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```example
>>> #set page(width: 300pt, margin: 30pt)
>>> #set text(font: "Libertinus Serif", 11pt)
#title[
  A Fluid Dynamic Model
  for Glacier Flow
]
```

<<<<<<< HEAD
タイトルはすでに太字で、周囲には余白もあります。
ただし、左揃えで、文字サイズも17ptではありません。
そこで、見た目を調整しましょう。
`title`関数には、フォントや文字サイズを設定する引数はありません。
代わりに、これらのプロパティは`text`関数と`align`関数で設定します。

<div class="info-box">

`title`関数で挿入したタイトルと、等号で作成した見出しは何が違うのでしょうか？

第1レベルの見出しを含め、見出しは文書内に複数回出現できます。
一方、タイトルは通常、文書の冒頭に一度だけ出現します。
両者を区別しておくと、スクリーンリーダーなどの支援技術を利用する読者にとってアクセシブルな文書を、Typstが生成しやすくなります。
</div>

ある要素の中にある別の種類の要素のプロパティをカスタマイズするには、show-setルールを使用できます。
まず、`show`に続けてカスタマイズする要素を指定します。
この指定を_セレクター_と呼びます。
続いてコロンを入力し、セレクターにマッチする要素へ適用するsetルールを記述します。
まとめると、構文は次のようになります。
=======
You can see that the title is already boldfaced and has some space around it.
However, it is left-aligned and not exactly 17pt large. Hence, we need to adjust
its appearance. The title function does not come with any arguments for
font or text size we could set. Instead, these properties are defined on the
`text` and `align` functions.

<div class="info-box">

What is the difference between what the `title` function inserted and the
headings we produced with equals signs?

Headings, even first-level headings, can appear multiple times in your document
whereas a title only appears once, usually at the beginning. Differentiating
between the two helps Typst make your document accessible for users of
Assistive Technology such as screen readers.
</div>

When we want to customize the properties of some element inside of another kind
of element, we can use show-set rules. First, we use `show` to select which
element we want to customize. We call this a _selector._ Then, we type a colon.
Next, we write the set rule that should apply to elements matching the selector.
Summarized, the syntax looks like this:
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```typ
#show your-selector: set some-element(/* ... */)
```

<<<<<<< HEAD
ここで確認しましょう。タイトルを中央揃えにし、文字サイズを17ptにしたいのでした。
そのため、次の2つのshow-setルールが必要です。

- セレクターが`title`、ルールが`{set text(size: 17pt)}`のshow-setルール
- セレクターが`title`、ルールが`{set align(center)}`のshow-setルール

例は次のようになります。
=======
Let's recall: We want to center-align the title and make it 17pt large. Hence,
we need two show-set rules:

- One with the selector `title` and the rule `{set text(size: 17pt)}`
- One with the selector `title` and the rule `{set align(center)}`

Our example now looks like this:
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```example
>>> #set page(width: 300pt, margin: 30pt)
>>> #set text(font: "Libertinus Serif", 11pt)
#show title: set text(size: 17pt)
#show title: set align(center)

#title[
  A Fluid Dynamic Model
  for Glacier Flow
]
```

<<<<<<< HEAD
これでよさそうです。著者一覧も追加しましょう。
この論文は指導教員と共同で執筆しているので、自分の名前と指導教員の名前を追加します。
=======
This looks right. Let's also add the author list: Since we are writing this
paper together with our supervisor, we'll add our own and their name.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```example
>>> #set page(width: 300pt, margin: 30pt)
>>> #set text(font: "Libertinus Serif", 11pt)
>>>
>>> #show title: set text(size: 17pt)
>>> #show title: set align(center)
>>>
>>> #title[
>>>   A Fluid Dynamic Model
>>>   for Glacier Flow
>>> ]

#grid(
  columns: (1fr, 1fr),
  align(center)[
    Therese Tungsten \
    Artos Institute \
    #link("mailto:tung@artos.edu")
  ],
  align(center)[
    Dr. John Doe \
    Artos Institute \
    #link("mailto:doe@artos.edu")
  ]
)
```

<<<<<<< HEAD
著者情報が記載された2つのブロックが隣り合わせにレイアウトされています。
このレイアウトを作るために[`grid`]($grid)関数を使っています。
これにより、各列の大きさや、どのコンテンツをどのセルに入れるかを正確に制御できます。
`columns`引数には、[相対長さ]($relative)または[割合]($fraction)の配列を渡します。
この場合、2つの等しい割合のサイズを渡し、使用可能なスペースを2つの等しい列に分割するように指示します。
次に、grid関数に2つのコンテンツ引数を渡しました。
ひとつは主著者であるあなたの情報で、もうひとつは指導教員の情報です。
ここでも `align` 関数を使用して、コンテンツを列の中央に配置しています。
grid関数はセルを指定するコンテンツ引数を任意の数で受け取れます。
行は自動的に追加されますが、`rows`引数を使えば手動でサイズを指定できます。

タイトルと著者一覧を見ると、少し近すぎます。
この問題は、別のshow-setルールを使ってタイトルの下の間隔を設定することで解決できます。
タイトルやグリッド、段落など、Typstがページの上から下へ配置する全ての要素を_ブロック_と呼びます。
各ブロックは[`block`]($block)関数によって制御されます。
この関数は、ブロック同士の間隔や、ブロック内で改ページできるかどうかなどを制御します。
つまり、タイトルを選択してブロックの間隔を設定するshow-setルールをさらに記述できます。
=======
The two author blocks are laid out next to each other. We use the [`grid`]
function to create this layout. With a grid, we can control exactly how large
each column is and which content goes into which cell. The `columns` argument
takes an array of [relative lengths]($relative) or [fractions]($fraction). In
this case, we passed it two equal fractional sizes, telling it to split the
available space into two equal columns. We then passed two content arguments to
the grid function. The first with our own details, and the second with our
supervisors'. We again use the `align` function to center the content within the
column. The grid takes an arbitrary number of content arguments specifying the
cells. Rows are added automatically, but they can also be manually sized with
the `rows` argument.

Looking at the authors and the title, they are a bit too close together. You can
address this by using another show-set rule to configure the space below the
title. The title, the grid, paragraphs, and all other elements that Typst
arranges from the top to the bottom of the page are called _blocks._ Each block
is controlled by the [`block`] function. It controls behaviors like their
distance and whether a block can contain a page break. That means that we can
write another show-set rule that selects the title to set the block spacing:
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```example
>>> #set page(width: 300pt, margin: 30pt)
>>> #set text(font: "Libertinus Serif", 11pt)
>>>
#show title: set text(size: 17pt)
#show title: set align(center)
#show title: set block(below: 1.2em)

#title[
  A Fluid Dynamic Model
  for Glacier Flow
]

#grid(
<<<   // ...
>>>   columns: (1fr, 1fr),
>>>   align(center)[
>>>     Therese Tungsten \
>>>     Artos Institute \
>>>     #link("mailto:tung@artos.edu")
>>>   ],
>>>   align(center)[
>>>     Dr. John Doe \
>>>     Artos Institute \
>>>     #link("mailto:doe@artos.edu")
>>>   ]
)
```

<<<<<<< HEAD
このshow-setルールで、タイトルの下の間隔を上書きしました。
ここでは`em`単位を使用しています。
`em`を使うと、フォントサイズの倍数で長さを表現できます。
この例では、タイトルと著者一覧の間隔をフォントサイズのちょうど1.2倍に設定しました。

それでは、アブストラクトを追加しましょう。
学会では、アブストラクトを両端揃えにせず、中央に配置することが求められている点を思い出してください。
=======
With this show-set rule, we overrode the spacing below the title. We have used
the `em` unit: It allows us to express lengths as multiples of the font size.
Here, we used it to space the title and the author list exactly 1.2× the font
size apart. Now, let's add the abstract. Remember that the conference wants the
abstract to be set ragged and centered.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```example:0,0,612,317.5
>>> #set page(
>>>   "us-letter",
>>>   margin: auto,
>>>   header: align(right + horizon)[
>>>     A Fluid Dynamic Model for
>>>     Glacier Flow
>>>   ],
>>>   numbering: "1",
>>> )
>>> #set par(justify: true)
>>> #set text(font: "Libertinus Serif", 11pt)
>>>
>>> #show title: set text(size: 17pt)
>>> #show title: set align(center)
>>> #show title: set block(below: 1.2em)
>>>
>>> #title[
>>>   A Fluid Dynamic Model
>>>   for Glacier Flow
>>> ]
>>>
>>> #grid(
>>>   columns: (1fr, 1fr),
>>>   align(center)[
>>>     Therese Tungsten \
>>>     Artos Institute \
>>>     #link("mailto:tung@artos.edu")
>>>   ],
>>>   align(center)[
>>>     Dr. John Doe \
>>>     Artos Institute \
>>>     #link("mailto:doe@artos.edu")
>>>   ]
>>> )
>>>
<<< ...

#align(center)[
  #set par(justify: false)
  *Abstract* \
  #lorem(80)
]
>>> #lorem(600)
```
<<<<<<< HEAD
できました！特筆すべき点は、`align`のコンテンツ引数の中にあるsetルールを使って、アブストラクトの両端揃えをオフにしたことです。
これは、最初のsetルールの後に指定されたにもかかわらず、文書の残りの部分には影響しません。
コンテンツ・ブロック内で設定されたものは、そのブロック内のコンテンツにのみ影響します。

さらに、ヘッダーと`title`要素の引数にタイトルを重複して記述している点も改善できます。
両方で同じタイトルを使うため、文書メタデータ用の領域にタイトルを保存できると便利です。
その場合、両方の場所からタイトルを取得する方法も必要になります。
タイトルの保存には`document`要素が役立ちます。
`document`要素をsetルールで使用すると、タイトル、説明、キーワードなどの文書メタデータを保存できます。
=======

Well done! One notable thing is that we used a set rule within the content
argument of `align` to turn off justification for the abstract. This does not
affect the remainder of the document even though it was specified after the
first set rule because content blocks _scope_ styling. Anything set within a
content block will only affect the content within that block.

Another tweak could be to remove the duplication between the header and the
title element's argument. Since they share the title, it would be convenient to
store it in a place designed to hold metadata about the document. We would then
need a way to retrieve the title in both places. The `document` element can help
us with the former: By using it in a set rule, we can store document metadata
like title, description, and keywords.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```typ
#set document(title: [A Fluid Dynamic Model for Glacier Flow])
```

<<<<<<< HEAD
ここで設定したタイトルは、PDFをエクスポートしたときにPDFリーダーのタイトルバーへ表示されます。
また、オペレーティングシステムの検索でも、このタイトルを使ってファイルを見つけられます。
さらに、タイトルの設定は文書のアクセシビリティ向上にも役立ち、アクセシビリティを重視したPDF規格であるPDF/UAに準拠する場合には必須です。

次に、設定した値をページ上のタイトルとヘッダーの両方で取得する方法が必要です。
`title`関数は`document`要素と連携するように設計されているため、引数なしで呼び出すとタイトルが表示されます。
一方、ヘッダーでは、より明示的に指定する必要があります。
ヘッダーにタイトルを挿入したいという意図をTypstは判断できないため、手動で指示する必要があります。

_コンテキスト_を使うと、これまでに要素へ設定した任意の値を取得できます。
`{context}`キーワードを使用すれば、`document`要素の`title`プロパティを含め、どの要素のどのプロパティにもアクセスできます。
次のように使用します。
=======
When exporting a PDF, the title set here will appear in the title bar of your
PDF reader. Your operating system will also use this title to make the file
retrievable with search. Last but not least, it contributes to making your
document more accessible and is required if you choose to comply with PDF/UA, a
PDF standard focused on accessibility.

Now, we need a way to retrieve the value we set in the main title and the
header. Because the `title` function is designed to work together with the
`document` element, calling it with no arguments will just print the title. For
the header, we will need to be more explicit: Because Typst has no way of
knowing that we want to insert the title there, we will need to tell it to do so
manually.

Using _context,_ we can retrieve the contents of any values we have set on
elements before. When we use the `{context}` keyword, we can access any property
of any element, including the document element's title property. Its use looks
like this:
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```example:single
#set document(title: [
  A Fluid Dynamic Model
  for Glacier Flow
])

<<< ...

#set page(
>>> "us-letter",
>>> margin: auto,
  header: align(
    right + horizon,
    // Retrieve the document
    // element's title property.
    context document.title,
  ),
<<<   ...
>>> numbering: "1",
)
>>> #set par(justify: true)
>>> #set text(font: "Libertinus Serif", 11pt)

>>> #show title: set text(size: 17pt)
>>>
>>> #show title: set align(center)
>>> #show title: set block(below: 1.2em)
#title()

<<< ...

>>> #grid(
>>>   columns: (1fr, 1fr),
>>>   align(center)[
>>>     Therese Tungsten \
>>>     Artos Institute \
>>>     #link("mailto:tung@artos.edu")
>>>   ],
>>>   align(center)[
>>>     Dr. John Doe \
>>>     Artos Institute \
>>>     #link("mailto:doe@artos.edu")
>>>   ]
>>> )
>>>
>>> #align(center)[
>>>   #set par(justify: false)
>>>   *Abstract* \
>>>   #lorem(80)
>>> ]
>>>
>>> #lorem(600)
```

<<<<<<< HEAD
まず、空の丸括弧で`title`関数を呼び出している点に注目してください。
引数を渡していないため、上で`document`要素に設定した値がデフォルトで使用されます。
空の丸括弧と空の角括弧の違いは重要です。
空の丸括弧は何も渡していないことを示しますが、空の角括弧は1つの引数、つまり空のコンテンツブロックを渡していることを示します。
空の角括弧で呼び出すと、タイトルには何も表示されません。

次に、ヘッダーを見てみましょう。
角括弧内にタイトルを直接記述する代わりに、`context`キーワードを使って文書のタイトルにアクセスしています。
これにより、上で設定した内容がそのまま挿入されます。
コンテキストの役割は、プロパティへのアクセスだけではありません。
文書内に特定の要素が存在するかを調べたり、別の要素の物理的な寸法を測定したりできます。
コンテキストを使えば、利用者の設定に応じて変化する強力なテンプレートを構築できます。
=======
First, notice how we called the title function with empty, round
parentheses. Because no argument was passed, it defaulted to what we set for the
document element above. The distinction between empty round and empty square
brackets is important: While empty round brackets show that you are passing
nothing, empty square brackets mean that you are passing one argument: an empty
content block. If called that way, the title would have no visible content.

Next, take a look at the header. Instead of the title in square parentheses, we
used the context keyword to access the document title. This inserted exactly
what we set above. The role of context is not limited to accessing properties:
With it, you can check if some elements are present in the document, measure the
physical dimensions of others, and more. Using context, you can build powerful
templates that react to the preferences of the end-user.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

<div class="info-box">

<details>
<summary>
<<<<<<< HEAD
要素のプロパティにアクセスするには、なぜ`context`キーワードが必要なのでしょうか？
</summary>

通常、変数にアクセスするときは、その値がどのような値になるか正確に分かっています。

- `[#sym.pi]`のように、変数がTypstの組み込み定数である場合
- 変数が引数によって定義されている場合
- 変数が現在のスコープで定義または上書きされている場合

しかし、それだけでは不十分な場合があります。
この章では、タイトルを含むページヘッダーを挿入しました。
ヘッダーには1つのコンテンツしか渡していませんが、ページごとに異なるヘッダーを表示したいこともあります。
例えば、章の名前を表示したり、ページ番号を使ったりしたい場合です。
コンテキストブロックを1つ記述しておくと、Typstはそれが挿入された場所を起点に、直前の見出しや現在のページ番号などを調べて処理します。
そのため、同じコンテキストブロックでも、挿入するページによって異なる出力を生成できます。

詳しくは、このチュートリアルを終えた後に[コンテキストのドキュメント]($context)を参照してください。
</details>
</div>

## 段組みと見出しの追加 { #columns-and-headings }
上の論文は、残念ながら文字が単調にぎっしり詰まっていて読みにくい見た目をしています。
これを修正するために、見出しを追加し、2段組のレイアウトに変更してみましょう。
幸いなことに、setルールで`page`に`column`引数を追加することで簡単に行えます。

引数リストに`{columns: 2}`を加えることで、文書全体を2段組みとなります。
しかし、これではタイトルと著者、アブストラクトにも影響が出てしまいます。
それらを1段組みのままに維持するためには、[`{place}`]($place)関数を呼び出して囲みましょう。
place関数は引数として配置とコンテンツを受け取ります。
オプション引数である`{scope}`引数を使えば、現在の段組みとその親（ページ）のどちらに対して配置するかを決めることが可能です。
これらに加えて、もうひとつ設定することがあります。
オプション引数がない場合、`{place}`はそのコンテンツを文書の流れから外し、他のレイアウトに影響を与えることなく、他のコンテンツの上に配置します。
=======
Why is the context keyword required to access element properties?
</summary>

Normally, when we access a variable, we know exactly what its value is going to
be:

- The variable could be a constant built into Typst, like `[#sym.pi]`
- The variable could be defined by an argument
- The variable could be defined or overwritten in the current scope

However, sometimes, that's not enough. In this chapter of the tutorial, we have
inserted a page header with the title. Even though we pass only one piece of
content for the header, we may want different pages to have different headers.
For example, we may want to print the chapter name or use the page number. When
we use context, we can write a single context block that tells Typst to take a
look at where it's inserted, look for the last heading, the current page number,
or anything else, and go from there. That means that the same context block,
inserted on different pages, can produce different output.

For more information, read up on context [in its docs]($context) after
completing this tutorial.
</details>
</div>

## Adding columns and headings { #columns-and-headings }
The paper above unfortunately looks like a wall of lead. To fix that, let's add
some headings and switch our paper to a two-column layout. Fortunately, that's
easy to do: We just need to amend our `page` set rule with the `columns`
argument.

By adding `{columns: 2}` to the argument list, we have wrapped the whole
document in two columns. However, that would also affect the title and authors
overview. To keep them spanning the whole page, we can wrap them in a function
call to [`{place}`]($place). Place expects an alignment and the content it
should place as positional arguments. Using the named `{scope}` argument, we can
decide if the items should be placed relative to the current column or its
parent (the page). There is one more thing to configure: If no other arguments
are provided, `{place}` takes its content out of the flow of the document and
positions it over the other content without affecting the layout of other
content in its container:
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```example
#place(
  top + center,
  rect(fill: black),
)
#lorem(30)
```

<<<<<<< HEAD
もしここで`{place}`を使わなければ、黒塗りの長方形は独立した行になるはずですが、
`{place}`を使うと、それに続く数行のテキストの上に重なります。
同様に、テキスト側もこの長方形がないかのように振る舞います。
この動作を変更するには、引数`{float: true}`を渡してください。
これにより`{place}`でページの上部または下部に配置されたアイテムが、他のコンテンツと重ならないように設定できます。
=======
If we hadn't used `{place}` here, the square would be in its own line, but here
it overlaps the few lines of text following it. Likewise, that text acts as if
there was no square. To change this behavior, we can pass the argument
`{float: true}` to ensure that the space taken up by the placed item at the top
or bottom of the page is not occupied by any other content.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```example:single
>>> #set document(title: [
>>>   A Fluid Dynamic Model
>>>   for Glacier Flow
>>> ])
>>>
#set page(
>>> margin: auto,
  paper: "us-letter",
  header: align(
    right + horizon,
    context document.title,
  ),
  numbering: "1",
  columns: 2,
)
>>> #set par(justify: true)
>>> #set text(font: "Libertinus Serif", 11pt)

#place(
  top + center,
  float: true,
  scope: "parent",
  clearance: 2em,
)[
>>> #show title: set text(size: 17pt)
>>> #show title: set align(center)
>>> #show title: set block(below: 1.2em)
>>>
>>> #title()
>>>
>>> #grid(
>>>   columns: (1fr, 1fr),
>>>   [
>>>     Therese Tungsten \
>>>     Artos Institute \
>>>     #link("mailto:tung@artos.edu")
>>>   ],
>>>   [
>>>     Dr. John Doe \
>>>     Artos Institute \
>>>     #link("mailto:doe@artos.edu")
>>>   ]
>>> )
<<<   ...

  #par(justify: false)[
    *Abstract* \
    #lorem(80)
  ]
]

= Introduction
#lorem(300)

= Related Work
#lorem(200)
```

<<<<<<< HEAD
この例では、`{place}` 関数の `clearance` 引数も使用しています。
これにより、[`{v}`]($v)関数を使用する代わりに、本文との間にスペースを設けています。
また、コンテンツはcenter引数を継承しているため、各パーツごとに行っていた明示的な `{align(center, ...)}` 呼び出しも削除できます。

最後に見出しのスタイルの設定をしましょう。
ガイドラインに従うため、見出しは中央揃えにして、小さな大文字を使わなければなりません。
`heading`関数はそのような設定を提供していないため、独自の見出しshowルールを書く必要があります。

- 見出しを中央揃えにするshow-setルール
- 見出しを13ptにし、太さを標準にするshow-setルール
- 見出し全体を`smallcaps`関数で囲むshowルール
=======
In this example, we also used the `clearance` argument of the `{place}` function
to provide the space between it and the body instead of using the [`{v}`]($v)
function. We can also remove the explicit `{align(center, ..)}` calls around the
various parts since they inherit the center alignment from the placement.

Now there is only one thing left to do: Style our headings. We need to make them
centered and use small capitals. These properties are not available on the
`heading` function, so we will need to write a few show-set rules and a show
rule:

- A show-set rule to make headings center-aligned
- A show-set rule to make headings 13pt large and use the regular weight
- A show rule to wrap the headings in a call to the `smallcaps` function
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```example:50,250,265,270
>>> #set document(title: [
>>>   A Fluid Dynamic Model
>>>   for Glacier Flow
>>> ])
>>>
>>> #set page(
>>>   "us-letter",
>>>   margin: auto,
>>>   header: align(
>>>     right + horizon,
>>>     context document.title,
>>>   ),
>>>   numbering: "1",
>>>   columns: 2,
>>> )
>>> #set par(justify: true)
>>> #set text(font: "Libertinus Serif", 11pt)
#show heading: set align(center)
#show heading: set text(
  size: 13pt,
  weight: "regular",
)
#show heading: smallcaps

<<< ...
>>> #place(
>>>   top + center,
>>>   float: true,
>>>   scope: "parent",
>>>   clearance: 2em,
>>> )[
>>>   #show title: set text(size: 17pt)
>>>   #show title: set align(center)
>>>   #show title: set block(below: 1.2em)
>>>
>>>   #title()
>>>
>>>   #grid(
>>>     columns: (1fr, 1fr),
>>>     [
>>>       Therese Tungsten \
>>>       Artos Institute \
>>>       #link("mailto:tung@artos.edu")
>>>     ],
>>>     [
>>>       Dr. John Doe \
>>>       Artos Institute \
>>>       #link("mailto:doe@artos.edu")
>>>     ]
>>>   )
>>>
>>>   #par(justify: false)[
>>>     *Abstract* \
>>>     #lorem(80)
>>>   ]
>>> ]

= Introduction
<<< ...
>>> #lorem(35)

== Motivation
<<< ...
>>> #lorem(45)
```

<<<<<<< HEAD
これで見た目が整いました！
ここでは、全ての見出しに適用されるshowルールを使用しました。
最後のshowルールでは、見出し全体に`smallcaps`関数を適用しています。
次の例のように、独自のshowルールを使えば、見出しのデフォルトの見た目を完全に上書きできます。

ただし、全ての見出しが同じように見えるという問題が残っています。
「Motivation」と「Problem Statement」のサブセクションは斜体の同行見出しにする必要がありますが、現状ではセクションの見出しと区別できません。
この問題は、`where`セレクターを使用すると解決できます。
`where`は、見出しなどの要素に対して呼び出せる[メソッド]($scripting/#methods)で、プロパティに基づいて要素を絞り込めます。
これにより、セクションとサブセクションの見出しを区別できます。
=======
This looks great! We used show rules that apply to all headings. In the final
show rule, we applied the `smallcaps` function to the complete heading. As we
will see in the next example, we can also provide a custom rule to completely
override the default look of headings.

The only remaining problem is that all headings look the same now. The
"Motivation" and "Problem Statement" subsections ought to be italic run-in
headers, but right now, they look indistinguishable from the section headings.
We can fix that by using a `where` selector on our show rule: This is a
[method]($scripting/#methods) we can call on headings (and other elements) that
allows us to filter them by their properties. We can use it to differentiate
between section and subsection headings:
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363

```example:50,250,265,245
>>> #set document(title: [
>>>   A Fluid Dynamic Model
>>>   for Glacier Flow
>>> ])
>>>
>>> #set page(
>>>   "us-letter",
>>>   margin: auto,
>>>   header: align(
>>>     right + horizon,
>>>     context document.title,
>>>   ),
>>>   numbering: "1",
>>>   columns: 2,
>>> )
>>> #set par(justify: true)
>>> #set text(font: "Libertinus Serif", 11pt)
>>>
#show heading.where(level: 1): set align(center)
#show heading.where(level: 1): set text(size: 13pt, weight: "regular")
#show heading.where(level: 1): smallcaps

#show heading.where(level: 2): set text(
  size: 11pt,
  weight: "regular",
  style: "italic",
)
#show heading.where(level: 2): it => {
  it.body + [.]
}
>>>
>>> #place(
>>>   top + center,
>>>   float: true,
>>>   scope: "parent",
>>>   clearance: 2em,
>>> )[
>>>   #show title: set text(size: 17pt)
>>>   #show title: set align(center)
>>>   #show title: set block(below: 1.2em)
>>>
>>>   #title()
>>>
>>>   #grid(
>>>     columns: (1fr, 1fr),
>>>     [
>>>       Therese Tungsten \
>>>       Artos Institute \
>>>       #link("mailto:tung@artos.edu")
>>>     ],
>>>     [
>>>       Dr. John Doe \
>>>       Artos Institute \
>>>       #link("mailto:doe@artos.edu")
>>>     ]
>>>   )
>>>
>>>   #par(justify: false)[
>>>     *Abstract* \
>>>     #lorem(80)
>>>   ]
>>> ]
>>>
>>> = Introduction
>>> #lorem(35)
>>>
>>> == Motivation
>>> #lorem(45)
```

<<<<<<< HEAD
この例では、まず`{.where(level: 1)}`でセレクターをより具体的にし、先ほどのルールの適用範囲を第1レベルの見出しに限定しています。
次に、第2レベルの見出しに適用するshow-setルールを追加します。
最後に、独自の関数を使ったshowルールが必要です。
デフォルトでは、見出しのコンテンツはブロックで囲まれます。
そのため、見出しは独立した行になります。
しかし、ここでは見出しを本文へ追い込みたいので、独自のshowルールを指定してこのブロックを取り除きます。

このルールには、見出しを引数として受け取る関数を指定します。
この引数は慣例として`it`と呼ばれますが、別の名前でも構いません。
この引数はコンテンツとして使用でき、その場合はデフォルトの見出し全体がそのまま表示されます。
一方、独自の見出しを作る場合は、`body`、`numbering`、`level`などのフィールドを使って見た目を組み立てられます。
ここでは、見出しの本文と末尾のピリオドだけを表示し、組み込みのshowルールが生成するブロックを省いています。
showルールで`it.numbering`を明示的に使用していないため、この見出しには見出しの番号付けなどに関するsetルールが反映されなくなることに注意してください。
このようなshowルールを記述し、文書を引き続きカスタマイズできるようにするには、これらのフィールドを考慮する必要があります。

これは素晴らしい！
第1レベルと第2レベルの見出しにそれぞれ選択的に適用される2つのshowルールを書きました。
`where`セレクターを使用して、見出しをレベルでフィルタリングしました。
そして、サブセクションの見出しを本文と改行せずにレンダリングしました。
また、サブセクションの見出しの最後にピリオドを自動的に追加しています。

ここで、学会のスタイルガイドを確認しましょう。

- フォントは11ptのセリフ体 ✓
- タイトルは17ptで太字 ✓
- アブストラクトは1段組みで本文は2段組み ✓
- アブストラクトは中央揃え ✓
- 本文は両端揃え ✓
- 第1レベルのセクションの見出しは13ptで中央に配置し、小さな大文字で表示 ✓
- 第2レベルの見出しは斜体で、本文と同じ大きさ ✓
- ページはUSレターサイズとし、下中央にページ番号を付け、各ページの右上に論文のタイトルを記載 ✓

これで、全てのスタイルに準拠し、論文を学会に提出できます！完成した論文は次のようになっています。

<!-- textlint-disable ja-technical-writing/ja-no-mixed-period -->
<img src="3-advanced-paper.png" alt="The finished paper" style="box-shadow: 0 4px 12px rgb(89 85 101 / 20%); width: 500px; max-width: 100%; display: block; margin: 24px auto;">
<!-- textlint-enable ja-technical-writing/ja-no-mixed-period -->

## まとめ
このセクションでは、ヘッダーとフッターの作成方法、関数とスコープを使用してローカルにスタイルをオーバーライドする方法、[`grid`]関数を使用してより複雑なレイアウトを作成する方法、個々の関数と文書全体のshowルールを記述する方法を学びました。
コンテキストを使用して要素のプロパティにアクセスする方法も学びました。
また、[`where`セレクター]($styling/#show-rules)を使用して、見出しをそのレベルによってフィルタリングする方法も学びました。

結果として論文は大成功でした！
あなたはその学会にて同じ志を持つ研究者にたくさん出会い、来年同じ学会で発表したいプロジェクトを計画しています。
その際に、同じスタイルガイドを使って新しい論文を書く必要があるため、あなたやあなたのチームのために、時間を節約できるテンプレートを作りたいと思うのではないでしょうか？

次のセクションでは、複数の文書で再利用できるテンプレートの作成方法を学びます。
これはより高度なトピックですので、今すぐには手が出せないという方は、後ほどお気軽にお越しください。
=======
In this example, we first scope our previous rules to first-level headings by
using `{.where(level: 1)}` to make the selector more specific. Then, we add a
show-set rule for the second heading level. Finally, we need a show rule with a
custom function: Headings enclose their contents with a block by default. This
has the effect that the heading gets its own line. However, we want it to run
into the text, so we need to provide our own show rule to get rid of this block.

We provide the rule with a function that takes the heading as a parameter.
This parameter is conventionally called `it`, but can have another name. The
parameter can be used as content and will just display the whole default
heading. Alternatively, when we want to build our own heading instead, we can
use its fields like `body`, `numbering`, and `level` to compose a custom look.
Here, we are just printing the body of the heading with a trailing dot and leave
out the block that the built-in show rule produces. Note that this heading will
no longer react to set rules for heading numbering and similar because we did
not explicitly use `it.numbering` in the show rule. If you are writing show
rules like this and want the document to remain customizable, you will need to
take these fields into account.

This looks great! We wrote show rules that selectively apply to the first and
second level headings. We used a `where` selector to filter the headings by
their level. We then rendered the subsection headings as run-ins. We
also automatically add a period to the end of the subsection headings.

Let's review the conference's style guide:
- The font should be an 11pt serif font ✓
- The title should be in 17pt and bold ✓
- The paper contains a single-column abstract and two-column main text ✓
- The abstract should be centered ✓
- The main text should be justified ✓
- First level section headings should be centered, rendered in small caps and in
  13pt ✓
- Second level headings are run-ins, italicized and have the same size as the
  body text ✓
- Finally, the pages should be US letter sized, numbered in the center and the
  top right corner of each page should contain the title of the paper ✓

We are now in compliance with all of these styles and can submit the paper to
the conference! The finished paper looks like this:

<img
  src="3-advanced-paper.png"
  alt="The finished paper"
  style="box-shadow: 0 4px 12px rgb(89 85 101 / 20%); width: 500px; max-width: 100%; display: block; margin: 24px auto;"
>

## Review
You have now learned how to create titles, headers, and footers, how to use
functions, show-set rules, and scopes to locally override styles, how to create
more complex layouts with the [`grid`] function, how to access element
properties with context, and how to write show rules for individual functions,
and the whole document. You also learned how to use the [`where`
selector]($styling/#show-rules) to filter the headings by their level.

The paper was a great success! You've met a lot of like-minded researchers at
the conference and are planning a project which you hope to publish at the same
venue next year. You'll need to write a new paper using the same style guide
though, so maybe now you want to create a time-saving template for you and your
team?

In the next section, we will learn how to create templates that can be reused in
multiple documents. This is a more advanced topic, so feel free to come back
to it later if you don't feel up to it right now.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
