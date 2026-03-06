use std::num::{NonZeroU32, NonZeroUsize};
use std::sync::Arc;

use ecow::EcoString;
use typst_utils::NonZeroExt;

use crate::diag::{HintedStrResult, HintedString, SourceResult, bail};
use crate::engine::Engine;
use crate::foundations::{
    Content, Packed, Smart, StyleChain, Synthesize, cast, elem, scope,
};
use crate::introspection::{Locatable, Tagged};
use crate::layout::resolve::{CellGrid, table_to_cellgrid};
use crate::layout::{
    Abs, Alignment, Celled, GridCell, GridFooter, GridHLine, GridHeader, GridVLine,
    Length, OuterHAlignment, OuterVAlignment, Rel, Sides, TrackSizings,
};
use crate::model::Figurable;
use crate::pdf::TableCellKind;
use crate::text::LocalName;
use crate::visualize::{Paint, Stroke};

/// 複数の項目からなる表。
///
/// 表はコンテンツをセルに配置するために用います。 
/// セルは複数の段落を含む任意の要素を含めることができ、その配置は行優先順序で指定します。
/// Typstにおける表の実践的な利用方法に関する網羅的な説明とカスタマイズについては[表ガイド]($guides/table-guide)をご覧ください。
///
/// 表は単にいくつかのセルのプロパティ（特に`stroke`と`inset`）のデフォルト値が異なるグリッドであるため、
/// 表の各行および列の大きさの指定、およびセルの外観に関するプロパティの指定についての詳細な情報は[gridのドキュメント]($grid/#track-size)を参照してください。
///
/// 表とグリッドのどちらを使用すべきかわからない場合は、配置するコンテンツが関連するデータ項目の集合として意味的にまとまっているか、あるいは無関係なコンテンツをグリッド状に配置することで文書の見た目を整えようとしているだけなのかを検討してください。
/// 前者の場合は表を使用するのが適切な選択ですが、後者の場合はグリッドの方が適しています。
/// 加えてスクリーンリーダーのような支援技術（Assistive Technology）は`table`に含まれるコンテンツを表形式として読み上げますが、グリッドの場合は文書内に順に配置した複数のコンテンツブロックと同じように発音されます。
/// 支援技術のユーザーはセルによって表を二次元的に移動操作できるようになります。
///
/// また、表中の特定のセルについてプロパティを上書きしたりshowルールを適用したい場合、[`table.cell`]($table.cell)要素を使用できます。
/// 詳細については当該ドキュメントを参照してください。
///
/// `table`と`grid`はほとんどのプロパティを共有していますが、一方に対するsetルールおよびshowルールの指定がもう一方に影響することはありません。
/// 表自体の使用を簡潔で読みやすく保つために、スタイル設定のほとんどはsetルールとshowルールに配置することが推奨されます。
/// またこのようにすることで、全ての表の外観を一か所で容易に変更することも可能になります。
///
/// 表を[`figure`]($figure)で囲むことで、表にキャプションを設けたり [参照可能な要素]($ref) にしたりすることができます。
///
/// # 例
///
/// 以下の例では最も一般的ないくつかの表のオプションを示します。
/// ```example
/// #table(
///   columns: (1fr, auto, auto),
///   inset: 10pt,
///   align: horizon,
///   table.header(
///     [], [*Volume*], [*Parameters*],
///   ),
///   image("cylinder.svg"),
///   $ pi h (D^2 - d^2) / 4 $,
///   [
///     $h$: height \
///     $D$: outer radius \
///     $d$: inner radius
///   ],
///   image("tetrahedron.svg"),
///   $ sqrt(2) / 12 a^3 $,
///   [$a$: edge length]
/// )
/// ```
///
/// グリッドを用いる場合と同様に、[`table.cell`]を使用することでそれぞれのセルの外見と配置をカスタマイズできます。
///
/// ```example
/// >>> #set page(width: auto)
/// >>> #set text(font: "IBM Plex Sans")
/// >>> #let gray = rgb("#565565")
/// >>>
/// #set table(
///   stroke: none,
///   gutter: 0.2em,
///   fill: (x, y) =>
///     if x == 0 or y == 0 { gray },
///   inset: (right: 1.5em),
/// )
///
/// #show table.cell: it => {
///   if it.x == 0 or it.y == 0 {
///     set text(white)
///     strong(it)
///   } else if it.body == [] {
///     // Replace empty cells with 'N/A'
///     pad(..it.inset)[_N/A_]
///   } else {
///     it
///   }
/// }
///
/// #let a = table.cell(
///   fill: green.lighten(60%),
/// )[A]
/// #let b = table.cell(
///   fill: aqua.lighten(60%),
/// )[B]
///
/// #table(
///   columns: 4,
///   [], [Exam 1], [Exam 2], [Exam 3],
///
///   [John], [], a, [],
///   [Mary], [], a, a,
///   [Robert], b, a, b,
/// )
/// ```
///
/// # アクセシビリティ
/// 表は支援技術（Assistive Technology）のユーザーにとっては取り扱いが困難なものです。
/// 支援技術ユーザーが利用しやすいように、我々は表のヘッダーとフッター領域に[`table.header`]と[`table.footer`]を使用することを強く推奨します。
/// これにより支援技術が各セルの列ラベルを発音できるようになります。
///
/// セルによる表の移動操作はそれを視覚的に読み上げるよりも扱いにくいものであるため、表における核となる情報はテキストでも利用可能にすることを検討するべきです。
/// このために表を[figure]要素でラップし、キャプションを表のコンテンツの要約に使用できます。
#[elem(scope, Locatable, Tagged, Synthesize, LocalName, Figurable)]
pub struct TableElem {
    /// 列のサイズ。表の行および列のサイズ指定についての詳細は[gridのドキュメント]($grid/#track-size)を参照してください。
    #[borrowed]
    pub columns: TrackSizings,

