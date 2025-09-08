use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::num::{NonZeroI64, NonZeroUsize};
use std::ops::{Add, AddAssign};

use comemo::Tracked;
use ecow::{eco_format, EcoString, EcoVec};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use typst_syntax::{Span, Spanned};

use crate::diag::{
    bail, At, DeprecationSink, HintedStrResult, SourceDiagnostic, SourceResult, StrResult,
};
use crate::engine::Engine;
use crate::foundations::{
    cast, func, ops, repr, scope, ty, Args, Bytes, CastInfo, Context, Dict, FromValue,
    Func, IntoValue, Reflect, Repr, Str, Value, Version,
};

/// Create a new [`Array`] from values.
#[macro_export]
#[doc(hidden)]
macro_rules! __array {
    ($value:expr; $count:expr) => {
        $crate::foundations::Array::from($crate::foundations::eco_vec![
            $crate::foundations::IntoValue::into_value($value);
            $count
        ])
    };

    ($($value:expr),* $(,)?) => {
        $crate::foundations::Array::from($crate::foundations::eco_vec![$(
            $crate::foundations::IntoValue::into_value($value)
        ),*])
    };
}

#[doc(inline)]
pub use crate::__array as array;

/// 複数の値からなるシーケンスです。
///
/// コンマで区切った値の並びを括弧で囲むことで配列を作成できます。値の型が同じである必要はありません。
///
/// 配列要素には`.at()`メソッドでアクセスしたり、更新したりできます。インデックスは0から始まり、
/// 負の値のインデックスは配列末尾から数えられます。配列の反復処理には[forループ]($scripting/#loops)
/// が使用できます。配列は`+`演算子で結合したり、[連結]($scripting/#blocks)したり、
/// 整数値と掛け合わせたりできます。
///
/// **注：** 要素が1つしかない配列には,`{(1,)}`のように末尾にコンマが必要です。これは、
/// `{(1 + 2) * 3}`のような括弧で囲まれた式と区別するためです。空の配列は`{()}`と表記します。
///
/// # 例
/// ```example
/// #let values = (1, 7, 4, -3, 2)
///
/// #values.at(0) \
/// #(values.at(0) = 3)
/// #values.at(-1) \
/// #values.find(calc.even) \
/// #values.filter(calc.odd) \
/// #values.map(calc.abs) \
/// #values.rev() \
/// #(1, (2, 3)).flatten() \
/// #(("A", "B", "C")
///     .join(", ", last: " and "))
/// ```
#[ty(scope, cast)]
#[derive(Default, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Array(EcoVec<Value>);

impl Array {
    /// Create a new, empty array.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new vec, with a known capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(EcoVec::with_capacity(capacity))
    }

    /// Return `true` if the length is 0.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Extract a slice of the whole array.
    pub fn as_slice(&self) -> &[Value] {
        self.0.as_slice()
    }

    /// Iterate over references to the contained values.
    pub fn iter(&self) -> std::slice::Iter<Value> {
        self.0.iter()
    }

    /// Mutably borrow the first value in the array.
    pub fn first_mut(&mut self) -> StrResult<&mut Value> {
        self.0.make_mut().first_mut().ok_or_else(array_is_empty)
    }

    /// Mutably borrow the last value in the array.
    pub fn last_mut(&mut self) -> StrResult<&mut Value> {
        self.0.make_mut().last_mut().ok_or_else(array_is_empty)
    }

    /// Mutably borrow the value at the given index.
    pub fn at_mut(&mut self, index: i64) -> StrResult<&mut Value> {
        let len = self.len();
        self.locate_opt(index, false)
            .and_then(move |i| self.0.make_mut().get_mut(i))
            .ok_or_else(|| out_of_bounds(index, len))
    }

    /// Resolve an index or throw an out of bounds error.
    fn locate(&self, index: i64, end_ok: bool) -> StrResult<usize> {
        self.locate_opt(index, end_ok)
            .ok_or_else(|| out_of_bounds(index, self.len()))
    }

    /// Resolve an index, if it is within bounds.
    ///
    /// `index == len` is considered in bounds if and only if `end_ok` is true.
    fn locate_opt(&self, index: i64, end_ok: bool) -> Option<usize> {
        let wrapped =
            if index >= 0 { Some(index) } else { (self.len() as i64).checked_add(index) };

        wrapped
            .and_then(|v| usize::try_from(v).ok())
            .filter(|&v| v < self.0.len() + end_ok as usize)
    }

    /// Repeat this array `n` times.
    pub fn repeat(&self, n: usize) -> StrResult<Self> {
        let count = self
            .len()
            .checked_mul(n)
            .ok_or_else(|| format!("cannot repeat this array {n} times"))?;

        Ok(self.iter().cloned().cycle().take(count).collect())
    }

    /// The internal implementation of [`Array::contains`].
    pub fn contains_impl(&self, value: &Value, sink: &mut dyn DeprecationSink) -> bool {
        self.0.iter().any(|v| ops::equal(v, value, sink))
    }
}

