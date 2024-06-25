---
description: Typstのチュートリアルです。
---
# Typstで執筆するには

让我们开始吧！假设你被要求为大学写一份技术报告，报告将包含文字、数学公式、标题和图表。
首先，请在 Typst App 上创建一个新项目。你将会看见一个分为两个面板的编辑器：
用于撰写文档的代码面板和查看渲染文档的预览面板。

![Typst app screenshot](1-writing-app.png)

您在心中对报告内容的方向已经有了不错的想法。那么，让我们从写报告的介绍开始。
在编辑器面板中输入一些文本，您会注意到文本会立即显示在预览的页面上。

```example
In this report, we will explore the
various factors that influence fluid
dynamics in glaciers and how they
contribute to the formation and
behavior of these natural structures.
```

_在本教程中，我们将展示与此类似的代码示例。就像在 App 中一样，第一个面板包含标记文本，第二个面板显示文稿预览。我们缩小了页面以适应示例，以便您看得更清楚。_

下一步是添加标题并对一些文本进行强调。
Typst 对最常见的格式使用简单的标记（markup）。要添加标题，请输入 `=` 字符，要用斜体强调某些文本，请将其括在 `[_下划线_]` 中。

```example
= Introduction
In this report, we will explore the
various factors that influence _fluid
dynamics_ in glaciers and how they
contribute to the formation and
behavior of these natural structures.
```

这很简单！要添加新段落，只需在两行文本之间添加一个空行即可。
如果该段落需要副标题，请输入 `==` 而非 `=` 来生成它。
`=` 字符的数量决定了标题的嵌套级别。

现在我们想列出一些影响冰川动力学的因素。
为此，我们使用有序列表。对于列表中的每个项目，我们在行首输入 `+` 字符。
Typst 将自动对项目进行编号。

```example
+ The climate
+ The topography
+ The geology
```

如果我们想添加一个无序列表，则应使用 `-` 字符而非 `+` 字符。
我们还可以嵌套列表：
例如，我们可以通过缩进将无序列表作为一个子列表，添加到上面列表的第一项中。

```example
+ The climate
  - Temperature
  - Precipitation
+ The topography
+ The geology
```

## 加入图表

您认为图片对一份优秀的报告至关重要，所以现在让我们添加一个图片。
Typst 支持 PNG、JPEG、GIF 和 SVG 格式的图像。
要将图像文件添加到项目中，请先通过单击左侧边栏中的箱子图标打开 _文件面板_ 。
在这里，您可以看到项目中所有文件的列表。
目前，这里只有一个文件：您正在编辑的 Typst 主文件。
要上传其他文件，请单击右上角带有箭头的按钮。
这将打开一个上传对话框，您可以在其中选择要从计算机上传的文件。
请为你的报告选择一个图像文件。

![Upload dialog](1-writing-upload.png)

我们之前已经看到，特定的符号（称为 _标记（markup）_ ）在 Typst 中具有特定的含义。
我们可以使用 `=`、`-`、`+` 和 `_` 分别创建标题、列表和强调文本。
但是，若是为我们想要插入到文档中的所有内容都设置一个特殊符号，很快语法就会变得怪异且笨拙。
为此，Typst 只为最常见的内容保留标记符号。
其他所有内容都将通过 _函数_ 来插入。
为了使图像显示在页面上，我们使用 Typst 的 [`image`]($image) 函数。

```example
#image("glacier.jpg")
```

通常，函数会为一组 _参数_ 生成一些输出。
当您在标记模式中 _调用_ 函数时，在读取到您输入的参数后，Typst 会将结果（函数的 _返回值_ ）插入到文档中。
在我们的例子中， `image` 函数接受一个参数：图像文件的路径。
要在标记模式中调用函数，我们首先需要键入 `#` 字符，并且紧跟函数的名称。
然后，我们将参数括在括号中。
Typst 可识别参数列表中的许多不同数据类型，
我们的文件路径是一个较短的 [文本字符串]($str)，所以我们需要用双引号括起来。

插入的图像默认宽度为页面宽度。要更改宽度，请将 `width` 参数传递给 `image` 函数。
这是一个 _命名（named）_ 参数，因此被指定为 `name: value` 对。
如果有多个参数，则逗号分隔它们，所以我们首先需要在路径参数后面加上一个逗号。

```example
#image("glacier.jpg", width: 70%)
```

`width` 参数传入的参数表示一个 [相对长度]($relative)。
在我们的例子中，我们指定了一个百分比，确定图像应占据页面宽度的 `{70%}`。
我们也可以指定一个绝对值，如 `{1cm}` 或 `{0.7in}`。

就像文本一样，默认情况下，图像现在在页面左侧对齐，并且它也缺少说明（caption）。
让我们使用 [figure]($figure) 函数来解决这个问题。
该函数将图表的内容作为位置参数，将可选的说明（caption）作为命名参数。

在 `figure` 函数的参数列表中，Typst 已经处于代码模式。
这意味着，您现在可以在 `image` 函数调用的前面删除井号。
井号仅在标记模式时才用的到（它被用来消除函数调用中的文本歧义）。

标题由任意标记文本组成。
为了给函数提供标记文本，我们将其括在方括号中。此被称为 _内容块_ 。

```example
#figure(
  image("glacier.jpg", width: 70%),
  caption: [
    _Glaciers_ form an important part
    of the earth's climate system.
  ],
)
```

