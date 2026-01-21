<<<<<<< HEAD
use typst_utils::singleton;

use crate::diag::{bail, SourceResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, dict, elem, scope, Args, Cast, Construct, Content, Dict, NativeElement, Packed,
    Smart, Unlabellable, Value,
};
use crate::introspection::{Count, CounterUpdate, Locatable};
use crate::layout::{Em, HAlignment, Length, OuterHAlignment};
use crate::model::Numbering;

/// テキストコンテンツの論理的な区分。
///
/// Typstは _インラインレベル_ の要素を自動的に段落にまとめます。
/// インラインレベルの要素には、[テキスト]($text)、 [水平方向の空白]($h)、
/// [ボックス]($box)、[インライン数式]($math.equation)が含まれます。
///
/// 段落を区切るには、空行（または明示的な[`parbreak`]）を使用します。
/// 段落は、任意のブロックレベルの要素
/// （[`block`]、[`place`]、またはこれらのいずれかとして表示されるもの）によっても自動的に中断されます。
///
/// `par`要素は主にsetルールにおいて段落のプロパティに影響を与えるために使用されますが、
/// その引数を明示的に独立した段落として表示するためにも使用できます。
/// その場合、
/// その段落の本文にはブロックレベルのコンテンツを含めることはできません。
///
/// # ボックスとブロック
/// 上記の通り、通常、段落はインラインレベルのコンテンツのみを含みます。
/// しかし、[`box`]でラップすることで、
/// あらゆる種類のブロックレベルのコンテンツを段落に組み込むことができます。
///
/// 逆に、インラインレベルのコンテンツを[`block`]でラップすることにより、
/// コンテンツを段落から分離できます。
/// この場合、そのコンテンツはどの段落にも属さなくなります。
/// なぜこれが重要なのか、また単にコンテンツの前後に段落区切りを追加することとどう異なるのかについては、
/// 次のセクションをお読みください。
///
/// # 何が段落になるのか？
/// インラインレベルのコンテンツをドキュメントに追加すると、
/// Typstは自動的にそれを段落でラップします。
/// しかし、一般的なドキュメントには、見出しやキャプションなど、
/// 意味的に段落の一部ではないテキストも含まれます。
///
/// Typstがインラインレベルのコンテンツを
/// 段落でラップするルールは次の通りです。
///
/// - ドキュメントのルート（最上位）にある全てのテキストは段落でラップされます。
///
/// - コンテナ（`block`など）内のテキストは、
///   そのコンテナにブロックレベルのコンテンツが含まれている場合にのみ段落でラップされます。
///   コンテンツが全てインラインレベルである場合は、段落は作成されません。
///
/// 組版された文書では、テキストが段落の一部になったかどうかはすぐにはわかりません。
/// しかし、いくつかの理由からこれは依然として重要です。
///
/// - `first-line-indent`などの特定の段落スタイルは正式な段落に対してのみ適用され、
/// 任意のテキストには適用されません。
///    同様に、`par`に対するshowルールももちろん段落に対してのみ適用されます。
///
/// - 段落とその他のテキストを適切に区別することは、
/// スクリーンリーダーなどの支援技術を利用する人々が文書を正しく読み進め、理解するのに役立ちます。
/// 現在はTypstがアクセシブルなPDFをまだ出力しないため、
/// この仕組みはHTMLエクスポートにのみ適用されますが、
/// 近い将来PDFへのサポートも計画されています。
///
/// - HTMLエクスポートでは、段落に対してのみ`<p>`タグが生成されます。
///
/// 独自の再利用可能なコンポーネントを作成する際には、
/// Typstが段落を作成するかどうかを自分で制御できますし、制御すべきです。
/// テキストを単に段落区切りで囲むのではなく、
/// `block`で囲むことで段落を作成させないようにできます。
/// 逆に、コンテナ内のコンテンツの後に`parbreak`を追加することで、
/// たとえ1つの単語であっても段落にすることができます。
/// これは、[非タイト]($list.tight)リストがその項目を強制的に段落にさせるために行う手法の例です。
///
/// # 例
=======
use ecow::eco_format;
use typst_utils::singleton;

use crate::diag::{HintedStrResult, SourceResult, StrResult, bail};
use crate::engine::Engine;
use crate::foundations::{
    AlternativeFold, Args, Cast, CastInfo, Construct, Content, Dict, Fold, FromValue,
    IntoValue, NativeElement, Packed, Reflect, Smart, Unlabellable, Value, cast, dict,
    elem, scope,
};
use crate::introspection::{Count, CounterUpdate, Locatable, Tagged, Unqueriable};
use crate::layout::{Abs, Em, HAlignment, Length, OuterHAlignment, Ratio, Rel};
use crate::model::Numbering;

