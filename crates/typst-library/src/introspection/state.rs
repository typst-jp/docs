use comemo::{Track, Tracked, TrackedMut};
use ecow::{eco_format, eco_vec, EcoString, EcoVec};
use typst_syntax::Span;

use crate::diag::{bail, At, SourceResult};
use crate::engine::{Engine, Route, Sink, Traced};
use crate::foundations::{
    cast, elem, func, scope, select_where, ty, Args, Construct, Content, Context, Func,
    LocatableSelector, NativeElement, Packed, Repr, Selector, Show, Str, StyleChain,
    Value,
};
use crate::introspection::{Introspector, Locatable, Location};
use crate::routines::Routines;
use crate::World;

/// 文書中の状態の管理。
///
/// 文書中で何回か計算を行い、最後の計算結果を次の計算で使用するために記憶しておきたいとします。
/// 以下と同等のコードを試すと10、13、26、21と出力されることを期待するでしょう。
/// しかしTypstでは**そうはなりません**。
/// このコードを試してみると、Typstは _Variables from outside the function are read-only and cannot be modified._ というエラーメッセージを出力することが分かります。
///
/// ```typ
/// // This doesn't work!
/// #let x = 0
/// #let compute(expr) = {
///   x = eval(
///     expr.replace("x", str(x))
///   )
///   [New value is #x. ]
/// }
///
/// #compute("10") \
/// #compute("x + 3") \
/// #compute("x * 2") \
/// #compute("x - 5")
/// ```
///
/// # 状態と文書のマークアップ { #state-and-markup }
/// なぜこうなるのでしょうか？
/// 一般的に副作用を伴うこの手の計算は文書のマークアップにおいて問題を引き起こすためで、Typstではこれをエラーとして扱います。
/// この結果を理解するには、計算処理が文書内で生成物がレイアウトされる順序と同じ順序で行われる必要があります。
/// 今回の単純な例ではこの条件が満たされますが、一般的には必ずしもそうとは限りません。
///
/// 見出しの番号付けという、類似した状態ですが、少し異なる例を見てみましょう。
/// 各見出しで見出しカウンターの値を増やしたいとします。
/// 簡単ですよね？
/// ただ1を足すだけです。
/// 残念ながらそう単純ではないのです。
/// 以下の例を考えます。
///
/// ```example
/// #set heading(numbering: "1.")
/// #let template(body) = [
///   = Outline
///   ...
///   #body
/// ]
///
/// #show: template
///
/// = Introduction
/// ...
/// ```
///
/// ここで、Typstはまずshowルール以降の文書本体を処理し、`Introduction`見出しを検知します。
/// 続いて`template`関数に生成コンテンツを渡します。
/// その後、初めて`Outline`を検知します。
/// 単にカウンター値を増やすと`Introduction`は`1`に、`Outline`は`2`となります。
///
/// # Typstにおける状態管理 { #state-in-typst }
/// それでは代わりにどうするのでしょうか？
/// Typstの状態管理システムを使用します。
/// 識別用のキーとなる文字列とオプションの初期値とともに`state`関数を呼び出すことで状態値が得られます。
/// この状態値はいくつかの関数を公開しており、最も重要な2つの関数が`get`と`update`です。
///
/// - [`get`]($state.get)関数は状態の現在値を取得します。
/// 値は文書中で変化するため、これは[コンテキスト]($context)が利用可能な場合にのみ使用できる _コンテキスト_ 関数です。
///
/// - [`update`]($state.update)関数は状態に修正を加えます。
/// 任意の値が使用できます。
/// 関数ではない値が渡された場合、状態にその値が設定されます。
/// 関数が与えられた場合、その関数は前の状態を受け取り、新しい状態を返さなければなりません。
///
/// 最初の例は以下のようになります。
///
/// ```example
/// #let s = state("x", 0)
/// #let compute(expr) = [
///   #s.update(x =>
///     eval(expr.replace("x", str(x)))
///   )
///   New value is #context s.get().
/// ]
///
/// #compute("10") \
/// #compute("x + 3") \
/// #compute("x * 2") \
/// #compute("x - 5")
/// ```
///
/// Typstが管理する状態は常に評価順ではなくレイアウト順で更新されます。
/// `update`メソッドはコンテンツを返し、その影響は文書に返されたコンテンツが挿入された場所で生じます。
///
/// このようにして、計算結果を変数に保存できるようになり、正しい結果を表示しています。
///
/// ```example
/// >>> #let s = state("x", 0)
/// >>> #let compute(expr) = [
/// >>>   #s.update(x =>
/// >>>     eval(expr.replace("x", str(x)))
/// >>>   )
/// >>>   New value is #context s.get().
/// >>> ]
/// <<< ...
///
/// #let more = [
///   #compute("x * 2") \
///   #compute("x - 5")
/// ]
///
/// #compute("10") \
/// #compute("x + 3") \
/// #more
/// ```
///
/// この例はもちろん少々極端ですが、これが実際に本当に必要となることがよくあります！
/// 良い例は見出しカウンターです。
/// これはTypstの[カウンターシステム]($counter)が状態システムにとてもよく似ているためです。
///
/// # タイムトラベル
/// Typstの状態管理システムを使用するとタイムトラベルもできます！
/// 文書内の任意の位置でその状態がどの値になっているのかを、どこからでも突き止めることができます。
/// 特に、`at`メソッドを用いると特定の任意の位置での状態値が取得でき、`final`メソッドを用いると文書の終わりでの状態値を取得できます。
///
/// ```example
/// >>> #let s = state("x", 0)
/// >>> #let compute(expr) = [
/// >>>   #s.update(x => {
/// >>>     eval(expr.replace("x", str(x)))
/// >>>   })
/// >>>   New value is #context s.get().
/// >>> ]
/// <<< ...
///
/// Value at `<here>` is
/// #context s.at(<here>)
///
/// #compute("10") \
/// #compute("x + 3") \
/// *Here.* <here> \
/// #compute("x * 2") \
/// #compute("x - 5")
/// ```
///
/// # 注意事項 { #caution }
/// 全ての状態値を解決するために、Typstはコードを複数回評価します。
/// しかしながら、実際に状態操作が完全に解決されるかは保証されません。
///
/// 例えば、状態の最終的な値に依存する更新を行う状態を作成した場合、決して収束しなくなるでしょう。
/// 以下の例はこの実演です。
/// 状態を`1`で初期化し、続いて自身の最終値に1を足した値に更新します。
/// したがって値は`2`になるべきですが、最終値が`2`となったので`3`に更新します。以下同様です。
/// この例では有限値が表示されていますが、これは単にTypstが数回試行した後に諦めるためです。
///
/// ```example
/// // This is bad!
/// #let s = state("x", 1)
/// #context s.update(s.final() + 1)
/// #context s.get()
/// ```
///
/// 一般に、コンテキスト内部で更新を行う状態を作成しないようにしてください。
/// 可能であれば、更新内容をコンテキストに依存しない値として、あるいは前の値から新しい値を計算する関数として定義してください。
/// どうしても避けられない場合がありますが、その場合は結果が適切に収束することを保証することはあなたの責任です。
#[ty(scope)]
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct State {
    /// The key that identifies the state.
    key: Str,
    /// The initial value of the state.
    init: Value,
}

