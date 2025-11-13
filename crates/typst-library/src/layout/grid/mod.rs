pub mod resolve;

use std::num::NonZeroUsize;
use std::sync::Arc;

use comemo::Track;
use smallvec::{smallvec, SmallVec};
use typst_utils::NonZeroExt;

use crate::diag::{bail, At, HintedStrResult, HintedString, SourceResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, elem, scope, Array, CastInfo, Content, Context, Fold, FromValue, Func,
    IntoValue, NativeElement, Packed, Reflect, Resolve, Show, Smart, StyleChain, Value,
};
use crate::layout::{
    Alignment, BlockElem, Length, OuterHAlignment, OuterVAlignment, Rel, Sides, Sizing,
};
use crate::model::{TableCell, TableFooter, TableHLine, TableHeader, TableVLine};
use crate::visualize::{Paint, Stroke};

/// グリッド状にコンテンツを配置。
///
/// グリッド要素を用いるとコンテンツをグリッド状に配置できます。
/// 行と列の数に加えて、それらの間隔を定義できます。
/// 複雑なレイアウトが作成できるような、列と行の大きさに関するモードが複数あります。
///
/// グリッド要素とテーブル要素はとてもよく似た挙動をする一方で、これらは異なるユースケースが想定されており、異なる意味論が提供されています。
/// グリッド要素はプレゼンテーションおよびレイアウトに使われることが想定されている一方で、[`{table}`]($table)要素は複数の関係データ点を表す広い用途が想定されています。
/// 将来、Typstは出力に注釈を付与し、スクリーンリーダーが`table`要素の内容を表として読み上げられるようになる予定です。
/// 一方、グリッドのコンテンツは、文書の流れに沿った複数のコンテンツブロックと同様に読み上げられる予定です。