/// A logical subdivison of textual content.
///
/// Typst automatically collects _inline-level_ elements into paragraphs.
/// Inline-level elements include [text], [horizontal spacing]($h),
/// [boxes]($box), and [inline equations]($math.equation).
///
/// To separate paragraphs, use a blank line (or an explicit [`parbreak`]).
/// Paragraphs are also automatically interrupted by any block-level element
/// (like [`block`], [`place`], or anything that shows itself as one of these).
///
/// The `par` element is primarily used in set rules to affect paragraph
/// properties, but it can also be used to explicitly display its argument as a
/// paragraph of its own. Then, the paragraph's body may not contain any
/// block-level content.
///
/// # Boxes and blocks
/// As explained above, usually paragraphs only contain inline-level content.
/// However, you can integrate any kind of block-level content into a paragraph
/// by wrapping it in a [`box`].
///
/// Conversely, you can separate inline-level content from a paragraph by
/// wrapping it in a [`block`]. In this case, it will not become part of any
/// paragraph at all. Read the following section for an explanation of why that
/// matters and how it differs from just adding paragraph breaks around the
/// content.
///
/// # What becomes a paragraph?
/// When you add inline-level content to your document, Typst will automatically
/// wrap it in paragraphs. However, a typical document also contains some text
/// that is not semantically part of a paragraph, for example in a heading or
/// caption.
///
/// The rules for when Typst wraps inline-level content in a paragraph are as
/// follows:
///
/// - All text at the root of a document is wrapped in paragraphs.
///
/// - Text in a container (like a `block`) is only wrapped in a paragraph if the
///   container holds any block-level content. If all of the contents are
///   inline-level, no paragraph is created.
///
/// In the laid-out document, it's not immediately visible whether text became
/// part of a paragraph. However, it is still important for various reasons:
///
/// - Certain paragraph styling like `first-line-indent` will only apply to
///   proper paragraphs, not any text. Similarly, `par` show rules of course
///   only trigger on paragraphs.
///
/// - A proper distinction between paragraphs and other text helps people who
///   rely on Assistive Technology (AT) (such as screen readers) navigate and
///   understand the document properly.
///
/// - PDF export will generate a `P` tag only for paragraphs.
/// - HTML export will generate a `<p>` tag only for paragraphs.
///
/// When creating custom reusable components, you can and should take charge
/// over whether Typst creates paragraphs. By wrapping text in a [`block`]
/// instead of just adding paragraph breaks around it, you can force the absence
/// of a paragraph. Conversely, by adding a [`parbreak`] after some content in a
/// container, you can force it to become a paragraph even if it's just one
/// word. This is, for example, what [non-`tight`]($list.tight) lists do to
/// force their items to become paragraphs.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// #set par(
///   first-line-indent: 1em,
///   spacing: 0.65em,
///   justify: true,
/// )
///
/// We proceed by contradiction.
/// Suppose that there exists a set
/// of positive integers $a$, $b$, and
/// $c$ that satisfies the equation
/// $a^n + b^n = c^n$ for some
/// integer value of $n > 2$.
///
/// Without loss of generality,
/// let $a$ be the smallest of the
/// three integers. Then, we ...
/// ```
<<<<<<< HEAD
#[elem(scope, title = "Paragraph")]
pub struct ParElem {
    /// 行間。
    ///
    /// leadingは、
    /// ある行の[下端]($text.bottom-edge)と次の行の[上端]($text.top-edge)との間隔を定義します。
    /// デフォルトでは、これら2つのプロパティはフォントによって決まりますが、
    /// テキストのsetルールを使用して手動で設定することもできます。
    ///
    /// top-edge、bottom-edge、およびleadingを設定することで、
    /// ベースライン間の距離を一定に揃えることも可能です。
    /// 例えば、leadingを `{1em}`、top-edgeを`{0.8em}`、
    /// bottom-edgeを `{-0.2em}` に設定すると、
    /// ちょうど`{2em}`のベースライン間隔になります。
    /// top-edgeとbottom-edgeの値の正確な配分が最初の行と最後の行の境界に影響を与えます。
    #[resolve]
    #[default(Em::new(0.65).into())]
    pub leading: Length,

    /// 段落間の間隔。
    ///
    /// leadingと同様に、
    /// このプロパティはある段落の最終行の下端と、
    /// 次の段落の最初の行の上端との間隔を定義します。
    ///
    /// 段落が、段落ではない[`block`]に隣接している場合、
    /// そのブロックの[`above`]($block.above)または[`below`]($block.below)プロパティが段落間の間隔よりも優先されます。
    /// 例えば、
    /// 見出しはより良い外観のためにデフォルトで下側の間隔を狭くしています。
    #[resolve]
    #[default(Em::new(1.2).into())]
    pub spacing: Length,

    /// 行内でテキストを両端揃えするかどうか。
    ///
    /// [text関数の`hyphenate`プロパティ]($text.hyphenate)が`{auto}`に設定され、
    /// かつ現在の言語が認識されている場合、
    /// 両端揃えが行われた段落ではハイフネーションが有効になります。
    ///
    /// 最後の行が[両端揃えされた改行]($linebreak.justify)で終わらない限り、
    /// 現在の[alignment]($align.alignment)は依然として
    /// 最終行の配置に影響を与えることに注意してください。
    #[default(false)]
    pub justify: bool,

