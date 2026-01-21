use std::fmt::{self, Debug, Formatter};
use std::ops::Add;

<<<<<<< HEAD
use ecow::{eco_format, eco_vec, EcoString, EcoVec};
use typst_syntax::{Span, Spanned};

use crate::diag::{bail, error, At, SourceDiagnostic, SourceResult, StrResult};
use crate::foundations::{
    cast, func, repr, scope, ty, Array, Dict, FromValue, IntoValue, Repr, Str, Value,
};

/// 関数に渡された引数。
///
/// # 引数シンク
/// 組み込み関数と同様に、カスタム関数も可変長引数を受け取れます。
/// 余分にある引数を全てまとめて受け取る _引数シンク_（キッチンシンクのようにさまざまなものが流れ込む先）は、`..sink`の形で指定できます。このとき生成される`sink`の値は`arguments`型になります。この型は、位置引数と名前付き引数の両方にアクセスするためのメソッドを提供しています。
=======
use ecow::{EcoString, EcoVec, eco_format, eco_vec};
use typst_syntax::{Span, Spanned};

use crate::diag::{At, SourceDiagnostic, SourceResult, StrResult, bail, error};
use crate::foundations::{
    Array, Dict, FromValue, IntoValue, Repr, Str, Value, cast, func, repr, scope, ty,
};

/// Captured arguments to a function.
///
/// # Argument Sinks
/// Like built-in functions, custom functions can also take a variable number of
/// arguments. You can specify an _argument sink_ which collects all excess
/// arguments as `..sink`. The resulting `sink` value is of the `arguments`
/// type. It exposes methods to access the positional and named arguments.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// #let format(title, ..authors) = {
///   let by = authors
///     .pos()
///     .join(", ", last: " and ")
///
///   [*#title* \ _Written by #by;_]
/// }
///
/// #format("ArtosFlow", "Jane", "Joe")
/// ```
///
<<<<<<< HEAD
/// # 引数の展開
/// 引数シンクとは逆に、`..spread`演算子を使うと、関数呼び出しにおいて引数や配列、辞書を展開して渡すことができます。
=======
/// # Spreading
/// Inversely to an argument sink, you can _spread_ arguments, arrays and
/// dictionaries into a function call with the `..spread` operator:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// #let array = (2, 3, 5)
/// #calc.min(..array)
/// #let dict = (fill: blue)
/// #text(..dict)[Hello]
/// ```
#[ty(scope, cast, name = "arguments")]
#[derive(Clone, Hash)]
#[allow(clippy::derived_hash_with_manual_eq)]
pub struct Args {
<<<<<<< HEAD
    /// 関数呼び出し箇所のスパン。これは引数リスト自体のスパンではなく、関数呼び出し全体のものです。
=======
    /// The callsite span for the function. This is not the span of the argument
    /// list itself, but of the whole function call.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    pub span: Span,
    /// The positional and named arguments.
    pub items: EcoVec<Arg>,
}

impl Args {
<<<<<<< HEAD
    /// スパンと値から位置引数を作成します。
=======
    /// Create positional arguments from a span and values.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    pub fn new<T: IntoValue>(span: Span, values: impl IntoIterator<Item = T>) -> Self {
        let items = values
            .into_iter()
            .map(|value| Arg {
                span,
                name: None,
                value: Spanned::new(value.into_value(), span),
            })
            .collect();
        Self { span, items }
    }

<<<<<<< HEAD
    /// 引数にスパンがアタッチされていない場合はアタッチします。
=======
    /// Attach a span to these arguments if they don't already have one.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    pub fn spanned(mut self, span: Span) -> Self {
        if self.span.is_detached() {
            self.span = span;
        }
        self
    }

<<<<<<< HEAD
    /// 残りの位置引数の個数を返します。
=======
    /// Returns the number of remaining positional arguments.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    pub fn remaining(&self) -> usize {
        self.items.iter().filter(|slot| slot.name.is_none()).count()
    }

