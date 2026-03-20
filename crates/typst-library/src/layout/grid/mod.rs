pub mod resolve;

use std::num::{NonZeroU32, NonZeroUsize};
use std::sync::Arc;

use comemo::Track;
use smallvec::{SmallVec, smallvec};
use typst_utils::NonZeroExt;

use crate::diag::{At, HintedStrResult, HintedString, SourceResult, bail};
use crate::engine::Engine;
use crate::foundations::{
    Array, CastInfo, Content, Context, Fold, FromValue, Func, IntoValue, Packed, Reflect,
    Resolve, Smart, StyleChain, Synthesize, Value, cast, elem, scope,
};
use crate::introspection::Tagged;
use crate::layout::resolve::{CellGrid, grid_to_cellgrid};
use crate::layout::{
    Alignment, Length, OuterHAlignment, OuterVAlignment, Rel, Sides, Sizing,
};
use crate::model::{TableCell, TableFooter, TableHLine, TableHeader, TableVLine};
use crate::visualize::{Paint, Stroke};

/// グリッド状にコンテンツを配置。
///
/// グリッド要素を用いるとコンテンツをグリッド状に配置できます。
/// 行と列の数に加えて、それらの間隔を定義できます。
/// 複雑なレイアウトが作成できるような、列と行の大きさに関する設定方法が複数あります。
///
/// グリッド要素とテーブル要素はとてもよく似た挙動をする一方で、これらは異なる用途が想定されており、セマンティクスが異なります。
/// グリッド要素はプレゼンテーションおよびレイアウトに使われることが想定されている一方で、[`table`]要素は、複数の関連するデータ項目を表すことが大まかに想定されています。
/// これらの要素に対するsetルールとshowルールは、互いに影響しません。
/// Refer to the [Accessibility Section]($grid/#accessibility) to learn how grids and
/// tables are presented to users of Assistive Technology (AT) like screen
/// readers.
///
/// # Sizing the tracks { #track-size }
///
/// グリッドの大きさは引数に指定されたトラックサイズによって決定されます。
/// There are multiple sizing parameters: [`columns`]($grid.columns),
/// [`rows`]($grid.rows) and [`gutter`]($grid.gutter).
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
/// また、1つのセルをグリッドの2つのトラックにまたがせるために[`grid.cell`]をどう使うのかも示しています。
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
/// # グリッドのスタイル設定 { #styling }
/// The grid and table elements work similarly. For a hands-on explanation,
/// refer to the [Table Guide]($guides/tables/#fills); for a quick overview,
/// continue reading.
///
/// グリッドの外観はさまざまなパラメーターでカスタマイズできます。
/// 以下のものが最も重要です。
///
/// - [`align`]($grid.align)はセルの配置方法を変更します。
/// - [`inset`]($grid.inset)はセル内に任意のパディングを追加します。
/// - [`fill`]($grid.fill)はセルに背景を設定します。
/// - [`stroke`]($grid.stroke)は特定のストロークでグリッドの線をオプションで有効化します。
///
/// To meet different needs, there are various ways to set them.
///
/// もし個々のセルに対して上記のオプションを上書きする必要がある場合は、[`grid.cell`]要素が使用できます。
/// 同様に、個々のグリッドの線も[`grid.hline`]や[`grid.vline`]要素を用いて上書きできます。
///
/// To configure an overall style for a grid, you may instead specify the option
/// in any of the following fashions:
///
/// - As a single value that applies to all cells.
/// - As an array of values corresponding to each column. The array will be
///   cycled if there are more columns than the array has items.
/// - As a function in the form of `(x, y) => value`. It receives the cell's
///   column and row indices (both starting from zero) and should return the
///   value to apply to that cell.
///
/// ```example
/// #grid(
///   columns: 5,
///
///   // By a single value
///   align: center,
///   // By a single but more complicated value
///   inset: (x: 2pt, y: 3pt),
///   // By an array of values (cycling)
///   fill: (rgb("#239dad50"), none),
///   // By a function that returns a value
///   stroke: (x, y) => if calc.rem(x + y, 3) == 0 { 0.5pt },
///
///   ..range(5 * 3).map(n => numbering("A", n + 1))
/// )
/// ```
///
/// On top of that, you may [apply styling rules]($styling) to [`grid`] and
/// [`grid.cell`]. Especially, the [`x`]($grid.cell.x) and [`y`]($grid.cell.y)
/// fields of `grid.cell` can be used in a [`where`]($function.where) selector,
/// making it possible to style cells at specific columns or rows, or individual
/// positions.
///
/// ## ストロークのスタイル設定の優先順位
/// 上記で説明したように、グリッドセルのストローク指定方法は3種類あります。
/// [`{grid.cell}`の`stroke`フィールド]($grid.cell.stroke)を用いる方法、[`{grid.hline}`]($grid.hline)と[`{grid.vline}`]($grid.vline)を用いる方法、[`{grid}`の`stroke`フィールド]($grid.stroke)を用いる方法です。
/// これらの設定が複数存在し、競合する場合、`hline`と`vline`の設定が最優先となり、続いて優先されるのが`cell`の設定で、最後に`grid`の設定が適用されます。
///
/// さらに、グリッドの繰り返されたヘッダーおよびフッターのストロークは、通常のセルのストロークよりも優先されます。
///
/// # Accessibility
/// Grids do not carry any special semantics. Assistive Technology (AT) does not
/// offer the ability to navigate two-dimensionally by cell in grids. If you
/// want to present tabular data, use the [`table`] element instead.
///
/// AT will read the grid cells in their semantic order. Usually, this is the
/// order in which you passed them to the grid. However, if you manually
/// positioned them using [`grid.cell`'s `x` and `y` arguments]($grid.cell.x),
/// cells will be read row by row, from left to right (in left-to-right
/// documents). A cell will be read when its position is first reached.
#[elem(scope, Synthesize, Tagged)]
pub struct GridElem {
    /// 列の数または各列の大きさ。
    ///
    /// トラックサイズの配列か整数を指定します。
    /// 整数を渡した場合、その数だけ`auto`サイズ列を持つグリッドが作成されます。
    /// rowsおよびguttersとは異なり、単一のトラックサイズを指定するとただ一つの列が作成されることに注意してください。
    ///
    /// See the [track size section](#track-size) above for more details.
    pub columns: TrackSizings,

