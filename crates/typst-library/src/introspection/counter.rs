use std::num::NonZeroUsize;
use std::str::FromStr;

use comemo::{Track, Tracked, TrackedMut};
use ecow::{eco_format, eco_vec, EcoString, EcoVec};
use smallvec::{smallvec, SmallVec};
use typst_syntax::Span;
use typst_utils::NonZeroExt;

use crate::diag::{bail, At, HintedStrResult, SourceResult};
use crate::engine::{Engine, Route, Sink, Traced};
use crate::foundations::{
    cast, elem, func, scope, select_where, ty, Args, Array, Construct, Content, Context,
    Element, Func, IntoValue, Label, LocatableSelector, NativeElement, Packed, Repr,
    Selector, Show, Smart, Str, StyleChain, Value,
};
use crate::introspection::{Introspector, Locatable, Location, Tag};
use crate::layout::{Frame, FrameItem, PageElem};
use crate::math::EquationElem;
use crate::model::{FigureElem, FootnoteElem, HeadingElem, Numbering, NumberingPattern};
use crate::routines::Routines;
use crate::World;

/// ページや要素などの数え上げ。
///
/// counter関数を用いることで、ページや見出し、図表などのカウンターにアクセスしたり、修正を加えたりできます。
/// さらに、独自のカウンターを定義して、他のものを数えることもできます。
///
/// カウンターは文書全体を通して変化するため、現在のその値は _コンテキスト依存_ です。
/// 先に進む前に[コンテキスト]($context)の章を読むことをおすすめします。
///
/// # カウンターへのアクセス { #accessing }
/// [`get`]($counter.get)関数を用いると、生のカウンター値にアクセスできます。
/// この関数は[配列]($array)を返します。
/// カウンターは（節や小節などの見出しの場合に）複数のレベルを持ち、配列の各アイテムが1つのレベルに対応します。
///
/// ```example
/// #set heading(numbering: "1.")
///
/// = Introduction
/// Raw value of heading counter is
/// #context counter(heading).get()
/// ```
///
/// # カウンターの表示 { #displaying }
/// しばしば、カウンター値をより人間が読みやすい形で表示したいことがあります。
/// そうするために、カウンターの[`display`]($counter.display)関数を呼び出します。
/// この関数は現在のカウンター値を取得し、与えられた形式か自動的に推論された[番号付け]($numbering)で整形します。
///
/// ```example
/// #set heading(numbering: "1.")
///
/// = Introduction
/// Some text here.
///
/// = Background
/// The current value is: #context {
///   counter(heading).display()
/// }
///
/// Or in roman numerals: #context {
///   counter(heading).display("I")
/// }
/// ```
///
/// # カウンターの変更 { #modifying }
/// `step`および`update`メソッドを用いてカウンターを変更できます。
///
/// - `step`メソッドは、カウンター値を1増やします。
///   カウンターは複数のレベルを持つことがあるため、オプションで`level`引数を取ります。
///   `level`が指定された場合、指定された深さのカウンター値を増やします。
///
/// - `update`メソッドを用いるとカウンターを任意に変更できます。
///   通常の形式では、整数（あるいは複数レベルに対しては配列を）与えます。
///   現在の値を受け取り新しい値を返す関数を代わりに与えると、より柔軟にできます。
///
/// 見出しのカウンターは見出しが表示される前にインクリメントされます。
/// そのため、2回目のupdateの後にカウンターが6であったとしても`Analysis`は7になります。
///
/// ```example
/// #set heading(numbering: "1.")
///
/// = Introduction
/// #counter(heading).step()
///
/// = Background
/// #counter(heading).update(3)
/// #counter(heading).update(n => n * 2)
///
/// = Analysis
/// Let's skip 7.1.
/// #counter(heading).step(level: 2)
///
/// == Analysis
/// Still at #context {
///   counter(heading).display()
/// }
/// ```
///
/// # ページカウンター
/// ページカウンターは特別です。
/// 改ページ毎に値がインクリメントされます。
/// しかし、他のカウンターと同様に手動でインクリメントもできます。
/// 例えば、前書きではローマ数字のページ番号を使い、メインのコンテンツではアラビア数字のページ番号に変更し、ページカウンターを1にリセットできます。
///
/// ```example
/// >>> #set page(
/// >>>   height: 100pt,
/// >>>   margin: (bottom: 24pt, rest: 16pt),
/// >>> )
/// #set page(numbering: "(i)")
///
/// = Preface
/// The preface is numbered with
/// roman numerals.
///
/// #set page(numbering: "1 / 1")
/// #counter(page).update(1)
///
/// = Main text
/// Here, the counter is reset to one.
/// We also display both the current
/// page and total number of pages in
/// Arabic numbers.
/// ```
///
/// # カスタムカウンター
/// 独自のカウンターを定義するには文字列をキーとして`counter`関数を呼び出します。
/// このキーはグローバルにカウンターを識別します。
///
/// ```example
/// #let mine = counter("mycounter")
/// #context mine.display() \
/// #mine.step()
/// #context mine.display() \
/// #mine.update(c => c * 3)
/// #context mine.display()
/// ```
///
/// # インクリメント方法
/// カスタムカウンターを定義して使用する場合、一般にカウンターを最初にインクリメントしてから表示するべきです。
/// こうすることで、カウンターのインクリメント動作をインクリメントする要素に依存させることができます。
/// 例えばtheoremのカウンターを実装する場合、theoremの定義では最初にカウンターのstepを書いてインクリメントを行い、その後に初めてカウンターとtheoremの内容を表示します。
///
/// ```example
/// #let c = counter("theorem")
/// #let theorem(it) = block[
///   #c.step()
///   *Theorem #context c.display():*
///   #it
/// ]
///
/// #theorem[$1 = 1$]
/// #theorem[$2 < 3$]
/// ```
///
/// この背景にある考え方は、見出しカウンターの例で説明するのが最適です。
/// 見出しカウンターの更新は、その見出しのレベルに依存します。
/// 見出しの直前にインクリメントすることで、第2レベルの見出しがあるときに`1`から`1.1`へと正しく更新できます。
/// もし見出しの後にインクリメントする場合、どれをインクリメントするのかわかりません。
///
/// カウンターは常に数える要素の前にインクリメントすべきなため、必ず0始まりです。
/// このようにして、（最初のインクリメントの後に）最初に表示されるときには1になります。
///
/// # タイムトラベル
/// カウンターはタイムトラベルができます！
/// カウンターの最終的な値を実際に到達する前に知ることができますし、文書の任意の特定の場所での値がどうなっていたかさえ決定できます。
///
/// ```example
/// #let mine = counter("mycounter")
///
/// = Values
/// #context [
///   Value here: #mine.get() \
///   At intro: #mine.at(<intro>) \
///   Final value: #mine.final()
/// ]
///
/// #mine.update(n => n + 3)
///
/// = Introduction <intro>
/// #lorem(10)
///
/// #mine.step()
/// #mine.step()
/// ```
///
/// # その他の状態 { #other-state }
/// `counter`型は[state]型と密接に関係しています。
/// Typstにおける状態管理のより詳しい詳細と、なぜ単に普通の変数をカウンターに用いないのかについてはstateのドキュメントを参照してください。
#[ty(scope)]
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Counter(CounterKey);