/// これらの要素に対するsetルールとshowルールは、互いに影響しません。
///
/// グリッドの大きさは引数に指定されたトラックサイズによって決定されます。
/// 大きさを設定する各パラメーターは同じ値を受け入れるため、ここでまとめて説明します。
/// 各sizing引数は個々のトラックサイズの配列を受け入れます。
/// トラックサイズは以下のいずれかです。
///
/// - `{auto}`: トラックはコンテンツに合わせた大きさとなり、残りのスペース全体まで大きくなります。
/// `{auto}`トラック幅が複数指定され、利用可能なスペースより大きなスペースが要求された場合、利用可能なスペースが`{auto}`トラックに等分配されます。
///
/// - 固定あるいは相対長さ（`{10pt}`や`{20% - 1cm}`など）: トラックは厳密にその大きさになります。
///
/// - 比率長さ（例えば`{1fr}`）: 他のトラック全ての大きさが確定し次第、残りのスペースは比率指定のトラックに指定された比率に応じて分配されます。
/// 例えば、`{1fr}`で比率指定されたトラックが2つある場合、それぞれ残りのスペースの半分になります。
///
/// 単一のトラックを指定する場合は、配列を省略して単一の値を指定できます。
/// 複数の`{auto}`のトラックを指定する場合は、配列の代わりにトラックの数を入力して下さい。
/// 例えば、`columns:` `{3}`は`columns:` `{(auto, auto, auto)}`と同じ意味になります。
///
/// # 例
/// 以下の例は異なるトラックサイズオプションの実演です。
/// また、1つのセルをグリッドの2つのトラックに跨がせるために[`grid.cell`]($grid.cell)をどう使うのかも示しています。
///
/// ```example
/// // We use `rect` to emphasize the
/// // area of cells.
/// #set rect(
///   inset: 8pt,
///   fill: rgb("e4e5ea"),
///   width: 100%,
/// )
///
/// #grid(
///   columns: (60pt, 1fr, 2fr),
///   rows: (auto, 60pt),
///   gutter: 3pt,
///   rect[Fixed width, auto height],
///   rect[1/3 of the remains],
///   rect[2/3 of the remains],
///   rect(height: 100%)[Fixed height],
///   grid.cell(
///     colspan: 2,
///     image("tiger.jpg", width: 100%),
///   ),
/// )
/// ```
///
/// また、文字列やコンテンツの配列をグリッドに[展開](#arguments/#spreading)して、セルを埋めることもできます。
///
/// ```example
/// #grid(
///   columns: 5,
///   gutter: 5pt,
///   ..range(25).map(str)
/// )
/// ```
///
/// # グリッドのスタイル設定
/// グリッドの外観は様々なパラメーターでカスタマイズできます。
/// 以下のものが最も重要です。
///
/// - [`fill`]($grid.fill)は全てのセルに背景を設定します。
/// - [`align`]($grid.align)はセルの配置方法を変更します。
/// - [`inset`]($grid.inset)は各セル内に任意のパディングを追加します。
/// - [`stroke`]($grid.stroke)は特定のストロークでグリッドの線をオプションで有効化します。
///
/// もし単一セルに対して上記のオプションの1つを上書きしなければならない場合は、[`grid.cell`]($grid.cell)要素が使用できます。
/// 同様に、個々のグリッドの線も[`grid.hline`]($grid.hline)要素や[`grid.vline`]($grid.vline)要素を用いて上書きできます。
///
/// 別の方法として、外観オプションをセルの位置（列と行）に依存させる必要がある場合、`fill`や`align`に`(column, row) => value`という形式の関数を指定できます。
/// [`grid.cell`]($grid.cell)に対してもshowルールを使用できます。
/// 詳細はその要素の例や以下の例を参照してください。
///
/// グリッドやテーブルの実際の使い方が簡素かつ読みやすくなるため、基本的にスタイル設定にはsetルールとshowルールを用いることを推奨します。
/// これによって、グリッドの外観を1か所で簡単に変更もできます。
///
/// ## ストロークのスタイル設定の優先順位
/// グリッドセルのストローク指定方法は3種類あります。
/// [`{grid.cell}`の`stroke`フィールド]($grid.cell.stroke)を用いる方法、[`{grid.hline}`]($grid.hline)と[`{grid.vline}`]($grid.vline)を用いる方法、[`{grid}`の`stroke`フィールド]($grid.stroke)を用いる方法です。
/// これらの設定が複数存在し、競合する場合、`hline`と`vline`の設定が最優先となり、続いて優先されるのが`cell`の設定で、最後に`grid`の設定が適用されます。
///
/// さらに、グリッドの繰り返されたヘッダーおよびフッターのストロークは、通常のセルのストロークよりも優先されます。
#[elem(scope, Show)]
pub struct GridElem {
    /// 列の数。
    ///
    /// Either specify a track size array or provide an integer to create a grid with that many `{auto}`-sized columns.
    /// Note that opposed to rows and gutters, providing a single track size will only ever create a single column.
    #[borrowed]
    pub columns: TrackSizings,

    /// 行の数。
    ///
    /// If there are more cells than fit the defined rows, the last row is repeated until there are no more cells.
    #[borrowed]
    pub rows: TrackSizings,

    /// 行間と列間の間隔。
    ///
    /// 定義した数よりも多くgutterがある場合、最後のgutterが繰り返されます。
    ///
    /// これは`column-gutter`と`row-gutter`を同じ値で設定する省略記法です。
    #[external]
    pub gutter: TrackSizings,