    /// 改行位置の決定方法
    ///
    /// このプロパティがデフォルトの`{auto}`に設定されている場合、
    /// 両端揃えされた段落に対して最適化された改行が行われます。
    /// また、段落が不揃いであっても最適化された改行を有効にすることで、
    /// テキストの見栄えが向上することがあります。
=======
#[elem(scope, title = "Paragraph", Locatable, Tagged)]
pub struct ParElem {
    /// The spacing between lines.
    ///
    /// Leading defines the spacing between the [bottom edge]($text.bottom-edge)
    /// of one line and the [top edge]($text.top-edge) of the following line. By
    /// default, these two properties are up to the font, but they can also be
    /// configured manually with a text set rule.
    ///
    /// By setting top edge, bottom edge, and leading, you can also configure a
    /// consistent baseline-to-baseline distance. You could, for instance, set
    /// the leading to `{1em}`, the top-edge to `{0.8em}`, and the bottom-edge
    /// to `{-0.2em}` to get a baseline gap of exactly `{2em}`. The exact
    /// distribution of the top- and bottom-edge values affects the bounds of
    /// the first and last line.
    ///
    /// ```preview
    /// // Color palette
    /// #let c = (
    ///   par-line: aqua.transparentize(60%),
    ///   leading-line: blue,
    ///   leading-text: blue.darken(20%),
    ///   spacing-line: orange.mix(red).darken(15%),
    ///   spacing-text: orange.mix(red).darken(20%),
    /// )
    ///
    /// // A sample text for measuring font metrics.
    /// #let sample-text = [A]
    ///
    /// // Number of lines in each paragraph
    /// #let n-lines = (4, 4, 2)
    /// #let annotated-lines = (4, 8)
    ///
    /// // The wide margin is for annotations
    /// #set page(width: 350pt, margin: (x: 20%))
    ///
    /// #context {
    ///   let text-height = measure(sample-text).height
    ///   let line-height = text-height + par.leading.to-absolute()
    ///
    ///   let jumps = n-lines
    ///     .map(n => ((text-height,) * n).intersperse(par.leading))
    ///     .intersperse(par.spacing)
    ///     .flatten()
    ///
    ///   place(grid(
    ///     ..jumps
    ///       .enumerate()
    ///       .map(((i, h)) => if calc.even(i) {
    ///         // Draw a stripe for the line
    ///         block(height: h, width: 100%, fill: c.par-line)
    ///       } else {
    ///         // Put an annotation for the gap
    ///         let sw(a, b) = if h == par.leading { a } else { b }
    ///
    ///         align(end, block(
    ///           height: h,
    ///           outset: (right: sw(0.5em, 1em)),
    ///           stroke: (
    ///             left: none,
    ///             rest: 0.5pt + sw(c.leading-line, c.spacing-line),
    ///           ),
    ///           if i / 2 <= sw(..annotated-lines) {
    ///             place(horizon, dx: 1.3em, text(
    ///               0.8em,
    ///               sw(c.leading-text, c.spacing-text),
    ///               sw([leading], [spacing]),
    ///             ))
    ///           },
    ///         ))
    ///       })
    ///   ))
    ///
    ///   // Mark top and bottom edges
    ///   place(
    ///     // pos: top/bottom edge
    ///     // dy: Δy to the last mark
    ///     // kind: leading/spacing
    ///     for (pos, dy, kind) in (
    ///       (bottom, text-height, "leading"),
    ///       (top, par.leading, "leading"),
    ///       (bottom, (n-lines.first() - 1) * line-height - par.leading, "spacing"),
    ///       (top, par.spacing, "spacing"),
    ///     ) {
    ///       v(dy)
    ///
    ///       let c-text = c.at(kind + "-text")
    ///       let c-line = c.at(kind + "-line")
    ///
    ///       place(end, box(
    ///         height: 0pt,
    ///         grid(
    ///           columns: 2,
    ///           column-gutter: 0.2em,
    ///           align: pos,
    ///           move(
    ///             // Compensate optical illusion
    ///             dy: if pos == top { -0.2em } else { 0.05em },
    ///             text(0.8em, c-text)[#repr(pos) edge],
    ///           ),
    ///           line(length: 1em, stroke: 0.5pt + c-line),
    ///         ),
    ///       ))
    ///     },
    ///   )
    /// }
    ///
    /// #set par(justify: true)
    /// #set text(luma(25%), overhang: false)
    /// #show ". ": it => it + parbreak()
    /// #lorem(55)
    /// ```
    #[default(Em::new(0.65).into())]
    pub leading: Length,

    /// The spacing between paragraphs.
    ///
    /// Just like leading, this defines the spacing between the bottom edge of a
    /// paragraph's last line and the top edge of the next paragraph's first
    /// line.
    ///
    /// When a paragraph is adjacent to a [`block`] that is not a paragraph,
    /// that block's [`above`]($block.above) or [`below`]($block.below) property
    /// takes precedence over the paragraph spacing. Headings, for instance,
    /// reduce the spacing below them by default for a better look.
    #[default(Em::new(1.2).into())]
    pub spacing: Length,

    /// Whether to justify text in its line.
    ///
    /// Hyphenation will be enabled for justified paragraphs if the
    /// [text function's `hyphenate` property]($text.hyphenate) is set to
    /// `{auto}` and the current language is known.
    ///
    /// Note that the current [alignment]($align.alignment) still has an effect
    /// on the placement of the last line except if it ends with a
    /// [justified line break]($linebreak.justify).
    ///
    /// By default, Typst only changes the spacing between words to achieve
    /// justification. However, you can also allow it to adjust the spacing
    /// between individual characters using the
    /// [`justification-limits` property]($par.justification-limits).
    #[default(false)]
    pub justify: bool,

