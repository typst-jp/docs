use std::num::NonZeroUsize;
use std::sync::Arc;

use typst_utils::NonZeroExt;

use crate::diag::{bail, HintedStrResult, HintedString, SourceResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, elem, scope, Content, NativeElement, Packed, Show, Smart, StyleChain,
    TargetElem,
};
use crate::html::{attr, tag, HtmlAttrs, HtmlElem, HtmlTag};
use crate::introspection::Locator;
use crate::layout::grid::resolve::{table_to_cellgrid, Cell, CellGrid, Entry};
use crate::layout::{
    show_grid_cell, Abs, Alignment, BlockElem, Celled, GridCell, GridFooter, GridHLine,
    GridHeader, GridVLine, Length, OuterHAlignment, OuterVAlignment, Rel, Sides,
    TrackSizings,
};
use crate::model::Figurable;
use crate::text::LocalName;
use crate::visualize::{Paint, Stroke};

/// 複数の項目からなる表。
///
/// 表はコンテンツをセルに配置するために用います。 
/// セルは複数の段落を含む任意の要素を含めることができ、その配置は行優先順序で指定します。
/// Typstにおける表の利用とカスタマイズについて、全ての手法の実践的な説明は[表ガイド]($guides/table-guide)をご覧ください。
///
/// 表は単にいくつかのセルのプロパティ（特に`stroke`と`inset`）のデフォルト値が異なるグリッドであるため、
/// 表の各行および列の大きさの指定、およびセルの外見に関するプロパティの指定についての詳細な情報は[gridのドキュメント]($grid)を参照してください。
///
/// 表とグリッドのどちらを使用すべきかわからない場合は、配置しようとしているコンテンツが1つの関連したデータの集合に属するのか、あるいは無関係なコンテンツを整列することで文書の見た目を整えようとしているだけなのかを検討してください。
/// 前者の場合は表を使用するのが適切な選択ですが、後者の場合はグリッドの方が適しています。
/// 加えてTypstは将来的に表には注釈をつけることを予定しています。
/// これにより、スクリーンリーダーは`table`に含まれるコンテンツを表形式として読み上げますが、グリッドの場合は文書内に順に配置した複数のコンテンツブロックと同じように発音されます。
///
/// また、表中の特定のセルについてプロパティを上書きしたりshowルールを適用したい場合、[`table.cell`]($table.cell)要素を使用できます。
/// 詳細については当該ドキュメントを参照してください。
///
/// `table`と`grid`はほとんどのプロパティを共有していますが、一方に対するsetルールおよびshowルールの指定がもう一方に影響することはありません。
///
/// 表を[`figure`]($figure)で囲むことで、表にキャプションを設けたり [_参照可能な要素_]($ref) にしたりすることができます。
///
/// # 例
///
/// 以下の例では最も一般的ないくつか表のオプションを示します。
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
/// グリッドを用いる場合と同様に、[`table.cell`]($table.cell)を使用することでそれぞれのセルの外見と配置をカスタマイズできます。
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
#[elem(scope, Show, LocalName, Figurable)]
pub struct TableElem {
    /// 列のサイズ。表の行および列のサイズ指定についての詳細は[gridのドキュメント]($grid)を参照してください。
    #[borrowed]
    pub columns: TrackSizings,

    /// 行のサイズ。表の行および列のサイズ指定についての詳細は[gridのドキュメント]($grid)を参照してください。
    #[borrowed]
    pub rows: TrackSizings,

    /// 各行および列間の間隔。これは`column-gutter`および`row-gutter`を同一の値に設定する場合の省略記法です。
    /// 各行および列間の間隔指定についての詳細は[gridのドキュメント]($grid)を参照してください。
    #[external]
    pub gutter: TrackSizings,