impl State {
    /// Create a new state identified by a key.
    pub fn new(key: Str, init: Value) -> State {
        Self { key, init }
    }

    /// Get the value of the state at the given location.
    pub fn at_loc(&self, engine: &mut Engine, loc: Location) -> SourceResult<Value> {
        let sequence = self.sequence(engine)?;
        let offset = engine.introspector.query_count_before(&self.selector(), loc);
        Ok(sequence[offset].clone())
    }

    /// Produce the whole sequence of states.
    ///
    /// This has to happen just once for all states, cutting down the number
    /// of state updates from quadratic to linear.
    fn sequence(&self, engine: &mut Engine) -> SourceResult<EcoVec<Value>> {
        self.sequence_impl(
            engine.routines,
            engine.world,
            engine.introspector,
            engine.traced,
            TrackedMut::reborrow_mut(&mut engine.sink),
            engine.route.track(),
        )
    }

    /// Memoized implementation of `sequence`.
    #[comemo::memoize]
    fn sequence_impl(
        &self,
        routines: &Routines,
        world: Tracked<dyn World + '_>,
        introspector: Tracked<Introspector>,
        traced: Tracked<Traced>,
        sink: TrackedMut<Sink>,
        route: Tracked<Route>,
    ) -> SourceResult<EcoVec<Value>> {
        let mut engine = Engine {
            routines,
            world,
            introspector,
            traced,
            sink,
            route: Route::extend(route).unnested(),
        };
        let mut state = self.init.clone();
        let mut stops = eco_vec![state.clone()];

        for elem in introspector.query(&self.selector()) {
            let elem = elem.to_packed::<StateUpdateElem>().unwrap();
            match &elem.update {
                StateUpdate::Set(value) => state = value.clone(),
                StateUpdate::Func(func) => {
                    state = func.call(&mut engine, Context::none().track(), [state])?
                }
            }
            stops.push(state.clone());
        }

        Ok(stops)
    }