    /// 行のサイズ。表の行および列のサイズ指定についての詳細は[gridのドキュメント]($grid/#track-size)を参照してください。
    #[borrowed]
    pub rows: TrackSizings,

    /// 行間と列間の間隔。これは`column-gutter`および`row-gutter`を同一の値に設定する場合の省略記法です。
    /// 各行および列間の間隔指定についての詳細は[gridのドキュメント]($grid.gutter)を参照してください。
    #[external]
    pub gutter: TrackSizings,

    /// 列間の間隔。`gutter`での指定よりも優先されます。
    /// 各行および列間の間隔指定についての詳細は[gridのドキュメント]($grid.gutter)を参照してください。
    #[borrowed]
    #[parse(
        let gutter = args.named("gutter")?;
        args.named("column-gutter")?.or_else(|| gutter.clone())
    )]
    pub column_gutter: TrackSizings,

    /// 行間の間隔。`gutter`での指定よりも優先されます。
    /// 各行および列間の間隔指定についての詳細は[gridのドキュメント]($grid.gutter)を参照してください。
    #[parse(args.named("row-gutter")?.or_else(|| gutter.clone()))]
    pub row_gutter: TrackSizings,

    /// セル内のコンテンツに対するパディングの大きさ。
    ///
    /// 全てのセルに同じインセットを指定するには、全ての側面に適用される単一のlengthを指定するか、個別の側面への指定を有するlengthのdictionaryを使用します。
    /// 詳細は[boxのドキュメント]($box.inset)を参照してください。
    ///
    /// 異なるセルに異なるインセットを指定する場合は、次のいずれかを使用できます。
    /// - 全てのセルへ適用される単一かつ均一なインセット
    /// - インセットの配列による各列への指定
    /// - セルのX/Y位置（共に0から開始）をインセットに変換する関数
    ///
    /// 詳細は[gridのドキュメント]($grid/#styling)を参照してください。
    ///
    /// ```example
    /// #table(
    ///   columns: 2,
    ///   inset: 10pt,
    ///   [Hello],
    ///   [World],
    /// )
    ///
    /// #table(
    ///   columns: 2,
    ///   inset: (x: 20pt, y: 10pt),
    ///   [Hello],
    ///   [World],
    /// )
    /// ```
    #[fold]
    #[default(Celled::Value(Sides::splat(Some(Abs::pt(5.0).into()))))]
    pub inset: Celled<Sides<Option<Rel<Length>>>>,

    /// どのようにセルのコンテンツを配置するか。
    ///
    /// `{auto}`に設定された場合、表の外部の配置設定が使用されます。
    ///
    /// 以下のいずれかの方法でalignmentを指定できます。
    /// - 全てのセルへ適用される単一のalignment
    /// - 各列に対応するalignmentの配列
    /// - セルのX/Y位置（共に0から開始）をalignmentに変換する関数
    ///
    /// 詳細は[表ガイド]($guides/tables/#alignment)を参照してください。
    ///
    /// ```example
    /// #table(
    ///   columns: 3,
    ///   align: (left, center, right),
    ///   [Hello], [Hello], [Hello],
    ///   [A], [B], [C],
    /// )
    /// ```
    pub align: Celled<Smart<Alignment>>,

    /// セルの塗り潰し方。
    ///
    /// 次のいずれかを使用できます。
    /// - 全てのセルへ適用される単一の塗り潰し
    /// - 各列に対応する塗り潰しの配列
    /// - セルの位置を塗り潰しに変換する関数
    ///
    /// 特に配列と関数による指定はストライプ柄の表の作成に便利です。
    /// 詳細は[表ガイド]($guides/tables/#alignment)を参照してください。
    ///
    /// ```example
    /// #table(
    ///   fill: (x, _) =>
    ///     if calc.odd(x) { luma(240) }
    ///     else { white },
    ///   align: (x, y) =>
    ///     if y == 0 { center }
    ///     else if x == 0 { left }
    ///     else { right },
    ///   columns: 4,
    ///   [], [*Q1*], [*Q2*], [*Q3*],
    ///   [Revenue:], [1000 €], [2000 €], [3000 €],
    ///   [Expenses:], [500 €], [1000 €], [1500 €],
    ///   [Profit:], [500 €], [1000 €], [1500 €],
    /// )
    /// ```
    pub fill: Celled<Option<Paint>>,

    /// セルの[ストローク]($stroke)をどうするか。
    ///
    /// ストロークを無効にする場合、これを`{none}`に指定します。
    ///
    /// `gutter`引数の指定によるセル間の間隔をまたいだ罫線が必要な場合、および複数の特定セル間のストロークを上書きする場合は、そのセルについて[`table.hline`]($table.hline)と[`table.vline`]($table.vline)またはその両方を指定することを検討してください。
    ///
    /// 全てのセルに同じストロークを指定する場合は、全ての側面に適用される単一の[stroke]を指定するか、個別の側面への指定を有する[stroke]のdictionaryを使用します。
    /// 詳細は[rectangleのドキュメント]($rect.stroke)を参照してください。
    ///
    /// 異なるセルに異なるストロークを指定する場合は、次のいずれかを使用できます。
    /// - 全てのセルに適用される単一のstroke
    /// - 各列に対応するstrokeの配列
    /// - セルの位置をstrokeに変換する関数
    ///
    /// 詳細は[表ガイド]($guides/tables/#strokes)を参照してください。
    #[resolve]
    #[fold]
    #[default(Celled::Value(Sides::splat(Some(Some(Arc::new(Stroke::default()))))))]
    pub stroke: Celled<Sides<Option<Option<Arc<Stroke>>>>>,

    /// 複雑な表の構造と目的についての要約。
    ///
    /// 詳細は[`crate::pdf::accessibility::table_summary`]関数を参照してください。
    #[internal]
    #[parse(None)]
    pub summary: Option<EcoString>,

    #[internal]
    #[synthesized]
    pub grid: Arc<CellGrid>,

    /// 表の各セルのコンテンツと、[`table.hline`]要素および[`table.vline`]要素による追加の罫線。
    #[variadic]
    pub children: Vec<TableChild>,
}