    /// How much the spacing between words and characters may be adjusted during
    /// justification.
    ///
    /// When justifying text, Typst needs to stretch or shrink a line to the
    /// full width of the measure. To achieve this, by default, it adjusts the
    /// spacing between words. Additionally, it can also adjust the spacing
    /// between individual characters. This property allows you to configure
    /// lower and upper bounds for these adjustments.
    ///
    /// The property accepts a dictionary with two entries, `spacing` and
    /// `tracking`, each containing a dictionary with the keys `min` and `max`.
    /// The `min` keys define down to which lower bound gaps may be shrunk while
    /// the `max` keys define up to which upper bound they may be stretched.
    ///
    /// - The `spacing` entry defines how much the width of spaces between words
    ///   may be adjusted. It is closely related to [`text.spacing`] and its
    ///   `min` and `max` keys accept [relative lengths]($relative), just like
    ///   the `spacing` property.
    ///
    ///   A `min` value of `{100%}` means that spaces should retain their normal
    ///   size (i.e. not be shrunk), while a value of `{90% - 0.01em}` would
    ///   indicate that a space can be shrunk to a width of 90% of its normal
    ///   width minus 0.01× the current font size. Similarly, a `max` value of
    ///   `{100% + 0.02em}` means that a space's width can be increased by 0.02×
    ///   the current font size. The ratio part must always be positive. The
    ///   length part, meanwhile, must not be positive for `min` and not be
    ///   negative for `max`.
    ///
    ///   Note that spaces may still be expanded beyond the `max` value if there
    ///   is no way to justify the line otherwise. However, other means of
    ///   justification (e.g. spacing apart characters if the `tracking` entry
    ///   is configured accordingly) are first used to their maximum.
    ///
    /// - The `tracking` entry defines how much the spacing between letters may
    ///   be adjusted. It is closely related to [`text.tracking`] and its `min`
    ///   and `max` keys accept [lengths]($length), just like the `tracking`
    ///   property. Unlike `spacing`, it does not accept relative lengths
    ///   because the base of the relative length would vary for each character,
    ///   leading to an uneven visual appearance. The behavior compared to
    ///   `spacing` is as if the base was `{100%}`.
    ///
    ///   Otherwise, the `min` and `max` values work just like for `spacing`. A
    ///   `max` value of `{0.01em}` means that additional spacing amounting to
    ///   0.01× of the current font size may be inserted between every pair of
    ///   characters. Note that this also includes the gaps between spaces and
    ///   characters, so for spaces the values of `tracking` act in addition to
    ///   the values for `spacing`.
    ///
    /// If you only specify one of `spacing` or `tracking`, the other retains
    /// its previously set value (or the default if it was not previously set).
    ///
    /// If you want to enable character-level justification, a good value for
    /// the `min` and `max` keys is around `{0.01em}` to `{0.02em}` (negated for
    /// `min`). Using the same value for both gives a good baseline, but
    /// tweaking the two values individually may produce more balanced results,
    /// as demonstrated in the example below. Be careful not to set the bounds
    /// too wide, as it quickly looks unnatural.
    ///
    /// Using character-level justification is an impactful microtypographical
    /// technique that can improve the appearance of justified text, especially
    /// in narrow columns. Note though that character-level justification does
    /// not work with every font or language. For example, cursive fonts connect
    /// letters. Using character-level justification would lead to jagged
    /// connections.
    ///
    /// ```example:"Character-level justification"
    /// #let example(name) = columns(2, gutter: 10pt)[
    ///   #place(top, float: true, scope: "parent", strong(name))
    /// >>> Anne Christine Bayley (1~June 1934 – 31~December 2024) was an
    /// >>> English surgeon. She was awarded the Order of the British Empire
    /// >>> for her research into HIV/AIDS patients in Zambia and for
    /// >>> documenting the spread of the disease among heterosexual patients in
    /// >>> Africa. In addition to her clinical work, she was a lecturer and
    /// >>> head of the surgery department at the University of Zambia School of
    /// >>> Medicine. In the 1990s, she returned to England, where she was
    /// >>> ordained as an Anglican priest. She continued to be active in Africa
    /// >>> throughout her retirement years.
    /// <<<   /* Text from https://en.wikipedia.org/wiki/Anne_Bayley */
    /// ]
    ///
    /// #set page(width: 440pt, height: 21em, margin: 15pt)
    /// #set par(justify: true)
    /// #set text(size: 0.8em)
    ///
    /// #grid(
    ///   columns: (1fr, 1fr),
    ///   gutter: 20pt,
    ///   {
    ///     // These are Typst's default limits.
    ///     set par(justification-limits: (
    ///       spacing: (min: 100% * 2 / 3, max: 150%),
    ///       tracking: (min: 0em, max: 0em),
    ///     ))
    ///     example[Word-level justification]
    ///   },
    ///   {
    ///     // These are our custom character-level limits.
    ///     set par(justification-limits: (
    ///       tracking: (min: -0.01em, max: 0.02em),
    ///     ))
    ///     example[Character-level justification]
    ///   },
    /// )
    /// ```
    #[fold]
    pub justification_limits: JustificationLimits,