    /// 列間の間隔。
    #[parse(
        let gutter = args.named("gutter")?;
        args.named("column-gutter")?.or_else(|| gutter.clone())
    )]
    #[borrowed]
    pub column_gutter: TrackSizings,

    /// 行間の間隔。
    #[parse(args.named("row-gutter")?.or_else(|| gutter.clone()))]
    #[borrowed]
    pub row_gutter: TrackSizings,

    /// セルの塗り潰し方。
    ///
    /// これはcolorかcolorを返す関数が使用可能です。
    /// 関数は0始まりの列番号と行番号を受け取ります。
    /// これは縞模様のグリッドの実装に使えます。
    ///
    /// ```example
    /// #grid(
    ///   fill: (x, y) =>
    ///     if calc.even(x + y) { luma(230) }
    ///     else { white },
    ///   align: center + horizon,
    ///   columns: 4,
    ///   inset: 2pt,
    ///   [X], [O], [X], [O],
    ///   [O], [X], [O], [X],
    ///   [X], [O], [X], [O],
    ///   [O], [X], [O], [X],
    /// )
    /// ```
    #[borrowed]
    pub fill: Celled<Option<Paint>>,

    /// セルのコンテンツをどう配置するか。
    ///
    /// 単一の配置、（各列に対応する）配置の配列、配置を返す関数のいずれかが使用可能です。
    /// The function receives the cells' column and row indices, starting from zero.
    /// `{auto}`に設定された場合は外側の配置が使用されます。
    ///
    /// この引数に関する例は[`table.align`]($table.align)パラメーターにあります。
    #[borrowed]
    pub align: Celled<Smart<Alignment>>,

    /// How to [stroke]($stroke) the cells.
    ///
    /// デフォルトではグリッドにストロークはありませんが、このオプションを所望のストロークに設定すれば変更できます。
    ///
    /// If it is necessary to place lines which can cross spacing between cells produced by the `gutter` option, or to override the stroke between multiple specific cells, consider specifying one or more of [`grid.hline`]($grid.hline) and [`grid.vline`]($grid.vline) alongside your grid cells.
    ///
    /// ```example
    /// #set page(height: 13em, width: 26em)
    ///
    /// #let cv(..jobs) = grid(
    ///   columns: 2,
    ///   inset: 5pt,
    ///   stroke: (x, y) => if x == 0 and y > 0 {
    ///     (right: (
    ///       paint: luma(180),
    ///       thickness: 1.5pt,
    ///       dash: "dotted"
    ///     ))
    ///   },
    ///   grid.header(grid.cell(colspan: 2)[
    ///     *Professional Experience*
    ///     #box(width: 1fr, line(length: 100%, stroke: luma(180)))
    ///   ]),
    ///   ..{
    ///     let last = none
    ///     for job in jobs.pos() {
    ///       (
    ///         if job.year != last [*#job.year*],
    ///         [
    ///           *#job.company* - #job.role _(#job.timeframe)_ \
    ///           #job.details
    ///         ]
    ///       )
    ///       last = job.year
    ///     }
    ///   }
    /// )
    ///
    /// #cv(
    ///   (
    ///     year: 2012,
    ///     company: [Pear Seed & Co.],
    ///     role: [Lead Engineer],
    ///     timeframe: [Jul - Dec],
    ///     details: [
    ///       - Raised engineers from 3x to 10x
    ///       - Did a great job
    ///     ],
    ///   ),
    ///   (
    ///     year: 2012,
    ///     company: [Mega Corp.],
    ///     role: [VP of Sales],
    ///     timeframe: [Mar - Jun],
    ///     details: [- Closed tons of customers],
    ///   ),
    ///   (
    ///     year: 2013,
    ///     company: [Tiny Co.],
    ///     role: [CEO],
    ///     timeframe: [Jan - Dec],
    ///     details: [- Delivered 4x more shareholder value],
    ///   ),
    ///   (
    ///     year: 2014,
    ///     company: [Glorbocorp Ltd],
    ///     role: [CTO],
    ///     timeframe: [Jan - Mar],
    ///     details: [- Drove containerization forward],
    ///   ),
    /// )
    /// ```
    #[resolve]
    #[fold]
    pub stroke: Celled<Sides<Option<Option<Arc<Stroke>>>>>,

    /// セル内のコンテンツに対するパディングの大きさ。
    ///
    /// この引数に関する例は[`table.inset`]($table.inset)パラメーターにあります。
    #[fold]
    pub inset: Celled<Sides<Option<Rel<Length>>>>,

    /// The contents of the grid cells, plus any extra grid lines specified with the [`grid.hline`]($grid.hline) and [`grid.vline`]($grid.vline) elements.
    ///
    /// セルは行優先で埋められます。
    #[variadic]
    pub children: Vec<GridChild>,
}