#[scope]
impl TableElem {
    #[elem]
    type TableCell;

    #[elem]
    type TableHLine;

    #[elem]
    type TableVLine;

    #[elem]
    type TableHeader;

    #[elem]
    type TableFooter;
}

impl Synthesize for Packed<TableElem> {
    fn synthesize(
        &mut self,
        engine: &mut Engine,
        styles: StyleChain,
    ) -> SourceResult<()> {
        let grid = table_to_cellgrid(self, engine, styles)?;
        self.grid = Some(Arc::new(grid));
        Ok(())
    }
}

impl LocalName for Packed<TableElem> {
    const KEY: &'static str = "table";
}

impl Figurable for Packed<TableElem> {}

cast! {
    TableElem,
    v: Content => v.unpack::<Self>().map_err(|_| "expected table")?,
}

/// Any child of a table element.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum TableChild {
    Header(Packed<TableHeader>),
    Footer(Packed<TableFooter>),
    Item(TableItem),
}

cast! {
    TableChild,
    self => match self {
        Self::Header(header) => header.into_value(),
        Self::Footer(footer) => footer.into_value(),
        Self::Item(item) => item.into_value(),
    },
    v: Content => {
        v.try_into()?
    },
}

impl TryFrom<Content> for TableChild {
    type Error = HintedString;

