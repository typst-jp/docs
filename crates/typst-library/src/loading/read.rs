use ecow::EcoString;
use typst_syntax::Spanned;

<<<<<<< HEAD
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
=======
use crate::diag::{LoadedWithin, SourceResult};
use crate::engine::Engine;
use crate::foundations::{Cast, func};
use crate::loading::{DataSource, Load, Readable};

/// Reads plain text or data from a file.
///
/// By default, the file will be read as UTF-8 and returned as a [string]($str).
///
/// If you specify `{encoding: none}`, this returns raw [bytes] instead.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
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
<<<<<<< HEAD
    /// ファイルのパス。
    ///
    /// 詳細については、[パスのセクション]($syntax/#paths)を参照してください。
    path: Spanned<EcoString>,
    /// ファイルを読み込む際に使用するエンコーディング。
    ///
    /// `{none}`に設定すると、この関数は生のバイトを返します。
=======
    /// Path to a file.
    ///
    /// For more details, see the [Paths section]($syntax/#paths).
    path: Spanned<EcoString>,
    /// The encoding to read the file with.
    ///
    /// If set to `{none}`, this function returns raw bytes.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[named]
    #[default(Some(Encoding::Utf8))]
    encoding: Option<Encoding>,
) -> SourceResult<Readable> {
<<<<<<< HEAD
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
=======
    let loaded = path.map(DataSource::Path).load(engine.world)?;
    Ok(match encoding {
        None => Readable::Bytes(loaded.data),
        Some(Encoding::Utf8) => Readable::Str(loaded.data.to_str().within(&loaded)?),
    })
}

/// An encoding of a file.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum Encoding {
    /// The Unicode UTF-8 encoding.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    Utf8,
}
