#[doc(inline)]
pub use typst_macros::func;

use std::fmt::{self, Debug, Formatter};
use std::sync::{Arc, LazyLock};

use comemo::{Tracked, TrackedMut};
<<<<<<< HEAD
use ecow::{eco_format, EcoString};
use typst_syntax::{ast, Span, SyntaxNode};
use typst_utils::{singleton, LazyHash, Static};

use crate::diag::{bail, At, DeprecationSink, SourceResult, StrResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, repr, scope, ty, Args, Bytes, CastInfo, Content, Context, Element, IntoArgs,
    PluginFunc, Scope, Selector, Type, Value,
};

/// 引数値から戻り値への写像。
///
/// 関数名の直後に括弧で囲まれたカンマ区切りの関数の _引数_ のリストを書くことにより関数を呼び出すことができます。
/// 加えて、通常の引数リストの後に任意の数のコンテンツブロック引数を関数に渡すこともできます。
/// 通常の引数リストが空の場合は省略が可能です。
/// Typstは位置引数と名前付き引数をサポートしています。
/// 前者は位置と型によって識別され、後者は`name: value`のように書きます。
///
/// 数式モードでは、関数呼び出しは特殊な振る舞いをします。
/// 詳細は[数式のドキュメント]($category/math)を参照して下さい。
///
/// # 例
=======
use ecow::{EcoString, eco_format};
use typst_syntax::{Span, SyntaxNode, ast};
use typst_utils::{LazyHash, Static, singleton};

use crate::diag::{At, DeprecationSink, SourceResult, StrResult, bail};
use crate::engine::Engine;
use crate::foundations::{
    Args, Bytes, CastInfo, Content, Context, Element, IntoArgs, PluginFunc, Scope,
    Selector, Type, Value, cast, repr, scope, ty,
};