    /// How to determine line breaks.
    ///
    /// When this property is set to `{auto}`, its default value, optimized line
    /// breaks will be used for justified paragraphs. Enabling optimized line
    /// breaks for ragged paragraphs may also be worthwhile to improve the
    /// appearance of the text.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set page(width: 207pt)
    /// #set par(linebreaks: "simple")
    /// Some texts feature many longer
    /// words. Those are often exceedingly
    /// challenging to break in a visually
    /// pleasing way.
    ///
    /// #set par(linebreaks: "optimized")
    /// Some texts feature many longer
    /// words. Those are often exceedingly
    /// challenging to break in a visually
    /// pleasing way.
    /// ```
    pub linebreaks: Smart<Linebreaks>,

<<<<<<< HEAD
    /// 段落の最初の行のインデント。
    ///
    /// デフォルトでは、
    /// 連続する段落のうち最初の行のみがインデントされます
    /// （文書やコンテナの先頭の段落、あるいは他のブロックレベル要素に続く段落はインデントされません）。
    ///
    /// 全ての段落をインデントしたい場合は、
    /// インデントの`amount`（長さ）と`{all: true}`を含む辞書を渡してください。
    /// `all`が辞書から省略された場合、
    /// デフォルトでは`{false}`になります。
    ///
    /// タイポグラフィの慣例として、段落の区切りは段落間の空白か最初の行のインデントのどちらかで示されます。
    /// 次の設定を検討してみてください。
    /// - [段落の`spacing`]($par.spacing)を
    ///   `{set par(spacing: 0.65em)}`を使用して[`leading`]($par.leading)と同じ長さまで減らす
    /// - [ブロックの`spacing`]($block.spacing)
    ///   （デフォルトでは段落の間隔を継承します）を`{set block(spacing: 1.2em)}`を使用して
    ///   元の段落間隔と同じ長さまで増やす
=======
    /// The indent the first line of a paragraph should have.
    ///
    /// By default, only the first line of a consecutive paragraph will be
    /// indented (not the first one in the document or container, and not
    /// paragraphs immediately following other block-level elements).
    ///
    /// If you want to indent all paragraphs instead, you can pass a dictionary
    /// containing the `amount` of indent as a length and the pair
    /// `{all: true}`. When `all` is omitted from the dictionary, it defaults to
    /// `{false}`.
    ///
    /// By typographic convention, paragraph breaks are indicated either by some
    /// space between paragraphs or by indented first lines. Consider
    /// - reducing the [paragraph `spacing`]($par.spacing) to the
    ///   [`leading`]($par.leading) using `{set par(spacing: 0.65em)}`
    /// - increasing the [block `spacing`]($block.spacing) (which inherits the
    ///   paragraph spacing by default) to the original paragraph spacing using
    ///   `{set block(spacing: 1.2em)}`
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set block(spacing: 1.2em)
    /// #set par(
    ///   first-line-indent: 1.5em,
    ///   spacing: 0.65em,
    /// )
    ///
    /// The first paragraph is not affected
    /// by the indent.
    ///
    /// But the second paragraph is.
    ///
    /// #line(length: 100%)
    ///
    /// #set par(first-line-indent: (
    ///   amount: 1.5em,
    ///   all: true,
    /// ))
    ///
    /// Now all paragraphs are affected
    /// by the first line indent.
    ///
    /// Even the first one.
    /// ```
    pub first_line_indent: FirstLineIndent,

<<<<<<< HEAD
    /// 段落の最初の行以外全ての行のインデント。
=======
    /// The indent that all but the first line of a paragraph should have.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set par(hanging-indent: 1em)
    ///
    /// #lorem(15)
    /// ```
<<<<<<< HEAD
    #[resolve]
    pub hanging_indent: Length,

    /// 段落のコンテンツ。
=======
    pub hanging_indent: Length,

    /// The contents of the paragraph.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[required]
    pub body: Content,
}

#[scope]
impl ParElem {
    #[elem]
    type ParLine;
}

<<<<<<< HEAD
/// How to determine line breaks in a paragraph.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum Linebreaks {
    /// シンプルなファーストフィット方式で改行位置を決定します。
    Simple,
    /// 段落全体の改行位置を最適化します。
    ///
    /// Typstは改行を計算する際に段落全体を考慮し、
    /// より均等に埋まった行を生成しようとします。
=======
/// Configures how justification may distribute spacing.
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct JustificationLimits {
    /// Limits for spacing, relative to the space width.
    spacing: Option<Limits<Rel>>,
    /// Limits for tracking, _in addition_ to the glyph width.
    tracking: Option<Limits<Length>>,
}

impl JustificationLimits {
    /// Access the spacing limits.
    pub fn spacing(&self) -> &Limits<Rel> {
        self.spacing.as_ref().unwrap_or(&Limits::SPACING_DEFAULT)
    }