impl Counter {
    /// Create a new counter identified by a key.
    pub fn new(key: CounterKey) -> Counter {
        Self(key)
    }

    /// The counter for the given element.
    pub fn of(func: Element) -> Self {
        Self::new(CounterKey::Selector(Selector::Elem(func, None)))
    }

    /// Gets the current and final value of the state combined in one state.
    pub fn both(
        &self,
        engine: &mut Engine,
        location: Location,
    ) -> SourceResult<CounterState> {
        let sequence = self.sequence(engine)?;
        let offset = engine.introspector.query_count_before(&self.selector(), location);
        let (mut at_state, at_page) = sequence[offset].clone();
        let (mut final_state, final_page) = sequence.last().unwrap().clone();
        if self.is_page() {
            let at_delta =
                engine.introspector.page(location).get().saturating_sub(at_page.get());
            at_state.step(NonZeroUsize::ONE, at_delta);
            let final_delta =
                engine.introspector.pages().get().saturating_sub(final_page.get());
            final_state.step(NonZeroUsize::ONE, final_delta);
        }
        Ok(CounterState(smallvec![at_state.first(), final_state.first()]))
    }

    /// Gets the value of the counter at the given location. Always returns an
    /// array of integers, even if the counter has just one number.
    pub fn at_loc(
        &self,
        engine: &mut Engine,
        location: Location,
    ) -> SourceResult<CounterState> {
        let sequence = self.sequence(engine)?;
        let offset = engine.introspector.query_count_before(&self.selector(), location);
        let (mut state, page) = sequence[offset].clone();
        if self.is_page() {
            let delta =
                engine.introspector.page(location).get().saturating_sub(page.get());
            state.step(NonZeroUsize::ONE, delta);
        }
        Ok(state)
    }