/// A mapping from argument values to a return value.
///
/// You can call a function by writing a comma-separated list of function
/// _arguments_ enclosed in parentheses directly after the function name.
/// Additionally, you can pass any number of trailing content block arguments
/// to a function _after_ the normal argument list. If the normal argument list
/// would become empty, it can be omitted. Typst supports positional and named
/// arguments. The former are identified by position and type, while the latter
/// are written as `name: value`.
///
/// Within math mode, function calls have special behaviour. See the
/// [math documentation]($category/math) for more details.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// // Call a function.
/// #list([A], [B])
///
/// // Named arguments and trailing
/// // content blocks.
/// #enum(start: 2)[A][B]
///
/// // Version without parentheses.
/// #list[A][B]
/// ```
///
<<<<<<< HEAD
/// 関数はTypstにおいて基礎となる構成要素です。
/// Typstはさまざまな組版タスクに応じた関数を提供しています。
/// さらには、作成されるマークアップの裏側では関数が用いられており、全てのスタイル設定は関数を介して行われます。
/// このリファレンスでは利用可能な全ての関数とその使い方を示します。
/// Typstで関数をさらに活用する方法については、[setルール]($styling/#set-rules)および[showルール]($styling/#show-rules)のドキュメントも参照して下さい。
///
/// # 要素関数
/// [見出し]($heading)や[表]($table)のような、いくつかの関数は _要素_ と結びついており、呼び出すとその種類に応じた要素を作成します。
/// さらに、通常の関数とは異なり、要素関数は[setルール]($styling/#set-rules)、[showルール]($styling/#show-rules)および [セレクター]($selector)で使用可能です。
///
/// # 関数スコープ
/// 関数は、[モジュール]($scripting/#modules)と同様に自身のスコープ内に関連する定義を保持できます。
/// この例は[`assert.eq`]($assert.eq)や[`list.item`]($list.item)です。
/// しかし、現在この機能が利用可能なのは組み込み関数のみです。
///
/// # ユーザー定義関数
/// [letバインディング]($scripting/#bindings)を用いることで、バインディング名の後に引数リストを持ったユーザー定義関数を定義することができます。
/// 引数リストには必須の位置引数、デフォルト値を持つ名前付き引数および[可変長引数]($arguments)を用いることができます。
///
/// 関数バインディングの右辺は関数本体で、ブロックか任意の式です。
/// 関数の戻り値を定義し、引数に依存させることができます。
/// 関数本体が[コードブロック]($scripting/#blocks)の場合、戻り値はブロック内の全ての式を連結させた結果になります。
///
/// 関数本体内では、`return`キーワードを用いて処理を途中で抜け出したり、必要に応じて戻り値を指定して返したりできます。
/// 戻り値が明示的に与えられない場合、本体は`return`の前の式全てを連結した結果として評価されます。
///
/// 意味のある値を何も返さない関数は、代わりに[`none`]を返します。
/// このような関数の戻り値の型はドキュメント中では明示的に指定されていません
/// （この例としては[`array.push`]が該当します）。
=======
/// Functions are a fundamental building block of Typst. Typst provides
/// functions for a variety of typesetting tasks. Moreover, the markup you write
/// is backed by functions and all styling happens through functions. This
/// reference lists all available functions and how you can use them. Please
/// also refer to the documentation about [set]($styling/#set-rules) and
/// [show]($styling/#show-rules) rules to learn about additional ways you can
/// work with functions in Typst.
///
/// # Element functions
/// Some functions are associated with _elements_ like [headings]($heading) or
/// [tables]($table). When called, these create an element of their respective
/// kind. In contrast to normal functions, they can further be used in [set
/// rules]($styling/#set-rules), [show rules]($styling/#show-rules), and
/// [selectors]($selector).
///
/// # Function scopes
/// Functions can hold related definitions in their own scope, similar to a
/// [module]($scripting/#modules). Examples of this are [`assert.eq`] or
/// [`list.item`]. However, this feature is currently only available for
/// built-in functions.
///
/// # Defining functions
/// You can define your own function with a [let binding]($scripting/#bindings)
/// that has a parameter list after the binding's name. The parameter list can
/// contain mandatory positional parameters, named parameters with default
/// values and [argument sinks]($arguments).
///
/// The right-hand side of a function binding is the function body, which can be
/// a block or any other expression. It defines the function's return value and
/// can depend on the parameters. If the function body is a [code
/// block]($scripting/#blocks), the return value is the result of joining the
/// values of each expression in the block.
///
/// Within a function body, the `return` keyword can be used to exit early and
/// optionally specify a return value. If no explicit return value is given, the
/// body evaluates to the result of joining all expressions preceding the
/// `return`.
///
/// Functions that don't return any meaningful value return [`none`] instead.
/// The return type of such functions is not explicitly specified in the
/// documentation. (An example of this is [`array.push`]).
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// #let alert(body, fill: red) = {
///   set text(white)
///   set align(center)
///   rect(
///     fill: fill,
///     inset: 8pt,
///     radius: 4pt,
///     [*Warning:\ #body*],
///   )
/// }
///
/// #alert[
///   Danger is imminent!
/// ]
///
/// #alert(fill: blue)[
///   KEEP OFF TRACKS
/// ]
/// ```
///
<<<<<<< HEAD
/// # 関数のインポート
/// 関数は、`{import}`を用いてあるファイル（[`module`]($scripting/#modules)）から別のファイルにインポートすることができます。
/// 例えば、上記の例にある`alert`関数を`foo.typ`というファイルに定義したとします。
/// この場合、`{import "foo.typ": alert}`と書くことで別のファイルにインポートできます。
///
/// # 無名関数 { #unnamed }
/// 引数リストに続けて `=>` と関数本体を指定することで、バインディングを作らずに無名関数も作成できます。
/// もし関数の引数が1つだけならば、引数リストの周りの括弧は必須ではありません。
/// 無名関数は主にshowルールで用いると便利ですが、page関数の[`footer`]($page.footer)プロパティのような、関数を引数に取る設定可能プロパティにも便利です。
=======
/// # Importing functions
/// Functions can be imported from one file ([`module`]($scripting/#modules)) into
/// another using `{import}`. For example, assume that we have defined the `alert`
/// function from the previous example in a file called `foo.typ`. We can import
/// it into another file by writing `{import "foo.typ": alert}`.
///
/// # Unnamed functions { #unnamed }
/// You can also create an unnamed function without creating a binding by
/// specifying a parameter list followed by `=>` and the function body. If your
/// function has just one parameter, the parentheses around the parameter list
/// are optional. Unnamed functions are mainly useful for show rules, but also
/// for settable properties that take functions like the page function's
/// [`footer`]($page.footer) property.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// #show "once?": it => [#it #it]
/// once?
/// ```
///
<<<<<<< HEAD
/// # 関数の純粋性に関する注意
/// Typstにおいて関数は全て _純粋_ です。
/// これは同じ引数からは常に同じ結果が返ってくることを意味します。
/// 純粋関数は2回目の呼び出し時に別の値を生成するために何かを「記憶」することはできません。
///
/// 唯一の例外は[`array.push(value)`]($array.push)のような組み込みメソッドです。
/// これらは呼び出された対象を変更できます。
=======
/// # Note on function purity
/// In Typst, all functions are _pure._ This means that for the same
/// arguments, they always return the same result. They cannot "remember" things to
/// produce another value when they are called a second time.
///
/// The only exception are built-in methods like
/// [`array.push(value)`]($array.push). These can modify the values they are
/// called on.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
#[ty(scope, cast, name = "function")]
#[derive(Clone, Hash)]
#[allow(clippy::derived_hash_with_manual_eq)]
pub struct Func {
    /// The internal representation.
    repr: Repr,
    /// The span with which errors are reported when this function is called.
    span: Span,
}