<<<<<<< HEAD
    /// 指定したインデックスに位置引数を挿入します。
=======
    /// Insert a positional argument at a specific index.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    pub fn insert(&mut self, index: usize, span: Span, value: Value) {
        self.items.insert(
            index,
            Arg {
                span: self.span,
                name: None,
                value: Spanned::new(value, span),
            },
        )
    }

<<<<<<< HEAD
    /// 位置引数をプッシュします。
=======
    /// Push a positional argument.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    pub fn push(&mut self, span: Span, value: Value) {
        self.items.push(Arg {
            span: self.span,
            name: None,
            value: Spanned::new(value, span),
        })
    }

<<<<<<< HEAD
    /// 最初の位置引数がある場合、それを取り出してキャストします。
=======
    /// Consume and cast the first positional argument if there is one.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    pub fn eat<T>(&mut self) -> SourceResult<Option<T>>
    where
        T: FromValue<Spanned<Value>>,
    {
        for (i, slot) in self.items.iter().enumerate() {
            if slot.name.is_none() {
                let value = self.items.remove(i).value;
                let span = value.span;
                return T::from_value(value).at(span).map(Some);
            }
        }
        Ok(None)
    }

<<<<<<< HEAD
    /// 可能ならn個の位置引数を取り出します。
=======
    /// Consume n positional arguments if possible.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    pub fn consume(&mut self, n: usize) -> SourceResult<Vec<Arg>> {
        let mut list = vec![];

        let mut i = 0;
        while i < self.items.len() && list.len() < n {
            if self.items[i].name.is_none() {
                list.push(self.items.remove(i));
            } else {
                i += 1;
            }
        }

        if list.len() < n {
            bail!(self.span, "not enough arguments");
        }

        Ok(list)
    }

<<<<<<< HEAD
    /// 最初の位置引数を取り出してキャストします。
    ///
    /// 位置引数が残っていなければ、`missing argument: {what}`エラーを返します。
=======
    /// Consume and cast the first positional argument.
    ///
    /// Returns a `missing argument: {what}` error if no positional argument is
    /// left.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    pub fn expect<T>(&mut self, what: &str) -> SourceResult<T>
    where
        T: FromValue<Spanned<Value>>,
    {
        match self.eat()? {
            Some(v) => Ok(v),
            None => bail!(self.missing_argument(what)),
        }
    }

<<<<<<< HEAD
    /// 引数が足りない場合のエラーメッセージ。
=======
    /// The error message for missing arguments.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    fn missing_argument(&self, what: &str) -> SourceDiagnostic {
        for item in &self.items {
            let Some(name) = item.name.as_deref() else { continue };
            if name == what {
                return error!(
                    item.span,
                    "the argument `{what}` is positional";
                    hint: "try removing `{}:`", name,
                );
            }
        }

        error!(self.span, "missing argument: {what}")
    }

    /// Find and consume the first castable positional argument.
    pub fn find<T>(&mut self) -> SourceResult<Option<T>>
    where
        T: FromValue<Spanned<Value>>,
    {
        for (i, slot) in self.items.iter().enumerate() {
            if slot.name.is_none() && T::castable(&slot.value.v) {
                let value = self.items.remove(i).value;
                let span = value.span;
                return T::from_value(value).at(span).map(Some);
            }
        }
        Ok(None)
    }

    /// Find and consume all castable positional arguments.
    pub fn all<T>(&mut self) -> SourceResult<Vec<T>>
    where
        T: FromValue<Spanned<Value>>,
    {
        let mut list = vec![];
        let mut errors = eco_vec![];
        self.items.retain(|item| {
            if item.name.is_some() {
                return true;
            }
            let span = item.value.span;
            let spanned = Spanned::new(std::mem::take(&mut item.value.v), span);
            match T::from_value(spanned).at(span) {
                Ok(val) => list.push(val),
                Err(diags) => errors.extend(diags),
            }
            false
        });
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(list)
    }

    /// Cast and remove the value for the given named argument, returning an
    /// error if the conversion fails.
    pub fn named<T>(&mut self, name: &str) -> SourceResult<Option<T>>
    where
        T: FromValue<Spanned<Value>>,
    {
        // We don't quit once we have a match because when multiple matches
        // exist, we want to remove all of them and use the last one.
        let mut i = 0;
        let mut found = None;
        while i < self.items.len() {
            if self.items[i].name.as_deref() == Some(name) {
                let value = self.items.remove(i).value;
                let span = value.span;
                found = Some(T::from_value(value).at(span)?);
            } else {
                i += 1;
            }
        }
        Ok(found)
    }

    /// Same as named, but with fallback to find.
    pub fn named_or_find<T>(&mut self, name: &str) -> SourceResult<Option<T>>
    where
        T: FromValue<Spanned<Value>>,
    {
        match self.named(name)? {
            Some(value) => Ok(Some(value)),
            None => self.find(),
        }
    }

    /// Take out all arguments into a new instance.
    pub fn take(&mut self) -> Self {
        Self {
            span: self.span,
            items: std::mem::take(&mut self.items),
        }
    }

    /// Return an "unexpected argument" error if there is any remaining
    /// argument.
    pub fn finish(self) -> SourceResult<()> {
        if let Some(arg) = self.items.first() {
            match &arg.name {
                Some(name) => bail!(arg.span, "unexpected argument: {name}"),
                _ => bail!(arg.span, "unexpected argument"),
            }
        }
        Ok(())
    }
}