    fn try_from(value: Content) -> HintedStrResult<Self> {
        if value.is::<GridHeader>() {
            bail!(
                "cannot use `grid.header` as a table header";
                hint: "use `table.header` instead"
            )
        }
        if value.is::<GridFooter>() {
            bail!(
                "cannot use `grid.footer` as a table footer";
                hint: "use `table.footer` instead"
            )
        }

        value
            .into_packed::<TableHeader>()
            .map(Self::Header)
            .or_else(|value| value.into_packed::<TableFooter>().map(Self::Footer))
            .or_else(|value| TableItem::try_from(value).map(Self::Item))
    }
}

/// 表設定の基本単位となる表の項目。
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum TableItem {
    HLine(Packed<TableHLine>),
    VLine(Packed<TableVLine>),
    Cell(Packed<TableCell>),
}

cast! {
    TableItem,
    self => match self {
        Self::HLine(hline) => hline.into_value(),
        Self::VLine(vline) => vline.into_value(),
        Self::Cell(cell) => cell.into_value(),
    },
    v: Content => {
        v.try_into()?
    },
}

impl TryFrom<Content> for TableItem {
    type Error = HintedString;

    fn try_from(value: Content) -> HintedStrResult<Self> {
        if value.is::<GridHeader>() {
            bail!("cannot place a grid header within another header or footer");
        }
        if value.is::<TableHeader>() {
            bail!("cannot place a table header within another header or footer");
        }
        if value.is::<GridFooter>() {
            bail!("cannot place a grid footer within another footer or header");
        }
        if value.is::<TableFooter>() {
            bail!("cannot place a table footer within another footer or header");
        }
        if value.is::<GridCell>() {
            bail!(
                "cannot use `grid.cell` as a table cell";
                hint: "use `table.cell` instead"
            );
        }
        if value.is::<GridHLine>() {
            bail!(
                "cannot use `grid.hline` as a table line";
                hint: "use `table.hline` instead"
            );
        }
        if value.is::<GridVLine>() {
            bail!(
                "cannot use `grid.vline` as a table line";
                hint: "use `table.vline` instead"
            );
        }

        Ok(value
            .into_packed::<TableHLine>()
            .map(Self::HLine)
            .or_else(|value| value.into_packed::<TableVLine>().map(Self::VLine))
            .or_else(|value| value.into_packed::<TableCell>().map(Self::Cell))
            .unwrap_or_else(|value| {
                let span = value.span();
                Self::Cell(Packed::new(TableCell::new(value)).spanned(span))
            }))
    }
}

