<<<<<<< HEAD
use smallvec::{smallvec, SmallVec};
use typst_syntax::Spanned;
use typst_utils::{default_math_class, Numeric};
use unicode_math_class::MathClass;

use crate::diag::{bail, At, HintedStrResult, StrResult};
use crate::foundations::{
    array, cast, dict, elem, Array, Content, Dict, Fold, NoneValue, Resolve, Smart,
    StyleChain, Symbol, Value,
=======
use smallvec::{SmallVec, smallvec};
use typst_syntax::Spanned;
use typst_utils::{Numeric, default_math_class};
use unicode_math_class::MathClass;

use crate::diag::{At, HintedStrResult, StrResult, bail};
use crate::foundations::{
    Array, Content, Dict, Fold, NoneValue, Resolve, Smart, StyleChain, Symbol, Value,
    array, cast, dict, elem,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
};
use crate::layout::{Abs, Em, HAlignment, Length, Rel};
use crate::math::Mathy;
use crate::visualize::Stroke;

const DEFAULT_ROW_GAP: Em = Em::new(0.2);
const DEFAULT_COL_GAP: Em = Em::new(0.5);

<<<<<<< HEAD
/// 列ベクトル。
///
/// ベクトルの要素内のコンテンツは[`align`]($math.vec.align)パラメーターか`&`記号を用いて配置できます。
///
/// # 例
=======
/// A column vector.
///
/// Content in the vector's elements can be aligned with the
/// [`align`]($math.vec.align) parameter, or the `&` symbol.
///
/// This function is for typesetting vector components. To typeset a symbol that
/// represents a vector, [`arrow`]($math.accent) and [`bold`]($math.bold) are
/// commonly used.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// $ vec(a, b, c) dot vec(1, 2, 3)
///     = a + 2b + 3c $
/// ```
#[elem(title = "Vector", Mathy)]
pub struct VecElem {
<<<<<<< HEAD
    /// 用いる区切り文字。
    ///
    /// 単一の文字で左区切り文字を指定する場合、右区切り文字は自動的に推論されます。
    /// それ以外の場合は、左区切り文字と右区切り文字を含む配列を指定します。
=======
    /// The delimiter to use.
    ///
    /// Can be a single character specifying the left delimiter, in which case
    /// the right delimiter is inferred. Otherwise, can be an array containing a
    /// left and a right delimiter.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set math.vec(delim: "[")
    /// $ vec(1, 2) $
    /// ```
    #[default(DelimiterPair::PAREN)]
    pub delim: DelimiterPair,

<<<<<<< HEAD
    /// 各要素の水平方向の配置。
=======
    /// The horizontal alignment that each element should have.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set math.vec(align: right)
    /// $ vec(-1, 1, -1) $
    /// ```
<<<<<<< HEAD
    #[resolve]
    #[default(HAlignment::Center)]
    pub align: HAlignment,

    /// 要素間の間隔。
=======
    #[default(HAlignment::Center)]
    pub align: HAlignment,

    /// The gap between elements.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set math.vec(gap: 1em)
    /// $ vec(1, 2) $
    /// ```
<<<<<<< HEAD
    #[resolve]
    #[default(DEFAULT_ROW_GAP.into())]
    pub gap: Rel<Length>,

    /// ベクトルの要素。
=======
    #[default(DEFAULT_ROW_GAP.into())]
    pub gap: Rel<Length>,

    /// The elements of the vector.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[variadic]
    pub children: Vec<Content>,
}

<<<<<<< HEAD
/// 行列。
///
/// 行内の要素はカンマで区切り、行自身はセミコロンで区切らなければなりません。
/// セミコロン構文は、直前にあるカンマ区切りの引数を配列にマージします。
/// 数式関数呼び出しに関するこの特殊な構文は、2次元データを引数に取るカスタム関数の定義にも使用できます。
///
/// セル内のコンテンツは[`align`]($math.mat.align)パラメーターを用いて配置できます。 また、同じ行にあるコンテンツは`&`記号を用いて配置できます。
///
/// # 例
=======
/// A matrix.
///
/// The elements of a row should be separated by commas, while the rows
/// themselves should be separated by semicolons. The semicolon syntax merges
/// preceding arguments separated by commas into an array. You can also use this
/// special syntax of math function calls to define custom functions that take
/// 2D data.
///
/// Content in cells can be aligned with the [`align`]($math.mat.align)
/// parameter, or content in cells that are in the same row can be aligned with
/// the `&` symbol.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// $ mat(
///   1, 2, ..., 10;
///   2, 2, ..., 10;
///   dots.v, dots.v, dots.down, dots.v;
///   10, 10, ..., 10;
/// ) $
/// ```
#[elem(title = "Matrix", Mathy)]
pub struct MatElem {
<<<<<<< HEAD
    /// 用いる区切り文字。
    ///
    /// 単一の文字で左区切り文字を指定する場合、右区切り文字は自動的に推論されます。
    /// それ以外の場合は、左区切り文字と右区切り文字を含む配列を指定します。
=======
    /// The delimiter to use.
    ///
    /// Can be a single character specifying the left delimiter, in which case
    /// the right delimiter is inferred. Otherwise, can be an array containing a
    /// left and a right delimiter.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set math.mat(delim: "[")
    /// $ mat(1, 2; 3, 4) $
    /// ```
    #[default(DelimiterPair::PAREN)]
    pub delim: DelimiterPair,

<<<<<<< HEAD
    /// 各セルの水平方向の配置。
=======
    /// The horizontal alignment that each cell should have.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set math.mat(align: right)
    /// $ mat(-1, 1, 1; 1, -1, 1; 1, 1, -1) $
    /// ```
<<<<<<< HEAD
    #[resolve]
    #[default(HAlignment::Center)]
    pub align: HAlignment,

    /// 行列内に補助線を描画。
    ///
    /// - `{none}`: 線は描画されません。
    /// - 単一の数値: 指定された列番号の後に垂直方向の線を描画します。
    /// 負数の場合は最後の列から数え始めます。
    /// - 辞書: 水平方向および垂直方向の両方で複数の補助線を描画できます。
    /// 加えて線のスタイルを設定可能です。
    /// 辞書には以下のキーを含めることができます。
    ///   - `hline`: 水平方向の線を描画するオフセット。
    ///     例えば、オフセットを`2`とすると行列の2行目の後に水平方向の線が描かれます。
    ///     単一の線を描く場合は整数を、複数の線の場合は整数の配列を受け取ります。
    ///     単一の数値を指定する場合と同様に、負数の場合は末尾から数え始めます。
    ///   - `vline`: 垂直方向の線を描画するオフセット。
    ///     例えば、オフセットを`2`とすると行列の2列目の後に垂直方向の線が描かれます。
    ///     単一の線を描く場合は整数を、複数の線の場合は整数の配列を受け取ります。
    ///   - `stroke`: 線の[ストローク]($stroke)。
    ///     `{auto}`が指定された場合、0.05emの太さで四角い線端になります。
    ///
    /// ```example
=======
    #[default(HAlignment::Center)]
    pub align: HAlignment,

    /// Draws augmentation lines in a matrix.
    ///
    /// - `{none}`: No lines are drawn.
    /// - A single number: A vertical augmentation line is drawn
    ///   after the specified column number. Negative numbers start from the end.
    /// - A dictionary: With a dictionary, multiple augmentation lines can be
    ///   drawn both horizontally and vertically. Additionally, the style of the
    ///   lines can be set. The dictionary can contain the following keys:
    ///   - `hline`: The offsets at which horizontal lines should be drawn.
    ///     For example, an offset of `2` would result in a horizontal line
    ///     being drawn after the second row of the matrix. Accepts either an
    ///     integer for a single line, or an array of integers
    ///     for multiple lines. Like for a single number, negative numbers start from the end.
    ///   - `vline`: The offsets at which vertical lines should be drawn.
    ///     For example, an offset of `2` would result in a vertical line being
    ///     drawn after the second column of the matrix. Accepts either an
    ///     integer for a single line, or an array of integers
    ///     for multiple lines. Like for a single number, negative numbers start from the end.
    ///   - `stroke`: How to [stroke]($stroke) the line. If set to `{auto}`,
    ///     takes on a thickness of 0.05 em and square line caps.
    ///
    /// ```example:"Basic usage"
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    /// $ mat(1, 0, 1; 0, 1, 2; augment: #2) $
    /// // Equivalent to:
    /// $ mat(1, 0, 1; 0, 1, 2; augment: #(-1)) $
    /// ```
    ///
<<<<<<< HEAD
    /// ```example
    /// $ mat(0, 0, 0; 1, 1, 1; augment: #(hline: 1, stroke: 2pt + green)) $
    /// ```
    #[resolve]
    #[fold]
    pub augment: Option<Augment>,

    /// 行間と列間の間隔。
    ///
    /// これは`row-gap`と`column-gap`を同じ値で設定する省略記法です。
=======
    /// ```example:"Customizing the augmentation line"
    /// $ mat(0, 0, 0; 1, 1, 1; augment: #(hline: 1, stroke: 2pt + green)) $
    /// ```
    #[fold]
    pub augment: Option<Augment>,

    /// The gap between rows and columns.
    ///
    /// This is a shorthand to set `row-gap` and `column-gap` to the same value.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set math.mat(gap: 1em)
    /// $ mat(1, 2; 3, 4) $
    /// ```
    #[external]
    pub gap: Rel<Length>,

<<<<<<< HEAD
    /// 行間の間隔。
=======
    /// The gap between rows.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set math.mat(row-gap: 1em)
    /// $ mat(1, 2; 3, 4) $
    /// ```
<<<<<<< HEAD
    #[resolve]
=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[parse(
        let gap = args.named("gap")?;
        args.named("row-gap")?.or(gap)
    )]
    #[default(DEFAULT_ROW_GAP.into())]
    pub row_gap: Rel<Length>,

<<<<<<< HEAD
    /// 列間の間隔。
=======
    /// The gap between columns.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set math.mat(column-gap: 1em)
    /// $ mat(1, 2; 3, 4) $
    /// ```
<<<<<<< HEAD
    #[resolve]
=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[parse(args.named("column-gap")?.or(gap))]
    #[default(DEFAULT_COL_GAP.into())]
    pub column_gap: Rel<Length>,

<<<<<<< HEAD
    /// 行列の各行を要素とする配列の配列。
=======
    /// An array of arrays with the rows of the matrix.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #let data = ((1, 2, 3), (4, 5, 6))
    /// #let matrix = math.mat(..data)
    /// $ v := matrix $
    /// ```
    #[variadic]
    #[parse(
        let mut rows = vec![];
        let mut width = 0;

        let values = args.all::<Spanned<Value>>()?;
        if values.iter().any(|spanned| matches!(spanned.v, Value::Array(_))) {
            for Spanned { v, span } in values {
                let array = v.cast::<Array>().at(span)?;
                let row: Vec<_> = array.into_iter().map(Value::display).collect();
                width = width.max(row.len());
                rows.push(row);
            }
        } else {
            rows = vec![values.into_iter().map(|spanned| spanned.v.display()).collect()];
        }

        for row in &mut rows {
            if row.len() < width {
                row.resize(width, Content::empty());
            }
        }

        rows
    )]
    pub rows: Vec<Vec<Content>>,
}

