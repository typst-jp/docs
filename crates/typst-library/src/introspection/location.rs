use std::fmt::{self, Debug, Formatter};
use std::num::NonZeroUsize;

use ecow::EcoString;

use crate::engine::Engine;
use crate::foundations::{func, scope, ty, Repr};
use crate::layout::Position;
use crate::model::Numbering;

/// 文書中の要素の識別。
///
/// locationは文書中の要素を一意に識別し、ページ中での絶対位置へのアクセスを提供します。
/// [`here`]関数を用いて現在のロケーションを取得可能です。
/// また、検索したロケーションや表示された要素のロケーションは、コンテンツの[`location()`]($content.location)メソッドを使って取得できます。
///
/// # ロケータブル要素 { #locatable }
/// 現在、要素関数の一部のみがロケーションを取得可能です。
/// 見出しや図表の他に、数式、参照、引用、全ての明示的なラベルを持つ要素が該当します。
/// したがって、例えば[`strong`]要素に対してクエリが実行 _可能_ ですが、見つかるのは明示的にラベルが付けられたもののみです。
/// この制限は将来的に解消される予定です。
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
        write!(f, "Location({})", self.0)
    }
}

impl Repr for Location {
    fn repr(&self) -> EcoString {
        "..".into()
    }
}

/// Makes this element as locatable through the introspector.
pub trait Locatable {}

/// Marks this element as not being queryable even though it is locatable for
/// internal reasons.
pub trait Unqueriable {}