    /// Displays the value of the counter at the given location.
    pub fn display_at_loc(
        &self,
        engine: &mut Engine,
        loc: Location,
        styles: StyleChain,
        numbering: &Numbering,
    ) -> SourceResult<Content> {
        let context = Context::new(Some(loc), Some(styles));
        Ok(self
            .at_loc(engine, loc)?
            .display(engine, context.track(), numbering)?
            .display())
    }

    /// Produce the whole sequence of counter states.
    ///
    /// This has to happen just once for all counters, cutting down the number
    /// of counter updates from quadratic to linear.
    fn sequence(
        &self,
        engine: &mut Engine,
    ) -> SourceResult<EcoVec<(CounterState, NonZeroUsize)>> {
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
    ) -> SourceResult<EcoVec<(CounterState, NonZeroUsize)>> {
        let mut engine = Engine {
            routines,
            world,
            introspector,
            traced,
            sink,
            route: Route::extend(route).unnested(),
        };

        let mut state = CounterState::init(matches!(self.0, CounterKey::Page));
        let mut page = NonZeroUsize::ONE;
        let mut stops = eco_vec![(state.clone(), page)];

        for elem in introspector.query(&self.selector()) {
            if self.is_page() {
                let prev = page;
                page = introspector.page(elem.location().unwrap());

                let delta = page.get() - prev.get();
                if delta > 0 {
                    state.step(NonZeroUsize::ONE, delta);
                }
            }

            if let Some(update) = match elem.with::<dyn Count>() {
                Some(countable) => countable.update(),
                None => Some(CounterUpdate::Step(NonZeroUsize::ONE)),
            } {
                state.update(&mut engine, update)?;
            }

            stops.push((state.clone(), page));
        }

        Ok(stops)
    }

    /// The selector relevant for this counter's updates.
    fn selector(&self) -> Selector {
        let mut selector = select_where!(CounterUpdateElem, Key => self.0.clone());

        if let CounterKey::Selector(key) = &self.0 {
            selector = Selector::Or(eco_vec![selector, key.clone()]);
        }

        selector
    }

    /// Whether this is the page counter.
    fn is_page(&self) -> bool {
        self.0 == CounterKey::Page
    }

