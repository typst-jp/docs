use std::ops::Deref;
use std::str::FromStr;

use comemo::Tracked;
use ecow::{EcoString, eco_format};

use crate::diag::{SourceResult, StrResult, bail};
use crate::engine::Engine;
use crate::foundations::{
    Args, Construct, Content, Label, Packed, Repr, Selector, ShowSet, Smart,
    StyleChain, Styles, cast, elem,
};
use crate::introspection::{
    Counter, CounterKey, Introspector, Locatable, Location, Tagged,
};
use crate::layout::{PageElem, Position};
use crate::model::{NumberingPattern, Refable};
use crate::text::{LocalName, TextElem};

/// URLや文書中の位置へのリンク。
///
/// デフォルトでは、リンクの外見は通常のテキストと変わりません。
/// しかし、showルールを使うことで、簡単に任意のスタイルを適用できます。
///
/// # 例
/// ```example
/// #show link: underline
///
/// https://example.com \
///
/// #link("https://example.com") \
/// #link("https://example.com")[
///   See example.com
/// ]
/// ```
///
/// # 構文
/// この関数には専用の構文もあります。
/// `http://`や`https://`で始まるテキストは、自動的にリンクに変換されます。
///
/// # ハイフネーション
/// ハイフネーションや両端揃えを有効にしていても、意図しないURL中のハイフネーションを防ぐため、
/// デフォルトではリンクには適用されません。
/// これを無効化するには、`{show link: set text(hyphenate: true)}`を使用します。
///
/// # アクセシビリティ
/// リンクの遷移先は、リンクテキスト自体、または少なくともその前後のテキストから明確であるべきです。
/// PDFエクスポートでは、Typstはリンクの遷移先に基づいてツールチップの説明を自動的に生成します。
/// URLへのリンクの場合は、URL自体がツールチップとして使われます。
///
/// # HTMLエクスポートにおけるリンク
/// HTMLエクスポートでは、[ラベル]($label)や[ロケーション]($location)へのリンクは、フラグメント識別子を用いたアンカーリンクに変換されます。
/// これに対応するため、既存のIDを持たないターゲットはDOM上で自動的にIDを受け取ります。
/// この仕組みは、リンクターゲットが変換されるHTMLノードの種類によって異なります。
///
/// - リンクターゲットが単一のHTML要素に変換される場合、その要素がIDを受け取ります。
///   例としては、（単一の`<h2>`要素に変換される）トップレベルの見出しにリンクする場合が典型的です。
///
/// - リンクターゲットが単一のテキストノードに変換される場合、そのノードは`<span>`でラップされ、その`<span>`がIDを受け取ります。
///
/// - リンクターゲットが複数のノードに変換される場合、最初のノードがIDを受け取ります。
///
/// - リンクターゲットが全くノードに変換されない場合、リンクターゲットとして機能する空の`<span>`が生成されます。
///
/// 特定のDOM構造に依存する場合は、コンパイラーがテキストのテキストノードへの正確な分割を保証しないため、リンクターゲットが1つまたは複数の要素に変換されることを保証する必要があります。
///
/// 自動的なID生成では、リンクターゲットのラベルが存在する場合、人間が読めるIDを作成するためにラベルの再利用を試みます。
/// ラベルは以下の条件を満たす場合に再利用できます。
///
/// - 全ての文字が、Unicode上で英字もしくは数字に分類されるか、あるいはハイフンかアンダースコアであること。
///
/// - ラベルが数字またはハイフンで始まらないこと。
///
/// これらのルールは、ラベルがCSS識別子としても、リンク用のURLフラグメントとしても有効であることを保証します。
///
/// IDはDOM内で一意でなければならないため、重複するラベルをIDとして再利用する際には、それらを区別する必要があるかもしれません。
/// 正確なルールは以下の通りです。
///
/// - ラベルが再利用可能で文書内で一意であれば、そのままIDとして使用されます。
///
/// - ラベルが再利用可能だが一意ではない場合、ハイフンと整数からなるサフィックスが追加されます。
///   例えば、`<mylabel>`というラベルが2つ存在すると、`mylabel-1`と`mylabel-2`に変換されます。
///
/// - それ以外の場合は、`loc-`に続けて整数を付けた一意なIDが生成されます。
#[elem(Locatable)]
pub struct LinkElem {
    /// リンクの遷移先。
    ///
    /// - Webページにリンクする場合、`dest`は有効なURL文字列である必要があります。
    ///   `mailto:`や`tel:`のURLスキームを含むURLが指定され、
    ///   かつ`body`パラメーターが省略された場合、
    ///   URLスキームを除いたメールアドレスまたは電話番号がリンクの本文になります。
    ///
    /// - 文書中の別の部分にリンクする場合、`dest`には次の3つのうちいずれかの形式を使用できます。
    ///   - 要素に付与された[ラベル]($label)。
    ///     要素に基づいて自動的にリンクの本文を生成したい場合は、
    ///     [参照]($ref)の使用を検討してください。
    ///
    ///   - [`location`]（通常は[`here`]や[`locate`]、
    ///     [`query`]から取得される）。
    ///
    ///   - [整数]($int)型の`page`キーと[`length`]型の`x`座標、`y`座標を持つ辞書。
    ///     ページ番号は1から始まり、
    ///     座標はページの左上隅からの相対位置です。
    ///
    /// ```example
    /// = Introduction <intro>
    /// #link("mailto:hello@typst.app") \
    /// #link(<intro>)[Go to intro] \
    /// #link((page: 1, x: 0pt, y: 0pt))[
    ///   Go to top
    /// ]
    /// ```
    #[required]
    #[parse(
        let dest = args.expect::<LinkTarget>("destination")?;
        dest.clone()
    )]
    pub dest: LinkTarget,

    /// リンクとして表示するコンテンツ。
    ///
    /// `dest`がURL文字列の場合、このパラメーターは省略可能です。
    /// この場合、URLがリンクとして表示されます。
    #[required]
    #[parse(match &dest {
        LinkTarget::Dest(Destination::Url(url)) => match args.eat()? {
            Some(body) => body,
            None => body_from_url(url),
        },
        _ => args.expect("body")?,
    })]
    pub body: Content,

    /// A destination style that should be applied to elements.
    #[internal]
    #[ghost]
    pub current: Option<Destination>,
}

