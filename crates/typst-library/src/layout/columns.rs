use std::num::NonZeroUsize;

use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{elem, Content, NativeElement, Packed, Show, StyleChain};
use crate::layout::{BlockElem, Length, Ratio, Rel};

/// 複数の同じ大きさを持つカラムへの領域の分割。
///
/// `column`関数を用いることで、あらゆるコンテナの内部を複数のカラムに分割することができます。
/// 現在、カラムの高さのバランスは取れません。
/// その代わり、カラムはコンテナの高さかページの残りの高さを占めます。
/// バランスを取ったカラムは将来的にサポートされる予定です。
///
/// # ページレベルのカラム { #page-level }
/// ドキュメント全体を渡るカラムを挿入する必要がある場合は、代わりに`{page}`関数の[`columns`パラメーター]($page.columns)を使用してください。
/// これは、レイアウトコンテナ内のコンテンツ全てをラップするのではなく、ページレベルのカラムを直接作成します。
/// 結果として[ページ区切り]($pagebreak)、[脚注]($footnote)および[行番号]($par.line)のようなものが期待通りの動作をし続けます。
/// より詳しくは[ページのセットアップガイドの関連する項目]($guides/page-setup-guide/#columns)をご覧ください。
///
/// # カラムの中断 { #breaking-out }
/// （例えば、論文のタイトルのように）カラムを一時的に中断する場合は、親スコープでのフロート配置を使用してください。
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
    /// カラムの数。
    #[positional]
    #[default(NonZeroUsize::new(2).unwrap())]
    pub count: NonZeroUsize,

    /// 各カラム間の段間の大きさ。
    #[resolve]
    #[default(Ratio::new(0.04).into())]
    pub gutter: Rel<Length>,

    /// カラム内にレイアウトされるべき内容。
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

/// 強制的なカラムの区切り。
///
/// この関数は、一段組かページ中の最後のカラムで使用されると、[ページ区切り]($pagebreak)と同じように振る舞います。それ以外の場合、カラム区切りの後のコンテンツは次のカラムに配置されます。
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
    /// `{true}`の場合、現在のカラムがすでに空のとき、カラム区切りが実行されません。
    #[default(false)]
    pub weak: bool,
}