#[scope]
impl GridElem {
    #[elem]
    type GridCell;

    #[elem]
    type GridHLine;

    #[elem]
    type GridVLine;

    #[elem]
    type GridHeader;

    #[elem]
    type GridFooter;
}

impl Show for Packed<GridElem> {
    fn show(&self, engine: &mut Engine, _: StyleChain) -> SourceResult<Content> {
        Ok(BlockElem::multi_layouter(self.clone(), engine.routines.layout_grid)
            .pack()
            .spanned(self.span()))
    }
}

/// Track sizing definitions.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct TrackSizings(pub SmallVec<[Sizing; 4]>);

cast! {
    TrackSizings,
    self => self.0.into_value(),
    sizing: Sizing => Self(smallvec![sizing]),
    count: NonZeroUsize => Self(smallvec![Sizing::Auto; count.get()]),
    values: Array => Self(values.into_iter().map(Value::cast).collect::<HintedStrResult<_>>()?),
}

/// Any child of a grid element.
#[derive(Debug, PartialEq, Clone, Hash)]
pub enum GridChild {
    Header(Packed<GridHeader>),
    Footer(Packed<GridFooter>),
    Item(GridItem),
}

cast! {
    GridChild,
    self => match self {
        Self::Header(header) => header.into_value(),
        Self::Footer(footer) => footer.into_value(),
        Self::Item(item) => item.into_value(),
    },
    v: Content => {
        v.try_into()?
    },
}

impl TryFrom<Content> for GridChild {
    type Error = HintedString;
    fn try_from(value: Content) -> HintedStrResult<Self> {
        if value.is::<TableHeader>() {
            bail!(
                "cannot use `table.header` as a grid header";
                hint: "use `grid.header` instead"
            )
        }
        if value.is::<TableFooter>() {
            bail!(
                "cannot use `table.footer` as a grid footer";
                hint: "use `grid.footer` instead"
            )
        }

        value
            .into_packed::<GridHeader>()
            .map(Self::Header)
            .or_else(|value| value.into_packed::<GridFooter>().map(Self::Footer))
            .or_else(|value| GridItem::try_from(value).map(Self::Item))
    }
}

/// A grid item, which is the basic unit of grid specification.
#[derive(Debug, PartialEq, Clone, Hash)]
pub enum GridItem {
    HLine(Packed<GridHLine>),
    VLine(Packed<GridVLine>),
    Cell(Packed<GridCell>),
}

cast! {
    GridItem,
    self => match self {
        Self::HLine(hline) => hline.into_value(),
        Self::VLine(vline) => vline.into_value(),
        Self::Cell(cell) => cell.into_value(),
    },
    v: Content => {
        v.try_into()?
    }
}

impl TryFrom<Content> for GridItem {
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
        if value.is::<TableCell>() {
            bail!(
                "cannot use `table.cell` as a grid cell";
                hint: "use `grid.cell` instead"
            );
        }
        if value.is::<TableHLine>() {
            bail!(
                "cannot use `table.hline` as a grid line";
                hint: "use `grid.hline` instead"
            );
        }
        if value.is::<TableVLine>() {
            bail!(
                "cannot use `table.vline` as a grid line";
                hint: "use `grid.vline` instead"
            );
        }

        Ok(value
            .into_packed::<GridHLine>()
            .map(Self::HLine)
            .or_else(|value| value.into_packed::<GridVLine>().map(Self::VLine))
            .or_else(|value| value.into_packed::<GridCell>().map(Self::Cell))
            .unwrap_or_else(|value| {
                let span = value.span();
                Self::Cell(Packed::new(GridCell::new(value)).spanned(span))
            }))
    }
}