#[scope]
impl Array {
    /// 値を配列に変換します。
    ///
    /// この関数は、個々の要素から配列を作成するためのものではなく、コレクションのような値を配列に変換する
    /// ことのみを目的としています。個々の要素から配列を作成する場合は、配列の構文である (1, 2, 3)
    ///  (または、単一要素の配列の場合は (1,)) を使用してください。
    ///
    /// ```example
    /// #let hi = "Hello 😃"
    /// #array(bytes(hi))
    /// ```
    #[func(constructor)]
    pub fn construct(
        /// 配列に変換したい値。
        value: ToArray,
    ) -> Array {
        value.0
    }

    /// 配列要素の個数を返します。
    #[func(title = "Length")]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// 配列の先頭の要素を返します。代入文の左辺でも使用可能です。配列が空の場合はエラーに
    /// なります。
    #[func]
    pub fn first(&self) -> StrResult<Value> {
        self.0.first().cloned().ok_or_else(array_is_empty)
    }

    /// 配列の末尾の要素を返します。代入文の左辺でも使用可能です。配列が空の場合はエラーに
    /// なります。
    #[func]
    pub fn last(&self) -> StrResult<Value> {
        self.0.last().cloned().ok_or_else(array_is_empty)
    }

    /// 指定されたインデックスにある配列要素を返します。代入文の左辺でも使用可能です。
    /// インデックスが範囲外である場合、デフォルト値が指定されていればそれの値が返されますが、
    /// 指定されていなければエラーになります。
    #[func]
    pub fn at(
        &self,
        /// 要素を取得するインデックス。負の値を指定すると、配列末尾から数えます。
        index: i64,
        /// インデックスが範囲外の場合に返されるデフォルト値。
        #[named]
        default: Option<Value>,
    ) -> StrResult<Value> {
        self.locate_opt(index, false)
            .and_then(|i| self.0.get(i).cloned())
            .or(default)
            .ok_or_else(|| out_of_bounds_no_default(index, self.len()))
    }

    /// 配列の末尾に値を追加します。
    #[func]
    pub fn push(
        &mut self,
        /// 配列の末尾に挿入する値。
        value: Value,
    ) {
        self.0.push(value);
    }

    /// 配列末尾の項目を削除して返します。配列が空の場合はエラーになります。
    #[func]
    pub fn pop(&mut self) -> StrResult<Value> {
        self.0.pop().ok_or_else(array_is_empty)
    }

    /// 指定されたインデックスに値を挿入し、それ以降の要素をすべて右にずらします。
    /// インデックスが範囲外の場合はエラーになります。
    ///
    /// 配列の要素を置き換えるには[`at`]($array.at)メソッドを使用してください。
    #[func]
    pub fn insert(
        &mut self,
        /// 要素を挿入するインデックス。負の値を指定すると後ろから数えます。
        index: i64,
        /// 配列に挿入する値。
        value: Value,
    ) -> StrResult<()> {
        let i = self.locate(index, true)?;
        self.0.insert(i, value);
        Ok(())
    }

    /// 指定されたインデックスにある値を配列から削除して返します。
    #[func]
    pub fn remove(
        &mut self,
        /// 要素を削除するインデックス。負の値を指定すると後ろから数えます。
        index: i64,
        /// インデックスが範囲外の場合に返されるデフォルト値。
        #[named]
        default: Option<Value>,
    ) -> StrResult<Value> {
        self.locate_opt(index, false)
            .map(|i| self.0.remove(i))
            .or(default)
            .ok_or_else(|| out_of_bounds_no_default(index, self.len()))
    }