    /// 各列間の間隔。`gutter`での指定よりも優先されます。
    /// 各行および列間の間隔指定についての詳細は[gridのドキュメント]($grid)を参照してください。
    #[borrowed]
    #[parse(
        let gutter = args.named("gutter")?;
        args.named("column-gutter")?.or_else(|| gutter.clone())
    )]
    pub column_gutter: TrackSizings,

    /// 各列間の間隔。`gutter`での指定よりも優先されます。
    /// 各行および列間の間隔指定についての詳細は[gridのドキュメント]($grid)を参照してください。
    #[parse(args.named("row-gutter")?.or_else(|| gutter.clone()))]
    #[borrowed]
    pub row_gutter: TrackSizings,

    /// どのようにセルを着色するか。
    ///
    /// 色または色を返す関数を指定できます。
    /// 関数を指定した場合、そのセルの列および行の0で始まる番号が引数に渡されます。
    /// これによってストライプ柄の表を作成できます。
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
    #[borrowed]
    pub fill: Celled<Option<Paint>>,

    /// セル内のコンテンツをどのように配置するか。
    ///
    /// 単一の`alignment`、それぞれの列についての指定となる`alignment`の配列、`alignment`を返す関数のいずれかを指定できます。
    /// 関数を指定した場合、そのセルの列および行の0で始まる番号が引数に渡されます。
    /// `{auto}`が指定された場合、表の外部の配置設定を使用します。
    ///
    /// ```example
    /// #table(
    ///   columns: 3,
    ///   align: (left, center, right),
    ///   [Hello], [Hello], [Hello],
    ///   [A], [B], [C],
    /// )
    /// ```
    #[borrowed]
    pub align: Celled<Smart<Alignment>>,

    /// セルの枠線をどのように描画するか。
    ///
    /// 枠線を非表示にする場合、これを`{none}`に指定します。
    ///
    /// `gutter`引数の指定によるセル間の間隔をまたいだ枠線が必要な場合、および複数の特定セル間の枠線の表示を上書きする場合は、そのセルについて[`table.hline`]($table.hline)と[`table.vline`]($table.vline)またはその両方を指定することを検討してください。
    ///
    /// 枠線指定についての詳細は[gridのドキュメント]($grid.stroke)を参照してください。
    #[resolve]
    #[fold]
    #[default(Celled::Value(Sides::splat(Some(Some(Arc::new(Stroke::default()))))))]
    pub stroke: Celled<Sides<Option<Option<Arc<Stroke>>>>>,

    /// セル内部のコンテンツまでの隙間をどの程度設けるか。
    ///
    /// ```example
    /// #table(
    ///   inset: 10pt,
    ///   [Hello],
    ///   [World],
    /// )
    ///
    /// #table(
    ///   columns: 2,
    ///   inset: (
    ///     x: 20pt,
    ///     y: 10pt,
    ///   ),
    ///   [Hello],
    ///   [World],
    /// )
    /// ```
    #[fold]
    #[default(Celled::Value(Sides::splat(Some(Abs::pt(5.0).into()))))]
    pub inset: Celled<Sides<Option<Rel<Length>>>>,

    /// 表の各セルのコンテンツ、および[`table.hline`]($table.hline)要素と[`table.vline`]($table.vline)要素による追加の行。
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

fn show_cell_html(tag: HtmlTag, cell: &Cell, styles: StyleChain) -> Content {
    let cell = cell.body.clone();
    let Some(cell) = cell.to_packed::<TableCell>() else { return cell };
    let mut attrs = HtmlAttrs::default();
    let span = |n: NonZeroUsize| (n != NonZeroUsize::MIN).then(|| n.to_string());
    if let Some(colspan) = span(cell.colspan(styles)) {
        attrs.push(attr::colspan, colspan);
    }
    if let Some(rowspan) = span(cell.rowspan(styles)) {
        attrs.push(attr::rowspan, rowspan);
    }
    HtmlElem::new(tag)
        .with_body(Some(cell.body.clone()))
        .with_attrs(attrs)
        .pack()
        .spanned(cell.span())
}