/// 繰り返し可能な表のヘッダー。
///
/// たとえその表が複数ページにわたるつもりではないとしても、表のヘッダーとなる行はこの関数によってラップされるべきです。
/// これによりTypstは表にアクセシビリティのためのメタデータを埋め込んだり、その文書における[ユニバーサルアクセス]($guides/accessibility/#basics)を提供できます。
///
/// `repeat`引数を用いてその表のヘッダーがページをまたいで繰り返されるかを制御できます。
///
/// 現在、この機能はヘッダー列や単一のヘッダーセルを作成する用途には適していません。
/// この場合は通常のセルを使用するか、PDFにエクスポートする場合は[`pdf.header-cell`]関数を用いてセルをヘッダーセルとしてマークできます。
/// 同様に[`pdf.data-cell`]関数を用いるとセルをデータセルとしてマークできます。
/// これらの関数は最終的なものではなく、またそのため`a11y-extras`機能を有効化した場合にのみ利用可能であることに注意してください。
/// 詳細は[PDFモジュールのドキュメント]($pdf)を参照してください。
///
/// ```example
/// #set page(height: 11.5em)
/// #set table(
///   fill: (x, y) =>
///     if x == 0 or y == 0 {
///       gray.lighten(40%)
///     },
///   align: right,
/// )
///
/// #show table.cell.where(x: 0): strong
/// #show table.cell.where(y: 0): strong
///
/// #table(
///   columns: 4,
///   table.header(
///     [], [Blue chip],
///     [Fresh IPO], [Penny st'k],
///   ),
///   table.cell(
///     rowspan: 6,
///     align: horizon,
///     rotate(-90deg, reflow: true)[
///       *USD / day*
///     ],
///   ),
///   [0.20], [104], [5],
///   [3.17], [108], [4],
///   [1.59], [84],  [1],
///   [0.26], [98],  [15],
///   [0.01], [195], [4],
///   [7.34], [57],  [2],
/// )
/// ```
#[elem(name = "header", title = "Table Header")]
pub struct TableHeader {
    /// ページごとにヘッダーを繰り返すかどうか。
    #[default(true)]
    pub repeat: bool,

    /// ヘッダーのレベル。0であってはいけません。
    ///
    /// 複数のヘッダーが一度に繰り返すことを可能にします。
    /// 異なるレベルを持つヘッダーは、それらのレベルが昇順になっている場合は同時に繰り返しができます
    ///
    /// 明確には、より低いレベルを持つヘッダーが繰り返しを始めるとき、それ以上のレベルを持つ全てのヘッダーは繰り返しを止めます（それらは新しいヘッダーに「置き換えられ」ます）。
    #[default(NonZeroU32::ONE)]
    pub level: NonZeroU32,

    /// ヘッダー内のセルと罫線。
    #[variadic]
    pub children: Vec<TableItem>,
}

/// 繰り返し可能な表のフッター。
///
/// [`table.header`]要素と同様に、フッターは表内で各ページごとに繰り返すことができます。
/// これによって大きい表においてヘッダーとフッターの両方に各列のラベルを追加したり、合計などの各ページごとに表示されるべき情報を付加したりすることができ、表を読みやすくすることができます。
///
/// いかなるセルもフッターよりも後には配置されません。
#[elem(name = "footer", title = "Table Footer")]
pub struct TableFooter {
    /// ページごとにフッターを繰り返すかどうか。
    #[default(true)]
    pub repeat: bool,

    /// フッター内のセルと罫線。
    #[variadic]
    pub children: Vec<TableItem>,
}

/// 表内の水平罫線。
///
/// 表の`stroke`フィールドによる指定を含めてセルごとに設定されたストロークを上書きします。
/// 表の[`column-gutter`]($table.column-gutter)オプションによるセル間の間隔をまたぐことができます。
///
/// 単一の表内の特定の位置に手動で罫線を配置したい場合は、表の`stroke`フィールドの代わりにこの関数を使用してください。
/// もし配置したい罫線が文書内の全ての表のデザインの一部である場合は[表の`stroke`]($table.stroke)フィールドか[`table.cell`の`stroke`]($table.cell.stroke)フィールドを使用してください。
///
/// ```example
/// #set table.hline(stroke: .6pt)
///
/// #table(
///   stroke: none,
///   columns: (auto, 1fr),
///   [09:00], [Badge pick up],
///   [09:45], [Opening Keynote],
///   [10:30], [Talk: Typst's Future],
///   [11:15], [Session: Good PRs],
///   table.hline(start: 1),
///   [Noon], [_Lunch break_],
///   table.hline(start: 1),
///   [14:00], [Talk: Tracked Layout],
///   [15:00], [Talk: Automations],
///   [16:00], [Workshop: Tables],
///   table.hline(),
///   [19:00], [Day 1 Attendee Mixer],
/// )
/// ```
#[elem(name = "hline", title = "Table Horizontal Line")]
pub struct TableHLine {
    /// この罫線が配置される行（最初の行は0）。
    /// [`grid.hline`]($grid.hline.y)の`y`フィールドと同様に機能します。
    pub y: Smart<usize>,

    /// この罫線が開始される列（最初の列は0、指定した列を含みます）。
    pub start: usize,

