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

/// テキストコンテンツの論理的な区分。
///
/// Typstは_インラインレベル_の要素を自動的に段落にまとめます。
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
/// - PDFエクスポートでは、段落に対してのみ`P`タグが生成されます。
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
#[elem(scope, title = "Paragraph", Locatable, Tagged)]
pub struct ParElem {
    /// 行間。
    ///
    /// leadingは、ある行の[下端]($text.bottom-edge)と次の行の
    /// [上端]($text.top-edge)との間隔を定義します。
    /// デフォルトでは、これら2つのプロパティはフォントによって決まりますが、
    /// テキストのsetルールを使用して手動で設定することもできます。
    ///
    /// top-edge、bottom-edge、およびleadingを設定することで、
    /// ベースライン間の距離を一定に揃えることも可能です。
    /// 例えば、leadingを `{1em}`、top-edgeを`{0.8em}`、
    /// bottom-edgeを `{-0.2em}` に設定すると、
    /// ちょうど`{2em}`のベースライン間隔になります。
    /// top-edgeとbottom-edgeの値の正確な配分が最初の行と最後の行の境界に影響を与えます。
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
    ///
    /// デフォルトでは、Typstは単語間の空白のみを調整して両端揃えを実現します。
    /// ただし、[`justification-limits`プロパティ]($par.justification-limits)を使うと、
    /// 文字間の間隔調整も許可できます。
    #[default(false)]
    pub justify: bool,

    /// 両端揃え中に単語間・文字間の間隔をどの程度まで調整できるか。
    ///
    /// 両端揃えでは、行の幅を測定幅いっぱいに揃えるために、
    /// 行を伸ばしたり縮めたりする必要があります。
    /// そのため、デフォルトでは単語間の空白を調整します。
    /// さらに、文字間の空白も調整できます。
    /// このプロパティで、これらの調整の下限と上限を設定できます。
    ///
    /// このプロパティは`spacing`と`tracking`の2つのエントリを持つ辞書を受け取り、
    /// それぞれに`min`と`max`のキーを含む辞書を指定します。
    /// `min`はどこまで縮められるかの下限、`max`はどこまで広げられるかの上限です。
    ///
    /// - `spacing`エントリは単語間の空白の幅をどの程度まで調整できるかを定義します。
    ///   これは[`text.spacing`]と密接に関係しており、`min`と`max`は
    ///   `spacing`プロパティと同様に[相対長さ]($relative)を受け取ります。
    ///
    ///   `min`が`{100%}`であれば空白は通常のサイズを維持し（縮まりません）、
    ///   `{90% - 0.01em}`であれば、空白の幅は通常の90%から
    ///   現在のフォントサイズの0.01倍を引いた幅まで縮められます。
    ///   同様に`max`が`{100% + 0.02em}`であれば、空白の幅は
    ///   現在のフォントサイズの0.02倍だけ広げられます。
    ///   比率部分は常に正でなければなりません。
    ///   一方、長さ部分は`min`では正であってはならず、`max`では負であってはなりません。
    ///
    ///   なお、行を両端揃えする他の方法がない場合、空白は`max`を超えて
    ///   拡張されることがあります。ただし、その前に他の両端揃えの手段
    ///   （`tracking`エントリの設定に応じて文字間隔を広げるなど）が
    ///   まず最大限まで使われます。
    ///
    /// - `tracking`エントリは文字間の間隔をどの程度まで調整できるかを定義します。
    ///   これは[`text.tracking`]と密接に関係しており、`min`と`max`は
    ///   `tracking`プロパティと同様に[長さ]($length)を受け取ります。
    ///   `spacing`とは異なり、相対長さは受け取りません。
    ///   文字ごとに相対長さの基準が変わってしまい、見た目が不均一になるためです。
    ///   `spacing`と比べた場合の挙動は、基準が常に`{100%}`であるかのように扱われます。
    ///
    ///   それ以外では、`min`と`max`の挙動は`spacing`と同じです。
    ///   `max`が`{0.01em}`であれば、すべての文字ペアの間に
    ///   現在のフォントサイズの0.01倍に相当する追加の間隔を挿入できます。
    ///   これは空白と文字の間の隙間も含むため、空白に対しては
    ///   `tracking`の値が`spacing`の値に加算されます。
    ///
    /// `spacing`または`tracking`のどちらか一方しか指定しない場合、
    /// もう一方は以前に設定された値（以前に設定されていなければデフォルト）を維持します。
    ///
    /// 文字単位の両端揃えを有効にする場合、`min`と`max`は
    /// `{0.01em}`〜`{0.02em}`程度（`min`は負の値）にするのがよい目安です。
    /// 同じ値を使うとベースラインとして良好ですが、
    /// 2つの値を個別に調整すると、下の例のようによりバランスよく見えることがあります。
    /// 範囲を広げすぎると不自然になりやすいので注意してください。
    ///
    /// 文字単位の両端揃えは、特に細いカラムで両端揃えの見栄えを改善できる
    /// 影響力のあるマイクロタイポグラフィ技法です。
    /// ただし、すべてのフォントや言語で機能するわけではありません。
    /// 例えば、筆記体のフォントは文字同士が連結されるため、
    /// 文字単位の両端揃えを行うと接続部分がぎこちなく見えることがあります。
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

