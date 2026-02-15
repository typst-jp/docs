use az::SaturatingAs;
use typst_syntax::Spanned;

use crate::diag::{LineCol, LoadError, LoadedWithin, ReportPos, SourceResult, bail};
use crate::engine::Engine;
use crate::foundations::{Array, Dict, IntoValue, Type, Value, cast, func, scope};
use crate::loading::{DataSource, Load, Readable};

/// CSVファイルから構造化データを読み込む。
///
/// CSVファイルは読み込まれ、文字列からなる2次元配列にパースされます。
/// 具体的には、CSVファイルの各行が文字列の配列として表現され、
/// 全ての行が単一の配列にまとめられます。
/// ヘッダー行は削除されません。
///
/// # 例
/// ```example
/// #let results = csv("example.csv")
///
/// #table(
///   columns: 2,
///   [*Condition*], [*Result*],
///   ..results.flatten(),
/// )
/// ```
#[func(scope, title = "CSV")]
pub fn csv(
    engine: &mut Engine,
    /// CSVファイルの[パス]($syntax/#paths)、または生のCSVバイト列。
    source: Spanned<DataSource>,
    /// CSVファイルの列を区切る区切り文字。
    /// 単一のASCII文字でなければなりません。
    #[named]
    #[default]
    delimiter: Delimiter,
    /// ファイルの各行の表現方法。
    ///
    /// - `array`に設定すると、
    ///   各行は単純な文字列の配列として表現されます。
    /// - `dictionary`に設定すると、
    ///   各行はヘッダーのキーと文字列を対応付けた辞書として表現されます。
    ///   このオプションは、CSVファイルにヘッダー行が存在する場合にのみ意味があります。
    #[named]
    #[default(RowType::Array)]
    row_type: RowType,
) -> SourceResult<Array> {
    let loaded = source.load(engine.world)?;

    let mut builder = ::csv::ReaderBuilder::new();
    let has_headers = row_type == RowType::Dict;
    builder.has_headers(has_headers);
    builder.delimiter(delimiter.0 as u8);

    // Counting lines from 1 by default.
    let mut line_offset: usize = 1;
    let mut reader = builder.from_reader(loaded.data.as_slice());
    let mut headers: Option<::csv::StringRecord> = None;

    if has_headers {
        // Counting lines from 2 because we have a header.
        line_offset += 1;
        headers = Some(
            reader
                .headers()
                .cloned()
                .map_err(|err| format_csv_error(err, 1))
                .within(&loaded)?,
        );
    }

    let mut array = Array::new();
    for (line, result) in reader.records().enumerate() {
        // Original solution was to use line from error, but that is
        // incorrect with `has_headers` set to `false`. See issue:
        // https://github.com/BurntSushi/rust-csv/issues/184
        let line = line + line_offset;
        let row = result.map_err(|err| format_csv_error(err, line)).within(&loaded)?;
        let item = if let Some(headers) = &headers {
            let mut dict = Dict::new();
            for (field, value) in headers.iter().zip(&row) {
                dict.insert(field.into(), value.into_value());
            }
            dict.into_value()
        } else {
            let sub = row.into_iter().map(|field| field.into_value()).collect();
            Value::Array(sub)
        };
        array.push(item);
    }

    Ok(array)
}

#[scope]
impl csv {
    /// CSVの文字列やバイト列から構造化データを読み込む。
    #[func(title = "Decode CSV")]
    #[deprecated(
        message = "`csv.decode`は非推奨です。代わりにバイト列を直接`csv`に渡してください。",
        until = "0.15.0"
    )]
    pub fn decode(
        engine: &mut Engine,
        /// CSVデータ。
        data: Spanned<Readable>,
        /// CSVファイルの列を区切る区切り文字。
        /// 単一のASCII文字でなければなりません。
        #[named]
        #[default]
        delimiter: Delimiter,
        /// ファイルの各行の表現方法。
        ///
        /// - `array`に設定すると、
        ///   各行は単純な文字列の配列として表現されます。
        /// - `dictionary`に設定すると、
        ///   各行はヘッダーのキーと文字列を対応付けた辞書として表現されます。
        ///   このオプションは、CSVファイルにヘッダー行が存在する場合にのみ意味があります。
        #[named]
        #[default(RowType::Array)]
        row_type: RowType,
    ) -> SourceResult<Array> {
        csv(engine, data.map(Readable::into_source), delimiter, row_type)
    }
}

/// The delimiter to use when parsing CSV files.
pub struct Delimiter(char);

impl Default for Delimiter {
    fn default() -> Self {
        Self(',')
    }
}

cast! {
    Delimiter,
    self => self.0.into_value(),
    c: char => if c.is_ascii() {
        Self(c)
    } else {
        bail!("delimiter must be an ASCII character")
    },
}

/// The type of parsed rows.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum RowType {
    Array,
    Dict,
}

cast! {
    RowType,
    self => match self {
        Self::Array => Type::of::<Array>(),
        Self::Dict => Type::of::<Dict>(),
    }.into_value(),
    ty: Type => {
        if ty == Type::of::<Array>() {
            Self::Array
        } else if ty == Type::of::<Dict>() {
            Self::Dict
        } else {
            bail!("expected `array` or `dictionary`");
        }
    },
}

/// Format the user-facing CSV error message.
fn format_csv_error(err: ::csv::Error, line: usize) -> LoadError {
    let msg = "failed to parse CSV";
    let pos = (err.kind().position())
        .map(|pos| {
            let start = pos.byte().saturating_as();
            ReportPos::from(start..start)
        })
        .unwrap_or(LineCol::one_based(line, 1).into());
    match err.kind() {
        ::csv::ErrorKind::Utf8 { .. } => {
            LoadError::new(pos, msg, "file is not valid utf-8")
        }
        ::csv::ErrorKind::UnequalLengths { expected_len, len, .. } => {
            let err =
                format!("found {len} instead of {expected_len} fields in line {line}");
            LoadError::new(pos, msg, err)
        }
        _ => LoadError::new(pos, "failed to parse CSV", err),
    }
}