impl LinkElem {
    /// Create a link element from a URL with its bare text.
    pub fn from_url(url: Url) -> Self {
        let body = body_from_url(&url);
        Self::new(LinkTarget::Dest(Destination::Url(url)), body)
    }
}

impl ShowSet for Packed<LinkElem> {
    fn show_set(&self, _: StyleChain) -> Styles {
        let mut out = Styles::new();
        out.set(TextElem::hyphenate, Smart::Custom(false));
        out
    }
}

pub(crate) fn body_from_url(url: &Url) -> Content {
    let stripped = url.strip_contact_scheme().map(|(_, s)| s.into());
    TextElem::packed(stripped.unwrap_or_else(|| url.clone().into_inner()))
}

/// A target where a link can go.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum LinkTarget {
    Dest(Destination),
    Label(Label),
}

impl LinkTarget {
    /// Resolves the destination.
    pub fn resolve(&self, introspector: Tracked<Introspector>) -> StrResult<Destination> {
        Ok(match self {
            LinkTarget::Dest(dest) => dest.clone(),
            LinkTarget::Label(label) => {
                let elem = introspector.query_label(*label)?;
                Destination::Location(elem.location().unwrap())
            }
        })
    }
}

cast! {
    LinkTarget,
    self => match self {
        Self::Dest(v) => v.into_value(),
        Self::Label(v) => v.into_value(),
    },
    v: Destination => Self::Dest(v),
    v: Label => Self::Label(v),
}

impl From<Destination> for LinkTarget {
    fn from(dest: Destination) -> Self {
        Self::Dest(dest)
    }
}

/// A link destination.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Destination {
    /// A link to a URL.
    Url(Url),
    /// A link to a point on a page.
    Position(Position),
    /// An unresolved link to a location in the document.
    Location(Location),
}

impl Destination {
    pub fn alt_text(
        &self,
        engine: &mut Engine,
        styles: StyleChain,
    ) -> SourceResult<EcoString> {
        match self {
            Destination::Url(url) => {
                let contact = url.strip_contact_scheme().map(|(scheme, stripped)| {
                    eco_format!("{} {stripped}", scheme.local_name_in(styles))
                });
                Ok(contact.unwrap_or_else(|| url.clone().into_inner()))
            }
            Destination::Position(pos) => {
                let page_nr = eco_format!("{}", pos.page.get());
                let page_str = PageElem::local_name_in(styles);
                Ok(eco_format!("{page_str} {page_nr}"))
            }
            &Destination::Location(loc) => {
                let fallback = |engine: &mut Engine| {
                    // Fall back to a generating a page reference.
                    let numbering = loc.page_numbering(engine).unwrap_or_else(|| {
                        NumberingPattern::from_str("1").unwrap().into()
                    });
                    let page_nr = Counter::new(CounterKey::Page)
                        .display_at_loc(engine, loc, styles, &numbering)?
                        .plain_text();
                    let page_str = PageElem::local_name_in(styles);
                    Ok(eco_format!("{page_str} {page_nr}"))
                };

                // Try to generate more meaningful alt text if the location is a
                // refable element.
                let loc_selector = Selector::Location(loc);
                if let Some(elem) = engine.introspector.query_first(&loc_selector)
                    && let Some(refable) = elem.with::<dyn Refable>()
                {
                    let counter = refable.counter();
                    let supplement = refable.supplement().plain_text();

                    if let Some(numbering) = refable.numbering() {
                        let numbers = counter.display_at_loc(
                            engine,
                            loc,
                            styles,
                            &numbering.clone().trimmed(),
                        )?;
                        return Ok(eco_format!("{supplement} {}", numbers.plain_text()));
                    } else {
                        let page_ref = fallback(engine)?;
                        return Ok(eco_format!("{supplement}, {page_ref}"));
                    }
                }

                fallback(engine)
            }
        }
    }
}