    /// Shared implementation of displaying between `counter.display` and
    /// `CounterDisplayElem`.
    fn display_impl(
        &self,
        engine: &mut Engine,
        location: Location,
        numbering: Smart<Numbering>,
        both: bool,
        styles: Option<StyleChain>,
    ) -> SourceResult<Value> {
        let numbering = numbering
            .custom()
            .or_else(|| {
                let styles = styles?;
                match self.0 {
                    CounterKey::Page => PageElem::numbering_in(styles).clone(),
                    CounterKey::Selector(Selector::Elem(func, _)) => {
                        if func == HeadingElem::elem() {
                            HeadingElem::numbering_in(styles).clone()
                        } else if func == FigureElem::elem() {
                            FigureElem::numbering_in(styles).clone()
                        } else if func == EquationElem::elem() {
                            EquationElem::numbering_in(styles).clone()
                        } else if func == FootnoteElem::elem() {
                            Some(FootnoteElem::numbering_in(styles).clone())
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            })
            .unwrap_or_else(|| NumberingPattern::from_str("1.1").unwrap().into());

        let state = if both {
            self.both(engine, location)?
        } else {
            self.at_loc(engine, location)?
        };

        let context = Context::new(Some(location), styles);
        state.display(engine, context.track(), &numbering)
    }

    /// Selects all state updates.
    pub fn select_any() -> Selector {
        CounterUpdateElem::elem().select()
    }
}

#[scope]
impl Counter {
    /// キーで識別される新しいカウンターの作成。
    #[func(constructor)]
    pub fn construct(
        /// このcounterを識別するキー。
        ///
        /// - 文字列の場合、手動更新の場合にのみ影響を受けるカスタムカウンターを作成します
        /// - [`page`]関数の場合、改ページに合わせてカウントされます
        /// - [セレクター]($selector)の場合、セレクターにマッチする要素が現れるたびにカウントされます。
        /// 例えば
        ///   - 要素関数が与えられた場合、その型を持つ要素がカウントされます
        ///   - [`{<label>}`]($label)が与えられた場合、そのラベルを持つ要素がカウントされます
        key: CounterKey,
    ) -> Counter {
        Self::new(key)
    }

    /// 現在のロケーションでのカウンター値を取得。
    /// カウンターが1つの数値しか持たない場合でも、常に整数の配列を返します。
    ///
    /// これは`{counter.at(here())}`と等価です。
    #[func(contextual)]
    pub fn get(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        span: Span,
    ) -> SourceResult<CounterState> {
        let loc = context.location().at(span)?;
        self.at_loc(engine, loc)
    }

    /// 番号付けされたカウンターの現在の値の表示および戻り値としての整形された出力。
    #[func(contextual)]
    pub fn display(
        self,
        engine: &mut Engine,
        context: Tracked<Context>,
        span: Span,
        /// カウンターをどのように表示するか指定する[番号付けのパターンまたは関数]($numbering)。
        /// 関数が与えられた場合、カウンターの各数値が別々の引数として関数に渡されます。
        /// 見出しの番号付けを関数指定するときなどの、引数の数値の数が変化する場合は[引数シンク]($arguments)が使用できます。
        ///
        /// 省略されるか`{auto}`に設定された場合、カウントする要素に設定された番号付け形式を用いてカウンターを表示します。
        /// そのようなスタイルが存在しない場合は`{"1.1"}`というパターン指定でカウンターが表示されます。
        #[default]
        numbering: Smart<Numbering>,
        /// 有効化された場合、トップレベルのカウンターの現在値と最終値を一緒に表示します。
        /// 両者のスタイル設定は1つの番号付けパターンで指定できます。
        /// これは、ページのnumberingプロパティで`{"1 / 1"}`のようなパターンが与えられたときに、現在のページ番号と総ページ数を表示するために使用されます。
        #[named]
        #[default(false)]
        both: bool,
    ) -> SourceResult<Value> {
        let loc = context.location().at(span)?;
        self.display_impl(engine, loc, numbering, both, context.styles().ok())
    }

    /// 指定された位置のカウンター値の取得。
    /// カウンターが1つの数値しか持たない場合でも、常に整数の配列を返します。
    ///
    /// `selector`は文書中で厳密に1つだけの要素にマッチしなければなりません。
    /// この目的で最も便利なセレクターは[ラベル]($label)と[ロケーション]($location)です。
    #[func(contextual)]
    pub fn at(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        span: Span,
        /// カウンター値を取得する場所。
        selector: LocatableSelector,
    ) -> SourceResult<CounterState> {
        let loc = selector.resolve_unique(engine.introspector, context).at(span)?;
        self.at_loc(engine, loc)
    }

    /// 文書の終わりでのカウンター値の取得。
    /// カウンターが1つの数値しか持たない場合でも、常に整数の配列を返します。
    #[func(contextual)]
    pub fn final_(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        span: Span,
    ) -> SourceResult<CounterState> {
        context.introspect().at(span)?;
        let sequence = self.sequence(engine)?;
        let (mut state, page) = sequence.last().unwrap().clone();
        if self.is_page() {
            let delta = engine.introspector.pages().get().saturating_sub(page.get());
            state.step(NonZeroUsize::ONE, delta);
        }
        Ok(state)
    }

    /// カウンター値を1増加。
    ///
    /// 更新は、返り値であるコンテンツが文書中に挿入された位置で適用されます。
    /// 文書中に出力がなければ何も起こりません！
    /// 例えば`{let _ = counter(page).step()}`と書いた場合が、この何も起きないときに該当します。
    /// カウンターの更新は常にレイアウト順に適用されるため、この場合にはTypstはいつカウンターをインクリメントするのか分かりません。
    #[func]
    pub fn step(
        self,
        span: Span,
        /// カウンターをインクリメントする深さ。
        /// デフォルトは`{1}`です。
        #[named]
        #[default(NonZeroUsize::ONE)]
        level: NonZeroUsize,
    ) -> Content {
        self.update(span, CounterUpdate::Step(level))
    }

    /// カウンター値の更新。
    ///
    /// `step`と同様に、生成されたコンテンツが文書中に配置されたときに限り更新が発生します。
    #[func]
    pub fn update(
        self,
        span: Span,
        /// 整数または整数の配列が与えられた場合、カウンターをその値に設定します。
        /// 関数が与えられた場合、その関数は（各数値を別々の引数として）前のカウンターの値を受け取り、新しい値を（整数または配列で）返さなければなりません。
        update: CounterUpdate,
    ) -> Content {
        CounterUpdateElem::new(self.0, update).pack().spanned(span)
    }
}

impl Repr for Counter {
    fn repr(&self) -> EcoString {
        eco_format!("counter({})", self.0.repr())
    }
}

/// Identifies a counter.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum CounterKey {
    /// The page counter.
    Page,
    /// Counts elements matching the given selectors. Only works for
    /// [locatable]($location/#locatable)
    /// elements or labels.
    Selector(Selector),
    /// Counts through manual counters with the same key.
    Str(Str),
}

cast! {
    CounterKey,
    self => match self {
        Self::Page => PageElem::elem().into_value(),
        Self::Selector(v) => v.into_value(),
        Self::Str(v) => v.into_value(),
    },
    v: Str => Self::Str(v),
    v: Label => Self::Selector(Selector::Label(v)),
    v: Element => {
        if v == PageElem::elem() {
            Self::Page
        } else {
            Self::Selector(LocatableSelector::from_value(v.into_value())?.0)
        }
    },
    v: LocatableSelector => Self::Selector(v.0),
}

impl Repr for CounterKey {
    fn repr(&self) -> EcoString {
        match self {
            Self::Page => "page".into(),
            Self::Selector(selector) => selector.repr(),
            Self::Str(str) => str.repr(),
        }
    }
}

/// An update to perform on a counter.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum CounterUpdate {
    /// Set the counter to the specified state.
    Set(CounterState),
    /// Increase the number for the given level by one.
    Step(NonZeroUsize),
    /// Apply the given function to the counter's state.
    Func(Func),
}

cast! {
    CounterUpdate,
    v: CounterState => Self::Set(v),
    v: Func => Self::Func(v),
}

/// Elements that have special counting behaviour.
pub trait Count {
    /// Get the counter update for this element.
    fn update(&self) -> Option<CounterUpdate>;
}

/// Counts through elements with different levels.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct CounterState(pub SmallVec<[usize; 3]>);

impl CounterState {
    /// Get the initial counter state for the key.
    pub fn init(page: bool) -> Self {
        // Special case, because pages always start at one.
        Self(smallvec![usize::from(page)])
    }

