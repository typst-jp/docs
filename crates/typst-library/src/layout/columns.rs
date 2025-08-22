use std::num::NonZeroUsize;

use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{elem, Content, NativeElement, Packed, Show, StyleChain};
use crate::layout::{BlockElem, Length, Ratio, Rel};

/// 領域を等幅の複数段に分割。
///
/// `column`関数を用いることで、あらゆるコンテナの内部を複数の段に分割することができます。
/// 現在、段の高さのバランスは取れません。
/// その代わり、段はコンテナの高さかページの残りの高さを占めます。
/// バランスを取った段組は将来的にサポートされる予定です。
///
/// # ページレベルの段組 { #page-level }
/// ドキュメント全体に渡る段組を挿入する必要がある場合は、代わりに`{page}`関数の[`columns`パラメーター]($page.columns)を使用してください。
/// これは、レイアウトコンテナ内のコンテンツ全てをラップするのではなく、ページレベルの段組を直接作成します。
/// 結果として[改ページ]($pagebreak)、[脚注]($footnote)および[行番号]($par.line)のようなものが期待通りの動作をし続けます。
/// より詳しくは[ページのセットアップガイドの関連する項目]($guides/page-setup-guide/#columns)をご覧ください。
///
/// # 段組の中断 { #breaking-out }
/// （例えば、論文のタイトルのように）段組を一時的に中断する場合は、親スコープでのフロート配置を使用してください。
///
/// ```example:single
/// #set page(columns: 2, height: 150pt)
///
/// #place(
///   top + center,
///   scope: "parent",
///   float: true,
///   text(1.4em, weight: "bold")[
///     My document
///   ],
/// )
///
/// #lorem(40)
/// ```
#[elem(Show)]
pub struct ColumnsElem {
    /// 段数。
    #[positional]
    #[default(NonZeroUsize::new(2).unwrap())]
    pub count: NonZeroUsize,

    /// 段間。
    #[resolve]
    #[default(Ratio::new(0.04).into())]
    pub gutter: Rel<Length>,

    /// 段内にレイアウトされるべきコンテンツ。
    #[required]
    pub body: Content,
}

impl Show for Packed<ColumnsElem> {
    fn show(&self, engine: &mut Engine, _: StyleChain) -> SourceResult<Content> {
        Ok(BlockElem::multi_layouter(self.clone(), engine.routines.layout_columns)
            .pack()
            .spanned(self.span()))
    }
}

/// 強制的な段の区切り。
///
/// この関数は、一段組やページ中の最後の段で使用されると、[改ページ]($pagebreak)と同じように振る舞います。
/// それ以外の場合、段区切りの後のコンテンツは次の段に配置されます。
///
/// # 例
/// ```example
/// #set page(columns: 2)
/// Preliminary findings from our
/// ongoing research project have
/// revealed a hitherto unknown
/// phenomenon of extraordinary
/// significance.
///
/// #colbreak()
/// Through rigorous experimentation
/// and analysis, we have discovered
/// a hitherto uncharacterized process
/// that defies our current
/// understanding of the fundamental
/// laws of nature.
/// ```
#[elem(title = "Column Break")]
pub struct ColbreakElem {
    /// `{true}`の場合、現在の段がすでに空のとき段区切りが実行されません。
    #[default(false)]
    pub weak: bool,
}
