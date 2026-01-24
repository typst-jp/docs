use ecow::EcoString;
use typst_syntax::Spanned;

use crate::diag::{LoadedWithin, SourceResult};
use crate::engine::Engine;
use crate::foundations::{Cast, func};
use crate::loading::{DataSource, Load, Readable};

/// ファイルからプレーンテキストやデータを読み込む。
///
/// デフォルトでは、ファイルはUTF-8として読み込まれ、[文字列]($str)として返されます。
///
/// `{encoding: none}`を指定した場合、この関数は代わりに生の[bytes]を返します。
///
/// # 例
/// ```example
/// An example for a HTML file: \
/// #let text = read("example.html")
/// #raw(text, lang: "html")
///
/// Raw bytes:
/// #read("tiger.jpg", encoding: none)
/// ```
#[func]
pub fn read(
    engine: &mut Engine,
    /// ファイルのパス。
    ///
    /// 詳細については、[パスのセクション]($syntax/#paths)を参照してください。
    path: Spanned<EcoString>,
    /// ファイルを読み込む際に使用するエンコーディング。
    ///
    /// `{none}`に設定すると、この関数は生のバイトを返します。
    #[named]
    #[default(Some(Encoding::Utf8))]
    encoding: Option<Encoding>,
) -> SourceResult<Readable> {
    let loaded = path.map(DataSource::Path).load(engine.world)?;
    Ok(match encoding {
        None => Readable::Bytes(loaded.data),
        Some(Encoding::Utf8) => Readable::Str(loaded.data.to_str().within(&loaded)?),
    })
}

/// ファイルのエンコーディング。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum Encoding {
    /// Unicode UTF-8エンコーディング。
    Utf8,
}