    /// この罫線が終了する列（最初の列は0、指定した列を含みません）。
    pub end: Option<NonZeroUsize>,

    /// この罫線のストローク。
    ///
    /// `{none}`を指定すると、この罫線の範囲に含まれているこれまで配置された全ての罫線が削除されます。
    /// これには他の`hline`による水平罫線やセルごとのストロークが含まれます。
    #[fold]
    #[default(Some(Arc::new(Stroke::default())))]
    pub stroke: Option<Arc<Stroke>>,

    /// 指定した行（`y`）に基づいてこの罫線が配置される位置。
    /// 指定した行の上部に描画する場合は`{top}`、下部に描画する場合は`{bottom}`を指定します。
    /// 
    /// `row-gutter`オプションによる行間隔の設定が無効になっている場合、ある行の下部とその次の行の上部が示す位置は一致します。
    /// このため、この設定は`row-gutter`オプションが設定されている場合にのみ意味があります。
    /// （そうでない場合は使用するべきではありません。代わりに`y`フィールドを1大きく指定してください）
    #[default(OuterVAlignment::Top)]
    pub position: OuterVAlignment,
}

/// 表内の垂直罫線。
/// この要素のフィールドの使用法についての詳細は[`grid.vline`]のドキュメントを参照してください。
///
/// 表の`stroke`フィールドによる指定を含めてセルごとに設定されたストロークを上書きします。
/// 表の[`row-gutter`]($table.row-gutter)オプションによるセル間の間隔をまたぐことができます。
///
/// [`table.hline`]と同様、単一の表内の特定の位置に手動で罫線を配置したい場合は、表の`stroke`フィールドの代わりにこの関数を使用してください。
/// もし配置したい罫線が文書内の全ての表のデザインの一部である場合は[表の`stroke`]($table.stroke)フィールドか[`table.cell`の`stroke`]($table.cell.stroke)フィールドを使用してください。
#[elem(name = "vline", title = "Table Vertical Line")]
pub struct TableVLine {
    /// この罫線が配置される列（最初の列は0）。
    ///
    /// [`grid.vline`]($grid.vline.x)の`x`フィールドと同様に機能します。
    pub x: Smart<usize>,

    /// この罫線が開始される行（最初の行は0、指定した行を含みます）。
    pub start: usize,

    /// この罫線が終了する行（最初の行は0、指定した行を含みません）。
    pub end: Option<NonZeroUsize>,

    /// この罫線のストローク。
    ///
    /// `{none}`を指定すると、この罫線の範囲に含まれているこれまで配置された全ての罫線が削除されます。
    /// これには他の`vline`による垂直罫線やセルごとのストロークが含まれます。
    #[fold]
    #[default(Some(Arc::new(Stroke::default())))]
    pub stroke: Option<Arc<Stroke>>,

    /// 指定した列（`x`）に基づいてこの罫線が配置される位置。
    /// 指定した列の前に描画する場合は`{start}`、後に描画する場合は`{end}`を指定します。
    ///
    /// 値`{left}`と`{right}`を使用することもできます。
    /// ただし左から右への向きのドキュメントと右から左への向きのドキュメント間で一貫性を損なうため推奨されていません。
    /// 
    /// `column-gutter`オプションによる列間隔の設定が無効になっている場合、ある列の後とその次の列の前が示す位置は一致します。
    /// このため、この設定は`column-gutter`オプションが設定されている場合にのみ関係します。
    /// （そうでない場合は使用するべきではありません。代わりに`x`フィールドを1大きく指定してください）
    #[default(OuterHAlignment::Start)]
    pub position: OuterHAlignment,
}