<<<<<<< HEAD
/// 場合分け。
///
/// `&`記号を用いると異なる分岐に属するコンテンツを整列できます。
///
/// # 例
=======
/// A case distinction.
///
/// Content across different branches can be aligned with the `&` symbol.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// $ f(x, y) := cases(
///   1 "if" (x dot y)/2 <= 0,
///   2 "if" x "is even",
///   3 "if" x in NN,
///   4 "else",
/// ) $
/// ```
#[elem(Mathy)]
pub struct CasesElem {
<<<<<<< HEAD
    /// 使用する区切り文字。
    ///
    /// 単一の文字で左区切り文字を指定する場合、右区切り文字は自動的に推論されます。
    /// それ以外の場合は、左区切り文字と右区切り文字を含む配列を指定します。
=======
    /// The delimiter to use.
    ///
    /// Can be a single character specifying the left delimiter, in which case
    /// the right delimiter is inferred. Otherwise, can be an array containing a
    /// left and a right delimiter.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set math.cases(delim: "[")
    /// $ x = cases(1, 2) $
    /// ```
    #[default(DelimiterPair::BRACE)]
    pub delim: DelimiterPair,

<<<<<<< HEAD
    /// 場合分けの向きを反転させるかどうか。
=======
    /// Whether the direction of cases should be reversed.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set math.cases(reverse: true)
    /// $ cases(1, 2) = x $
    /// ```
    #[default(false)]
    pub reverse: bool,

<<<<<<< HEAD
    /// 分岐間の間隔。
=======
    /// The gap between branches.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #set math.cases(gap: 1em)
    /// $ x = cases(1, 2) $
    /// ```
<<<<<<< HEAD
    #[resolve]
    #[default(DEFAULT_ROW_GAP.into())]
    pub gap: Rel<Length>,