    /// Access the tracking limits.
    pub fn tracking(&self) -> &Limits<Length> {
        self.tracking.as_ref().unwrap_or(&Limits::TRACKING_DEFAULT)
    }
}

cast! {
    JustificationLimits,
    self => {
        let mut dict = Dict::new();
        if let Some(spacing) = &self.spacing {
            dict.insert("spacing".into(), spacing.into_value());
        }
        if let Some(tracking) = &self.tracking {
            dict.insert("tracking".into(), tracking.into_value());
        }
        Value::Dict(dict)
    },
    mut dict: Dict => {
        let spacing = dict
            .take("spacing")
            .ok()
            .map(|v| Limits::cast(v, "spacing"))
            .transpose()?;
        let tracking = dict
            .take("tracking")
            .ok()
            .map(|v| Limits::cast(v, "tracking"))
            .transpose()?;
        dict.finish(&["spacing", "tracking"])?;
        Self { spacing, tracking }
    },
}

impl Fold for JustificationLimits {
    fn fold(self, outer: Self) -> Self {
        Self {
            spacing: self.spacing.fold_or(outer.spacing),
            tracking: self.tracking.fold_or(outer.tracking),
        }
    }
}

impl Default for JustificationLimits {
    fn default() -> Self {
        Self {
            spacing: Some(Limits::SPACING_DEFAULT),
            tracking: Some(Limits::TRACKING_DEFAULT),
        }
    }
}

/// Determines the minimum and maximum size by or to which spacing may be shrunk
/// and stretched.
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct Limits<T> {
    /// Minimum allowable adjustment.
    pub min: T,
    /// Maximum allowable adjustment.
    pub max: T,
}

impl Limits<Rel> {
    const SPACING_DEFAULT: Self = Self {
        min: Rel::new(Ratio::new(2.0 / 3.0), Length::zero()),
        max: Rel::new(Ratio::new(1.5), Length::zero()),
    };
}

impl Limits<Length> {
    const TRACKING_DEFAULT: Self = Self { min: Length::zero(), max: Length::zero() };
}

impl<T: Reflect> Reflect for Limits<T> {
    fn input() -> CastInfo {
        Dict::input()
    }

    fn output() -> CastInfo {
        Dict::output()
    }

    fn castable(value: &Value) -> bool {
        Dict::castable(value)
    }
}

impl<T: IntoValue> IntoValue for Limits<T> {
    fn into_value(self) -> Value {
        Value::Dict(dict! {
            "min" => self.min,
            "max" => self.max,
        })
    }
}

impl<T> Limits<T> {
    /// Not implementing `FromValue` here because we want to pass the `field`
    /// for the error message. Ideally, the casting infrastructure would be
    /// bit more flexible here.
    fn cast(value: Value, field: &str) -> HintedStrResult<Self>
    where
        T: FromValue + Limit,
    {
        let mut dict: Dict = value.cast()?;
        let mut take = |key, check: fn(T) -> StrResult<T>| {
            dict.take(key)?
                .cast::<T>()
                .map_err(|hinted| hinted.message().clone())
                .and_then(check)
                .map_err(|err| {
                    eco_format!("`{key}` value of `{field}` is invalid ({err})")
                })
        };
        let min = take("min", Limit::checked_min)?;
        let max = take("max", Limit::checked_max)?;
        dict.finish(&["min", "max"])?;
        Ok(Self { min, max })
    }
}

impl<T> Fold for Limits<T> {
    fn fold(self, _: Self) -> Self {
        self
    }
}

/// Validation for limit components.
trait Limit: Sized {
    fn checked_min(self) -> StrResult<Self>;
    fn checked_max(self) -> StrResult<Self>;
}

impl Limit for Length {
    fn checked_min(self) -> StrResult<Self> {
        if self.abs > Abs::zero() || self.em > Em::zero() {
            bail!("length must be negative or zero");
        }
        Ok(self)
    }

    fn checked_max(self) -> StrResult<Self> {
        if self.abs < Abs::zero() || self.em < Em::zero() {
            bail!("length must be positive or zero");
        }
        Ok(self)
    }
}

impl Limit for Rel<Length> {
    fn checked_min(self) -> StrResult<Self> {
        if self.rel <= Ratio::zero() {
            bail!("ratio must be positive");
        }
        self.abs.checked_min()?;
        Ok(self)
    }

    fn checked_max(self) -> StrResult<Self> {
        if self.rel <= Ratio::zero() {
            bail!("ratio must be positive");
        }
        self.abs.checked_max()?;
        Ok(self)
    }
}

/// How to determine line breaks in a paragraph.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum Linebreaks {
    /// Determine the line breaks in a simple first-fit style.
    Simple,
    /// Optimize the line breaks for the whole paragraph.
    ///
    /// Typst will try to produce more evenly filled lines of text by
    /// considering the whole paragraph when calculating line breaks.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    Optimized,
}

/// Configuration for first line indent.
#[derive(Debug, Default, Copy, Clone, PartialEq, Hash)]
pub struct FirstLineIndent {
    /// The amount of indent.
    pub amount: Length,
    /// Whether to indent all paragraphs, not just consecutive ones.
    pub all: bool,
}

cast! {
    FirstLineIndent,
    self => Value::Dict(self.into()),
    amount: Length => Self { amount, all: false },
    mut dict: Dict => {
        let amount = dict.take("amount")?.cast()?;
        let all = dict.take("all").ok().map(|v| v.cast()).transpose()?.unwrap_or(false);
        dict.finish(&["amount", "all"])?;
        Self { amount, all }
    },
}

impl From<FirstLineIndent> for Dict {
    fn from(indent: FirstLineIndent) -> Self {
        dict! {
            "amount" => indent.amount,
            "all" => indent.all,
        }
    }
}