    /// Advance the counter and return the numbers for the given heading.
    pub fn update(
        &mut self,
        engine: &mut Engine,
        update: CounterUpdate,
    ) -> SourceResult<()> {
        match update {
            CounterUpdate::Set(state) => *self = state,
            CounterUpdate::Step(level) => self.step(level, 1),
            CounterUpdate::Func(func) => {
                *self = func
                    .call(engine, Context::none().track(), self.0.iter().copied())?
                    .cast()
                    .at(func.span())?
            }
        }
        Ok(())
    }

    /// Advance the number of the given level by the specified amount.
    pub fn step(&mut self, level: NonZeroUsize, by: usize) {
        let level = level.get();

        while self.0.len() < level {
            self.0.push(0);
        }

        self.0[level - 1] = self.0[level - 1].saturating_add(by);
        self.0.truncate(level);
    }

    /// Get the first number of the state.
    pub fn first(&self) -> usize {
        self.0.first().copied().unwrap_or(1)
    }

    /// Display the counter state with a numbering.
    pub fn display(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        numbering: &Numbering,
    ) -> SourceResult<Value> {
        numbering.apply(engine, context, &self.0)
    }
}

cast! {
    CounterState,
    self => Value::Array(self.0.into_iter().map(IntoValue::into_value).collect()),
    num: usize => Self(smallvec![num]),
    array: Array => Self(array
        .into_iter()
        .map(Value::cast)
        .collect::<HintedStrResult<_>>()?),
}

/// Executes an update of a counter.
#[elem(Construct, Locatable, Show, Count)]
struct CounterUpdateElem {
    /// The key that identifies the counter.
    #[required]
    key: CounterKey,

