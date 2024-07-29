---
description: Typstのスクリプティング機能を用いて、ドキュメントを自動化しましょう。
---

# スクリプティング

Typstには強力なスクリプティング言語が組み込まれています。これにより、ドキュメントを自動化し、コードを使用してより洗練されたスタイルを作成できます。
以下は、スクリプティングの概念の概要です。

## 表达式 { #expressions }

Typst 里面，标记和代码相互交融在一起。
除了最常用的文档元素，其他所有均是由 _函数_ 生成。
为了尽可能的便利，Typst 设计了精巧的语法，用来将代码嵌入在标记中：用 `#`(井号) 来引入一个代码表达式，
表达式结束后，再恢复到正常的标记语法解析。
有些字符能够使其后字符继续解析为表达式，如果想将其解释为文本，可以用分号(`;`)来强制结束表达式解析。

```example
#emph[Hello] \
#emoji.face \
#"hello".len()
```

上面示例展示了一些用到的表达式，
有[函数调用]($function)，
[字段访问]($scripting/#fields)，
[方法调用]($scripting/#methods)。
本章余下部分讲解更多类型表达式。
有几种表达式与井号语法不一致（比如二元运算表达式），
如果需要插入标记模式中，需要使用圆括号，比如 `[#(1+2)]`。

## 块 { #blocks }
为了架构代码以及将标记嵌入代码中，Typst 设计了两种 _块_：

- **代码块：** `{{ let x = 1; x + 2 }}` \
  编写代码时，一个计算过程可能需要分解为多个语句，创建多个中间变量，等等。
  可以将多个表达式组成一个代码块，就像一个表达式一样。在代码块中，多个表达式由换行符或者分号分割。
  其中每个表达式的输出值被合并起来，作为代码块的值。
  有些表达式没有有用的输出，比如 `{let}` 绑定返回 `{none}`，与其他值合并，没有任何效果。

- **文档内容块** `{[*Hey* there!]}` \
  使用文档内容块，可以将标记/文档内容作为可编程值，存储到变量，传送给[函数]($function)。
  文档内容块由方括号包裹，可以包含任何标记。
  一个文档内容块产生一个 [content 类型]($content)的值。
  文档内容块可以后缀参数形式任意多个传递给函数，就是说，`{list[A][B]}` 等效于 `{list([A], [B])}`。

文档内容块和代码块可以相互内嵌，下面示例中，`{[hello]}` 与 `{a + [ the ] + b}` 合并，生成 `{[hello from the *world*]}`。


```example
#{
  let a = [from]
  let b = [*world*]
  [hello ]
  a + [ the ] + b
}
```

## 绑定和解构 { #bindings }

上面已经展示，变量由 `{let}` 绑定定义。
= 符号后表达式的值被赋值给变量，这里赋值可以被省略，如果没有赋值，变量会初始化为 `{none}`。
`{let}` 关键词也可以用来生成一个[自定义的有名函数]($function/#definitions)。
let 绑定的变量可以在接下来的块中或者文档中被访问。

```example
#let name = "Typst"
This is #name's documentation.
It explains #name.

#let add(x, y) = x + y
Sum is #add(2, 3).
```

let 绑定也常用来解构[数组]($array)和[字典]($dictionary)，
解构时，等号左边的形式需要与数组或字典相似，
`..` 模式操作符只可被使用一次，用来指代数组或字典剩余的条目。

```example
#let (x, y) = (1, 2)
The coordinates are #x, #y.

#let (a, .., b) = (1, 2, 3, 4)
The first element is #a.
The last element is #b.

#let books = (
  Shakespeare: "Hamlet",
  Homer: "The Odyssey",
  Austen: "Persuasion",
)

#let (Austen,) = books
Austen wrote #Austen.

#let (Homer: h) = books
Homer wrote #h.

#let (Homer, ..other) = books
#for (author, title) in other [
  #author wrote #title.
]
```

在解构匹配模式中，可以使用 _ 下划线来丢弃一个元素。

```example
#let (_, y, _) = (1, 2, 3)
The y coordinate is #y.
```

解构形式也可用于函数的参数列表中...

```example
#let left = (2, 4, 5)
#let right = (3, 2, 6)
#left.zip(right).map(
  ((a,b)) => a + b
)
```

... 和普通赋值的左半部分，这通常用于交换两个变量的值。

```example
#{
  let a = 1
  let b = 2
  (a, b) = (b, a)
  [a = #a, b = #b]
}
```

## 条件控制 { #conditionals }

使用条件控制语句，可以根据某种条件是否满足，来展示或计算不同的事情。
Typst 设计了 `{if}`, `{else if}`, `{else}` 表达式。
当条件值为 `{true}` 时，条件语句会返回 if 从句的值，否则返回 else 从句的值。

```example
#if 1 < 2 [
  This is shown
] else [
  This is not.
]
```

每个分支从句可以写为语句块或者文档内容块。

- `{if condition {..}}`
- `{if condition [..]}`
- `{if condition [..] else {..}}`
- `{if condition [..] else if condition {..} else [..]}`

## 循环控制 { #loops }

使用循环控制语句，可以反复的显示文档内容或者计算。
Typst 支持两种循环控制： `{for}` 循环和 `{while}` 循环。
`{for}` 循环用来遍历特定集合，`{while}` 循环根据某条件是否满足来决定是否再次迭代循环。
和块类似，循环结构 _合并_ 每一次迭代循环的结果。

下面示例中，for 循环生成了三句话，然后将其合并成一个文档内容。
while 循环生成数个长度为1的数组，然后将其合并成一个大数组。

```example
#for c in "ABC" [
  #c is a letter.
]

#let n = 2
#while n < 10 {
  n = (n * 2) - 1
  (n,)
}
```

for 循环可以遍历多种集合：

- `{for letter in "abc" {..}}` \
  遍历[字符串]($str)的每个字符。
  （专业的说，是遍历字符串的每个形位符，大多时候，一个形位符对应一个字符/码位，
  然而，有些表情标记符号由多个码位组成，但它仍然是一个形位符）。

- `{for value in array {..}}` \
  遍历[数组]($array)中的条目。[let 绑定]($scripting/#bindings)中的解构语法也可使用于此。

- `{for pair in dict {..}}` \
  遍历[字典]($dictionary)的键值对。键值对也可以用 `{for (key, value) in dict {..}}` 语法解构。

Typst 用 `{break}` 和 `{continue}` 语句来控制循环的执行，
`{break}` 用来跳出循环，`{continue}` 用来提前结束本次循环，然后执行下一次循环。

```example
#for letter in "abc nope" {
  if letter == " " {
    break
  }

  letter
}
```
循环体可以是代码块，也可以是文档内容块：

- `{for .. in collection {..}}`
- `{for .. in collection [..]}`
- `{while condition {..}}`
- `{while condition [..]}`

## 字段 { #fields }
可以使用 _点号_ 来访问一个值的字段，这个值可以是：
- 有特定键的[字典]($dictionary)，
- 有变体的[符号]($symbol)，
- 有定义的[模块]($module)，
- 有特定字段的[文档元素]($content)，可访问的字段与文档元素的[构造函数]($function/#element-functions)参数相匹配。

```example
#let dict = (greet: "Hello")
#dict.greet \
#emoji.face

#let it = [= Heading]
#it.body \
#it.level
```

## Methods
A _method call_ is a convenient way to call a function that is scoped to a
value's [type]($type). For example, we can call the [`str.len`]($str.len)
function in the following two equivalent ways:

```example
#str.len("abc") is the same as
#"abc".len()
```

The structure of a method call is `{value.method(..args)}` and its equivalent
full function call is `{type(value).method(value, ..args)}`. The documentation
of each type lists it's scoped functions. You cannot currently define your own
methods.

```example
#let array = (1, 2, 3, 4)
#array.pop() \
#array.len() \

#("a, b, c"
    .split(", ")
    .join[ --- ])

#"abc".len() is the same as
#str.len("abc")
```

There are a few special functions that modify the value they are called on (e.g.
[`array.push`]($array.push)). These functions _must_ be called in method form.
In some cases, when the method is only called for its side effect, its return
value should be ignored (and not participate in joining). The canonical way to
discard a value is with a let binding: `{let _ = array.remove(1)}`.

## 模块 { #modules }

一个 Typst 工程项目可以拆解为多个 _模块_ 文件。
一个模块可以使用多种方式引用其他模块的文档内容和定义。

- **插入：** `{include "bar.typ"}` \
  计算 `bar.typ` 路径文件，返回其[文档内容]($content)结果。

- **导入：** `{import "bar.typ"}` \
  在路径 `bar.typ` 处加载文档并插入结果
  [模块]($module) 作为 `bar` 进入当前范围（不带后缀拓展名的文档名）。您可以使用 `as` 关键字重命名导入的模块：
  `{import "bar.typ" as baz}`

- **Import items:** `{import "bar.typ": a, b}` \
  Evaluates the file at the path `bar.typ`, extracts the values of the variables
  `a` and `b` (that need to be defined in `bar.typ`, e.g. through `{let}`
  bindings) and defines them in the current file. Replacing `a, b` with `*`
  loads all variables defined in a module. You can use the `as` keyword to
  rename the individual items: `{import "bar.typ": a as one, b as two}`

- **导入项目:** `{import "bar.typ": a, b}` \
  加载路径为 `bar.typ` 的文档，提取变量
  `a` 和 `b` (需要在 `bar.typ` 中定义，例如通过 `{let}`
  绑定) 的值，并在当前文档中定义它们。将 `a, b` 替换为 `*`
  可以加载模块中定义的所有变量。您可以使用 `as` 关键字来
  重命名各个项目: `{import "bar.typ": a as one, b as two}`

除了使用路径，也可以使用 [模块值]($module)，如下面示例：

```example
#import emoji: face
#face.grin
```

## 包 { #packages }
可以创建并导入 Typst _包_，在多个工程项目中进行复用。
一个包的导入需有三部分指定：包命名空间，包名称，包版本号。

```example
>>> #let add(x, y) = x + y
<<< #import "@preview/example:0.1.0": add
#add(2, 7)
```

`preview` 包命名空间包含了 Typst 社区分享的包。可以在 [package]($packages) 章节搜索可用的社区分享包。

在本地使用 Typst 时，可以创建本地包。更多详情参考 [包仓库](https://github.com/typst/packages)。

## 操作符 { #operators }

下表类出了所有一元和二元操作符的作用、参数数量(一元、二元)和优先级（优先级越高，越优先执行)

| 操作符   | 作用说明                           | 参数数量  | 优先级 |
|:----------:|---------------------------------|:------:|:----------:|
|  `{-}`     | 负号                            | 一元  |     7      |
|  `{+}`     | 正号，无作用，仅仅为了对称性      | 一元  |     7      |
|  `{*}`     | 乘号                            | 二元 |     6      |
|  `{/}`     | 除号                            | 二元 |     6      |
|  `{+}`     | 加号                            | 二元 |     5      |
|  `{-}`     | 减号                            | 二元 |     5      |
|  `{==}`    | 等号                            | 二元 |     4      |
|  `{!=}`    | 不等于号                        | 二元 |     4      |
|  `{<}`     | 小于号                          | 二元 |     4      |
|  `{<=}`    | 小于等于号                      | 二元 |     4      |
|  `{>}`     | 大于号                         | 二元 |     4      |
|  `{>=}`    | 大于等于号                     | 二元 |     4      |
|  `{in}`    | 属于                          | 二元 |     4      |
| `{not in}` | 不属于                        | 二元 |     4      |
|  `{not}`   | 逻辑非                        | 一元  |     3      |
|  `{and}`   | 短路式逻辑并                   | 二元 |     3      |
|  `{or}`    | 短路式逻辑或                   | 二元 |     2      |
|  `{=}`     | 赋值                          | 二元 |     1      |
|  `{+=}`    | 相加赋值                      | 二元 |     1      |
|  `{-=}`    | 相减赋值                      | 二元 |     1      |
|  `{*=}`    | 相乘赋值                      | 二元 |     1      |
|  `{/=}`    | 相除赋值                      | 二元 |     1      |

[semver]: https://semver.org/