/// The different kinds of function representations.
#[derive(Clone, PartialEq, Hash)]
enum Repr {
    /// A native Rust function.
    Native(Static<NativeFuncData>),
    /// A function for an element.
    Element(Element),
    /// A user-defined closure.
    Closure(Arc<LazyHash<Closure>>),
    /// A plugin WebAssembly function.
    Plugin(Arc<PluginFunc>),
    /// A nested function with pre-applied arguments.
    With(Arc<(Func, Args)>),
}

impl Func {
    /// The function's name (e.g. `min`).
    ///
    /// Returns `None` if this is an anonymous closure.
    pub fn name(&self) -> Option<&str> {
        match &self.repr {
            Repr::Native(native) => Some(native.name),
            Repr::Element(elem) => Some(elem.name()),
            Repr::Closure(closure) => closure.name(),
            Repr::Plugin(func) => Some(func.name()),
            Repr::With(with) => with.0.name(),
        }
    }

    /// The function's title case name, for use in documentation (e.g. `Minimum`).
    ///
    /// Returns `None` if this is a closure.
    pub fn title(&self) -> Option<&'static str> {
        match &self.repr {
            Repr::Native(native) => Some(native.title),
            Repr::Element(elem) => Some(elem.title()),
            Repr::Closure(_) => None,
            Repr::Plugin(_) => None,
            Repr::With(with) => with.0.title(),
        }
    }

    /// Documentation for the function (as Markdown).
    pub fn docs(&self) -> Option<&'static str> {
        match &self.repr {
            Repr::Native(native) => Some(native.docs),
            Repr::Element(elem) => Some(elem.docs()),
            Repr::Closure(_) => None,
            Repr::Plugin(_) => None,
            Repr::With(with) => with.0.docs(),
        }
    }

    /// Whether the function is known to be contextual.
    pub fn contextual(&self) -> Option<bool> {
        match &self.repr {
            Repr::Native(native) => Some(native.contextual),
            _ => None,
        }
    }

    /// Get details about this function's parameters if available.
    pub fn params(&self) -> Option<&'static [ParamInfo]> {
        match &self.repr {
            Repr::Native(native) => Some(&native.0.params),
            Repr::Element(elem) => Some(elem.params()),
            Repr::Closure(_) => None,
            Repr::Plugin(_) => None,
            Repr::With(with) => with.0.params(),
        }
    }

    /// Get the parameter info for a parameter with the given name if it exist.
    pub fn param(&self, name: &str) -> Option<&'static ParamInfo> {
        self.params()?.iter().find(|param| param.name == name)
    }

    /// Get details about the function's return type.
    pub fn returns(&self) -> Option<&'static CastInfo> {
        match &self.repr {
            Repr::Native(native) => Some(&native.0.returns),
            Repr::Element(_) => {
                Some(singleton!(CastInfo, CastInfo::Type(Type::of::<Content>())))
            }
            Repr::Closure(_) => None,
            Repr::Plugin(_) => None,
            Repr::With(with) => with.0.returns(),
        }
    }

    /// Search keywords for the function.
    pub fn keywords(&self) -> &'static [&'static str] {
        match &self.repr {
            Repr::Native(native) => native.keywords,
            Repr::Element(elem) => elem.keywords(),
            Repr::Closure(_) => &[],
            Repr::Plugin(_) => &[],
            Repr::With(with) => with.0.keywords(),
        }
    }

    /// The function's associated scope of sub-definition.
    pub fn scope(&self) -> Option<&'static Scope> {
        match &self.repr {
            Repr::Native(native) => Some(&native.0.scope),
            Repr::Element(elem) => Some(elem.scope()),
            Repr::Closure(_) => None,
            Repr::Plugin(_) => None,
            Repr::With(with) => with.0.scope(),
        }
    }

    /// Get a field from this function's scope, if possible.
    pub fn field(
        &self,
        field: &str,
        sink: impl DeprecationSink,
    ) -> StrResult<&'static Value> {
        let scope =
            self.scope().ok_or("cannot access fields on user-defined functions")?;
        match scope.get(field) {
            Some(binding) => Ok(binding.read_checked(sink)),
            None => match self.name() {
                Some(name) => bail!("function `{name}` does not contain field `{field}`"),
                None => bail!("function does not contain field `{field}`"),
            },
        }
    }

    /// Extract the element function, if it is one.
    pub fn element(&self) -> Option<Element> {
        match self.repr {
            Repr::Element(func) => Some(func),
            _ => None,
        }
    }

    /// Extract the plugin function, if it is one.
    pub fn to_plugin(&self) -> Option<&PluginFunc> {
        match &self.repr {
            Repr::Plugin(func) => Some(func),
            _ => None,
        }
    }

    /// Call the function with the given context and arguments.
    pub fn call<A: IntoArgs>(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        args: A,
    ) -> SourceResult<Value> {
        self.call_impl(engine, context, args.into_args(self.span))
    }

    /// Non-generic implementation of `call`.
    #[typst_macros::time(name = "func call", span = self.span())]
    fn call_impl(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        mut args: Args,
    ) -> SourceResult<Value> {
        match &self.repr {
            Repr::Native(native) => {
<<<<<<< HEAD
                let value = (native.function)(engine, context, &mut args)?;
=======
                let value = (native.function.0)(engine, context, &mut args)?;
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
                args.finish()?;
                Ok(value)
            }
            Repr::Element(func) => {
                let value = func.construct(engine, &mut args)?;
                args.finish()?;
                Ok(Value::Content(value))
            }
            Repr::Closure(closure) => (engine.routines.eval_closure)(
                self,
                closure,
                engine.routines,
                engine.world,
                engine.introspector,
                engine.traced,
                TrackedMut::reborrow_mut(&mut engine.sink),
                engine.route.track(),
                context,
                args,
            ),
            Repr::Plugin(func) => {
                let inputs = args.all::<Bytes>()?;
                let output = func.call(inputs).at(args.span)?;
                args.finish()?;
                Ok(Value::Bytes(output))
            }
            Repr::With(with) => {
                args.items = with.1.items.iter().cloned().chain(args.items).collect();
                with.0.call(engine, context, args)
            }
        }
    }

    /// The function's span.
    pub fn span(&self) -> Span {
        self.span
    }

    /// Attach a span to this function if it doesn't already have one.
    pub fn spanned(mut self, span: Span) -> Self {
        if self.span.is_detached() {
            self.span = span;
        }
        self
    }
}