fn show_cellgrid_html(grid: CellGrid, styles: StyleChain) -> Content {
    let elem = |tag, body| HtmlElem::new(tag).with_body(Some(body)).pack();
    let mut rows: Vec<_> = grid.entries.chunks(grid.non_gutter_column_count()).collect();

    let tr = |tag, row: &[Entry]| {
        let row = row
            .iter()
            .flat_map(|entry| entry.as_cell())
            .map(|cell| show_cell_html(tag, cell, styles));
        elem(tag::tr, Content::sequence(row))
    };

    let footer = grid.footer.map(|ft| {
        let rows = rows.drain(ft.unwrap().start..);
        elem(tag::tfoot, Content::sequence(rows.map(|row| tr(tag::td, row))))
    });
    let header = grid.header.map(|hd| {
        let rows = rows.drain(..hd.unwrap().end);
        elem(tag::thead, Content::sequence(rows.map(|row| tr(tag::th, row))))
    });

    let mut body = Content::sequence(rows.into_iter().map(|row| tr(tag::td, row)));
    if header.is_some() || footer.is_some() {
        body = elem(tag::tbody, body);
    }

    let content = header.into_iter().chain(core::iter::once(body)).chain(footer);
    elem(tag::table, Content::sequence(content))
}

impl Show for Packed<TableElem> {
    fn show(&self, engine: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        Ok(if TargetElem::target_in(styles).is_html() {
            // TODO: This is a hack, it is not clear whether the locator is actually used by HTML.
            // How can we find out whether locator is actually used?
            let locator = Locator::root();
            show_cellgrid_html(table_to_cellgrid(self, engine, locator, styles)?, styles)
        } else {
            BlockElem::multi_layouter(self.clone(), engine.routines.layout_table).pack()
        }
        .spanned(self.span()))
    }
}

impl LocalName for Packed<TableElem> {
    const KEY: &'static str = "table";
}

impl Figurable for Packed<TableElem> {}

/// Any child of a table element.
#[derive(Debug, PartialEq, Clone, Hash)]
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

/// A table item, which is the basic unit of table specification.
#[derive(Debug, PartialEq, Clone, Hash)]
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
/// これによりTypstは将来的に表にアクセシビリティのためのメタデータを埋め込んだり、その文書における普遍的なアクセスを提供できるようになります。
///
/// `repeat`引数を用いてその表のヘッダーがページをまたいで繰り返されるかを制御できます。
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
    /// このヘッダーがページをまたいで繰り返されるべきかどうか。
    #[default(true)]
    pub repeat: bool,

    /// ヘッダー内の各セルと各行。
    #[variadic]
    pub children: Vec<TableItem>,
}

/// 繰り返し可能な表のフッター。
///
/// [`table.header`]($table.header)要素と同様に、フッターは表内で各ページごとに繰り返すことができます。
/// これによって大きい表においてヘッダーとフッターの両方に各列のラベルを追加したり、合計などの各ページごとに表示されるべき情報を付加したりすることができ、表を読みやすくすることができます。
///
/// いかなるセルもフッターよりも後には配置されません。
#[elem(name = "footer", title = "Table Footer")]
pub struct TableFooter {
    /// このフッターがページをまたいで繰り返されるべきかどうか。
    #[default(true)]
    pub repeat: bool,

    /// フッター内の各セルと各行。
    #[variadic]
    pub children: Vec<TableItem>,
}

/// 表内の水平罫線。
///
/// 表の`stroke`フィールドによる指定を含むセルごとの枠線設定を上書きします。
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
    /// この罫線が配置される行。（最初の行は0）
    /// [`grid.hline`]($grid.hline.y)の`y`フィールドと同様に機能します。
    pub y: Smart<usize>,

    /// この罫線が開始される行。（最初の行は0、指定した行を含みます）
    pub start: usize,

    /// この罫線が終了する行。（最初の行は0、指定した行を含みません）
    pub end: Option<NonZeroUsize>,

    /// この罫線のstroke。
    ///
    /// `{none}`が指定された場合、他の水平罫線とセルごとのstroke設定を含むこの罫線の範囲にまたがって配置されたいかなる罫線も削除されます。
    #[resolve]
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
/// この要素のフィールドの使用法についての詳細は[`grid.vline`]($grid.vline)のドキュメントを参照してください。
///
/// 表の`stroke`フィールドによる指定を含むセルごとの枠線設定を上書きします。
/// 表の[`row-gutter`]($table.row-gutter)オプションによるセル間の間隔をまたぐことができます。
///
/// [`table.hline`]($table.hline)と同様、単一の表内の特定の位置に手動で罫線を配置したい場合は、表の`stroke`フィールドの代わりにこの関数を使用してください。
/// もし配置したい罫線が文書内の全ての表のデザインの一部である場合は[表の`stroke`]($table.stroke)フィールドか[`table.cell`の`stroke`]($table.cell.stroke)フィールドを使用してください。
#[elem(name = "vline", title = "Table Vertical Line")]
pub struct TableVLine {
    /// この罫線が配置される列。（最初の列は0）
    ///
    /// [`grid.vline`]($grid.vline.x)の`x`フィールドと同様に機能します。
    pub x: Smart<usize>,