    /// 行の数。
    ///
    /// 定義した行に収まらないセルがある場合、セルが無くなるまで最後の行が繰り返されます。
    ///
    /// See the [track size section](#track-size) above for more details.
    pub rows: TrackSizings,

    /// 行間と列間の間隔。
    /// これは[`column-gutter`]($grid.column-gutter)と[`row-gutter`]($grid.row-gutter)を同じ値で設定する省略記法です。
    ///
    /// 定義した数よりも多くgutterがある場合、最後のgutterが繰り返されます。
    ///
    /// See the [track size section](#track-size) above for more details.
    #[external]
    pub gutter: TrackSizings,

    /// 列間の間隔。
    #[parse(
        let gutter = args.named("gutter")?;
        args.named("column-gutter")?.or_else(|| gutter.clone())
    )]
    pub column_gutter: TrackSizings,

    /// 行間の間隔。
    #[parse(args.named("row-gutter")?.or_else(|| gutter.clone()))]
    pub row_gutter: TrackSizings,

    /// How much to pad the cells' content.
    ///
    /// To specify a uniform inset for all cells, you can use a single length
    /// for all sides, or a dictionary of lengths for individual sides. See the
    /// [box's documentation]($box.inset) for more details.
    ///
    /// To specify varying inset for different cells, you can:
    /// - use a single inset for all cells
    /// - use an array of insets corresponding to each column
    /// - use a function that maps a cell's position to its inset
    ///
    /// See the [styling section](#styling) above for more details.
    ///
    /// In addition, you can find an example at the [`table.inset`] parameter.
    #[fold]
    pub inset: Celled<Sides<Option<Rel<Length>>>>,

    /// How to align the cells' content.
    ///
    /// If set to `{auto}`, the outer alignment is used.
    ///
    /// You can specify the alignment in any of the following fashions:
    /// - use a single alignment for all cells
    /// - use an array of alignments corresponding to each column
    /// - use a function that maps a cell's position to its alignment
    ///
    /// See the [styling section](#styling) above for details.
    ///
    /// In addition, you can find an example at the [`table.align`] parameter.
    pub align: Celled<Smart<Alignment>>,

    /// セルの塗り潰し方。
    ///
    /// This can be:
    /// - a single color for all cells
    /// - an array of colors corresponding to each column
    /// - a function that maps a cell's position to its color
    ///
    /// Most notably, arrays and functions are useful for creating striped grids.
    /// See the [styling section](#styling) above for more details.
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
    pub fill: Celled<Option<Paint>>,

    /// セルの[ストローク]($stroke)をどうするか。
    ///
    /// デフォルトではグリッドにストロークはありませんが、このオプションを所望のストロークに設定すれば変更できます。
    ///
    /// [`gutter`]($grid.gutter)オプションによって作成されたセル間の空白を横切る線を配置する必要がある場合や、複数の特定のセル間のストロークを上書きする必要がある場合は、グリッドセルにあわせて[`grid.hline`]および[`grid.vline`]のいずれか、または両方を指定することを検討してください。
    ///
    /// To specify the same stroke for all cells, you can use a single [stroke]
    /// for all sides, or a dictionary of [strokes]($stroke) for individual
    /// sides. See the [rectangle's documentation]($rect.stroke) for more
    /// details.
    ///
    /// To specify varying strokes for different cells, you can:
    /// - use a single stroke for all cells
    /// - use an array of strokes corresponding to each column
    /// - use a function that maps a cell's position to its stroke
    ///
    /// See the [styling section](#styling) above for more details.
    ///
    /// ```example:"Passing a function to set a stroke based on position"
    /// #set page(width: 420pt)
    /// #set text(number-type: "old-style")
    /// #show grid.cell.where(y: 0): set text(size: 1.3em)
    ///
    /// #grid(
    ///   columns: (1fr, 2fr, 2fr),
    ///   row-gutter: 1.5em,
    ///   inset: (left: 0.5em),
    ///   stroke: (x, y) => if x > 0 { (left: 0.5pt + gray) },
    ///   align: horizon,
    ///
    ///   [Winter \ 2007 \ Season],
    ///   [Aaron Copland \ *The Tender Land* \ January 2007],
    ///   [Eric Satie \ *Gymnopedie 1, 2* \ February 2007],
    ///
    ///   [],
    ///   [Jan 12 \ *Middlebury College \ Center for the Arts* \ 20:00],
    ///   [Feb 2 \ *Johnson State College Dibden Center for the Arts* \ 19:30],
    ///
    ///   [],
    ///   [Skip a week \ #text(0.8em)[_Prepare your exams!_]],
    ///   [Feb 9 \ *Castleton State College \ Fine Arts Center* \ 19:30],
    ///
    ///   [],
    ///   [Jan 26, 27 \ *Lyndon State College Alexander Twilight Theater* \ 20:00],
    ///   [
    ///     Feb 17 --- #smallcaps[Anniversary] \
    ///     *Middlebury College \ Center for the Arts* \
    ///     19:00 #text(0.7em)[(for a special guest)]
    ///   ],
    /// )
    /// ```
    ///
    /// ```example:"Folding the stroke dictionary"
    /// #set page(height: 13em, width: 26em)
    ///
    /// #let cv(..jobs) = grid(
    ///   columns: 2,
    ///   inset: 5pt,
    ///   stroke: (x, y) => if x == 0 and y > 0 {
    ///     (right: (
    ///       paint: luma(180),
    ///       thickness: 1.5pt,
    ///       dash: "dotted",
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
    #[fold]
    pub stroke: Celled<Sides<Option<Option<Arc<Stroke>>>>>,

    #[internal]
    #[synthesized]
    pub grid: Arc<CellGrid>,

    /// グリッドセルのコンテンツと、[`grid.hline`]および[`grid.vline`]要素で指定される任意のグリッド線。
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