#[scope]
impl Func {
<<<<<<< HEAD
    /// 指定した引数を事前に適用した新しい関数を返します。
=======
    /// Returns a new function that has the given arguments pre-applied.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[func]
    pub fn with(
        self,
        args: &mut Args,
<<<<<<< HEAD
        /// 関数に適用する引数。
=======
        /// The arguments to apply to the function.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        #[external]
        #[variadic]
        arguments: Vec<Value>,
    ) -> Func {
        let span = self.span;
        Self {
            repr: Repr::With(Arc::new((self, args.take()))),
            span,
        }
    }

<<<<<<< HEAD
    /// この関数に属する要素のうち、与えられた引数と同じ値のフィールドを持つものを絞り込むセレクターを返します。
=======
    /// Returns a selector that filters for elements belonging to this function
    /// whose fields have the values of the given arguments.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ///
    /// ```example
    /// #show heading.where(level: 2): set text(blue)
    /// = Section
    /// == Subsection
    /// === Sub-subsection
    /// ```
    #[func]
    pub fn where_(
        self,
        args: &mut Args,
<<<<<<< HEAD
        /// 絞り込むフィールド。
=======
        /// The fields to filter for.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        #[variadic]
        #[external]
        fields: Vec<Value>,
    ) -> StrResult<Selector> {
        let fields = args.to_named();
        args.items.retain(|arg| arg.name.is_none());

        let element = self
            .element()
            .ok_or("`where()` can only be called on element functions")?;

        let fields = fields
            .into_iter()
            .map(|(key, value)| {
                element.field_id(&key).map(|id| (id, value)).ok_or_else(|| {
                    eco_format!(
                        "element `{}` does not have field `{}`",
                        element.name(),
                        key
                    )
                })
            })
            .collect::<StrResult<smallvec::SmallVec<_>>>()?;

        Ok(element.where_(fields))
    }
}

