---
description: Typst の書式設定に関するリファレンス
---

# 形式

Typstには柔軟なスタイリングシステムがあり、お好みのスタイリングをドキュメントに自動的に適用します。
_setルール_ では要素の基本プロパティを設定できます。
しかし、やりたいことすべてに対応するビルトイン・プロパティがあるとは限りません。
このため、Typstは要素の外観を完全に再定義できる _showルール_ をさらにサポートしています。

## setルール { #set-rules }

setルールを使うと、要素の外観をカスタマイズできます。
これらは、`{set}` キーワード（マークアップでは `[#set]`）を前に置いた要素関数への[関数呼び出し]($function)として記述されます。
setルールには、その関数のオプションのパラメーターのみを指定することができます。
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

トップレベルのsetルールは、ファイルの最後まで適用されます。
ブロックの中にネストされると、そのブロックの終わりまで適用されます。
ブロックを使えば、ルールの効果を文書の特定のセグメントに限定できます。
以下では、content ブロックを用いてスコープすることで、特定のリストのスタイルのみを変更しています。

```example
This list is affected: #[
  #set list(marker: [--])
  - Dash
]

This one is not:
- Bullet
```

ときには、setルールを条件付きで設定したい場合もあるでしょう。
その場合には _set-if_ ルールを使用します。

```example
#let task(body, critical: false) = {
  set text(red) if critical
  [- #body]
}

#task(critical: true)[Food today?]
#task(critical: false)[Work deadline]
```

## showルール { #show-rules }

showルールを使えば、特定のタイプの要素の外観を深くカスタマイズできます。
showルールの基本的な形式は、show-setルールです。
`{show}` キーワードの後に [selector]($selector)、コロン、setルールを続けて記述します。
selector の基本的な形式は、[element関数]($function/#element-functions)であり、setルールは選択された要素にのみ適用されます。
下の例では、見出しは紺色になり、他のテキストは黒色のままです。

```example
#show heading: set text(navy)

= This is navy-blue
But this stays black.
```

show-setルールを使えば、さまざまな関数のプロパティを組み合わせることが可能です。
しかし、Typstであらかじめ定義されているものに限定されます。
最大限の柔軟性を得るには、要素をゼロからフォーマットする方法を定義するshowルールを書くことができます。
このようなshowルールを書くには、コロンの後のsetルールを任意の[関数]($function)に置き換えてください。
この関数は対象の要素を受け取り、任意の内容を返すことができます。
関数に渡されたエレメントで利用可能な[フィールド]($scripting/#fields)は、それぞれのelement関数のパラメーターと一致します。
以下は、ファンタジー百科事典の見出しをフォーマットするshowルールを定義する例です。

```example
#set heading(numbering: "(I)")
#show heading: it => block[
  #set align(center)
  #set text(font: "Inria Serif")
  \~ #emph(it.body)
     #counter(heading).display() \~
]

= Dragon
With a base health of 15, the
dragon is the most powerful
creature.

= Manticore
While less powerful than the
dragon, the manticore gets
extra style points.
```

setルールと同様に、showルールは、現在のブロックまたはファイルの終わりまで有効です。

関数の代わりに、showルールのコロン右側は、要素に直接置換されるべきリテラル文字列またはコンテンツブロックを取ることもできます。
またshowルールのコロン左側は、以下に示すように、変換を適用する対象を定義する _selector_ を受け取ることができます。

- **すべて：** `{show: rest => ..}` \
  showルール以降のすべてを変換する。
  個別の関数呼び出しでラップすることなく、複雑なレイアウトをドキュメント全体に適用するのに便利です。

- **文字列：** `{show "Text": ..}` \
  設定した文字列に対して、スタイル変更や文字の置き換えを行います。

- **正規表現：** `{show regex("\w+"): ..}` \
  正規表現にマッチする文字列に対して、スタイル変更や文字の置き換えを行います。
  正規表現については[regex 関数]($regex)を参照してください。

- **関数やフィールド：** `{show heading.where(level: 1): ..}` \
  指定されたフィールドを持つ要素のみを変換します。
  たとえば、レベル 1 の見出しのスタイルだけを変更したい場合などに有効です。

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