<<<<<<< HEAD
/// 段落区切り。
///
/// 新しい段落を開始します。
/// 特に[forループ]($scripting/#loops)などのコード内で使用する場合に便利です。
/// 複数の連続した段落区切りは、単一の段落区切りにまとめられます。
///
/// # 例
=======
/// A paragraph break.
///
/// This starts a new paragraph. Especially useful when used within code like
/// [for loops]($scripting/#loops). Multiple consecutive
/// paragraph breaks collapse into a single one.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// #for i in range(3) {
///   [Blind text #i: ]
///   lorem(5)
///   parbreak()
/// }
/// ```
///
<<<<<<< HEAD
/// # 構文
/// この関数を呼び出す代わりに、
/// マークアップ内に空行を挿入することで段落区切りを作成できます。
=======
/// # Syntax
/// Instead of calling this function, you can insert a blank line into your
/// markup to create a paragraph break.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
#[elem(title = "Paragraph Break", Unlabellable)]
pub struct ParbreakElem {}

impl ParbreakElem {
    /// Get the globally shared paragraph element.
    pub fn shared() -> &'static Content {
        singleton!(Content, ParbreakElem::new().pack())
    }
}

impl Unlabellable for Packed<ParbreakElem> {}

<<<<<<< HEAD
/// 段落の行。
///
/// この要素はsetルールを用いた行番号の設定にのみ使用され、
/// 直接配置することはできません。
///
/// [`numbering`]($par.line.numbering)オプションは、
/// 番号付け形式を指定して行番号を有効化するために使用されます。
=======
/// A paragraph line.
///
/// This element is exclusively used for line number configuration through set
/// rules and cannot be placed.
///
/// The [`numbering`]($par.line.numbering) option is used to enable line
/// numbers by specifying a numbering format.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// >>> #set page(margin: (left: 3em))
/// #set par.line(numbering: "1")
///
/// Roses are red. \
/// Violets are blue. \
/// Typst is there for you.
/// ```
///
<<<<<<< HEAD
/// `numbering`オプションには、予め定義された[番号付けパターン]($numbering)か、
/// スタイル付きコンテンツを返す関数のいずれかを指定します。
/// show-setルールを用いてnumberingを`{none}`に設定することで、
/// 特定要素内のテキストの行番号を無効にすることができます。
=======
/// The `numbering` option takes either a predefined
/// [numbering pattern]($numbering) or a function returning styled content. You
/// can disable line numbers for text inside certain elements by setting the
/// numbering to `{none}` using show-set rules.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// >>> #set page(margin: (left: 3em))
/// // Styled red line numbers.
/// #set par.line(
///   numbering: n => text(red)[#n]
/// )
///
/// // Disable numbers inside figures.
/// #show figure: set par.line(
///   numbering: none
/// )
///
/// Roses are red. \
/// Violets are blue.
///
/// #figure(
///   caption: [Without line numbers.]
/// )[
///   Lorem ipsum \
///   dolor sit amet
/// ]
///
/// The text above is a sample \
/// originating from distant times.
/// ```
///
<<<<<<< HEAD
/// この要素は、行番号の[alignment]($par.line.number-align)[margin]($par.line.number-margin)など、
/// 行の番号付けのさまざまな設定を制御できる追加オプションを提供します。
/// さらに、
/// [`numbering-scope`]($par.line.numbering-scope)オプションを使用すると、
/// ページごとに番号をリセットするかどうかの制御が可能です。
#[elem(name = "line", title = "Paragraph Line", keywords = ["line numbering"], Construct, Locatable)]
pub struct ParLine {
    /// 各行を番号付けする方法。
    /// [番号付けパターンまたは関数]($numbering)を指定できます。
=======
/// This element exposes further options which may be used to control other
/// aspects of line numbering, such as its [alignment]($par.line.number-align)
/// or [margin]($par.line.number-margin). In addition, you can control whether
/// the numbering is reset on each page through the
/// [`numbering-scope`]($par.line.numbering-scope) option.
#[elem(name = "line", title = "Paragraph Line", keywords = ["line numbering"], Construct, Locatable)]
pub struct ParLine {
    /// How to number each line. Accepts a
    /// [numbering pattern or function]($numbering) taking a single number.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// >>> #set page(margin: (left: 3em))
    /// #set par.line(numbering: "I")
    ///
    /// Roses are red. \
    /// Violets are blue. \
    /// Typst is there for you.
    /// ```
<<<<<<< HEAD
    #[ghost]
    pub numbering: Option<Numbering>,

    /// 各行に付随する行番号の配置。
    ///
    /// デフォルトの`{auto}`は、
    /// 行番号が余白や現在のテキストの方向を考慮しつつ、
    /// テキストから離れる方向へ水平に伸びるスマートな設定を示します。
=======
    ///
    /// ```example
    /// >>> #set page(width: 200pt, margin: (left: 3em))
    /// #set par.line(
    ///   numbering: i => if calc.rem(i, 5) == 0 or i == 1 { i },
    /// )
    ///
    /// #lorem(60)
    /// ```
    #[ghost]
    pub numbering: Option<Numbering>,