    /// 配列の一部を抽出します。開始インデックスまたは終了インデックスが範囲外の場合は
    /// エラーになります。
    #[func]
    pub fn slice(
        &self,
        /// 開始インデックス（ここから）。負の値を指定すると、後ろから数えます。
        start: i64,
        /// 終了インデックス（この手前まで）。省略された場合、配列の最後までが抽出されます。
        /// 負の値を指定した場合、後ろから数えます。
        #[default]
        end: Option<i64>,
        /// 抽出する要素の個数。`start + count`を`end`位置として渡すのと同等です。
        /// `end`と同時に使用することはできません。
        #[named]
        count: Option<i64>,
    ) -> StrResult<Array> {
        let mut end = end;
        if end.is_none() {
            end = count.map(|c: i64| start + c);
        }
        let start = self.locate(start, true)?;
        let end = self.locate(end.unwrap_or(self.len() as i64), true)?.max(start);
        Ok(self.0[start..end].into())
    }

    /// 配列に指定された値が含まれているかどうかを調べます。
    ///
    /// このメソッドには専用の構文もあり、`{(1, 2, 3).contains(2)}`の代わりに
    /// `{2 in (1, 2, 3)}`と書くこともできます。
    #[func]
    pub fn contains(
        &self,
        engine: &mut Engine,
        span: Span,
        /// 検索する値。
        value: Value,
    ) -> bool {
        self.contains_impl(&value, &mut (engine, span))
    }

    /// 指定した関数が`{true}`を返す項目を検索し、最初に見つかった項目を返します。
    /// 一致するものがなければ`{none}`を返します。
    #[func]
    pub fn find(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        /// 各要素に適用する関数。戻り値は論理型でなくてはなりません。
        searcher: Func,
    ) -> SourceResult<Option<Value>> {
        for item in self.iter() {
            if searcher
                .call(engine, context, [item.clone()])?
                .cast::<bool>()
                .at(searcher.span())?
            {
                return Ok(Some(item.clone()));
            }
        }
        Ok(None)
    }

    /// 指定した関数が`{true}`を返す項目を検索し、最初に見つかった項目の
    /// インデックスを返します。一致するものがなければ`{none}`を返します。
    #[func]
    pub fn position(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        /// 各要素に適用する関数。戻り値は論理型でなくてはなりません。
        searcher: Func,
    ) -> SourceResult<Option<i64>> {
        for (i, item) in self.iter().enumerate() {
            if searcher
                .call(engine, context, [item.clone()])?
                .cast::<bool>()
                .at(searcher.span())?
            {
                return Ok(Some(i as i64));
            }
        }

        Ok(None)
    }

    /// 数列で構成される配列を作成します。
    /// 位置引数を1つだけ渡した場合、それは範囲の`終了`位置と解釈されます。
    /// 2つ渡した場合は、範囲の`開始`と`終了`位置を示します。
    /// この関数は、配列のスコープとグローバル・スコープの両方で利用可能です。
    ///
    /// ```example
    /// #range(5) \
    /// #range(2, 5) \
    /// #range(20, step: 4) \
    /// #range(21, step: 4) \
    /// #range(5, 2, step: -1)
    /// ```
    #[func]
    pub fn range(
        args: &mut Args,
        /// 範囲の開始位置(ここから)。
        #[external]
        #[default]
        start: i64,
        /// 範囲の終了位置（この手前まで）。
        #[external]
        end: i64,
        /// 生成される数値間の距離。
        #[named]
        #[default(NonZeroI64::new(1).unwrap())]
        step: NonZeroI64,
    ) -> SourceResult<Array> {
        let first = args.expect::<i64>("end")?;
        let (start, end) = match args.eat::<i64>()? {
            Some(second) => (first, second),
            None => (0, first),
        };

        let step = step.get();

        let mut x = start;
        let mut array = Self::new();

        while x.cmp(&end) == 0.cmp(&step) {
            array.push(x.into_value());
            x += step;
        }

        Ok(array)
    }

    /// 元の配列のうち、指定された関数が`true`を返す要素のみで構成される
    /// 新たな配列を生成します。
    #[func]
    pub fn filter(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        /// 各要素に適用する関数。戻り値は論理型でなくてはなりません。
        test: Func,
    ) -> SourceResult<Array> {
        let mut kept = EcoVec::new();
        for item in self.iter() {
            if test
                .call(engine, context, [item.clone()])?
                .cast::<bool>()
                .at(test.span())?
            {
                kept.push(item.clone())
            }
        }
        Ok(kept.into())
    }