/// A repeatable grid header.
///
/// If `repeat` is set to `true`, the header will be repeated across pages. For
/// an example, refer to the [`table.header`]($table.header) element and the
/// [`grid.stroke`]($grid.stroke) parameter.
#[elem(name = "header", title = "Grid Header")]
pub struct GridHeader {
    /// Whether this header should be repeated across pages.
    #[default(true)]
    pub repeat: bool,

    /// The cells and lines within the header.
    #[variadic]
    pub children: Vec<GridItem>,
}

/// A repeatable grid footer.
///
/// Just like the [`grid.header`]($grid.header) element, the footer can repeat
/// itself on every page of the table.
///
/// No other grid cells may be placed after the footer.
#[elem(name = "footer", title = "Grid Footer")]
pub struct GridFooter {
    /// Whether this footer should be repeated across pages.
    #[default(true)]
    pub repeat: bool,

    /// The cells and lines within the footer.
    #[variadic]
    pub children: Vec<GridItem>,
}

/// A horizontal line in the grid.
///
/// Overrides any per-cell stroke, including stroke specified through the grid's
/// `stroke` field. Can cross spacing between cells created through the grid's
/// `column-gutter` option.
///
/// An example for this function can be found at the
/// [`table.hline`]($table.hline) element.
#[elem(name = "hline", title = "Grid Horizontal Line")]
pub struct GridHLine {
    /// The row above which the horizontal line is placed (zero-indexed).
    /// If the `position` field is set to `{bottom}`, the line is placed below
    /// the row with the given index instead (see that field's docs for
    /// details).
    ///
    /// Specifying `{auto}` causes the line to be placed at the row below the
    /// last automatically positioned cell (that is, cell without coordinate
    /// overrides) before the line among the grid's children. If there is no
    /// such cell before the line, it is placed at the top of the grid (row 0).
    /// Note that specifying for this option exactly the total amount of rows
    /// in the grid causes this horizontal line to override the bottom border
    /// of the grid, while a value of 0 overrides the top border.
    pub y: Smart<usize>,

    /// The column at which the horizontal line starts (zero-indexed, inclusive).
    pub start: usize,

    /// The column before which the horizontal line ends (zero-indexed,
    /// exclusive).
    /// Therefore, the horizontal line will be drawn up to and across column
    /// `end - 1`.
    ///
    /// A value equal to `{none}` or to the amount of columns causes it to
    /// extend all the way towards the end of the grid.
    pub end: Option<NonZeroUsize>,

    /// The line's stroke.
    ///
    /// Specifying `{none}` removes any lines previously placed across this
    /// line's range, including hlines or per-cell stroke below it.
    #[resolve]
    #[fold]
    #[default(Some(Arc::new(Stroke::default())))]
    pub stroke: Option<Arc<Stroke>>,

    /// The position at which the line is placed, given its row (`y`) - either
    /// `{top}` to draw above it or `{bottom}` to draw below it.
    ///
    /// This setting is only relevant when row gutter is enabled (and
    /// shouldn't be used otherwise - prefer just increasing the `y` field by
    /// one instead), since then the position below a row becomes different
    /// from the position above the next row due to the spacing between both.
    #[default(OuterVAlignment::Top)]
    pub position: OuterVAlignment,
}

/// A vertical line in the grid.
///
/// Overrides any per-cell stroke, including stroke specified through the
/// grid's `stroke` field. Can cross spacing between cells created through
/// the grid's `row-gutter` option.
#[elem(name = "vline", title = "Grid Vertical Line")]
pub struct GridVLine {
    /// The column before which the horizontal line is placed (zero-indexed).
    /// If the `position` field is set to `{end}`, the line is placed after the
    /// column with the given index instead (see that field's docs for
    /// details).
    ///
    /// Specifying `{auto}` causes the line to be placed at the column after
    /// the last automatically positioned cell (that is, cell without
    /// coordinate overrides) before the line among the grid's children. If
    /// there is no such cell before the line, it is placed before the grid's
    /// first column (column 0).
    /// Note that specifying for this option exactly the total amount of
    /// columns in the grid causes this vertical line to override the end
    /// border of the grid (right in LTR, left in RTL), while a value of 0
    /// overrides the start border (left in LTR, right in RTL).
    pub x: Smart<usize>,