impl Debug for Func {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Func({})", self.name().unwrap_or(".."))
    }
}

impl repr::Repr for Func {
    fn repr(&self) -> EcoString {
<<<<<<< HEAD
        match self.name() {
            Some(name) => name.into(),
            None => "(..) => ..".into(),
=======
        const DEFAULT: &str = "(..) => ..";
        match &self.repr {
            Repr::Native(native) => native.name.into(),
            Repr::Element(elem) => elem.name().into(),
            Repr::Closure(closure) => closure.name().unwrap_or(DEFAULT).into(),
            Repr::Plugin(func) => func.name().clone(),
            Repr::With(_) => DEFAULT.into(),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        }
    }
}

impl PartialEq for Func {
    fn eq(&self, other: &Self) -> bool {
        self.repr == other.repr
    }
}

impl PartialEq<&'static NativeFuncData> for Func {
    fn eq(&self, other: &&'static NativeFuncData) -> bool {
        match &self.repr {
            Repr::Native(native) => *native == Static(*other),
            _ => false,
        }
    }
}

<<<<<<< HEAD
=======
impl PartialEq<Element> for Func {
    fn eq(&self, other: &Element) -> bool {
        match &self.repr {
            Repr::Element(elem) => elem == other,
            _ => false,
        }
    }
}

>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
impl From<Repr> for Func {
    fn from(repr: Repr) -> Self {
        Self { repr, span: Span::detached() }
    }
}

impl From<&'static NativeFuncData> for Func {
    fn from(data: &'static NativeFuncData) -> Self {
        Repr::Native(Static(data)).into()
    }
}

impl From<Element> for Func {
    fn from(func: Element) -> Self {
        Repr::Element(func).into()
    }
}

impl From<Closure> for Func {
    fn from(closure: Closure) -> Self {
        Repr::Closure(Arc::new(LazyHash::new(closure))).into()
    }
}

impl From<PluginFunc> for Func {
    fn from(func: PluginFunc) -> Self {
        Repr::Plugin(Arc::new(func)).into()
    }
}

/// A Typst function that is defined by a native Rust type that shadows a
/// native Rust function.
pub trait NativeFunc {
    /// Get the function for the native Rust type.
    fn func() -> Func {
        Func::from(Self::data())
    }

    /// Get the function data for the native Rust function.
    fn data() -> &'static NativeFuncData;
}

/// Defines a native function.
#[derive(Debug)]
pub struct NativeFuncData {
<<<<<<< HEAD
    /// Invokes the function from Typst.
    pub function: fn(&mut Engine, Tracked<Context>, &mut Args) -> SourceResult<Value>,
=======
    /// The implementation of the function.
    pub function: NativeFuncPtr,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    /// The function's normal name (e.g. `align`), as exposed to Typst.
    pub name: &'static str,
    /// The function's title case name (e.g. `Align`).
    pub title: &'static str,
    /// The documentation for this function as a string.
    pub docs: &'static str,
    /// A list of alternate search terms for this function.
    pub keywords: &'static [&'static str],
    /// Whether this function makes use of context.
    pub contextual: bool,
    /// Definitions in the scope of the function.