impl Synthesize for Packed<GridElem> {
    fn synthesize(
        &mut self,
        engine: &mut Engine,
        styles: StyleChain,
    ) -> SourceResult<()> {
        let grid = grid_to_cellgrid(self, engine, styles)?;
        self.grid = Some(Arc::new(grid));
        Ok(())
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
#[derive(Debug, Clone, PartialEq, Hash)]
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
#[derive(Debug, Clone, PartialEq, Hash)]
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

/// 繰り返し可能なグリッドのヘッダー。
///
/// `repeat`が`true`に設定されている場合、ヘッダーは改ページごとに繰り返されます。
/// 例として[`table.header`]要素および[`grid.stroke`]パラメーターのドキュメントを参照してください。
#[elem(name = "header", title = "Grid Header")]
pub struct GridHeader {
    /// ページごとにヘッダーを繰り返すかどうか。
    #[default(true)]
    pub repeat: bool,

    /// The level of the header. Must not be zero.
    ///
    /// This allows repeating multiple headers at once. Headers with different
    /// levels can repeat together, as long as they have ascending levels.
    ///
    /// Notably, when a header with a lower level starts repeating, all higher
    /// or equal level headers stop repeating (they are "replaced" by the new
    /// header).
    #[default(NonZeroU32::ONE)]
    pub level: NonZeroU32,

    /// ヘッダー内のセルと線。
    #[variadic]
    pub children: Vec<GridItem>,
}

/// 繰り返し可能なグリッドのフッター。
///
/// [`grid.header`]要素と同様に各ページで繰り返し可能です。
///
/// フッターの後に他のグリッドセルを配置できません。
#[elem(name = "footer", title = "Grid Footer")]
pub struct GridFooter {
    /// ページごとにフッターを繰り返すかどうか。
    #[default(true)]
    pub repeat: bool,

    /// フッター内のセルと線。
    #[variadic]
    pub children: Vec<GridItem>,
}

/// グリッドの水平方向の線。
///
/// グリッドの`stroke`フィールドを用いて指定されたものを含めてセルごとに設定されたストロークを上書きします。
/// グリッドの`column-gutter`オプションで作成されたセル間の間隔をまたげます。
///
/// この関数の例は[`table.hline`]要素のドキュメントにあります。
#[elem(name = "hline", title = "Grid Horizontal Line")]
pub struct GridHLine {
    /// 配置される水平方向の線の上にある行（0始まり）。
    /// `position`フィールドが`{bottom}`に設定されている場合、指定された番号の行の下に線が配置されます（詳細は[`grid.hline.position`]を参照してください）。
    ///
    /// `{auto}`を指定すると、その行が配置されるのは、グリッドの子要素のうち、その行より前にある、最後に自動配置されたセル（すなわち座標が上書きされていないセル）の下の行になります。
    /// その行の前にそのようなセルが存在しない場合、グリッドの一番上（0行目）に配置されます。
    /// このオプションをグリッドにある行の数と全く同じ値に設定すると、水平方向の線は一番下の境界線を上書きし、0に設定すると一番上の境界線を上書きすることに注意してください。
    pub y: Smart<usize>,

    /// 水平方向の線を開始する列（0始まりで、指定した列を含む）。
    pub start: usize,

    /// 水平方向の線が終了する直前の列（0始まりで、指定した列は含まない）。
    /// したがって、水平方向の線は`end - 1`列目まで引かれます。
    ///
    /// 値を`{none}`または列の数と同じにすると、線はグリッドの終端まで延びます。
    pub end: Option<NonZeroUsize>,

    /// 線のストローク。
    ///
    /// `{none}`を指定すると、水平方向の線の範囲にこれまで配置された全ての線が削除されます。
    /// これには水平方向の線の下にあるhlineやセルごとのストロークが含まれます。
    #[fold]
    #[default(Some(Arc::new(Stroke::default())))]
    pub stroke: Option<Arc<Stroke>>,

    /// 行（`y`）が与えられた場合の線が配置される位置。
    /// `{top}`か`{bottom}`のいずれかを指定し、それぞれその行の上または下に描画します。
    ///
    /// この設定は`row-gutter`が設定されている場合にのみ有効です（それ以外の場合は使用せず、単に`y`フィールドの値を1ずつ増やしてください）。
    /// これは、行の下部の位置と次の行の上部の位置の間に空白が発生し、両者が一致しなくなるためです。
    #[default(OuterVAlignment::Top)]
    pub position: OuterVAlignment,
}

/// グリッドの垂直方向の線。
///
/// グリッドのstrokeフィールドを用いて指定されたものを含めてセルごとに設定されたストロークを上書きします。
/// グリッドの`row-gutter`オプションで作成されたセル間の間隔をまたげます。
#[elem(name = "vline", title = "Grid Vertical Line")]
pub struct GridVLine {
    /// 配置される垂直方向の線の前にある列（0始まり）。
    /// `position`フィールドが`{end}`に設定されている場合、指定された番号の列の後に線が配置されます（詳細は[`grid.vline.position`]を参照してください）。
    ///
    /// `{auto}`を指定すると、垂直方向の線が配置されるのは、グリッドの子要素のうち、その線より前にある、最後に自動配置されたセル（すなわち座標が上書きされていないセル）の後の列になります。
    /// 垂直方向の線の前にそのようなセルが存在しない場合、グリッドの最初の列（0列目）に配置されます。
    /// このオプションをグリッドにある列の数と全く同じ値に設定すると、垂直方向の線は終端の境界線（LTRでは右端、RTLでは左端）を上書きし、0に設定すると始端の境界線（LTRでは左端、RTLでは右端）を上書きすることに注意してください。
    pub x: Smart<usize>,

    /// 垂直方向の線を開始する行（0始まりで、指定した行を含む）。
    pub start: usize,

    /// 垂直方向の線が終了する直前の行（0始まりで、指定した行は含まない）。
    /// したがって、垂直方向の線は`end - 1`行目まで引かれます。
    /// `{none}`または行数と同じ値を指定すると、線はグリッドの一番下まで延びます。
    pub end: Option<NonZeroUsize>,

    /// 線のストローク。
    ///
    /// `{none}`を指定すると、垂直方向の線の範囲にこれまで配置された全ての線が削除されます。
    /// これには垂直方向の線の下にあるvlineやセルごとのストロークが含まれます。
    #[fold]
    #[default(Some(Arc::new(Stroke::default())))]
    pub stroke: Option<Arc<Stroke>>,

    /// 列（`x`）が与えられた場合の線が配置される位置。
    /// `{start}`か`{end}`のいずれかを指定し、それぞれその前またはその後に描画します。
    ///
    /// `{left}`および`{right}`の値も使用可能ですが、左から右に書く文書と右から左に書く文書間でグリッドの挙動が一貫しなくなるため、非推奨です。
    ///
    /// この設定は`column-gutter`が設定されている場合にのみ有効です（それ以外の場合は使用せず、単に`x`フィールドの値を1ずつ増やしてください）。
    /// これは、ある列の直後の位置と次の列の直前の位置の間に空白が発生し、両者が一致しなくなるためです。

    #[default(OuterHAlignment::Start)]
    pub position: OuterHAlignment,
}

/// グリッドのセル。
/// グリッドの引数リストでこの関数を用いると各セルのスタイルプロパティを上書きしたり、グリッド内にセルを手動で配置したりできます。
/// showルールにこの関数を用いると複数のセルに対して特定のスタイルを一度に適用できます。
///
/// 例えば、以下のようにある単一セルの位置とストロークを上書きできます。
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
/// `grid.cell`に対してshowルールを用いると全てのセルに一括でスタイル設定ができます。
/// 例えば、セルの位置に基づいてスタイルを適用できます。
/// より詳しく知りたい場合は[`table.cell`]要素の例を参照してください。
#[elem(name = "cell", title = "Grid Cell")]
pub struct GridCell {
    /// セルの本文。
    #[required]
    pub body: Content,

    /// セルの列（0始まり）。
    /// このフィールドをshowルールで用いるとセルの列に応じたスタイルを適用できます。
    ///
    /// このフィールドを上書きすることでセルを配置する列を選択できます。
    /// 行（`y`）が選択されていない場合、セルは（0行目から始まる）使用可能な（存在しなければ新しい）最初の行に配置されます。
    /// 一方、`x`と`y`の両方が選択された場合は正確にその位置に配置されます。
    /// その位置が利用できない場合、エラーが発生します（したがって、通常はセルを自動配置する前に、カスタム位置を指定する方が賢明です）。
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

    /// セルの行（0始まり）。
    /// このフィールドをshowルールで用いるとセルの行に応じたスタイルを適用できます。
    ///
    /// このフィールドを上書きすることでセルを配置する行を選択できます。
    /// 列（`x`）が選択されていない場合、セルは（0列目から始まる）使用可能な最初の列に配置されます。
    /// 選ばれた行にある全ての列が既に埋まっている場合、エラーが発生します。
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

    /// このcellがまたぐ列の数。
    #[default(NonZeroUsize::ONE)]
    pub colspan: NonZeroUsize,

    /// このcellがまたぐ行の数。
    #[default(NonZeroUsize::ONE)]
    pub rowspan: NonZeroUsize,

    /// セルの[inset]($grid.inset)の上書き。
    pub inset: Smart<Sides<Option<Rel<Length>>>>,

    /// セルの[alignment]($grid.align)の上書き。
    pub align: Smart<Alignment>,

    /// セルの[fill]($grid.fill)の上書き。
    pub fill: Smart<Option<Paint>>,

    /// セルの[stroke]($grid.stroke)の上書き。
    #[fold]
    pub stroke: Sides<Option<Option<Arc<Stroke>>>>,

    #[internal]
    #[parse(Some(false))]
    pub is_repeated: bool,

    /// このcellがまたぐ行のページまたぎを許すかどうか。
    /// `{auto}`に設定された場合、固定サイズの行のみをまたぐセルは分割不可となり、`{auto}`サイズの行を少なくとも1つ含むセルは分割可能となります。
    pub breakable: Smart<bool>,
}

cast! {
    GridCell,
    v: Content => v.into(),
}

impl Default for Packed<GridCell> {
    fn default() -> Self {
        Packed::new(
            // Explicitly set colspan and rowspan to ensure they won't be
            // overridden by set rules (default cells are created after
            // colspans and rowspans are processed in the resolver)
            GridCell::new(Content::default())
                .with_colspan(NonZeroUsize::ONE)
                .with_rowspan(NonZeroUsize::ONE),
        )
    }
}

impl From<Content> for GridCell {
    fn from(value: Content) -> Self {
        #[allow(clippy::unwrap_or_default)]
        value.unpack::<Self>().unwrap_or_else(Self::new)
    }
}

/// A value that can be configured per cell.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Celled<T> {
    /// A bare value, the same for all cells.
    Value(T),
    /// A closure mapping from cell coordinates to a value.
    Func(Func),
    /// An array of values corresponding to each column. The array will be
    /// cycled if there are more columns than the array has items.
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