/// 表のセル。セルを手動で配置する場合やスタイル設定をする場合に使用します。
/// スタイル設定をする場合、この関数を用いて特定のセルのプロパティを上書きするかshowルールによって特定のスタイルを複数のセルに一度に指定することができます。
///
/// おそらく`{table.cell}`の最も重要な利用用途は`colspan`と`rowspan`フィールドを用いて複数の行または列をまたいだセルを作成することです。
///
/// ```example
/// >>> #set page(width: auto)
/// #show table.cell.where(y: 0): strong
/// #set table(
///   stroke: (x, y) => if y == 0 {
///     (bottom: 0.7pt + black)
///   },
///   align: (x, y) => (
///     if x > 0 { center }
///     else { left }
///   )
/// )
///
/// #table(
///   columns: 3,
///   table.header(
///     [Substance],
///     [Subcritical °C],
///     [Supercritical °C],
///   ),
///   [Hydrochloric Acid],
///   [12.0], [92.1],
///   [Sodium Myreth Sulfate],
///   [16.6], [104],
///   [Potassium Hydroxide],
///   table.cell(colspan: 2)[24.7],
/// )
/// ```
///
/// 例えば、表中の単一のセルについてfill、alignmentあるいはinsetを上書きすることができます。
///
/// ```example
/// >>> #set page(width: auto)
/// // You can also import those.
/// #import table: cell, header
///
/// #table(
///   columns: 2,
///   align: center,
///   header(
///     [*Trip progress*],
///     [*Itinerary*],
///   ),
///   cell(
///     align: right,
///     fill: fuchsia.lighten(80%),
///     [🚗],
///   ),
///   [Get in, folks!],
///   [🚗], [Eat curbside hotdog],
///   cell(align: left)[🌴🚗],
///   cell(
///     inset: 0.06em,
///     text(1.62em)[🏝️🌅🌊],
///   ),
/// )
/// ```
///
/// `table.cell`にshowルールを適用することで、全てのセルに対して同時にスタイルを設定するために使用することもできます。
/// セレクターと組み合わせることで、セルの位置に基づいたスタイル指定も可能です。
///
/// ```example
/// #show table.cell.where(x: 0): strong
///
/// #table(
///   columns: 3,
///   gutter: 3pt,
///   [Name], [Age], [Strength],
///   [Hannes], [36], [Grace],
///   [Irma], [50], [Resourcefulness],
///   [Vikram], [49], [Perseverance],
/// )
/// ```
#[elem(name = "cell", title = "Table Cell")]
pub struct TableCell {
    /// セルの本文。
    #[required]
    pub body: Content,

    /// セルの列（最初の列は0）。
    ///
    /// [`grid.cell`]の`x`フィールドと同様に機能します。
    pub x: Smart<usize>,

    /// セルの行（最初の行は0）。
    ///
    /// [`grid.cell`]の`y`フィールドと同様に機能します。
    pub y: Smart<usize>,

    /// このセルがまたぐ列の数。
    #[default(NonZeroUsize::ONE)]
    pub colspan: NonZeroUsize,

    /// このセルがまたぐ行の数。
    #[default(NonZeroUsize::ONE)]
    pub rowspan: NonZeroUsize,

    /// そのセルの[inset]($table.inset)を上書きします。
    pub inset: Smart<Sides<Option<Rel<Length>>>>,

    /// そのセルの[alignment]($table.align)を上書きします。
    pub align: Smart<Alignment>,

    /// そのセルの[fill]($table.fill)を上書きします。
    pub fill: Smart<Option<Paint>>,

    /// そのセルの[stroke]($table.stroke)を上書きします。
    #[fold]
    pub stroke: Sides<Option<Option<Arc<Stroke>>>>,

    /// このセルがまたぐ行が複数ページにまたがって配置できるかどうか。
    /// `{auto}`に設定された場合、固定サイズの行のみをまたぐセルは分割不可となり、`{auto}`サイズの行を少なくとも1つ含むセルは分割可能となります。
    pub breakable: Smart<bool>,

    #[internal]
    #[parse(Some(Smart::Auto))]
    pub kind: Smart<TableCellKind>,

    #[internal]
    #[parse(Some(false))]
    pub is_repeated: bool,
}

cast! {
    TableCell,
    v: Content => v.into(),
}

impl Default for Packed<TableCell> {
    fn default() -> Self {
        Packed::new(
            // Explicitly set colspan and rowspan to ensure they won't be
            // overridden by set rules (default cells are created after
            // colspans and rowspans are processed in the resolver)
            TableCell::new(Content::default())
                .with_colspan(NonZeroUsize::ONE)
                .with_rowspan(NonZeroUsize::ONE),
        )
    }
}

impl From<Content> for TableCell {
    fn from(value: Content) -> Self {
        #[allow(clippy::unwrap_or_default)]
        value.unpack::<Self>().unwrap_or_else(Self::new)
    }
}