    /// The update to perform on the counter.
    #[required]
    #[internal]
    update: CounterUpdate,
}

impl Construct for CounterUpdateElem {
    fn construct(_: &mut Engine, args: &mut Args) -> SourceResult<Content> {
        bail!(args.span, "cannot be constructed manually");
    }
}

impl Show for Packed<CounterUpdateElem> {
    fn show(&self, _: &mut Engine, _: StyleChain) -> SourceResult<Content> {
        Ok(Content::empty())
    }
}

impl Count for Packed<CounterUpdateElem> {
    fn update(&self) -> Option<CounterUpdate> {
        Some(self.update.clone())
    }
}

/// Executes a display of a counter.
#[elem(Construct, Locatable, Show)]
pub struct CounterDisplayElem {
    /// The counter.
    #[required]
    #[internal]
    counter: Counter,

    /// The numbering to display the counter with.
    #[required]
    #[internal]
    numbering: Smart<Numbering>,

    /// Whether to display both the current and final value.
    #[required]
    #[internal]
    both: bool,
}

impl Construct for CounterDisplayElem {
    fn construct(_: &mut Engine, args: &mut Args) -> SourceResult<Content> {
        bail!(args.span, "cannot be constructed manually");
    }
}

impl Show for Packed<CounterDisplayElem> {
    fn show(&self, engine: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        Ok(self
            .counter
            .display_impl(
                engine,
                self.location().unwrap(),
                self.numbering.clone(),
                self.both,
                Some(styles),
            )?
            .display())
    }
}

/// An specialized handler of the page counter that tracks both the physical
/// and the logical page counter.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ManualPageCounter {
    physical: NonZeroUsize,
    logical: usize,
}

impl ManualPageCounter {
    /// Create a new fast page counter, starting at 1.
    pub fn new() -> Self {
        Self { physical: NonZeroUsize::ONE, logical: 1 }
    }

    /// Get the current physical page counter state.
    pub fn physical(&self) -> NonZeroUsize {
        self.physical
    }

    /// Get the current logical page counter state.
    pub fn logical(&self) -> usize {
        self.logical
    }

    /// Advance past a page.
    pub fn visit(&mut self, engine: &mut Engine, page: &Frame) -> SourceResult<()> {
        for (_, item) in page.items() {
            match item {
                FrameItem::Group(group) => self.visit(engine, &group.frame)?,
                FrameItem::Tag(Tag::Start(elem)) => {
                    let Some(elem) = elem.to_packed::<CounterUpdateElem>() else {
                        continue;
                    };
                    if elem.key == CounterKey::Page {
                        let mut state = CounterState(smallvec![self.logical]);
                        state.update(engine, elem.update.clone())?;
                        self.logical = state.first();
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Step past a page _boundary._
    pub fn step(&mut self) {
        self.physical = self.physical.saturating_add(1);
        self.logical += 1;
    }
}

impl Default for ManualPageCounter {
    fn default() -> Self {
        Self::new()
    }
}
