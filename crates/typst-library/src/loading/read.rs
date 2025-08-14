use ecow::EcoString;
use typst_syntax::Spanned;

use crate::diag::{At, FileError, SourceResult};
use crate::engine::Engine;
use crate::foundations::{func, Cast};
use crate::loading::Readable;
use crate::World;

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
    let Spanned { v: path, span } = path;
    let id = span.resolve_path(&path).at(span)?;
    let data = engine.world.file(id).at(span)?;
    Ok(match encoding {
        None => Readable::Bytes(data),
        Some(Encoding::Utf8) => {
            Readable::Str(data.to_str().map_err(FileError::from).at(span)?)
        }
    })
}

/// ファイルのエンコーディング。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum Encoding {
    /// Unicode UTF-8エンコーディング。
    Utf8,
}