您继续编写报告，现在想要引用该图表。
为此，首先在图表上贴上标签。
标签用于唯一标识文档中的元素，你只需要通过在尖括号中将标签名称括起来，并在图表后加入。
然后，您可以通过在文本中键入 `[@]` 符号，后面跟标签名称来引用该图表。
标题和表达式也可以加入标签，以使其可引用。

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

到目前为止，我们已经将内容块（方括号中的标记文本）和字符串（双引号中的文本）传递给我们的函数。
两者都似乎包含文本，它们有什么区别呢？

内容块可以包含文本，也可以包含任何其他类型的标记、函数调用等，
而字符串实际上只是 _字符构成的串_，不包含其他内容。

例如，`image` 函数需要图像文件的路径，而将一段文本或另一张图像，作为图像的路径参数传递是没有意义的。
这就是为什么这里只允许输入字符串作为参数。
相反，字符串适用于任何需要内容块的地方，因为文本是一种有效的内容块。

</div>

## 添加参考文献

在撰写报告时，您需要引用资料来支持您的一些论证。
您可以使用 [`bibliography`]($bibliography) 函数向文档添加参考文献。
此函数需要参考文献文件的路径。

Typst 的原生参考文献格式是 [Hayagriva](https://github.com/typst/hayagriva/blob/main/docs/file-format.md)，
考虑到兼容性，Typst 同样支持 BibLaTeX 文件。
由于您的同学已经完成了文献调查并向您发送了 `.bib` 文件，您可以直接使用该文件。
请在文件面板中上传文件，以在 Typst App 中访问它。

一旦文档引入了参考文献，您就可以引用它了。
引用参考文献的语法与对标签的引用相同。在您第一次引用一个资料来源后，它就会出现在您文档的参考文献部分。
Typst 支持不同的引用和参考文献样式。有关的详细信息，请阅读 [参考]($bibliography.style)。

```example
= Methods
We follow the glacier melting models
established in @glacier-melt.

#bibliography("works.bib")
```

## 数学

充实了方法部分后，您继续完成这篇文稿的实质部分：数学公式。
Typst 具有内置的数学排版引擎，并使用自己的数学记号。
让我们从一个简单的等式开始。我们使用 `[$]` 符号括住它，让 Typst 知道它正在处理一个数学表达式：

```example
The equation $Q = rho A v + C$
defines the glacial flow rate.
```

公式是内联排版的，与周围的文本在同一行上。
如果你想把它放在它自己的新行上，你应该在其开头和结尾插入一个空格：

```example
The flow rate of a glacier is
defined by the following equation:

$ Q = rho A v + C $
```

我们可以看到，Typst 按原样显示单个字母 `Q`，`A`，`v` 和 `C`，而它将 `rho` 翻译成希腊字母。
数学模式将始终原样地显示单个字母。然而，多个字母将被视作为符号、变量或函数名称。
要隐式地表明这是单个字母之间的乘法，请在它们之间加入空格。

如果你想要一个由多个字母组成的变量，你可以用引号括起来：

```example
The flow rate of a glacier is given
by the following equation:

$ Q = rho A v + "time offset" $
```

您还需要在论文中加入一个求和公式。
我们可以使用 `sum` 符号，然后在下标和上标中指定求和的范围：

```example
Total displaced soil by glacial flow:

$ 7.32 beta +
  sum_(i=0)^nabla Q_i / 2 $
```

要向符号或变量添加下标，请输入 `_` 字符，然后输入下标。
同样，为上标使用 `^` 字符。
如果下标或上标由多个内容部分组成，则必须将它们括在圆括号中。

上面的例子还向我们展示了如何插入分数：
只需在分子和分母之间放置一个 `/` 字符，Typst 就会自动将其变成分数。
括号会被巧妙地解析，因此您可以像在计算器中一样输入数学表达式，
Typst 将用适当的符号替换括号中的子表达式。

```example
Total displaced soil by glacial flow:

$ 7.32 beta +
  sum_(i=0)^nabla
    (Q_i (a_i - epsilon)) / 2 $
```

并非所有数学结构都有特殊的语法。
作为替代，我们会使用函数，就像我们之前看到的 `image` 函数一样。
例如，要插入列向量，我们可以使用 [`vec`]($math.vec) 函数。
在数学模式下，函数调用不需要以 `#` 字符开头。

```example
$ v := vec(x_1, x_2, x_3) $
```

某些函数仅在数学模式下可用。
例如，[`cal`]($math.cal) 函数用于排版通常用于集合论的书法字母。
[参考中的数学部分]($category/math) 提供了数学模式支持的所有函数的完整列表。

还有一件事：许多符号（如箭头）有很多变体。
您可以通过在符号名称后附加点和修饰符名称来选择这些变体：

```example
$ a arrow.squiggly b $
```

这种记号在标记模式下也可以使用，但符号名称前面必须带有 `#sym`。
有关所有可用符号的列表，请参阅 [符号部分]($category/symbols/sym)。

## 回顾

现在您已经了解了如何在 Typst 中编写基本的文档，包括如何强调文本、编写列表、插入图像、对齐内容和排版数学表达式。
您还了解了 Typst 的函数机制。
Typst 允许您将更多种类的内容插入到文档中，例如 [表格]($table)、[形状]($category/visualize) 和 [代码块]($raw)。
您可以仔细阅读 [参考]($reference) 以了解有关这些函数和其他函数的更多信息。

目前，您已完成报告的编写。
您已经通过单击右上角的下载按钮保存了 PDF。
但您可能认为报告看起来不应该那么朴素。
在下一节中，我们将学习如何自定义文档的外观。
