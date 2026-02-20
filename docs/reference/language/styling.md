---
description: Typst で文書のスタイル設定をするために必要な概念
---

# スタイル設定

Typstには柔軟なスタイル設定機能を持ち、出力される文書に対して自動的に任意のスタイル設定を適用します。
_setルール_では要素の基本プロパティを設定できます。
しかし、やりたいこと全てに対応するプロパティがあらかじめ実装されているとは限りません。
このため、Typstは要素の外観を完全に再定義できる_showルール_もサポートしています。

## setルール { #set-rules }

setルールを使うと、要素の外観をカスタマイズできます。
これらは、`{set}`キーワード（マークアップでは`[#set]`）を前に置いた[要素関数]($function/#element-functions)への[関数呼び出し]($function)として記述されます。
setルールに指定できるのは、その関数のオプションのパラメーターだけです。
どのパラメーターがオプションであるかは、各関数のドキュメントを参照してください。
以下の例では、2つのsetルールを使って、[フォント]($text.font)と[見出し番号]($heading.numbering)を変更しています。

```example
#set heading(numbering: "I.")
#set text(
  font: "New Computer Modern"
)

= Introduction
With set rules, you can style
your document.
```

setルールは、そのまま記述するとファイルの最後まで適用されます。
ブロックの中にネストすると、そのブロックの終わりまで適用されます。
ブロックを使えば、setルールの効果を指定した部分に限定できます。
以下では、contentブロックを用いてスコープすることで、特定のリストのスタイルのみを変更しています。

```example
This list is affected: #[
  #set list(marker: [--])
  - Dash
]

This one is not:
- Bullet
```

ときには、setルールを条件付きで設定したい場合もあるでしょう。
その場合には_set-if_ルールを使用します。

```example
#let task(body, critical: false) = {
  set text(red) if critical
  [- #body]
}

#task(critical: true)[Food today?]
#task(critical: false)[Work deadline]
```

## showルール { #show-rules }

showルールを使えば、特定の種類の要素の外観を詳細に設定できます。
showルールの基本的な記述方法は、show-setルールです。
`{show}` キーワードの後に [セレクター]($selector)、コロン、setルールと続けて記述します。
セレクターの基本的な記述方法は [要素関数]($function/#element-functions)を置くことであり、setルールは選択された要素にのみ適用されます。
下の例では、見出しは紺色になり、他のテキストは黒色のままです。

```example
#show heading: set text(navy)

= This is navy-blue
But this stays black.
```

With show-set rules you can mix and match properties from different functions to
achieve many different effects. But they still limit you to what is predefined
in Typst. For maximum flexibility, you can instead write a _transformational_
show rule that defines how to format an element from scratch. To write such a
show rule, replace the set rule after the colon with an arbitrary [function]($function).
This function receives the element in question and can return arbitrary content.
The function is often defined inline as `{it => ..}` using the
[unnamed function syntax]($function/#unnamed). The function's parameter is
typically named `it` by convention.

The available [fields]($scripting/#fields) on the element passed to the function
match the parameters of the respective element function. Below, we define a show
rule that formats headings for a fantasy encyclopedia.

The show rule itself adds tilde characters around the title (these must be
escaped with a backslash because otherwise they would indicate a non-breaking
space), emphasizes the title with italics, and then displays the heading counter
after the title.

For this example, we also wanted center alignment and a different font. While we
could've added these set rules into the existing show rule, we instead added
them as separate show-set rules. This is good practice because now these rules
can still be overridden by later show-set rules in the document, keeping styling
composable. In contrast, set rules within a transformational show rule would not
be overridable anymore.

```example
#set heading(numbering: "(I)")
#show heading: set align(center)
#show heading: set text(font: "Inria Serif")
#show heading: it => block[
  \~
  #emph(it.body)
  #counter(heading).display(it.numbering)
  \~
]

= Dragon
With a base health of 15, the dragon is the most
powerful creature.

= Manticore
While less powerful than the dragon, the manticore
gets extra style points.
```

setルールと同様に、showルールは、現在のブロック内またはファイルの終わりまで有効です。

関数の代わりに、showルールのコロン右側は、要素に直接置換されるべきリテラル文字列またはコンテンツブロックを取ることもできます。
またshowルールのコロン左側は、以下に示すように、変換を適用する対象を定義する_セレクター_を受け取ることができます。

- **全て：** `{show: rest => ..}` \
  showルール以降の全てを変換する。
  個別の関数呼び出しでラップすることなく、複雑なレイアウトを文書全体に適用するのに便利です。

- **文字列：** `{show "Text": ..}` \
  設定した文字列に対して、スタイル変更や文字の置き換えを行います。

- **正規表現：** `{show regex("\w+"): ..}` \
  正規表現にマッチする文字列に対して、スタイル変更や文字の置き換えを行います。
  正規表現については[regex 関数]($regex)を参照してください。

- **関数やフィールド：** `{show heading.where(level: 1): ..}` \
  指定されたフィールドを持つ要素のみを変換します。
  例えば、レベル1の見出しのスタイルだけを変更したい場合などに有効です。

- **ラベル：** `{show <intro>: ..}` \
  指定されたラベルを持つ要素に対して適用する。
  ラベルについては[labelタイプ]($label)を参照してください。

```example
#show "Project": smallcaps
#show "badly": "great"

We started Project in 2019
and are still working on it.
Project is progressing badly.
```
