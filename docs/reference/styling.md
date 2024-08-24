---
description: Typst で文書のスタイル設定をするために必要な概念
---

# スタイル設定

Typst には柔軟なスタイル設定機能を持ち、出力される文書に対して自動的に任意のスタイル設定を適用します。
_setルール_ ではエレメントの基本プロパティを設定できます。
しかし、やりたいことすべてに対応するプロパティがあらかじめ実装されているとは限りません。
このため、Typst はエレメントの外観を完全に再定義できる _showルール_ もサポートしています。

## setルール { #set-rules }

set ルールを使うと、エレメントの外観をカスタマイズできます。
これらは、`{set}` キーワード（マークアップでは `[#set]`）を前に置いた [element 関数]($function/#element-functions) への[関数呼び出し]($function)として記述されます。
set ルールに指定できるのは、その関数のオプションのパラメーターだけです。
どのパラメーターがオプションであるかは、各関数のドキュメントを参照してください。
以下の例では、2 つの set ルールを使って、[フォント]($text.font)と[見出し番号]($heading.numbering)を変更しています。

```example
#set heading(numbering: "I.")
#set text(
  font: "New Computer Modern"
)

= Introduction
With set rules, you can style
your document.
```

set ルールは、そのまま記述するとファイルの最後まで適用されます。
ブロックの中にネストすると、そのブロックの終わりまで適用されます。
ブロックを使えば、set ルールの効果を指定した部分に限定できます。
以下では、content ブロックを用いてスコープすることで、特定のリストのスタイルのみを変更しています。

```example
This list is affected: #[
  #set list(marker: [--])
  - Dash
]

This one is not:
- Bullet
```

ときには、set ルールを条件付きで設定したい場合もあるでしょう。
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

show ルールを使えば、特定の種類のエレメントの外観を詳細に設定できます。
show ルールの基本的な記述方法は、show-set ルールです。
`{show}` キーワードの後に [セレクター]($selector)、コロン、set ルールと続けて記述します。
セレクターの基本的な記述方法は [element関数]($function/#element-functions) を置くことであり、set ルールは選択されたエレメントにのみ適用されます。
下の例では、見出しは紺色になり、他のテキストは黒色のままです。

```example
#show heading: set text(navy)

= This is navy-blue
But this stays black.
```

show-set ルールを使えば、さまざまな関数のプロパティを組み合わせることが可能です。
しかし、組み合わせられるプロパティは Typst であらかじめ定義されているものに限定されます。
最大限の柔軟性を得るには、エレメントをゼロからフォーマットする方法を定義する show ルールを書くことができます。
このような show ルールを書くには、コロンの後の set ルールを任意の[関数]($function)に置き換えてください。
この関数は対象のエレメントを受け取り、任意の内容を返すことができます。
関数に渡されたエレメントで利用可能な[フィールド]($scripting/#fields)は、それぞれの element 関数のパラメーターと一致します。
以下は、ファンタジー百科事典の見出しをフォーマットする show ルールを定義する例です。

```example
#set heading(numbering: "(I)")
#show heading: it => [
  #set align(center)
  #set text(font: "Inria Serif")
  \~ #emph(it.body)
     #counter(heading).display(
       it.numbering
     ) \~
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

set ルールと同様に、show ルールは、現在のブロック内またはファイルの終わりまで有効です。

関数の代わりに、show ルールのコロン右側は、エレメントに直接置換されるべきリテラル文字列またはコンテンツブロックを取ることもできます。
また show ルールのコロン左側は、以下に示すように、変換を適用する対象を定義する _セレクター_ を受け取ることができます。

- **すべて：** `{show: rest => ..}` \
  showルール以降のすべてを変換する。
  個別の関数呼び出しでラップすることなく、複雑なレイアウトを文書全体に適用するのに便利です。

- **文字列：** `{show "Text": ..}` \
  設定した文字列に対して、スタイル変更や文字の置き換えを行います。

- **正規表現：** `{show regex("\w+"): ..}` \
  正規表現にマッチする文字列に対して、スタイル変更や文字の置き換えを行います。
  正規表現については[regex 関数]($regex)を参照してください。

- **関数やフィールド：** `{show heading.where(level: 1): ..}` \
  指定されたフィールドを持つエレメントのみを変換します。
  たとえば、レベル 1 の見出しのスタイルだけを変更したい場合などに有効です。

- **ラベル：** `{show <intro>: ..}` \
  指定されたラベルを持つエレメントに対して適用する。
  ラベルについては[labelタイプ]($label)を参照してください。

```example
#show "Project": smallcaps
#show "badly": "great"

We started Project in 2019
and are still working on it.
Project is progressing badly.
```