    /// 元の配列の各要素を指定した関数で変換した値で構成される、新たな配列を生成します。
    #[func]
    pub fn map(
        self,
        engine: &mut Engine,
        context: Tracked<Context>,
        /// 各要素に適用する関数。
        mapper: Func,
    ) -> SourceResult<Array> {
        self.into_iter()
            .map(|item| mapper.call(engine, context, [item]))
            .collect()
    }

    /// インデックスと値をペアにした新しい配列を返します。
    ///
    /// 返される配列は、`(index, value)`ペアを要素とする、長さ2の配列で構成されます。
    /// これらは、`let`バインディングや`for`ループで[分割]($scripting/#bindings)できます。
    #[func]
    pub fn enumerate(
        self,
        /// リストの最初のペアに対応するインデックス。
        #[named]
        #[default(0)]
        start: i64,
    ) -> StrResult<Array> {
        self.into_iter()
            .enumerate()
            .map(|(i, value)| {
                Ok(array![
                    start
                        .checked_add_unsigned(i as u64)
                        .ok_or("array index is too large")?,
                    value
                ]
                .into_value())
            })
            .collect()
    }

    /// 配列を他の配列と一括り（zip）にします。
    ///
    /// このメソッドは、配列の配列を返します。その`i`番目の内部配列には、元の各配列の`i`番目の
    /// 要素がすべて含まれます。
    /// zipされる配列の長さが異なる場合、最も短い配列の最後の要素までが処理され、残りの要素は
    /// すべて無視されます。
    /// この関数は可変長引数に対応しており、複数の配列を一度にzip可能です。例えば、
    /// `{(1, 2).zip(("A", "B"), (10, 20))}`は`{((1, "A", 10), (2, "B", 20))}`を
    /// 生成します。
    #[func]
    pub fn zip(
        self,
        args: &mut Args,
        /// すべての配列が同じ長さである必要があるかどうか。
        /// 例えば、`{(1, 2).zip((1, 2, 3), exact: true)}`はエラーになります。
        #[named]
        #[default(false)]
        exact: bool,
        /// zipする他の配列。
        #[external]
        #[variadic]
        others: Vec<Array>,
    ) -> SourceResult<Array> {
        let remaining = args.remaining();

        // Fast path for one array.
        if remaining == 0 {
            return Ok(self.into_iter().map(|item| array![item].into_value()).collect());
        }

        // Fast path for just two arrays.
        if remaining == 1 {
            let Spanned { v: other, span: other_span } =
                args.expect::<Spanned<Array>>("others")?;
            if exact && self.len() != other.len() {
                bail!(
                    other_span,
                    "second array has different length ({}) from first array ({})",
                    other.len(),
                    self.len()
                );
            }
            return Ok(self
                .into_iter()
                .zip(other)
                .map(|(first, second)| array![first, second].into_value())
                .collect());
        }

        // If there is more than one array, we use the manual method.
        let mut out = Self::with_capacity(self.len());
        let arrays = args.all::<Spanned<Array>>()?;
        if exact {
            let errs = arrays
                .iter()
                .filter(|sp| sp.v.len() != self.len())
                .map(|Spanned { v, span }| {
                    SourceDiagnostic::error(
                        *span,
                        eco_format!(
                            "array has different length ({}) from first array ({})",
                            v.len(),
                            self.len()
                        ),
                    )
                })
                .collect::<EcoVec<_>>();
            if !errs.is_empty() {
                return Err(errs);
            }
        }

        let mut iterators =
            arrays.into_iter().map(|i| i.v.into_iter()).collect::<Vec<_>>();

        for this in self {
            let mut row = Self::with_capacity(1 + iterators.len());
            row.push(this.clone());

            for iterator in &mut iterators {
                let Some(item) = iterator.next() else {
                    return Ok(out);
                };

                row.push(item);
            }

            out.push(row.into_value());
        }

        Ok(out)
    }

    /// 累算関数を使って、配列のすべての要素を1つの値に畳み込みます。
    #[func]
    pub fn fold(
        self,
        engine: &mut Engine,
        context: Tracked<Context>,
        /// 累算値の初期値。
        init: Value,
        /// 畳み込むための関数。この関数は、累算値と要素の2つの引数を取る必要があります。
        folder: Func,
    ) -> SourceResult<Value> {
        let mut acc = init;
        for item in self {
            acc = folder.call(engine, context, [acc, item])?;
        }
        Ok(acc)
    }