    /// The alignment of line numbers associated with each line.
    ///
    /// The default of `{auto}` indicates a smart default where numbers grow
    /// horizontally away from the text, considering the margin they're in and
    /// the current text direction.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// >>> #set page(margin: (left: 3em))
    /// #set par.line(
    ///   numbering: "I",
    ///   number-align: left,
    /// )
    ///
    /// Hello world! \
    /// Today is a beautiful day \
    /// For exploring the world.
    /// ```
    #[ghost]
    pub number_align: Smart<HAlignment>,

<<<<<<< HEAD
    /// 行番号を表示する位置の余白。
    ///
    /// _注意_: 複数段組みの文書では、
    /// この設定に関わらず最後の段の段落につく行番号が常に`{end}`の余白（左から右のテキストでは右の余白、
    /// 右から左のテキストでは左の余白）に表示されます。
    /// 現時点では、
    /// この挙動を変更することはできません。
=======
    /// The margin at which line numbers appear.
    ///
    /// _Note:_ In a multi-column document, the line numbers for paragraphs
    /// inside the last column will always appear on the `{end}` margin (right
    /// margin for left-to-right text and left margin for right-to-left),
    /// regardless of this configuration. That behavior cannot be changed at
    /// this moment.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// >>> #set page(margin: (right: 3em))
    /// #set par.line(
    ///   numbering: "1",
    ///   number-margin: right,
    /// )
    ///
    /// = Report
    /// - Brightness: Dark, yet darker
    /// - Readings: Negative
    /// ```
    #[ghost]
    #[default(OuterHAlignment::Start)]
    pub number_margin: OuterHAlignment,

<<<<<<< HEAD
    /// 行番号とテキストの間隔。
    ///
    /// デフォルトの値である `{auto}` では、ページ幅に応じて間隔が自動調整され、
    /// ほとんどの場合において適切な間隔が得られます。
=======
    /// The distance between line numbers and text.
    ///
    /// The default value of `{auto}` results in a clearance that is adaptive to
    /// the page width and yields reasonable results in most cases.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// >>> #set page(margin: (left: 3em))
    /// #set par.line(
    ///   numbering: "1",
    ///   number-clearance: 4pt,
    /// )
    ///
    /// Typesetting \
    /// Styling \
    /// Layout
    /// ```
    #[ghost]
    #[default]
    pub number_clearance: Smart<Length>,

<<<<<<< HEAD
    /// 行番号をリセットするタイミングを制御する。
    ///
    /// _注意:_ 行番号のスコープは、
    /// ページラン（改ページが明示的に挿入されていない連続したページ）内で統一されている必要があります。
    /// そのため、setルールによる設定は、
    /// ページコンテンツの前、通常は文書の最初などで定義することが望ましいです。
=======
    /// Controls when to reset line numbering.
    ///
    /// _Note:_ The line numbering scope must be uniform across each page run (a
    /// page run is a sequence of pages without an explicit pagebreak in
    /// between). For this reason, set rules for it should be defined before any
    /// page content, typically at the very start of the document.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// >>> #set page(margin: (left: 3em))
    /// #set par.line(
    ///   numbering: "1",
    ///   numbering-scope: "page",
    /// )
    ///
    /// First line \
    /// Second line
    /// #pagebreak()
    /// First line again \
    /// Second line again
    /// ```
    #[ghost]
    #[default(LineNumberingScope::Document)]
    pub numbering_scope: LineNumberingScope,
}

impl Construct for ParLine {
    fn construct(_: &mut Engine, args: &mut Args) -> SourceResult<Content> {
        bail!(args.span, "cannot be constructed manually");
    }
}

/// Possible line numbering scope options, indicating how often the line number
/// counter should be reset.
///
/// Note that, currently, manually resetting the line number counter is not
/// supported.
<<<<<<< HEAD
#[derive(Debug, Cast, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineNumberingScope {
    /// 行番号カウンターが文書全体にまたがり、
    /// 決して自動的にリセットされないことを示します。
    Document,
    /// 行番号カウンターが各新規ページの
    /// 先頭でリセットされることを示します。
=======
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum LineNumberingScope {
    /// Indicates that the line number counter spans the whole document, i.e.,
    /// it's never automatically reset.
    Document,
    /// Indicates that the line number counter should be reset at the start of
    /// every new page.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    Page,
}

/// A marker used to indicate the presence of a line.
///
/// This element is added to each line in a paragraph and later searched to
/// find out where to add line numbers.
<<<<<<< HEAD
#[elem(Construct, Locatable, Count)]
=======
#[elem(Construct, Unqueriable, Locatable, Count)]
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
pub struct ParLineMarker {
    #[internal]
    #[required]
    pub numbering: Numbering,

    #[internal]
    #[required]
    pub number_align: Smart<HAlignment>,

    #[internal]
    #[required]
    pub number_margin: OuterHAlignment,

    #[internal]
    #[required]
    pub number_clearance: Smart<Length>,
}

impl Construct for ParLineMarker {
    fn construct(_: &mut Engine, args: &mut Args) -> SourceResult<Content> {
        bail!(args.span, "cannot be constructed manually");
    }
}

impl Count for Packed<ParLineMarker> {
    fn update(&self) -> Option<CounterUpdate> {
        // The line counter must be updated manually by the root flow.
        None
    }
}