impl Repr for Destination {
    fn repr(&self) -> EcoString {
        eco_format!("{self:?}")
    }
}

cast! {
    Destination,
    self => match self {
        Self::Url(v) => v.into_value(),
        Self::Position(v) => v.into_value(),
        Self::Location(v) => v.into_value(),
    },
    v: Url => Self::Url(v),
    v: Position => Self::Position(v),
    v: Location => Self::Location(v),
}

/// A uniform resource locator with a maximum length.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Url(EcoString);

impl Url {
    /// Create a URL from a string, checking the maximum length.
    pub fn new(url: impl Into<EcoString>) -> StrResult<Self> {
        let url = url.into();
        if url.len() > 8000 {
            bail!("URL is too long")
        } else if url.is_empty() {
            bail!("URL must not be empty")
        }
        Ok(Self(url))
    }

    /// Extract the underlying [`EcoString`].
    pub fn into_inner(self) -> EcoString {
        self.0
    }

    pub fn strip_contact_scheme(&self) -> Option<(UrlContactScheme, &str)> {
        [UrlContactScheme::Mailto, UrlContactScheme::Tel]
            .into_iter()
            .find_map(|scheme| {
                let stripped = self.strip_prefix(scheme.as_str())?;
                Some((scheme, stripped))
            })
    }
}

impl Deref for Url {
    type Target = EcoString;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

cast! {
    Url,
    self => self.0.into_value(),
    v: EcoString => Self::new(v)?,
}

/// This is a temporary hack to dispatch to
/// - a raw link that does not go through `LinkElem` in paged
/// - `LinkElem` in HTML (there is no equivalent to a direct link)
///
/// We'll want to dispatch all kinds of links to `LinkElem` in the future, but
/// this is a visually breaking change in paged export as e.g.
/// `show link: underline` will suddenly also affect references, bibliography
/// back references, footnote references, etc. We'll want to do this change
/// carefully and in a way where we provide a good way to keep styling only URL
/// links, which is a bit too complicated to achieve right now for such a basic
/// requirement.
#[elem(Construct)]
pub struct DirectLinkElem {
    #[required]
    #[internal]
    pub loc: Location,
    #[required]
    #[internal]
    pub body: Content,
    #[required]
    #[internal]
    pub alt: Option<EcoString>,
}

impl Construct for DirectLinkElem {
    fn construct(_: &mut Engine, args: &mut Args) -> SourceResult<Content> {
        bail!(args.span, "cannot be constructed manually");
    }
}

/// An element that wraps all content that is [`Content::linked`] to a
/// destination.
#[elem(Tagged, Construct)]
pub struct LinkMarker {
    /// The content.
    #[internal]
    #[required]
    pub body: Content,
    #[internal]
    #[required]
    pub alt: Option<EcoString>,
}

impl Construct for LinkMarker {
    fn construct(_: &mut Engine, args: &mut Args) -> SourceResult<Content> {
        bail!(args.span, "cannot be constructed manually");
    }
}

#[derive(Copy, Clone)]
pub enum UrlContactScheme {
    /// The `mailto:` prefix.
    Mailto,
    /// The `tel:` prefix.
    Tel,
}

impl UrlContactScheme {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Mailto => "mailto:",
            Self::Tel => "tel:",
        }
    }

    pub fn local_name_in(self, styles: StyleChain) -> &'static str {
        match self {
            UrlContactScheme::Mailto => Email::local_name_in(styles),
            UrlContactScheme::Tel => Telephone::local_name_in(styles),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Email;
impl LocalName for Email {
    const KEY: &'static str = "email";
}

#[derive(Copy, Clone)]
pub struct Telephone;
impl LocalName for Telephone {
    const KEY: &'static str = "telephone";
}