    /// The row at which the vertical line starts (zero-indexed, inclusive).
    pub start: usize,

    /// The row on top of which the vertical line ends (zero-indexed,
    /// exclusive).
    /// Therefore, the vertical line will be drawn up to and across row
    /// `end - 1`.
    ///
    /// A value equal to `{none}` or to the amount of rows causes it to extend
    /// all the way towards the bottom of the grid.
    pub end: Option<NonZeroUsize>,

    /// The line's stroke.
    ///
    /// Specifying `{none}` removes any lines previously placed across this
    /// line's range, including vlines or per-cell stroke below it.
    #[resolve]
    #[fold]
    #[default(Some(Arc::new(Stroke::default())))]
    pub stroke: Option<Arc<Stroke>>,

    /// The position at which the line is placed, given its column (`x`) -
    /// either `{start}` to draw before it or `{end}` to draw after it.
    ///
    /// The values `{left}` and `{right}` are also accepted, but discouraged as
    /// they cause your grid to be inconsistent between left-to-right and
    /// right-to-left documents.
    ///
    /// This setting is only relevant when column gutter is enabled (and
    /// shouldn't be used otherwise - prefer just increasing the `x` field by
    /// one instead), since then the position after a column becomes different
    /// from the position before the next column due to the spacing between
    /// both.
    #[default(OuterHAlignment::Start)]
    pub position: OuterHAlignment,
}

/// A cell in the grid. You can use this function in the argument list of a grid
/// to override grid style properties for an individual cell or manually
/// positioning it within the grid. You can also use this function in show rules
/// to apply certain styles to multiple cells at once.
///
/// For example, you can override the position and stroke for a single cell:
///
/// ```example
/// >>> #set page(width: auto)
/// >>> #set text(15pt, font: "Noto Sans Symbols 2", bottom-edge: -.2em)
/// <<< #set text(15pt, font: "Noto Sans Symbols 2")
/// #show regex("[♚-♟︎]"): set text(fill: rgb("21212A"))
/// #show regex("[♔-♙]"): set text(fill: rgb("111015"))
///
/// #grid(
///   fill: (x, y) => rgb(
///     if calc.odd(x + y) { "7F8396" }
///     else { "EFF0F3" }
///   ),
///   columns: (1em,) * 8,
///   rows: 1em,
///   align: center + horizon,
///
///   [♖], [♘], [♗], [♕], [♔], [♗], [♘], [♖],
///   [♙], [♙], [♙], [♙], [],  [♙], [♙], [♙],
///   grid.cell(
///     x: 4, y: 3,
///     stroke: blue.transparentize(60%)
///   )[♙],
///
///   ..(grid.cell(y: 6)[♟],) * 8,
///   ..([♜], [♞], [♝], [♛], [♚], [♝], [♞], [♜])
///     .map(grid.cell.with(y: 7)),
/// )
/// ```
///
/// You may also apply a show rule on `grid.cell` to style all cells at once,
/// which allows you, for example, to apply styles based on a cell's position.
/// Refer to the examples of the [`table.cell`]($table.cell) element to learn
/// more about this.
#[elem(name = "cell", title = "Grid Cell", Show)]
pub struct GridCell {
    /// The cell's body.
    #[required]
    pub body: Content,

