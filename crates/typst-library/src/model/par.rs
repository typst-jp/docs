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
/// （[`block`]、[`place`]、またはこれらのいずれかとして表示されるもの）によっても自動的に区切られます。
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
/// Typstは自動的にそれを段落としてラップします。
/// しかし、一般的なドキュメントには、見出しやキャプションなど、
/// 意味的に段落の一部ではないテキストも含まれます。
///
/// Typstがインラインレベルのコンテンツを
/// 段落としてラップするルールは次の通りです。
///
/// - ドキュメントのルート（最上位）にある全てのテキストは段落としてラップされます。
///
/// - コンテナ（`block`など）内のテキストは、
///   そのコンテナにブロックレベルの要素が含まれている場合にのみ段落としてラップされます。
///   コンテナの内容が全てインラインレベル要素である場合は、段落は作成されません。
///
/// 組版された文書では、テキストが段落の一部になったかどうかはすぐにはわかりません。
/// しかし、いくつかの理由からこれは依然として重要です。
///
/// - `first-line-indent`などの特定の段落スタイルは正しい段落に対してのみ適用され、
/// 任意のテキストには適用されません。
///    同様に、`par`に対するshowルールももちろん段落に対してのみ適用されます。
///
/// - 段落とその他のテキストを適切に区別することは、
/// スクリーンリーダーなどの支援技術を利用する人々が文書を正しく読み解き、理解するのに役立ちます。
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
/// これは、[非タイト]($list.tight)リストがその項目を段落にするために行う手法の例です。
///
/// # 例
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
#[elem(scope, title = "Paragraph")]
pub struct ParElem {
    /// 行間。
    ///
    /// leadingは、
    /// ある行の[下端]($text.bottom-edge)と次の行の[上端]($text.top-edge)との間隔を定義します。
    /// デフォルトではこれら2つのプロパティはフォントによって決まりますが、
    /// テキストのsetルールを使用して手動で設定することもできます。
    ///
    /// top-edge、bottom-edge、およびleadingを設定することで、
    /// ベースライン間の距離を一定に揃えることも可能です。
    /// たとえば、leadingを `{1em}`、top-edgeを`{0.8em}`、
    /// bottom-edgeを'{-0.2em}'に設定すると、
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
    ///   デフォルトでは段落の間隔を継承します）を`{set block(spacing: 1.2em)}`を使用して
    ///   元の段落間隔と同じ長さまで増やす
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

    /// 段落の最初の行以外全ての行のインデント。
    ///
    /// ```example
    /// #set par(hanging-indent: 1em)
    ///
    /// #lorem(15)
    /// ```
    #[resolve]
    pub hanging_indent: Length,

    /// 段落の内容。
    #[required]
    pub body: Content,
}

#[scope]
impl ParElem {
    #[elem]
    type ParLine;
}

/// How to determine line breaks in a paragraph.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum Linebreaks {
    /// シンプルなファーストフィット方式で改行位置を決定します。
    Simple,
    /// 段落全体の改行位置を最適化します。
    ///
    /// Typstは改行を計算する際に段落全体を考慮し、
    /// より均等に埋まった行を生成しようとします。
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

/// A paragraph break.
///
/// This starts a new paragraph. Especially useful when used within code like
/// [for loops]($scripting/#loops). Multiple consecutive
/// paragraph breaks collapse into a single one.
///
/// # Example
/// ```example
/// #for i in range(3) {
///   [Blind text #i: ]
///   lorem(5)
///   parbreak()
/// }
/// ```
///
/// # Syntax
/// Instead of calling this function, you can insert a blank line into your
/// markup to create a paragraph break.
#[elem(title = "Paragraph Break", Unlabellable)]
pub struct ParbreakElem {}

impl ParbreakElem {
    /// Get the globally shared paragraph element.
    pub fn shared() -> &'static Content {
        singleton!(Content, ParbreakElem::new().pack())
    }
}

impl Unlabellable for Packed<ParbreakElem> {}

/// A paragraph line.
///
/// This element is exclusively used for line number configuration through set
/// rules and cannot be placed.
///
/// The [`numbering`]($par.line.numbering) option is used to enable line
/// numbers by specifying a numbering format.
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
/// The `numbering` option takes either a predefined
/// [numbering pattern]($numbering) or a function returning styled content. You
/// can disable line numbers for text inside certain elements by setting the
/// numbering to `{none}` using show-set rules.
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
/// This element exposes further options which may be used to control other
/// aspects of line numbering, such as its [alignment]($par.line.number-align)
/// or [margin]($par.line.number-margin). In addition, you can control whether
/// the numbering is reset on each page through the
/// [`numbering-scope`]($par.line.numbering-scope) option.
#[elem(name = "line", title = "Paragraph Line", keywords = ["line numbering"], Construct, Locatable)]
pub struct ParLine {
    /// How to number each line. Accepts a
    /// [numbering pattern or function]($numbering).
    ///
    /// ```example
    /// >>> #set page(margin: (left: 3em))
    /// #set par.line(numbering: "I")
    ///
    /// Roses are red. \
    /// Violets are blue. \
    /// Typst is there for you.
    /// ```
    #[ghost]
    pub numbering: Option<Numbering>,

    /// The alignment of line numbers associated with each line.
    ///
    /// The default of `{auto}` indicates a smart default where numbers grow
    /// horizontally away from the text, considering the margin they're in and
    /// the current text direction.
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

    /// The margin at which line numbers appear.
    ///
    /// _Note:_ In a multi-column document, the line numbers for paragraphs
    /// inside the last column will always appear on the `{end}` margin (right
    /// margin for left-to-right text and left margin for right-to-left),
    /// regardless of this configuration. That behavior cannot be changed at
    /// this moment.
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

    /// The distance between line numbers and text.
    ///
    /// The default value of `{auto}` results in a clearance that is adaptive to
    /// the page width and yields reasonable results in most cases.
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

    /// Controls when to reset line numbering.
    ///
    /// _Note:_ The line numbering scope must be uniform across each page run (a
    /// page run is a sequence of pages without an explicit pagebreak in
    /// between). For this reason, set rules for it should be defined before any
    /// page content, typically at the very start of the document.
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
#[derive(Debug, Cast, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineNumberingScope {
    /// Indicates that the line number counter spans the whole document, i.e.,
    /// it's never automatically reset.
    Document,
    /// Indicates that the line number counter should be reset at the start of
    /// every new page.
    Page,
}

/// A marker used to indicate the presence of a line.
///
/// This element is added to each line in a paragraph and later searched to
/// find out where to add line numbers.
#[elem(Construct, Locatable, Count)]
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