/// A key that can be used to get an argument: either the index of a positional
/// argument, or the name of a named argument.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ArgumentKey {
    Index(i64),
    Name(Str),
}

cast! {
    ArgumentKey,
    v: i64 => Self::Index(v),
    v: Str => Self::Name(v),
}

impl Args {
    fn get(&self, key: &ArgumentKey) -> Option<&Value> {
        let item = match key {
            &ArgumentKey::Index(index) => {
                let mut iter = self.items.iter().filter(|item| item.name.is_none());
                if index < 0 {
                    let index = (-(index + 1)).try_into().ok()?;
                    iter.nth_back(index)
                } else {
                    let index = index.try_into().ok()?;
                    iter.nth(index)
                }
            }
            // Accept the last argument with the right name.
            ArgumentKey::Name(name) => {
                self.items.iter().rfind(|item| item.name.as_ref() == Some(name))
            }
        };
        item.map(|item| &item.value.v)
    }
}

#[scope]
impl Args {
<<<<<<< HEAD
    /// 展開可能な引数をその場で生成します。
    ///
    /// この関数は、`{let args(..sink) = sink}`のように動作します。
=======
    /// Construct spreadable arguments in place.
    ///
    /// This function behaves like `{let args(..sink) = sink}`.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #let args = arguments(stroke: red, inset: 1em, [Body])
    /// #box(..args)
    /// ```
    #[func(constructor)]
    pub fn construct(
        args: &mut Args,
<<<<<<< HEAD
        /// 作成する引数。
=======
        /// The arguments to construct.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        #[external]
        #[variadic]
        arguments: Vec<Value>,
    ) -> Args {
        args.take()
    }

<<<<<<< HEAD
    /// 指定したインデックスの位置引数、または指定した名前の名前付き引数を返します。
    ///
    /// キーが[整数型]($int)の場合、それはまず[`pos`]($arguments.pos)メソッドを呼んでから、次に[`array.at`]を呼ぶのと同等です。キーが[文字列型]($str)である場合、まず[`named`]($arguments.named)メソッドを呼び、次に[`dictionary.at`]を呼ぶのと同等です。
    #[func]
    pub fn at(
        &self,
        /// 取得する引数のインデックスまたは名前。
        key: ArgumentKey,
        /// キーが無効な場合に返すデフォルト値。
=======
    /// Returns the positional argument at the specified index, or the named
    /// argument with the specified name.
    ///
    /// If the key is an [integer]($int), this is equivalent to first calling
    /// [`pos`]($arguments.pos) and then [`array.at`]. If it is a [string]($str),
    /// this is equivalent to first calling [`named`]($arguments.named) and then
    /// [`dictionary.at`].
    #[func]
    pub fn at(
        &self,
        /// The index or name of the argument to get.
        key: ArgumentKey,
        /// A default value to return if the key is invalid.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        #[named]
        default: Option<Value>,
    ) -> StrResult<Value> {
        self.get(&key)
            .cloned()
            .or(default)
            .ok_or_else(|| missing_key_no_default(key))
    }

<<<<<<< HEAD
    /// 渡された位置引数を配列の形で返します。
=======
    /// Returns the captured positional arguments as an array.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[func(name = "pos", title = "Positional")]
    pub fn to_pos(&self) -> Array {
        self.items
            .iter()
            .filter(|item| item.name.is_none())
            .map(|item| item.value.v.clone())
            .collect()
    }

<<<<<<< HEAD
    /// 渡された名前付き引数を辞書の形で返します。
=======
    /// Returns the captured named arguments as a dictionary.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[func(name = "named")]
    pub fn to_named(&self) -> Dict {
        self.items
            .iter()
            .filter_map(|item| item.name.clone().map(|name| (name, item.value.v.clone())))
            .collect()
    }
}

impl Debug for Args {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_list().entries(&self.items).finish()
    }
}

