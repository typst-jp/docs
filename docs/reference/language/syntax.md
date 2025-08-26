---
description: |
   Typstの構文に関するコンパクトなリファレンスです。詳細については、言語のマークアップモード、数式モード、およびコードモードを参照してください。
---

# 構文

Typstはマークアップ言語です。
これは、シンプルな構文を使用して一般的なレイアウトタスクを簡単に行えるということです。
Typstの軽量なマークアップ構文は、文書を簡単かつ自動的にスタイリングできるsetルールとshowルールによって補完されています。
これらすべては、組み込み関数およびユーザー定義関数を備えた、緊密に統合されたスクリプト言語によって支えられています。

## モード { #modes }

Typstには3種類の構文モードがあります。マークアップモード、数式モード、そしてコードモードです。
Typst文書では、マークアップモードがデフォルトであり、数式モードでは数式を書くことができ、コードモードではTypstのスクリプト機能を利用することができます。

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
以下の表は、利用可能なすべてのマークアップと、その構文と使用法について詳しく学ぶための最適なページへのリンクを示しています。

| 名称             | 例                       | 参照                                 |
| ---------------- | ------------------------ | ------------------------------------ |
| 段落区切り       | 空行                     | [`parbreak`]($parbreak)              |
| 強調(太字)       | `[*strong*]`             | [`strong`]($strong)                  |
| 強調(イタリック) | `[_emphasis_]`           | [`emph`]($emph)                      |
| rawテキスト      | ``[`print(1)`]``         | [`raw`]($raw)                        |
| リンク           | `[https://typst.app/]`   | [`link`]($link)                      |
| ラベル           | `[<intro>]`              | [`label`]($label)                    |
| 参照             | `[@intro]`               | [`ref`]($ref)                        |
| 見出し           | `[= Heading]`            | [`heading`]($heading)                |
| 箇条書きリスト   | `[- item]`               | [`list`]($list)                      |
| 番号付きリスト   | `[+ item]`               | [`enum`]($enum)                      |
| 用語リスト       | `[/ Term: description]`  | [`terms`]($terms)                    |
| 数式             | `[$x^2$]`                | [Math]($category/math)               |
| 改行             | `[\]`                    | [`linebreak`]($linebreak)            |
| スマートクオート | `['single' or "double"]` | [`smartquote`]($smartquote)          |
| 短縮記号         | `[~]`, `[---]`           | [Symbols]($category/symbols/sym)     |
| コード構文       | `[#rect(width: 1cm)]`    | [Scripting]($scripting/#expressions) |
| 文字エスケープ   | `[Tweet at us \#ad]`     | [Below]($category/syntax/#escapes)   |
| コメント         | `[/* block */]`, `[// line]` | [Below]($category/syntax/#comments) |

## 数式モード { #math }

数式モードは、数式を組版するために使用される特別なマークアップモードです。
数式を `[$]` の文字で囲むことによって、数式モードに入ることができます。
これはマークアップモードとコードモードの両方で機能します。
数式が少なくとも1つのスペースで始まり終わる場合、その数式は独自のブロックに組版されます（例：`[$ x^2 $]`）。
インライン数式は、スペースを省略することで作成できます（例：`[$x^2$]`）。
以下に、数式モードに特有の構文の概要を示します。

| 名称                    | 例                      | 参照                                 |
| ----------------------- | ----------------------- | ------------------------------------ |
| インライン数式          | `[$x^2$]`               | [Math]($category/math)               |
| ブロック数式            | `[$ x^2 $]`             | [Math]($category/math)               |
| 下付き添え字            | `[$x_1$]`               | [`attach`]($category/math/attach)    |
| 上付き添え字            | `[$x^2$]`               | [`attach`]($category/math/attach)    |
| 分数                    | `[$1 + (a+b)/5$]`       | [`frac`]($math.frac)                 |
| 改行                    | `[$x \ y$]`             | [`linebreak`]($linebreak)            |
| 揃え位置                | `[$x &= 2 \ &= 3$]`     | [Math]($category/math)               |
| 変数アクセス            | `[$#x$, $pi$]`          | [Math]($category/math)               |
| フィールドアクセス      | `[$arrow.r.long$]`      | [Scripting]($scripting/#fields)      |
| 暗黙の乗算              | `[$x y$]`               | [Math]($category/math)               |
| 短縮記号                | `[$->$]`, `[$!=$]`      | [Symbols]($category/symbols/sym)     |
| 数式内のテキスト/文字列 | `[$a "is natural"$]`    | [Math]($category/math)               |
| 数式関数呼び出し        | `[$floor(x)$]`          | [Math]($category/math)               |
| コード構文              | `[$#rect(width: 1cm)$]` | [Scripting]($scripting/#expressions) |
| 文字エスケープ          | `[$x\^2$]`              | [Below]($category/syntax/#escapes)   |
| コメント                | `[$/* comment */$]`     | [Below]($category/syntax/#comments)  |

## コードモード { #code }

コードブロックや式の中では、新しい式は先頭に`#`を付けずに始めることができます。
多くの構文要素は式に特有のものです。
以下に、コードモードで利用可能なすべての構文の一覧表を示します。

| 名称                       | 例                            | 参照                                  |
| -------------------------- | ----------------------------- | ------------------------------------- |
| none                       | `{none}`                      | [`none`]($reference/foundations/none) |
| 自動                       | `{auto}`                      | [`auto`]($reference/foundations/auto) |
| ブール値                   | `{false}`, `{true}`           | [`bool`]($reference/foundations/bool) |
| 整数                       | `{10}`, `{0xff}`              | [`int`]($reference/foundations/int)   |
| 浮動小数点数               | `{3.14}`, `{1e5}`             | [`float`]($reference/foundations/float) |
| 長さ                       | `{2pt}`, `{3mm}`, `{1em}`, .. | [`length`]($reference/layout/length)  |
| 角度                       | `{90deg}`, `{1rad}`           | [`angle`]($reference/layout/angle)    |
| 比率                       | `{2fr}`                       | [`fraction`]($reference/layout/fraction) |
| 割合                       | `{50%}`                       | [`ratio`]($reference/layout/ratio)    |
| 文字列                     | `{"hello"}`                   | [`str`]($reference/foundations/str)   |
| ラベル                     | `{<intro>}`                   | [`label`]($reference/foundations/label) |
| 数式                       | `[$x^2$]`                     | [Math]($category/math)                |
| rawテキスト                | ``[`print(1)`]``              | [`raw`]($reference/text/raw)          |
| 変数アクセス               | `{x}`                         | [Scripting]($scripting/#blocks)       |
| コードブロック             | `{{ let x = 1; x + 2 }}`      | [Scripting]($scripting/#blocks)       |
| コンテンツブロック         | `{[*Hello*]}`                 | [Scripting]($scripting/#blocks)       |
| 括弧付き式                 | `{(1 + 2)}`                   | [Scripting]($scripting/#blocks)       |
| 配列                       | `{(1, 2, 3)}`                 | [Array]($array)                 |
| 辞書                       | `{(a: "hi", b: 2)}`           | [Dictionary]($dictionary)       |
| 単項演算子                 | `{-x}`                        | [Scripting]($scripting/#operators)    |
| 二項演算子                 | `{x + y}`                     | [Scripting]($scripting/#operators)    |
| 代入                       | `{x = 1}`                     | [Scripting]($scripting/#operators)    |
| フィールドアクセス         | `{x.y}`                       | [Scripting]($scripting/#fields)       |
| メソッド呼び出し           | `{x.flatten()}`               | [Scripting]($scripting/#methods)      |
| 関数呼び出し               | `{min(x, y)}`                 | [Function]($function)           |
| 引数展開                   | `{min(..nums)}`               | [Arguments]($arguments)         |
| 無名関数                   | `{(x, y) => x + y}`           | [Function]($function)           |
| letバインディング          | `{let x = 1}`                 | [Scripting]($scripting/#bindings)     |
| 名前付き関数               | `{let f(x) = 2 * x}`          | [Function]($function)           |
| setルール                  | `{set text(14pt)}`            | [Styling]($styling/#set-rules)        |
| set-ifルール               | `{set text(..) if .. }`       | [Styling]($styling/#set-rules)        |
| show-setルール             | `{show heading: set block(..)}` | [Styling]($styling/#show-rules)     |
| 関数付きshowルール         | `{show raw: it => {..}}`      | [Styling]($styling/#show-rules)       |
| show-everythingルール      | `{show: template}`            | [Styling]($styling/#show-rules)       |
| コンテキスト式             | `{context text.lang}`         | [Context]($context)                   |
| 条件式                     | `{if x == 1 {..} else {..}}`  | [Scripting]($scripting/#conditionals) |
| forループ                  | `{for x in (1, 2, 3) {..}}`   | [Scripting]($scripting/#loops)        |
| whileループ                | `{while x < 10 {..}}`         | [Scripting]($scripting/#loops)        |
| ループ制御フロー           | `{break, continue}`           | [Scripting]($scripting/#loops)        |
| 関数からのリターン         | `{return x}`                  | [Function]($function)           |
| モジュールをインクルード   | `{include "bar.typ"}`         | [Scripting]($scripting/#modules)      |
| モジュールをインポート     | `{import "bar.typ"}`          | [Scripting]($scripting/#modules)      |
| モジュールからのインポート | `{import "bar.typ": a, b, c}` | [Scripting]($scripting/#modules)      |
| コメント                   | `{/* block */}`, `{// line}`  | [Below]($category/syntax/#comments)   |

## コメント { #comments }

コメントはTypstによって無視され、出力には含まれません。
これは古いバージョンを除外したり、注釈を追加したりするのに便利です。
単一行をコメントアウトするには、行の先頭に`//`を付けます。

```example
// our data barely supports
// this claim

We show with $p < 0.05$
that the difference is
significant.
```

コメントは `/*` と `*/` で囲むこともできます。この場合、コメントを複数行にわたって書くことができます。

```example
Our study design is as follows:
/* Somebody write this up:
   - 1000 participants.
   - 2x2 data design. */
```

## エスケープシーケンス { #escapes }

エスケープシーケンスは、Typstで入力が難しい特殊文字や他に特別な意味を持つ文字を挿入するために使用されます。
文字をエスケープするには、バックスラッシュをその前に置きます。
任意のUnicodeコードポイントを挿入するためには、16進エスケープシーケンス（`[\u{1f600}]`）を使用できます。
このエスケープシーケンスは[文字列]($str)でも機能します。

```example
I got an ice cream for
\$1.50! \u{1f600}
```

## パス { #paths }

Typstは、画像、Typstファイル、データファイルなどのリソース（他ファイル）を参照するためにファイルパスを必要とするさまざまな機能を持ちます。
パスは[文字列型]($str)で指定し、相対パスと絶対パスの2種類があります。

- **相対パス**は、その機能を呼び出すTypstファイルの場所からのパスを指定します。デフォルトはこれです。

  ```typ
  #image("images/logo.png")
  ```

- **絶対パス**はプロジェクトの _ルートディレクトリ_ からのパスを指定します。先頭を`/`から始めます。

  ```typ
  #image("/assets/logo.png")
  ```

### プロジェクトのルートディレクトリ

デフォルトでは、プロジェクトのルートディレクトリはメインのTypstファイルの親ディレクトリです。
セキュリティ上の理由により、ルートディレクトリの外にあるファイルは読み取れません。

プロジェクトのルートディレクトリとして特定のフォルダーを設定したい場合は、CLIの`--root`フラグを使用できます。
対象のファイルがそのフォルダーのサブディレクトリ内に含まれていることを確認してください。

```bash
typst compile --root .. file.typ
```

Webアプリでは、プロジェクト自体がルートディレクトリとして扱われます。
どのファイルをプレビューしていても（プレビューはファイルパネル内の各Typstファイルの横にある目のアイコンで切り替え可能）、
その中にあるすべてのファイルを常に読み取れます。

### パスとパッケージ

パッケージは、それ自身のディレクトリ内にあるファイルしか読み込むことができません。
その中では、絶対パスはプロジェクトのルートディレクトリではなく、パッケージのルートディレクトリを指します。
このため、パッケージはプロジェクトディレクトリ内のファイルを直接読み込むことができません。
パッケージがプロジェクトのリソース（たとえばロゴ画像など）を必要とする場合は、
あらかじめ読み込んだ画像を`{logo: image("mylogo.svg")}`のように名前付きパラメーターとして渡す必要があります。
その後でも、パッケージ内でsetルールを記述することで画像の見た目をカスタマイズすることも可能です。

将来的には、パスが[文字列とは異なるパス専用の型](https://github.com/typst/typst/issues/971)になるかもしれません。
そうすることで、そのパスがどこで生成されたかという情報を保持できるようになります。
これにより、異なるルートからリソースを読み込むことも可能になるでしょう。
