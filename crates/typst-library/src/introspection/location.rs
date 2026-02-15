use std::fmt::{self, Debug, Formatter};
use std::num::NonZeroUsize;

use ecow::EcoString;

use crate::engine::Engine;
use crate::foundations::{Repr, func, scope, ty};
use crate::layout::Position;
use crate::model::Numbering;

/// 文書中の要素の識別。
///
/// locationは文書中の要素を一意に識別し、ページ中での絶対位置へのアクセスを提供します。
/// [`here`]関数を用いて現在のロケーションを取得可能です。
/// また、検索したロケーションや表示された要素のロケーションは、コンテンツの[`location()`]($content.location)メソッドを使って取得できます。
///
/// # ロケータブル要素 { #locatable }
/// ロケーションが自動的に割り当てられる要素は _ロケータブル_ と呼ばれます。
/// 効率上の理由から、全ての要素がロケータブルであるわけではありません。
///
/// - [Modelカテゴリ]($category/model)では、ほとんどの要素がロケータブルです。
///   これは[見出し]($heading)や[図表]($figure)などの意味的要素が内省で
///   よく使われるためです。
///
/// - [Textカテゴリ]($category/text)では、[`raw`]要素と装飾要素
///   [`underline`], [`overline`], [`strike`], [`highlight`]がロケータブルです。
///
/// - [Introspectionカテゴリ]($category/introspection)では、[`metadata`]
///   要素がロケータブルです。
///
/// - その他のカテゴリでは、多くの要素がロケータブルではありません。
///   例外として、[`math.equation`]と[`image`]があります。
///
/// 特定の要素がロケータブルかどうかは、[`query`]を試すことで確認できます。
///
/// ロケータブルでない要素でも、ラベルが付いている場合は
/// クエリで観測できることがあります。
#[ty(scope)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Location(u128);

impl Location {
    /// Create a new location from a unique hash.
    pub fn new(hash: u128) -> Self {
        Self(hash)
    }

    /// Extract the raw hash.
    pub fn hash(self) -> u128 {
        self.0
    }

    /// Produces a well-known variant of this location.
    ///
    /// This is a synthetic location created from another one and is used, for
    /// example, in bibliography management to create individual linkable
    /// locations for reference entries from the bibliography's location.
    pub fn variant(self, n: usize) -> Self {
        Self(typst_utils::hash128(&(self.0, n)))
    }
}

#[scope]
impl Location {
    /// このlocationのページ番号を返します。
    ///
    /// このlocationの[ページカウンター]($counter)の値を返すのではなく、（1始まりの）実際のページ番号を返すことに注意してください。
    ///
    /// ページカウンターの値が知りたい場合は代わりに`{counter(page).at(loc)}`を使用してください。
    ///
    /// [`here`]と組み合わせることで現在のコンテキストにおける実際のページ番号が取得できます。
    /// ```example
    /// #context [
    ///   I am located on
    ///   page #here().page()
    /// ]
    /// ```
    #[func]
    pub fn page(self, engine: &mut Engine) -> NonZeroUsize {
        engine.introspector.page(self)
    }

    /// このlocationのページ番号とx座標とy座標を辞書で返します。
    /// ページ番号は1始まりで、座標はページの左上から測ります。
    ///
    /// ページ番号のみに興味がある場合は、代わりに`page()`を使用すると不要な処理を省略できます。
    #[func]
    pub fn position(self, engine: &mut Engine) -> Position {
        engine.introspector.position(self)
    }

    /// このlocationのページ番号の番号付けパターンを返します。
    /// これにより、ページカウンターの表示する際に、その位置での番号付けを取得できます。
    /// これは独自の索引やアウトラインを作成する場合に便利です。
    ///
    /// そのロケーションのページの番号付けが`{none}`に設定されていた場合、`{none}`を返します。
    #[func]
    pub fn page_numbering(self, engine: &mut Engine) -> Option<Numbering> {
        engine.introspector.page_numbering(self).cloned()
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "Location({})", self.0)
        } else {
            // Print a shorter version by default to make it more readable.
            let truncated = self.0 as u16;
            write!(f, "Location({truncated})")
        }
    }
}

impl Repr for Location {
    fn repr(&self) -> EcoString {
        "..".into()
    }
}

/// Can be used to have a location as a key in an ordered set or map.
///
/// [`Location`] itself does not implement [`Ord`] because comparing hashes like
/// this has no semantic meaning. The potential for misuse (e.g. checking
/// whether locations have a particular relative ordering) is relatively high.
///
/// Still, it can be useful to have orderable locations for things like sets.
/// That's where this type comes in.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LocationKey(u128);

impl LocationKey {
    /// Create a location key from a location.
    pub fn new(location: Location) -> Self {
        Self(location.0)
    }
}

impl From<Location> for LocationKey {
    fn from(location: Location) -> Self {
        Self::new(location)
    }
}

/// Make this element available in the introspector.
pub trait Locatable {}

/// Make this element not queriable for the user.
pub trait Unqueriable: Locatable {}

/// Marks this element as tagged in PDF files.
pub trait Tagged {}