    /// この罫線が開始される列。（最初の列は0、指定した列を含みます）
    pub start: usize,

    /// この罫線が終了する列。（最初の列は0、指定した列を含みません）
    pub end: Option<NonZeroUsize>,

    /// この罫線のstroke。
    ///
    /// `{none}`が指定された場合、他の垂直罫線とセルごとのstroke設定を含むこの罫線の範囲にまたがって配置されたいかなる罫線も削除されます。
    #[resolve]
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

/// 表中のセル。セルを手動で配置する場合やスタイル設定をする場合に使用します。
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
/// 例えば、表中の単一のセルについてfill、alignementあるいはinsetを上書きすることができます。
/// For example, you can override the fill, alignment or inset for a single
/// cell:
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
///     text(1.62em)[🛖🌅🌊],
///   ),
/// )
/// ```
///
/// 全てのセルに対して同時にスタイルを設定するために使用することもできます。
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
#[elem(name = "cell", title = "Table Cell", Show)]
pub struct TableCell {
    /// セル内の要素。
    #[required]
    pub body: Content,

    /// セルの列の位置。（最初の要素は0）
    ///
    /// [`grid.cell`]($grid.cell)の`x`フィールドと同様に機能します。
    pub x: Smart<usize>,

    /// セルの行の位置。（最初の要素は0）
    ///
    /// [`grid.cell`]($grid.cell)の`y`フィールドと同様に機能します。
    pub y: Smart<usize>,

    /// このセルがまたぐ列の数。
    #[default(NonZeroUsize::ONE)]
    pub colspan: NonZeroUsize,

    /// このセルがまたぐ行の数。
    #[default(NonZeroUsize::ONE)]
    pub rowspan: NonZeroUsize,

    /// そのセルの[fill]($table.fill)を上書きします。
    pub fill: Smart<Option<Paint>>,

    /// そのセルの[alignment]($table.align)を上書きします。
    pub align: Smart<Alignment>,

    /// そのセルの[inset]($table.inset)を上書きします。
    pub inset: Smart<Sides<Option<Rel<Length>>>>,

    /// そのセルの[stroke]($table.stroke)を上書きします。
    #[resolve]
    #[fold]
    pub stroke: Sides<Option<Option<Arc<Stroke>>>>,

    /// このセルがまたがる行が別のページに配置できるかどうか。
    /// 値が`{auto}`の場合、固定サイズの行のみをまたぐセルは改ページされず、少なくとも1つの`{auto}`でサイズ指定された行をまたいでいるセルは改ページできます。
    pub breakable: Smart<bool>,
}

cast! {
    TableCell,
    v: Content => v.into(),
}

impl Show for Packed<TableCell> {
    fn show(&self, _engine: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        show_grid_cell(self.body.clone(), self.inset(styles), self.align(styles))
    }
}

impl Default for Packed<TableCell> {
    fn default() -> Self {
        Packed::new(TableCell::new(Content::default()))
    }
}

impl From<Content> for TableCell {
    fn from(value: Content) -> Self {
        #[allow(clippy::unwrap_or_default)]
        value.unpack::<Self>().unwrap_or_else(Self::new)
    }
}
