<<<<<<< HEAD
use crate::foundations::{elem, scope, Cast, Content, Packed, Smart};
use crate::introspection::{Locatable, Unqueriable};
use crate::layout::{Alignment, Em, Length, Rel};

/// 親コンテナに対して相対的なコンテンツの配置。
///
/// コンテンツはオーバーレイ（デフォルト）またはフロートのいずれかで配置できます。
/// オーバーレイのコンテンツは、指定された[`alignment`]($place.alignment)に従って親コンテナにあわせて配置され、これまでに追加された他のコンテンツの上に表示されます。
/// フロートのコンテンツは、親コンテナの上部または下部に配置され、他のコンテンツを下または上にそれぞれずらします。
/// 両方の場合で、コンテンツの位置は[`dx`]($place.dx)および[`dy`]($place.dy)オフセットを用いてレイアウトに影響を与えることなく調整可能です。
///
/// [`block`]、[`box`]、[`rect`]などの任意のコンテナが親になることができます。
/// トップレベルの`place`の呼び出しは現在のページのテキスト領域に直接コンテンツを配置します。
/// これはページ上の絶対位置指定に使用できます。
/// `top + left` [`alignment`]($place.alignment)と組み合わせると、`dx`および`dy`オフセットはテキスト領域の左上隅を基準として要素の左上隅の位置を設定します。
/// マージンを含めたページ全体における絶対位置指定は、[`page.foreground`]か[`page.background`]中で`place`を使うと可能です。
///
/// # 例
=======
use crate::foundations::{Cast, Content, Smart, elem, scope};
use crate::introspection::{Locatable, Tagged, Unqueriable};
use crate::layout::{Alignment, Em, Length, Rel};