    /// The selector for this state's updates.
    fn selector(&self) -> Selector {
        select_where!(StateUpdateElem, Key => self.key.clone())
    }

    /// Selects all state updates.
    pub fn select_any() -> Selector {
        StateUpdateElem::elem().select()
    }
}

#[scope]
impl State {
    /// キーで識別される新しい状態の作成。
    #[func(constructor)]
    pub fn construct(
        /// 状態を識別するキー。
        key: Str,
        /// 状態の初期値。
        #[default]
        init: Value,
    ) -> State {
        Self::new(key, init)
    }

    /// 現在位置での状態値を取得。
    ///
    /// これは`{state.at(here())}`と等価です。
    #[typst_macros::time(name = "state.get", span = span)]
    #[func(contextual)]
    pub fn get(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        span: Span,
    ) -> SourceResult<Value> {
        let loc = context.location().at(span)?;
        self.at_loc(engine, loc)
    }

    /// 指定したセレクターで一意に特定される対象の状態値を取得。
    ///
    /// `selector`は文書中で厳密に1つだけの要素にマッチしなければなりません。
    /// この目的で最も便利なセレクターは[ラベル]($label)と[位置]($location)です。
    #[typst_macros::time(name = "state.at", span = span)]
    #[func(contextual)]
    pub fn at(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        span: Span,
        /// 状態値を取得する場所。
        selector: LocatableSelector,
    ) -> SourceResult<Value> {
        let loc = selector.resolve_unique(engine.introspector, context).at(span)?;
        self.at_loc(engine, loc)
    }

    /// 文書の終わりでの状態値の取得。
    #[func(contextual)]
    pub fn final_(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        span: Span,
    ) -> SourceResult<Value> {
        context.introspect().at(span)?;
        let sequence = self.sequence(engine)?;
        Ok(sequence.last().unwrap().clone())
    }

    /// 状態値を更新。
    ///
    /// 更新は、返り値であるコンテンツが文書中に挿入された位置で適用されます。
    /// 文書中に出力がなければ何も起こりません！
    /// 例えば`{let _ = state("key").update(7)}`と書いた場合が、この何も起きないときに該当します。
    /// 状態の更新は常にレイアウト順に適用され、Typstはいつ状態を更新するのか知りません。
    #[func]
    pub fn update(
        self,
        span: Span,
        /// 関数ではない値が与えられた場合、状態にその値を設定します。
        /// 関数が与えられた場合、その関数は前の状態を受け取り、新しい状態を返さなければなりません。
        update: StateUpdate,
    ) -> Content {
        StateUpdateElem::new(self.key, update).pack().spanned(span)
    }
}

impl Repr for State {
    fn repr(&self) -> EcoString {
        eco_format!("state({}, {})", self.key.repr(), self.init.repr())
    }
}

/// An update to perform on a state.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum StateUpdate {
    /// Set the state to the specified value.
    Set(Value),
    /// Apply the given function to the state.
    Func(Func),
}

cast! {
    StateUpdate,
    v: Func => Self::Func(v),
    v: Value => Self::Set(v),
}

/// Executes a display of a state.
#[elem(Construct, Locatable, Show)]
struct StateUpdateElem {
    /// The key that identifies the state.
    #[required]
    key: Str,

    /// The update to perform on the state.
    #[required]
    #[internal]
    update: StateUpdate,
}

impl Construct for StateUpdateElem {
    fn construct(_: &mut Engine, args: &mut Args) -> SourceResult<Content> {
        bail!(args.span, "cannot be constructed manually");
    }
}

impl Show for Packed<StateUpdateElem> {
    fn show(&self, _: &mut Engine, _: StyleChain) -> SourceResult<Content> {
        Ok(Content::empty())
    }
}