    /// 改行位置の決定方法。
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
    ///   （デフォルトでは段落の間隔を継承します）を`{set block(spacing: 1.2em)}`を使用して
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
    pub hanging_indent: Length,

    /// 段落のコンテンツ。
    #[required]
    pub body: Content,
}

#[scope]
impl ParElem {
    #[elem]
    type ParLine;
}

/// 両端揃え時の空白配分の範囲を設定します。
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct JustificationLimits {
    /// 単語間の空白に対する制限（空白幅に対する相対値）。
    spacing: Option<Limits<Rel>>,
    /// 文字間の空白に対する制限（グリフ幅に追加）。
    tracking: Option<Limits<Length>>,
}

impl JustificationLimits {
    /// 単語間の空白に対する制限を取得します。
    pub fn spacing(&self) -> &Limits<Rel> {
        self.spacing.as_ref().unwrap_or(&Limits::SPACING_DEFAULT)
    }

    /// 文字間の空白に対する制限を取得します。
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

/// 空白を縮められる最小値と伸ばせる最大値を定めます。
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct Limits<T> {
    /// 許容される最小の調整量。
    pub min: T,
    /// 許容される最大の調整量。
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

/// 段落の改行位置の決定方法。
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

/// 段落区切り。
///
/// 新しい段落を開始します。
/// 特に[forループ]($scripting/#loops)などのコード内で使用する場合に便利です。
/// 複数の連続した段落区切りは、単一の段落区切りにまとめられます。
///
/// # 例
/// ```example
/// #for i in range(3) {
///   [Blind text #i: ]
///   lorem(5)
///   parbreak()
/// }
/// ```
///
/// # 構文
/// この関数を呼び出す代わりに、
/// マークアップ内に空行を挿入することで段落区切りを作成できます。
#[elem(title = "Paragraph Break", Unlabellable)]
pub struct ParbreakElem {}

impl ParbreakElem {
    /// Get the globally shared paragraph element.
    pub fn shared() -> &'static Content {
        singleton!(Content, ParbreakElem::new().pack())
    }
}

impl Unlabellable for Packed<ParbreakElem> {}

/// 段落の行。
///
/// この要素はsetルールを用いた行番号の設定にのみ使用され、
/// 直接配置することはできません。
///
/// [`numbering`]($par.line.numbering)オプションは、
/// 番号付け形式を指定して行番号を有効化するために使用されます。
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
/// `numbering`オプションには、予め定義された[番号付けパターン]($numbering)か、
/// スタイル付きコンテンツを返す関数のいずれかを指定します。
/// show-setルールを用いてnumberingを`{none}`に設定することで、
/// 特定要素内のテキストの行番号を無効にすることができます。
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
/// この要素は、行番号の[alignment]($par.line.number-align)[margin]($par.line.number-margin)など、
/// 行の番号付けのさまざまな設定を制御できる追加オプションを提供します。
/// さらに、
/// [`numbering-scope`]($par.line.numbering-scope)オプションを使用すると、
/// ページごとに番号をリセットするかどうかの制御が可能です。
#[elem(name = "line", title = "Paragraph Line", keywords = ["line numbering"], Construct, Locatable)]
pub struct ParLine {
    /// 各行を番号付けする方法。
    /// [番号付けパターンまたは関数]($numbering)を指定できます。
    ///
    /// ```example
    /// >>> #set page(margin: (left: 3em))
    /// #set par.line(numbering: "I")
    ///
    /// Roses are red. \
    /// Violets are blue. \
    /// Typst is there for you.
    /// ```
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

    /// 各行に付随する行番号の配置。
    ///
    /// デフォルトの`{auto}`は、
    /// 行番号が余白や現在のテキストの方向を考慮しつつ、
    /// テキストから離れる方向へ水平に伸びるスマートな設定を示します。
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

    /// 行番号を表示する位置の余白。
    ///
    /// _注意_: 複数段組みの文書では、
    /// この設定に関わらず最後の段の段落につく行番号が常に`{end}`の余白（左から右のテキストでは右の余白、
    /// 右から左のテキストでは左の余白）に表示されます。
    /// 現時点では、
    /// この挙動を変更することはできません。
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

    /// 行番号とテキストの間隔。
    ///
    /// デフォルトの値である `{auto}` では、ページ幅に応じて間隔が自動調整され、
    /// ほとんどの場合において適切な間隔が得られます。
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

    /// 行番号をリセットするタイミングを制御する。
    ///
    /// _注意:_ 行番号のスコープは、
    /// ページラン（改ページが明示的に挿入されていない連続したページ）内で統一されている必要があります。
    /// そのため、setルールによる設定は、
    /// ページコンテンツの前、通常は文書の最初などで定義することが望ましいです。
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
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum LineNumberingScope {
    /// 行番号カウンターが文書全体にまたがり、
    /// 決して自動的にリセットされないことを示します。
    Document,
    /// 行番号カウンターが各新規ページの
    /// 先頭でリセットされることを示します。
    Page,
}

/// A marker used to indicate the presence of a line.
///
/// This element is added to each line in a paragraph and later searched to
/// find out where to add line numbers.
#[elem(Construct, Unqueriable, Locatable, Count)]
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