/// Places content relatively to its parent container.
///
/// Placed content can be either overlaid (the default) or floating. Overlaid
/// content is aligned with the parent container according to the given
/// [`alignment`]($place.alignment), and shown over any other content added so
/// far in the container. Floating content is placed at the top or bottom of
/// the container, displacing other content down or up respectively. In both
/// cases, the content position can be adjusted with [`dx`]($place.dx) and
/// [`dy`]($place.dy) offsets without affecting the layout.
///
/// The parent can be any container such as a [`block`], [`box`],
/// [`rect`], etc. A top level `place` call will place content directly
/// in the text area of the current page. This can be used for absolute
/// positioning on the page: with a `top + left`
/// [`alignment`]($place.alignment), the offsets `dx` and `dy` will set the
/// position of the element's top left corner relatively to the top left corner
/// of the text area. For absolute positioning on the full page including
/// margins, you can use `place` in [`page.foreground`] or [`page.background`].
///
/// # Examples
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// #set page(height: 120pt)
/// Hello, world!
///
/// #rect(
///   width: 100%,
///   height: 2cm,
///   place(horizon + right, square()),
/// )
///
/// #place(
///   top + left,
///   dx: -5pt,
///   square(size: 5pt, fill: red),
/// )
/// ```
///
<<<<<<< HEAD
/// # 他の要素の位置に対する影響 { #effect-on-other-elements }
/// 要素のオーバーレイはコンテンツの流れの中にスペースを取りませんが、`place`の呼び出しは、流れの中にブロックレベルの不可視要素を挿入します。
/// これは現在の段落を中断することでレイアウトに影響を与える可能性があります。
/// これを避けるために、段落の途中で`place`を呼び出す場合には、それを[`box`]でラップするとよいでしょう。
/// このとき、配置とオフセットは、この大きさを持たないボックスを基準にしたものになります。
/// 間隔に影響しないように、ワードジョイナーを使用してボックスを単語に結合してください。
///
/// 例えば、以下は直後の単語に注釈を付与する関数を定義しています。
=======
/// # Effect on the position of other elements { #effect-on-other-elements }
/// Overlaid elements don't take space in the flow of content, but a `place`
/// call inserts an invisible block-level element in the flow. This can
/// affect the layout by breaking the current paragraph. To avoid this,
/// you can wrap the `place` call in a [`box`] when the call is made
/// in the middle of a paragraph. The alignment and offsets will then be
/// relative to this zero-size box. To make sure it doesn't interfere with
/// spacing, the box should be attached to a word using a word joiner.
///
/// For example, the following defines a function for attaching an annotation
/// to the following word:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// >>> #set page(height: 70pt)
/// #let annotate(..args) = {
///   box(place(..args))
///   sym.wj
///   h(0pt, weak: true)
/// }
///
/// A placed #annotate(square(), dy: 2pt)
/// square in my text.
/// ```
///
<<<<<<< HEAD
/// ゼロ幅の弱い空白は、関数呼び出しと次の単語との間の空白を削除する役割を果たします。
#[elem(scope, Locatable, Unqueriable)]
pub struct PlaceElem {
    /// コンテンツを配置する基準となる親コンテナ内の位置。
    ///
    /// - `float`が`{false}`の場合、`{auto}`以外の任意のalignmentを指定できます。
    /// - `float`が`{true}`の場合、`{auto}`、`{top}`あるいは`{bottom}`のいずれかを指定しなければなりません。
    ///
    /// `float`が`{false}`で、かつvertical alignmentが指定されていなければ、コンテンツは垂直方向の軸上の現在の位置に配置されます。
=======
/// The zero-width weak spacing serves to discard spaces between the function
/// call and the next word.
///
/// # Accessibility
/// Assistive Technology (AT) will always read the placed element at the point
/// where it logically appears in the document, regardless of where this
/// function physically moved it. Put its markup where it would make the most
/// sense in the reading order.
#[elem(scope, Unqueriable, Locatable, Tagged)]
pub struct PlaceElem {
    /// Relative to which position in the parent container to place the content.
    ///
    /// - If `float` is `{false}`, then this can be any alignment other than `{auto}`.
    /// - If `float` is `{true}`, then this must be `{auto}`, `{top}`, or `{bottom}`.
    ///
    /// When `float` is `{false}` and no vertical alignment is specified, the
    /// content is placed at the current position on the vertical axis.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[positional]
    #[default(Smart::Custom(Alignment::START))]
    pub alignment: Smart<Alignment>,

<<<<<<< HEAD
    /// 何かを配置するときの基準となる包含スコープ。
    ///
    /// 親スコープは主に図表に使用されるため、figure関数にはそれを反映した[`scope`パラメーター]($figure.scope)が用意されています。
    /// しかしながら、段組を中断することは、より一般的な場合に有用であることもあります。
    /// 典型的な例は2段組の文書で1段組のタイトル節を作成することでしょう。
    ///
    /// 現在、親スコープでの配置は`float`が`{true}`の場合のみサポートされています。 この挙動は将来変更される可能性があります。
=======
    /// Relative to which containing scope something is placed.
    ///
    /// The parent scope is primarily used with figures and, for
    /// this reason, the figure function has a mirrored [`scope`
    /// parameter]($figure.scope). Nonetheless, it can also be more generally
    /// useful to break out of the columns. A typical example would be to
    /// [create a single-column title section]($guides/page-setup/#columns)
    /// in a two-column document.
    ///
    /// Note that parent-scoped placement is currently only supported if `float`
    /// is `{true}`. This may change in the future.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set page(height: 150pt, columns: 2)
    /// #place(
    ///   top + center,
    ///   scope: "parent",
    ///   float: true,
    ///   rect(width: 80%, fill: aqua),
    /// )
    ///
    /// #lorem(25)
    /// ```
    pub scope: PlacementScope,

<<<<<<< HEAD
    /// 要素をフロートレイアウトで配置するかどうか。
    ///
    /// フロートの要素は流れの中にあるコンテンツをずらして親コンテナの上部または下部に位置取ります。
    ///
    /// それらは常に互いのフロー内での順序を保ち、後続の[`place.flush`]要素に続くコンテンツより前に配置されます。
=======
    /// Whether the placed element has floating layout.
    ///
    /// Floating elements are positioned at the top or bottom of the parent
    /// container, displacing in-flow content. They are always placed in the
    /// in-flow order relative to each other, as well as before any content
    /// following a later [`place.flush`] element.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set page(height: 150pt)
    /// #let note(where, body) = place(
    ///   center + where,
    ///   float: true,
    ///   clearance: 6pt,
    ///   rect(body),
    /// )
    ///
    /// #lorem(10)
    /// #note(bottom)[Bottom 1]
    /// #note(bottom)[Bottom 2]
    /// #lorem(40)
    /// #note(top)[Top]
    /// #lorem(10)
    /// ```
    pub float: bool,

<<<<<<< HEAD
    /// フロートレイアウトでの配置された要素と他の要素との間隔。
    ///
    /// `float`が`{false}`の場合は影響がありません。