impl Repr for Args {
    fn repr(&self) -> EcoString {
        let pieces = self.items.iter().map(Arg::repr).collect::<Vec<_>>();
        eco_format!("arguments{}", repr::pretty_array_like(&pieces, false))
    }
}

impl PartialEq for Args {
    fn eq(&self, other: &Self) -> bool {
        self.to_pos() == other.to_pos() && self.to_named() == other.to_named()
    }
}

impl Add for Args {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.items.retain(|item| {
            !item.name.as_ref().is_some_and(|name| {
                rhs.items.iter().any(|a| a.name.as_ref() == Some(name))
            })
        });
        self.items.extend(rhs.items);
        self.span = Span::detached();
        self
    }
}

/// An argument to a function call: `12` or `draw: false`.
#[derive(Clone, Hash)]
#[allow(clippy::derived_hash_with_manual_eq)]
pub struct Arg {
    /// The span of the whole argument.
    pub span: Span,
    /// The name of the argument (`None` for positional arguments).
    pub name: Option<Str>,
    /// The value of the argument.
    pub value: Spanned<Value>,
}

impl Debug for Arg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(name) = &self.name {
            name.fmt(f)?;
            f.write_str(": ")?;
            self.value.v.fmt(f)
        } else {
            self.value.v.fmt(f)
        }
    }
}

impl Repr for Arg {
    fn repr(&self) -> EcoString {
        if let Some(name) = &self.name {
            eco_format!("{}: {}", name, self.value.v.repr())
        } else {
            self.value.v.repr()
        }
    }
}

impl PartialEq for Arg {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value.v == other.value.v
    }
}

/// Things that can be used as arguments.
pub trait IntoArgs {
    /// Convert into arguments, attaching the `fallback` span in case `Self`
    /// doesn't have a span.
    fn into_args(self, fallback: Span) -> Args;
}

impl IntoArgs for Args {
    fn into_args(self, fallback: Span) -> Args {
        self.spanned(fallback)
    }
}

impl<I, T> IntoArgs for I
where
    I: IntoIterator<Item = T>,
    T: IntoValue,
{
    fn into_args(self, fallback: Span) -> Args {
        Args::new(fallback, self)
    }
}

/// The missing key access error message when no default was given.
#[cold]
fn missing_key_no_default(key: ArgumentKey) -> EcoString {
    eco_format!(
        "arguments do not contain key {} \
         and no default value was specified",
        match key {
            ArgumentKey::Index(i) => i.repr(),
            ArgumentKey::Name(name) => name.repr(),
        }
    )
}