    /// すべての配列要素を合計します（加算可能なすべての型で動作します）。
    #[func]
    pub fn sum(
        self,
        engine: &mut Engine,
        span: Span,
        /// 配列が空の場合に返される値。配列が空である可能性がある場合、
        /// この値を設定する必要があります。
        #[named]
        default: Option<Value>,
    ) -> HintedStrResult<Value> {
        let mut iter = self.into_iter();
        let mut acc = iter
            .next()
            .or(default)
            .ok_or("cannot calculate sum of empty array with no default")?;
        for item in iter {
            acc = ops::add(acc, item, &mut (&mut *engine, span))?;
        }
        Ok(acc)
    }

    /// すべての配列要素の積を計算します（乗算可能なすべての型で動作します）。
    #[func]
    pub fn product(
        self,
        /// 配列が空の場合に返される値。配列が空である可能性がある場合、
        /// この値を設定する必要があります。
        #[named]
        default: Option<Value>,
    ) -> HintedStrResult<Value> {
        let mut iter = self.into_iter();
        let mut acc = iter
            .next()
            .or(default)
            .ok_or("cannot calculate product of empty array with no default")?;
        for item in iter {
            acc = ops::mul(acc, item)?;
        }
        Ok(acc)
    }

    /// 指定した関数が配列内のいずれかの要素に対して`{true}`を返すかどうか。
    #[func]
    pub fn any(
        self,
        engine: &mut Engine,
        context: Tracked<Context>,
        /// 各要素に適用する関数。戻り値は論理型でなくてはなりません。
        test: Func,
    ) -> SourceResult<bool> {
        for item in self {
            if test.call(engine, context, [item])?.cast::<bool>().at(test.span())? {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// 指定した関数が配列内のすべての要素に対して`{true}`を返すかどうか。
    #[func]
    pub fn all(
        self,
        engine: &mut Engine,
        context: Tracked<Context>,
        /// 各要素に適用する関数。戻り値は論理型でなくてはなりません。
        test: Func,
    ) -> SourceResult<bool> {
        for item in self {
            if !test.call(engine, context, [item])?.cast::<bool>().at(test.span())? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// ネストされたすべての配列を、1つのフラットな配列に結合します。
    #[func]
    pub fn flatten(self) -> Array {
        let mut flat = EcoVec::with_capacity(self.0.len());
        for item in self {
            if let Value::Array(nested) = item {
                flat.extend(nested.flatten());
            } else {
                flat.push(item);
            }
        }
        flat.into()
    }

    /// 元の配列と同じ要素を逆順に含む新しい配列を返します。
    #[func(title = "Reverse")]
    pub fn rev(self) -> Array {
        self.into_iter().rev().collect()
    }

    /// 指定した値が出現する箇所で配列を分割します。
    #[func]
    pub fn split(
        &self,
        /// 分割する値。
        at: Value,
    ) -> Array {
        self.as_slice()
            .split(|value| *value == at)
            .map(|subslice| Value::Array(subslice.iter().cloned().collect()))
            .collect()
    }

    /// 配列内のすべての要素を1つに結合します。
    #[func]
    pub fn join(
        self,
        engine: &mut Engine,
        span: Span,
        /// 配列の各要素の間に挿入する値。
        #[default]
        separator: Option<Value>,
        /// 最後の2つの要素の間に挿入する、代替の区切り文字。
        #[named]
        last: Option<Value>,
    ) -> StrResult<Value> {
        let len = self.0.len();
        let separator = separator.unwrap_or(Value::None);

        let mut last = last;
        let mut result = Value::None;
        for (i, value) in self.into_iter().enumerate() {
            if i > 0 {
                if i + 1 == len && last.is_some() {
                    result = ops::join(
                        result,
                        last.take().unwrap(),
                        &mut (&mut *engine, span),
                    )?;
                } else {
                    result =
                        ops::join(result, separator.clone(), &mut (&mut *engine, span))?;
                }
            }

            result = ops::join(result, value, &mut (&mut *engine, span))?;
        }

        Ok(result)
    }

    /// 隣接する要素の間に区切り文字のコピーを配置した新しい配列を返します。
    #[func]
    pub fn intersperse(
        self,
        /// 隣接する各要素の間に配置される値。
        separator: Value,
    ) -> Array {
        // TODO: Use once stabilized:
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.intersperse
        let size = match self.len() {
            0 => return Array::new(),
            n => (2 * n) - 1,
        };
        let mut vec = EcoVec::with_capacity(size);
        let mut iter = self.into_iter();

        if let Some(first) = iter.next() {
            vec.push(first);
        }

        for value in iter {
            vec.push(separator.clone());
            vec.push(value);
        }

        Array(vec)
    }

    /// 配列を、重なり合わない複数のチャンク（塊）に分割します。
    /// 配列の先頭から順に分割し、余りの要素は配列の末尾に1つのチャンクにまとめます。
    ///
    /// 最後のチャンク以外はすべて、`chunk-size`で指定された要素数になります。
    /// `exact`を`{true}`に設定した場合、`chunk-size`より少ない余りの要素は破棄されます。
    ///
    /// ```example
    /// #let array = (1, 2, 3, 4, 5, 6, 7, 8)
    /// #array.chunks(3) \
    /// #array.chunks(3, exact: true)
    /// ```
    #[func]
    pub fn chunks(
        self,
        /// 各チャンクが含むことのできる最大要素数。
        chunk_size: NonZeroUsize,
        /// 余りの要素が`chunk-size`より少なかった場合、それをチャンクとして保持するかどうか。
        #[named]
        #[default(false)]
        exact: bool,
    ) -> Array {
        let to_array = |chunk| Array::from(chunk).into_value();
        if exact {
            self.0.chunks_exact(chunk_size.get()).map(to_array).collect()
        } else {
            self.0.chunks(chunk_size.get()).map(to_array).collect()
        }
    }

    /// ウィンドウ（指定幅の枠）を少しずつずらしながら、`window-size`で指定した数の配列要素を
    /// ひとまとまりにした配列を要素として含む配列を作成して返します。
    ///
    /// 配列の長さが`window-size`より短い場合、空の配列が返されます。
    ///
    /// ```example
    /// #let array = (1, 2, 3, 4, 5, 6, 7, 8)
    /// #array.windows(5)
    /// ```
    #[func]
    pub fn windows(
        self,
        /// How many elements each window will contain.
        window_size: NonZeroUsize,
    ) -> Array {
        self.0
            .windows(window_size.get())
            .map(|window| Array::from(window).into_value())
            .collect()
    }

    /// 配列のソート（並び替え）されたバージョンを返します。オプションとして、キー関数による
    /// ソートも可能です。使用されるソートアルゴリズムでは、同順位要素の前後関係は
    /// 変化しません（安定）。
    ///
    /// 2つの値を比較できなかった場合、または（キー関数が与えられている場合で）キー関数がエラー
    /// を返した場合、エラーが返されます。
    ///
    /// 複数の基準で同時にソートする場合、例えば、ある基準間で同順位になった場合などには、キー
    /// 関数が配列を返すことができます。結果は辞書式順序で並べられます。
    ///
    /// ```example
    /// #let array = (
    ///   (a: 2, b: 4),
    ///   (a: 1, b: 5),
    ///   (a: 2, b: 3),
    /// )
    /// #array.sorted(key: it => (it.a, it.b))
    /// ```
    #[func]
    pub fn sorted(
        self,
        engine: &mut Engine,
        context: Tracked<Context>,
        span: Span,
        /// If given, applies this function to the elements in the array to
        /// determine the keys to sort by.
        #[named]
        key: Option<Func>,
    ) -> SourceResult<Array> {
        let mut result = Ok(());
        let mut vec = self.0;
        let mut key_of = |x: Value| match &key {
            // NOTE: We are relying on `comemo`'s memoization of function
            // evaluation to not excessively reevaluate the `key`.
            Some(f) => f.call(engine, context, [x]),
            None => Ok(x),
        };
        vec.make_mut().sort_by(|a, b| {
            // Until we get `try` blocks :)
            match (key_of(a.clone()), key_of(b.clone())) {
                (Ok(a), Ok(b)) => ops::compare(&a, &b).unwrap_or_else(|err| {
                    if result.is_ok() {
                        result = Err(err).at(span);
                    }
                    Ordering::Equal
                }),
                (Err(e), _) | (_, Err(e)) => {
                    if result.is_ok() {
                        result = Err(e);
                    }
                    Ordering::Equal
                }
            }
        });
        result.map(|_| vec.into())
    }

    /// 配列内の要素の重複を解消します。
    ///
    /// 要素の重複をすべて解消した新しい配列を返します。重複があった要素は、そのうち最初の
    /// ものだけが保持されます。
    ///
    /// ```example
    /// #(1, 1, 2, 3, 1).dedup()
    /// ```
    #[func(title = "Deduplicate")]
    pub fn dedup(
        self,
        engine: &mut Engine,
        context: Tracked<Context>,
        span: Span,
        /// 指定がある場合、この関数を配列の要素に適用し、重複を判定するためのキーを決定します。
        #[named]
        key: Option<Func>,
    ) -> SourceResult<Array> {
        let mut out = EcoVec::with_capacity(self.0.len());
        let key_of = |engine: &mut Engine, x: Value| match &key {
            // NOTE: We are relying on `comemo`'s memoization of function
            // evaluation to not excessively reevaluate the `key`.
            Some(f) => f.call(engine, context, [x]),
            None => Ok(x),
        };

        // This algorithm is O(N^2) because we cannot rely on `HashSet` since:
        // 1. We would like to preserve the order of the elements.
        // 2. We cannot hash arbitrary `Value`.
        'outer: for value in self {
            let key = key_of(&mut *engine, value.clone())?;
            if out.is_empty() {
                out.push(value);
                continue;
            }

            for second in out.iter() {
                if ops::equal(
                    &key,
                    &key_of(&mut *engine, second.clone())?,
                    &mut (&mut *engine, span),
                ) {
                    continue 'outer;
                }
            }

            out.push(value);
        }

        Ok(Self(out))
    }

    /// ペアの配列を辞書に変換します。各ペアの最初の値がキー、2番目の値が値になります。
    ///
    /// 同じキーが複数回出現した場合、最後の値が優先されます。
    ///
    /// ```example
    /// #(
    ///   ("apples", 2),
    ///   ("peaches", 3),
    ///   ("apples", 5),
    /// ).to-dict()
    /// ```
    #[func]
    pub fn to_dict(self) -> StrResult<Dict> {
        self.into_iter()
            .map(|value| {
                let value_ty = value.ty();
                let pair = value.cast::<Array>().map_err(|_| {
                    eco_format!("expected (str, any) pairs, found {}", value_ty)
                })?;
                if let [key, value] = pair.as_slice() {
                    let key = key.clone().cast::<Str>().map_err(|_| {
                        eco_format!("expected key of type str, found {}", value.ty())
                    })?;
                    Ok((key, value.clone()))
                } else {
                    bail!("expected pairs of length 2, found length {}", pair.len());
                }
            })
            .collect()
    }

    /// すべての要素に繰り返し集約操作を適用することで、要素を1つに集約します。
    ///
    /// 配列が空の場合は`{none}`を返し、そうでない場合は集約結果を返します。
    /// 集約関数は、2つの引数（"累算値"と要素）を取る関数です。
    ///
    /// 1つ以上の要素を持つ配列の場合、これは`[array.fold]`と同じです。このとき、配列の最初の
    /// 要素が累算値の開始値として用いられ、それに続くすべての要素が畳み込まれます。
    #[func]
    pub fn reduce(
        self,
        engine: &mut Engine,
        context: Tracked<Context>,
        /// 集約関数。この関数は、累算値と要素の2つの引数を取る必要があります。
        reducer: Func,
    ) -> SourceResult<Value> {
        let mut iter = self.into_iter();
        let mut acc = iter.next().unwrap_or_default();
        for item in iter {
            acc = reducer.call(engine, context, [acc, item])?;
        }
        Ok(acc)
    }
}

/// A value that can be cast to bytes.
pub struct ToArray(Array);

cast! {
    ToArray,
    v: Array => Self(v),
    v: Bytes => Self(v.iter().map(|&b| Value::Int(b.into())).collect()),
    v: Version => Self(v.values().iter().map(|&v| Value::Int(v as i64)).collect())
}

impl Debug for Array {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_list().entries(&self.0).finish()
    }
}

impl Repr for Array {
    fn repr(&self) -> EcoString {
        let max = 40;
        let mut pieces: Vec<_> = self
            .iter()
            .take(max)
            .map(|value| eco_format!("{}", value.repr()))
            .collect();
        if self.len() > max {
            pieces.push(eco_format!(".. ({} items omitted)", self.len() - max));
        }
        repr::pretty_array_like(&pieces, self.len() == 1).into()
    }
}

impl Add for Array {
    type Output = Self;

    fn add(mut self, rhs: Array) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for Array {
    fn add_assign(&mut self, rhs: Self) {
        self.0.extend(rhs.0);
    }
}

impl Extend<Value> for Array {
    fn extend<T: IntoIterator<Item = Value>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl FromIterator<Value> for Array {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for Array {
    type Item = Value;
    type IntoIter = ecow::vec::IntoIter<Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Array {
    type Item = &'a Value;
    type IntoIter = std::slice::Iter<'a, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl From<EcoVec<Value>> for Array {
    fn from(v: EcoVec<Value>) -> Self {
        Array(v)
    }
}

impl From<&[Value]> for Array {
    fn from(v: &[Value]) -> Self {
        Array(v.into())
    }
}

impl<T> Reflect for Vec<T> {
    fn input() -> CastInfo {
        Array::input()
    }

    fn output() -> CastInfo {
        Array::output()
    }

    fn castable(value: &Value) -> bool {
        Array::castable(value)
    }
}

impl<T: Reflect, const N: usize> Reflect for SmallVec<[T; N]> {
    fn input() -> CastInfo {
        Array::input()
    }

    fn output() -> CastInfo {
        Array::output()
    }

    fn castable(value: &Value) -> bool {
        Array::castable(value)
    }
}

impl<T: IntoValue> IntoValue for Vec<T> {
    fn into_value(self) -> Value {
        Value::Array(self.into_iter().map(IntoValue::into_value).collect())
    }
}

impl<T: IntoValue, const N: usize> IntoValue for SmallVec<[T; N]> {
    fn into_value(self) -> Value {
        Value::Array(self.into_iter().map(IntoValue::into_value).collect())
    }
}

impl<T: FromValue> FromValue for Vec<T> {
    fn from_value(value: Value) -> HintedStrResult<Self> {
        value.cast::<Array>()?.into_iter().map(Value::cast).collect()
    }
}

impl<T: FromValue, const N: usize> FromValue for SmallVec<[T; N]> {
    fn from_value(value: Value) -> HintedStrResult<Self> {
        value.cast::<Array>()?.into_iter().map(Value::cast).collect()
    }
}

/// One element, or multiple provided as an array.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct OneOrMultiple<T>(pub Vec<T>);

impl<T: Reflect> Reflect for OneOrMultiple<T> {
    fn input() -> CastInfo {
        T::input() + Array::input()
    }

    fn output() -> CastInfo {
        T::output() + Array::output()
    }

    fn castable(value: &Value) -> bool {
        Array::castable(value) || T::castable(value)
    }
}

impl<T: IntoValue + Clone> IntoValue for OneOrMultiple<T> {
    fn into_value(self) -> Value {
        self.0.into_value()
    }
}

impl<T: FromValue> FromValue for OneOrMultiple<T> {
    fn from_value(value: Value) -> HintedStrResult<Self> {
        if T::castable(&value) {
            return Ok(Self(vec![T::from_value(value)?]));
        }
        if Array::castable(&value) {
            return Ok(Self(
                Array::from_value(value)?
                    .into_iter()
                    .map(|value| T::from_value(value))
                    .collect::<HintedStrResult<_>>()?,
            ));
        }
        Err(Self::error(&value))
    }
}

impl<T> Default for OneOrMultiple<T> {
    fn default() -> Self {
        Self(vec![])
    }
}

/// The error message when the array is empty.
#[cold]
fn array_is_empty() -> EcoString {
    "array is empty".into()
}

/// The out of bounds access error message.
#[cold]
fn out_of_bounds(index: i64, len: usize) -> EcoString {
    eco_format!("array index out of bounds (index: {index}, len: {len})")
}

/// The out of bounds access error message when no default value was given.
#[cold]
fn out_of_bounds_no_default(index: i64, len: usize) -> EcoString {
    eco_format!(
        "array index out of bounds (index: {index}, len: {len}) \
         and no default value was specified",
    )
}