    #[default(Em::new(1.5).into())]
    #[resolve]
    pub clearance: Length,

    /// 配置したコンテンツの水平方向の変位。
=======
    /// The spacing between the placed element and other elements in a floating
    /// layout.
    ///
    /// Has no effect if `float` is `{false}`.
    #[default(Em::new(1.5).into())]
    pub clearance: Length,

    /// The horizontal displacement of the placed content.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set page(height: 100pt)
    /// #for i in range(16) {
    ///   let amount = i * 4pt
    ///   place(center, dx: amount - 32pt, dy: amount)[A]
    /// }
    /// ```
    ///
<<<<<<< HEAD
    /// これは流れの中のコンテンツのレイアウトには影響しません。
    /// 言い換えると、配置されたコンテンツは[`move`]要素にラップされたかのように扱われます。
    pub dx: Rel<Length>,

    /// 配置したコンテンツの垂直方向の変位。
    ///
    /// これは流れの中のコンテンツのレイアウトには影響しません。
    /// 言い換えると、配置されたコンテンツは[`move`]要素にラップされたかのように扱われます。
    pub dy: Rel<Length>,

    /// 配置するコンテンツ。
=======
    /// This does not affect the layout of in-flow content.
    /// In other words, the placed content is treated as if it
    /// were wrapped in a [`move`] element.
    pub dx: Rel<Length>,

    /// The vertical displacement of the placed content.
    ///
    /// This does not affect the layout of in-flow content.
    /// In other words, the placed content is treated as if it
    /// were wrapped in a [`move`] element.
    pub dy: Rel<Length>,

    /// The content to place.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[required]
    pub body: Content,
}

<<<<<<< HEAD
/// `PlaceElem` must be locatable to support logical ordering of floats, but I
/// do not want to expose `query(place)` for now.
impl Unqueriable for Packed<PlaceElem> {}

=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
#[scope]
impl PlaceElem {
    #[elem]
    type FlushElem;
}

/// Relative to which containing scope something shall be placed.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum PlacementScope {
<<<<<<< HEAD
    /// 現在の列に配置する。
    #[default]
    Column,
    /// 親要素に対して相対的な位置に配置され、コンテンツが全ての列にまたがって表示されます。
    Parent,
}

/// コンテンツの続行前に保留中のフロート要素を配置するようレイアウトアルゴリズムに指示。
///
/// これは、次のセクションにフロートの図表が流れ込むのを防ぐのに便利です。
=======
    /// Place into the current column.
    #[default]
    Column,
    /// Place relative to the parent, letting the content span over all columns.
    Parent,
}

/// Asks the layout algorithm to place pending floating elements before
/// continuing with the content.
///
/// This is useful for preventing floating figures from spilling
/// into the next section.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// >>> #set page(height: 160pt, width: 150pt)
/// #lorem(15)
///
/// #figure(
///   rect(width: 100%, height: 50pt),
///   placement: auto,
///   caption: [A rectangle],
/// )
///
/// #place.flush()
///
/// This text appears after the figure.
/// ```
#[elem]
pub struct FlushElem {}