<<<<<<< HEAD
    pub scope: LazyLock<Scope>,
    /// A list of parameter information for each parameter.
    pub params: LazyLock<Vec<ParamInfo>>,
    /// Information about the return value of this function.
    pub returns: LazyLock<CastInfo>,
=======
    pub scope: DynLazyLock<Scope>,
    /// A list of parameter information for each parameter.
    pub params: DynLazyLock<Vec<ParamInfo>>,
    /// Information about the return value of this function.
    pub returns: DynLazyLock<CastInfo>,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

cast! {
    &'static NativeFuncData,
    self => Func::from(self).into_value(),
}

<<<<<<< HEAD
=======
/// A pointer to a native function's implementation.
pub struct NativeFuncPtr(pub &'static NativeFuncSignature);

/// The signature of a native function's implementation.
type NativeFuncSignature =
    dyn Fn(&mut Engine, Tracked<Context>, &mut Args) -> SourceResult<Value> + Send + Sync;

impl Debug for NativeFuncPtr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad("NativeFuncPtr(..)")
    }
}

/// A `LazyLock` that uses a static closure for initialization instead of only
/// working with function pointers.
///
/// Can be created from a normal function or closure by prepending with a `&`,
/// e.g. `LazyLock::new(&|| "hello")`. Can be created from a dynamic closure
/// by allocating and then leaking it. This is equivalent to having it
/// statically allocated, but allows for it to be generated at runtime.
type DynLazyLock<T> = LazyLock<T, &'static (dyn Fn() -> T + Send + Sync)>;

>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// Describes a function parameter.
#[derive(Debug, Clone)]
pub struct ParamInfo {
    /// The parameter's name.
    pub name: &'static str,
    /// Documentation for the parameter.
    pub docs: &'static str,
    /// Describe what values this parameter accepts.
    pub input: CastInfo,
    /// Creates an instance of the parameter's default value.
    pub default: Option<fn() -> Value>,
    /// Is the parameter positional?
    pub positional: bool,
    /// Is the parameter named?
    ///
    /// Can be true even if `positional` is true if the parameter can be given
    /// in both variants.
    pub named: bool,
    /// Can the parameter be given any number of times?
    pub variadic: bool,
    /// Is the parameter required?
    pub required: bool,
    /// Is the parameter settable with a set rule?
    pub settable: bool,
}

<<<<<<< HEAD
/// A user-defined closure.
#[derive(Debug, Hash)]
pub struct Closure {
    /// The closure's syntax node. Must be either castable to `ast::Closure` or
    /// `ast::Expr`. In the latter case, this is a synthesized closure without
    /// any parameters (used by `context` expressions).
    pub node: SyntaxNode,
=======
/// Distinguishes between variants of closures.
#[derive(Debug, Hash)]
pub enum ClosureNode {
    /// A regular closure. Must always be castable to a `ast::Closure`.
    Closure(SyntaxNode),
    /// Synthetic closure used for `context` expressions. Can be any `ast::Expr`
    /// and has no parameters.
    Context(SyntaxNode),
}

/// A user-defined closure.
#[derive(Debug, Hash)]
pub struct Closure {
    /// The closure's syntax node.
    pub node: ClosureNode,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    /// Default values of named parameters.
    pub defaults: Vec<Value>,
    /// Captured values from outer scopes.
    pub captured: Scope,
    /// The number of positional parameters in the closure.
    pub num_pos_params: usize,
}

impl Closure {
    /// The name of the closure.
    pub fn name(&self) -> Option<&str> {
<<<<<<< HEAD
        self.node.cast::<ast::Closure>()?.name().map(|ident| ident.as_str())
=======
        match self.node {
            ClosureNode::Closure(ref node) => {
                node.cast::<ast::Closure>()?.name().map(|ident| ident.as_str())
            }
            _ => None,
        }
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }
}

cast! {
    Closure,
    self => Value::Func(self.into()),
}