    /// The cell's column (zero-indexed).
    /// This field may be used in show rules to style a cell depending on its
    /// column.
    ///
    /// You may override this field to pick in which column the cell must
    /// be placed. If no row (`y`) is chosen, the cell will be placed in the
    /// first row (starting at row 0) with that column available (or a new row
    /// if none). If both `x` and `y` are chosen, however, the cell will be
    /// placed in that exact position. An error is raised if that position is
    /// not available (thus, it is usually wise to specify cells with a custom
    /// position before cells with automatic positions).
    ///
    /// ```example
    /// #let circ(c) = circle(
    ///     fill: c, width: 5mm
    /// )
    ///
    /// #grid(
    ///   columns: 4,
    ///   rows: 7mm,
    ///   stroke: .5pt + blue,
    ///   align: center + horizon,
    ///   inset: 1mm,
    ///
    ///   grid.cell(x: 2, y: 2, circ(aqua)),
    ///   circ(yellow),
    ///   grid.cell(x: 3, circ(green)),
    ///   circ(black),
    /// )
    /// ```
    pub x: Smart<usize>,

    /// The cell's row (zero-indexed).
    /// This field may be used in show rules to style a cell depending on its
    /// row.
    ///
    /// You may override this field to pick in which row the cell must be
    /// placed. If no column (`x`) is chosen, the cell will be placed in the
    /// first column (starting at column 0) available in the chosen row. If all
    /// columns in the chosen row are already occupied, an error is raised.
    ///
    /// ```example
    /// #let tri(c) = polygon.regular(
    ///   fill: c,
    ///   size: 5mm,
    ///   vertices: 3,
    /// )
    ///
    /// #grid(
    ///   columns: 2,
    ///   stroke: blue,
    ///   inset: 1mm,
    ///
    ///   tri(black),
    ///   grid.cell(y: 1, tri(teal)),
    ///   grid.cell(y: 1, tri(red)),
    ///   grid.cell(y: 2, tri(orange))
    /// )
    /// ```
    pub y: Smart<usize>,

    /// The amount of columns spanned by this cell.
    #[default(NonZeroUsize::ONE)]
    pub colspan: NonZeroUsize,

    /// The amount of rows spanned by this cell.
    #[default(NonZeroUsize::ONE)]
    pub rowspan: NonZeroUsize,

    /// The cell's [fill]($grid.fill) override.
    pub fill: Smart<Option<Paint>>,

    /// The cell's [alignment]($grid.align) override.
    pub align: Smart<Alignment>,

    /// The cell's [inset]($grid.inset) override.
    pub inset: Smart<Sides<Option<Rel<Length>>>>,

    /// The cell's [stroke]($grid.stroke) override.
    #[resolve]
    #[fold]
    pub stroke: Sides<Option<Option<Arc<Stroke>>>>,

    /// Whether rows spanned by this cell can be placed in different pages.
    /// When equal to `{auto}`, a cell spanning only fixed-size rows is
    /// unbreakable, while a cell spanning at least one `{auto}`-sized row is
    /// breakable.
    pub breakable: Smart<bool>,
}

cast! {
    GridCell,
    v: Content => v.into(),
}

impl Show for Packed<GridCell> {
    fn show(&self, _engine: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        show_grid_cell(self.body.clone(), self.inset(styles), self.align(styles))
    }
}

impl Default for Packed<GridCell> {
    fn default() -> Self {
        Packed::new(GridCell::new(Content::default()))
    }
}

impl From<Content> for GridCell {
    fn from(value: Content) -> Self {
        #[allow(clippy::unwrap_or_default)]
        value.unpack::<Self>().unwrap_or_else(Self::new)
    }
}

/// Function with common code to display a grid cell or table cell.
pub(crate) fn show_grid_cell(
    mut body: Content,
    inset: Smart<Sides<Option<Rel<Length>>>>,
    align: Smart<Alignment>,
) -> SourceResult<Content> {
    let inset = inset.unwrap_or_default().map(Option::unwrap_or_default);

    if inset != Sides::default() {
        // Only pad if some inset is not 0pt.
        // Avoids a bug where using .padded() in any way inside Show causes
        // alignment in align(...) to break.
        body = body.padded(inset);
    }

    if let Smart::Custom(alignment) = align {
        body = body.aligned(alignment);
    }

    Ok(body)
}