    /// 場合分けの各分岐を表す子要素。
=======
    #[default(DEFAULT_ROW_GAP.into())]
    pub gap: Rel<Length>,

    /// The branches of the case distinction.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[variadic]
    pub children: Vec<Content>,
}

/// A delimiter is a single character that is used to delimit a matrix, vector
/// or cases. The character has to be a Unicode codepoint tagged as a math
/// "opening", "closing" or "fence".
///
/// Typically, the delimiter is stretched to fit the height of whatever it
/// delimits.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Delimiter(Option<char>);

cast! {
    Delimiter,
    self => self.0.into_value(),
    _: NoneValue => Self::none(),
<<<<<<< HEAD
    v: Symbol => Self::char(v.get())?,
=======
    v: Symbol => Self::char(v.get().parse::<char>().map_err(|_| "expected a single-codepoint symbol")?)?,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    v: char => Self::char(v)?,
}

impl Delimiter {
    pub fn none() -> Self {
        Self(None)
    }

    pub fn char(c: char) -> StrResult<Self> {
        if !matches!(
            default_math_class(c),
            Some(MathClass::Opening | MathClass::Closing | MathClass::Fence),
        ) {
            bail!("invalid delimiter: \"{}\"", c)
        }
        Ok(Self(Some(c)))
    }