/// A value that can be configured per cell.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Celled<T> {
    /// A bare value, the same for all cells.
    Value(T),
    /// A closure mapping from cell coordinates to a value.
    Func(Func),
    /// An array of alignment values corresponding to each column.
    Array(Vec<T>),
}

impl<T: Default + Clone + FromValue> Celled<T> {
    /// Resolve the value based on the cell position.
    pub fn resolve(
        &self,
        engine: &mut Engine,
        styles: StyleChain,
        x: usize,
        y: usize,
    ) -> SourceResult<T> {
        Ok(match self {
            Self::Value(value) => value.clone(),
            Self::Func(func) => func
                .call(engine, Context::new(None, Some(styles)).track(), [x, y])?
                .cast()
                .at(func.span())?,
            Self::Array(array) => x
                .checked_rem(array.len())
                .and_then(|i| array.get(i))
                .cloned()
                .unwrap_or_default(),
        })
    }
}

impl<T: Default> Default for Celled<T> {
    fn default() -> Self {
        Self::Value(T::default())
    }
}

impl<T: Reflect> Reflect for Celled<T> {
    fn input() -> CastInfo {
        T::input() + Array::input() + Func::input()
    }

    fn output() -> CastInfo {
        T::output() + Array::output() + Func::output()
    }

    fn castable(value: &Value) -> bool {
        Array::castable(value) || Func::castable(value) || T::castable(value)
    }
}

impl<T: IntoValue> IntoValue for Celled<T> {
    fn into_value(self) -> Value {
        match self {
            Self::Value(value) => value.into_value(),
            Self::Func(func) => func.into_value(),
            Self::Array(arr) => arr.into_value(),
        }
    }
}

impl<T: FromValue> FromValue for Celled<T> {
    fn from_value(value: Value) -> HintedStrResult<Self> {
        match value {
            Value::Func(v) => Ok(Self::Func(v)),
            Value::Array(array) => Ok(Self::Array(
                array.into_iter().map(T::from_value).collect::<HintedStrResult<_>>()?,
            )),
            v if T::castable(&v) => Ok(Self::Value(T::from_value(v)?)),
            v => Err(Self::error(&v)),
        }
    }
}

impl<T: Fold> Fold for Celled<T> {
    fn fold(self, outer: Self) -> Self {
        match (self, outer) {
            (Self::Value(inner), Self::Value(outer)) => Self::Value(inner.fold(outer)),
            (self_, _) => self_,
        }
    }
}

impl<T: Resolve> Resolve for Celled<T> {
    type Output = ResolvedCelled<T>;

    fn resolve(self, styles: StyleChain) -> Self::Output {
        match self {
            Self::Value(value) => ResolvedCelled(Celled::Value(value.resolve(styles))),
            Self::Func(func) => ResolvedCelled(Celled::Func(func)),
            Self::Array(values) => ResolvedCelled(Celled::Array(
                values.into_iter().map(|value| value.resolve(styles)).collect(),
            )),
        }
    }
}

/// The result of resolving a Celled's value according to styles.
/// Holds resolved values which depend on each grid cell's position.
/// When it is a closure, however, it is only resolved when the closure is
/// called.
#[derive(Default, Clone)]
pub struct ResolvedCelled<T: Resolve>(Celled<T::Output>);

impl<T> ResolvedCelled<T>
where
    T: FromValue + Resolve,
    <T as Resolve>::Output: Default + Clone,
{
    /// Resolve the value based on the cell position.
    pub fn resolve(
        &self,
        engine: &mut Engine,
        styles: StyleChain,
        x: usize,
        y: usize,
    ) -> SourceResult<T::Output> {
        Ok(match &self.0 {
            Celled::Value(value) => value.clone(),
            Celled::Func(func) => func
                .call(engine, Context::new(None, Some(styles)).track(), [x, y])?
                .cast::<T>()
                .at(func.span())?
                .resolve(styles),
            Celled::Array(array) => x
                .checked_rem(array.len())
                .and_then(|i| array.get(i))
                .cloned()
                .unwrap_or_default(),
        })
    }
}