    pub fn get(self) -> Option<char> {
        self.0
    }

    pub fn find_matching(self) -> Self {
        match self.0 {
            None => Self::none(),
            Some('[') => Self(Some(']')),
            Some(']') => Self(Some('[')),
            Some('{') => Self(Some('}')),
            Some('}') => Self(Some('{')),
            Some(c) => match default_math_class(c) {
                Some(MathClass::Opening) => Self(char::from_u32(c as u32 + 1)),
                Some(MathClass::Closing) => Self(char::from_u32(c as u32 - 1)),
                _ => Self(Some(c)),
            },
        }
    }
}

/// A pair of delimiters (one closing, one opening) used for matrices, vectors
/// and cases.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct DelimiterPair {
    open: Delimiter,
    close: Delimiter,
}

cast! {
    DelimiterPair,

    self => array![self.open, self.close].into_value(),

    v: Array => match v.as_slice() {
        [open, close] => Self {
            open: open.clone().cast()?,
            close: close.clone().cast()?,
        },
        _ => bail!("expected 2 delimiters, found {}", v.len())
    },
    v: Delimiter => Self { open: v, close: v.find_matching() }
}

impl DelimiterPair {
    const PAREN: Self = Self {
        open: Delimiter(Some('(')),
        close: Delimiter(Some(')')),
    };
    const BRACE: Self = Self {
        open: Delimiter(Some('{')),
        close: Delimiter(Some('}')),
    };

    /// The delimiter's opening character.
    pub fn open(self) -> Option<char> {
        self.open.get()
    }

    /// The delimiter's closing character.
    pub fn close(self) -> Option<char> {
        self.close.get()
    }
}

/// Parameters specifying how augmentation lines
/// should be drawn on a matrix.
#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct Augment<T: Numeric = Length> {
    pub hline: AugmentOffsets,
    pub vline: AugmentOffsets,
    pub stroke: Smart<Stroke<T>>,
}

impl<T: Numeric + Fold> Fold for Augment<T> {
    fn fold(self, outer: Self) -> Self {
        Self {
            stroke: match (self.stroke, outer.stroke) {
                (Smart::Custom(inner), Smart::Custom(outer)) => {
                    Smart::Custom(inner.fold(outer))
                }
                // Usually, folding an inner `auto` with an `outer` prefers
                // the explicit `auto`. However, here `auto` means unspecified
                // and thus we want `outer`.
                (inner, outer) => inner.or(outer),
            },
            ..self
        }
    }
}

impl Resolve for Augment {
    type Output = Augment<Abs>;

    fn resolve(self, styles: StyleChain) -> Self::Output {
        Augment {
            hline: self.hline,
            vline: self.vline,
            stroke: self.stroke.resolve(styles),
        }
    }
}

cast! {
    Augment,
    self => {
        // if the stroke is auto and there is only one vertical line,
        if self.stroke.is_auto() && self.hline.0.is_empty() && self.vline.0.len() == 1 {
            return self.vline.0[0].into_value();
        }

        dict! {
            "hline" => self.hline,
            "vline" => self.vline,
            "stroke" => self.stroke,
        }.into_value()
    },
    v: isize => Augment {
        hline: AugmentOffsets::default(),
        vline: AugmentOffsets(smallvec![v]),
        stroke: Smart::Auto,
    },
    mut dict: Dict => {
        let mut take = |key| dict.take(key).ok().map(AugmentOffsets::from_value).transpose();
        let hline = take("hline")?.unwrap_or_default();
        let vline = take("vline")?.unwrap_or_default();
        let stroke = dict.take("stroke")
            .ok()
            .map(Stroke::from_value)
            .transpose()?
            .map(Smart::Custom)
            .unwrap_or(Smart::Auto);
        Augment { hline, vline, stroke }
    },
}

cast! {
    Augment<Abs>,
    self => self.into_value(),
}

/// The offsets at which augmentation lines should be drawn on a matrix.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct AugmentOffsets(pub SmallVec<[isize; 1]>);

cast! {
    AugmentOffsets,
    self => self.0.into_value(),
    v: isize => Self(smallvec![v]),
    v: Array => Self(v.into_iter().map(Value::cast).collect::<HintedStrResult<_>>()?),
}
