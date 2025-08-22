use std::cell::LazyCell;
use std::ops::Range;
use std::sync::{Arc, LazyLock};

use comemo::Tracked;
use ecow::{eco_format, EcoString, EcoVec};
use syntect::highlighting as synt;
use syntect::parsing::{SyntaxDefinition, SyntaxSet, SyntaxSetBuilder};
use typst_syntax::{split_newlines, LinkedNode, Span, Spanned};
use typst_utils::ManuallyHash;
use unicode_segmentation::UnicodeSegmentation;

use super::Lang;
use crate::diag::{At, FileError, SourceResult, StrResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, elem, scope, Bytes, Content, Derived, NativeElement, OneOrMultiple, Packed,
    PlainText, Show, ShowSet, Smart, StyleChain, Styles, Synthesize, TargetElem,
};
use crate::html::{tag, HtmlElem};
use crate::layout::{BlockBody, BlockElem, Em, HAlignment};
use crate::loading::{DataSource, Load};
use crate::model::{Figurable, ParElem};
use crate::text::{FontFamily, FontList, LinebreakElem, LocalName, TextElem, TextSize};
use crate::visualize::Color;
use crate::World;

/// オプションでシンタックスハイライトを持つ生テキスト。
///
/// テキストをそのまま等幅フォントで表示します。
/// これは通常、計算機のコードをドキュメント内に埋め込むために使います。
///
/// # 例
/// ````example
/// Adding `rbx` to `rcx` gives
/// the desired result.
///
/// What is ```rust fn main()``` in Rust
/// would be ```c int main()``` in C.
///
/// ```rust
/// fn main() {
///     println!("Hello World!");
/// }
/// ```
///
/// This has ``` `backticks` ``` in it
/// (but the spaces are trimmed). And
/// ``` here``` the leading space is
/// also trimmed.
/// ````
///
/// また、プログラミング的に文字列から[`raw`]要素を作成できます（オプションの[`lang`]($raw.lang)引数を用いて言語タグを提供できます）。
/// ```example
/// #raw("fn " + "main() {}", lang: "rust")
/// ```
///
/// # 構文
/// この関数には専用の構文もあります。
/// テキストを1つまたは3つ以上のバッククォート（`` ` ``）で囲むと生テキストにできます。
/// 2つのバッククォートは空の生テキストを作成します。
/// これはマークアップ中でもコード中でも動作します。
///
/// 3つ以上のバッククォートを使用する場合、シンタックスハイライトのために開きバッククォートの直後に言語タグを追加で指定することができます。
/// rawブロック内部では、（もし言語タグが利用可能なら、それ以外の）全てがそのままレンダリングされます。特に、エスケープシーケンスはありません。
///
/// 言語タグは、3つ以上のバッククォートがある場合にのみ使用できる、開きバッククォートの直後に付ける識別子です。
/// 識別子のようなものからテキストが始まるものの、シンタックスハイライトが不要な場合、単一の空白（トリムされます）からテキストを始めるか、単一のバッククォート構文を使用してください。
/// バッククォートでテキストが始まるか終わるかしなければならない場合は、その前か後に空白を置いてください（トリムされます）。
#[elem(
    scope,
    title = "Raw Text / Code",
    Synthesize,
    Show,
    ShowSet,
    LocalName,
    Figurable,
    PlainText
)]
pub struct RawElem {
    /// 生テキスト。
    ///
    /// 自動化のために、rawブロックを使ってカスタム構文をクリエイティブに作成することもできます。
    ///
    /// ````example
    /// // Parse numbers in raw blocks with the
    /// // `mydsl` tag and sum them up.
    /// #show raw.where(lang: "mydsl"): it => {
    ///   let sum = 0
    ///   for part in it.text.split("+") {
    ///     sum += int(part.trim())
    ///   }
    ///   sum
    /// }
    ///
    /// ```mydsl
    /// 1 + 2 + 3 + 4 + 5
    /// ```
    /// ````
    #[required]
    pub text: RawContent,

    /// 生テキストを独立したブロックとして表示するかどうか。
    ///
    /// マークアップモードでは、バッククォート1つを使うとこれは`{false}`になります。
    /// バッククォート3つ使うと、囲まれたコンテンツが1つ以上の改行を含む場合は`{true}`になります。
    ///
    /// ````example
    /// // Display inline code in a small box
    /// // that retains the correct baseline.
    /// #show raw.where(block: false): box.with(
    ///   fill: luma(240),
    ///   inset: (x: 3pt, y: 0pt),
    ///   outset: (y: 3pt),
    ///   radius: 2pt,
    /// )
    ///
    /// // Display block code in a larger block
    /// // with more padding.
    /// #show raw.where(block: true): block.with(
    ///   fill: luma(240),
    ///   inset: 10pt,
    ///   radius: 4pt,
    /// )
    ///
    /// With `rg`, you can search through your files quickly.
    /// This example searches the current directory recursively
    /// for the text `Hello World`:
    ///
    /// ```bash
    /// rg "Hello World"
    /// ```
    /// ````
    #[default(false)]
    pub block: bool,

    /// シンタックスハイライトを行う言語。
    ///
    /// Markdownで用いられている一般的な言語以外では、[Typst markup]($reference/syntax/#markup)の`{"typ"}`、[Typst code]($reference/syntax/#code)の`{"typc"}`、[Typst math]($reference/syntax/#math)の`{"typm"}`のタグがそれぞれサポートされています。
    ///
    /// ````example
    /// ```typ
    /// This is *Typst!*
    /// ```
    ///
    /// This is ```typ also *Typst*```, but inline!
    /// ````
    #[borrowed]
    pub lang: Option<EcoString>,

    /// rawブロック中の各行が持つべき水平方向の配置。
    /// このオプションはrawブロックではない（`block: false`が指定されるか、マークアップモードで1つのバッククォートが使用された）場合は無視されます。
    ///
    /// これはデフォルトでは`{start}`で、現在のコンテキストの配置によらず、生テキストがブロック内部の書き始めの位置揃えになることを意味します（例えば、内部テキストを中央揃えにせずにrawブロックを中央揃えにできます）。
    ///
    /// ````example
    /// #set raw(align: center)
    ///
    /// ```typc
    /// let f(x) = x
    /// code = "centered"
    /// ```
    /// ````
    #[default(HAlignment::Start)]
    pub align: HAlignment,

    /// 追加で読み込む構文定義。
    /// 構文定義は[`sublime-syntax`ファイル形式](https://www.sublimetext.com/docs/syntax.html)でなければなりません。
    ///
    /// 以下の値のいずれかを渡すことができます。
    ///
    /// - 与えられたパスから構文ファイルを読み込みためのパス文字列。
    /// パスに関する詳細は[パスのセクション]($syntax/#paths)を参照してください。
    /// - 構文をデコードするための生バイト列。
    /// - 各アイテムが上記のいずれかである配列。
    ///
    /// ````example
    /// #set raw(syntaxes: "SExpressions.sublime-syntax")
    ///
    /// ```sexp
    /// (defun factorial (x)
    ///   (if (zerop x)
    ///     ; with a comment
    ///     1
    ///     (* x (factorial (- x 1)))))
    /// ```
    /// ````
    #[parse(match args.named("syntaxes")? {
        Some(sources) => Some(RawSyntax::load(engine.world, sources)?),
        None => None,
    })]
    #[fold]
    pub syntaxes: Derived<OneOrMultiple<DataSource>, Vec<RawSyntax>>,

    /// シンタックスハイライトに用いるテーマ。
    /// テーマは[`tmTheme`ファイル形式](https://www.sublimetext.com/docs/color_schemes_tmtheme.html)でなければなりません。
    ///
    /// 以下の値のいずれかを渡すことができます。
    ///
    /// - `{none}`: シンタックスハイライトを無効化します。
    /// - `{auto}`: Typstのデフォルトテーマでハイライトします。
    /// - 与えられたパスからテーマファイルを読み込みためのパス文字列。
    /// パスに関する詳細は[パスのセクション]($syntax/#paths)を参照してください。
    /// - テーマをデコードするための生バイト列。
    ///
    /// テーマの適用はハイライトされたテキストの色にのみ影響を与えます。テーマの前景および背景プロパティは無視され、生テキストの色の制御は残ります。
    /// [`text`]関数を用いて前景色を、[ブロックの塗り潰し]($block.fill)を用いて背景色を、それぞれ手動で設定できます。
    /// [`xml`]関数でもこれらのプロパティをテーマから抽出できます。
    ///
    /// ````example
    /// #set raw(theme: "halcyon.tmTheme")
    /// #show raw: it => block(
    ///   fill: rgb("#1d2433"),
    ///   inset: 8pt,
    ///   radius: 5pt,
    ///   text(fill: rgb("#a2aabc"), it)
    /// )
    ///
    /// ```typ
    /// = Chapter 1
    /// #let hi = "Hello World"
    /// ```
    /// ````
    #[parse(match args.named::<Spanned<Smart<Option<DataSource>>>>("theme")? {
        Some(Spanned { v: Smart::Custom(Some(source)), span }) => Some(Smart::Custom(
            Some(RawTheme::load(engine.world, Spanned::new(source, span))?)
        )),
        Some(Spanned { v: Smart::Custom(None), .. }) => Some(Smart::Custom(None)),
        Some(Spanned { v: Smart::Auto, .. }) => Some(Smart::Auto),
        None => None,
    })]
    #[borrowed]
    pub theme: Smart<Option<Derived<DataSource, RawTheme>>>,

    /// スペースで測ったタブ幅。
    /// タブは、次のタブ幅の整数倍位置までのスペースで置き換えられます。
    ///
    /// ````example
    /// #set raw(tab-size: 8)
    /// ```tsv
    /// Year	Month	Day
    /// 2000	2	3
    /// 2001	2	1
    /// 2002	3	10
    /// ```
    /// ````
    #[default(2)]
    pub tab_size: usize,

    /// The stylized lines of raw text.
    ///
    /// Made accessible for the [`raw.line` element]($raw.line).
    /// Allows more styling control in `show` rules.
    #[synthesized]
    pub lines: Vec<Packed<RawLine>>,
}

#[scope]
impl RawElem {
    #[elem]
    type RawLine;
}

impl RawElem {
    /// The supported language names and tags.
    pub fn languages() -> Vec<(&'static str, Vec<&'static str>)> {
        RAW_SYNTAXES
            .syntaxes()
            .iter()
            .map(|syntax| {
                (
                    syntax.name.as_str(),
                    syntax.file_extensions.iter().map(|s| s.as_str()).collect(),
                )
            })
            .chain([
                ("Typst", vec!["typ"]),
                ("Typst (code)", vec!["typc"]),
                ("Typst (math)", vec!["typm"]),
            ])
            .collect()
    }
}

impl Synthesize for Packed<RawElem> {
    fn synthesize(&mut self, _: &mut Engine, styles: StyleChain) -> SourceResult<()> {
        let seq = self.highlight(styles);
        self.push_lines(seq);
        Ok(())
    }
}

impl Packed<RawElem> {
    #[comemo::memoize]
    fn highlight(&self, styles: StyleChain) -> Vec<Packed<RawLine>> {
        let elem = self.as_ref();
        let lines = preprocess(&elem.text, styles, self.span());

        let count = lines.len() as i64;
        let lang = elem
            .lang(styles)
            .as_ref()
            .as_ref()
            .map(|s| s.to_lowercase())
            .or(Some("txt".into()));

        let non_highlighted_result = |lines: EcoVec<(EcoString, Span)>| {
            lines.into_iter().enumerate().map(|(i, (line, line_span))| {
                Packed::new(RawLine::new(
                    i as i64 + 1,
                    count,
                    line.clone(),
                    TextElem::packed(line).spanned(line_span),
                ))
                .spanned(line_span)
            })
        };

        let syntaxes = LazyCell::new(|| elem.syntaxes(styles));
        let theme: &synt::Theme = match elem.theme(styles) {
            Smart::Auto => &RAW_THEME,
            Smart::Custom(Some(theme)) => theme.derived.get(),
            Smart::Custom(None) => return non_highlighted_result(lines).collect(),
        };

        let foreground = theme.settings.foreground.unwrap_or(synt::Color::BLACK);

        let mut seq = vec![];
        if matches!(lang.as_deref(), Some("typ" | "typst" | "typc" | "typm")) {
            let text =
                lines.iter().map(|(s, _)| s.clone()).collect::<Vec<_>>().join("\n");
            let root = match lang.as_deref() {
                Some("typc") => typst_syntax::parse_code(&text),
                Some("typm") => typst_syntax::parse_math(&text),
                _ => typst_syntax::parse(&text),
            };

            ThemedHighlighter::new(
                &text,
                LinkedNode::new(&root),
                synt::Highlighter::new(theme),
                &mut |i, _, range, style| {
                    // Find span and start of line.
                    // Note: Dedent is already applied to the text
                    let span = lines.get(i).map_or_else(Span::detached, |l| l.1);
                    let span_offset = text[..range.start]
                        .rfind('\n')
                        .map_or(0, |i| range.start - (i + 1));
                    styled(&text[range], foreground, style, span, span_offset)
                },
                &mut |i, range, line| {
                    let span = lines.get(i).map_or_else(Span::detached, |l| l.1);
                    seq.push(
                        Packed::new(RawLine::new(
                            (i + 1) as i64,
                            count,
                            EcoString::from(&text[range]),
                            Content::sequence(line.drain(..)),
                        ))
                        .spanned(span),
                    );
                },
            )
            .highlight();
        } else if let Some((syntax_set, syntax)) = lang.and_then(|token| {
            // Prefer user-provided syntaxes over built-in ones.
            syntaxes
                .derived
                .iter()
                .map(|syntax| syntax.get())
                .chain(std::iter::once(&*RAW_SYNTAXES))
                .find_map(|set| {
                    set.find_syntax_by_token(&token).map(|syntax| (set, syntax))
                })
        }) {
            let mut highlighter = syntect::easy::HighlightLines::new(syntax, theme);
            for (i, (line, line_span)) in lines.into_iter().enumerate() {
                let mut line_content = vec![];
                let mut span_offset = 0;
                for (style, piece) in highlighter
                    .highlight_line(line.as_str(), syntax_set)
                    .into_iter()
                    .flatten()
                {
                    line_content.push(styled(
                        piece,
                        foreground,
                        style,
                        line_span,
                        span_offset,
                    ));
                    span_offset += piece.len();
                }

                seq.push(
                    Packed::new(RawLine::new(
                        i as i64 + 1,
                        count,
                        line,
                        Content::sequence(line_content),
                    ))
                    .spanned(line_span),
                );
            }
        } else {
            seq.extend(non_highlighted_result(lines));
        };

        seq
    }
}

impl Show for Packed<RawElem> {
    #[typst_macros::time(name = "raw", span = self.span())]
    fn show(&self, _: &mut Engine, styles: StyleChain) -> SourceResult<Content> {
        let lines = self.lines().map(|v| v.as_slice()).unwrap_or_default();

        let mut seq = EcoVec::with_capacity((2 * lines.len()).saturating_sub(1));
        for (i, line) in lines.iter().enumerate() {
            if i != 0 {
                seq.push(LinebreakElem::shared().clone());
            }

            seq.push(line.clone().pack());
        }

        let mut realized = Content::sequence(seq);

        if TargetElem::target_in(styles).is_html() {
            return Ok(HtmlElem::new(if self.block(styles) {
                tag::pre
            } else {
                tag::code
            })
            .with_body(Some(realized))
            .pack()
            .spanned(self.span()));
        }

        if self.block(styles) {
            // Align the text before inserting it into the block.
            realized = realized.aligned(self.align(styles).into());
            realized = BlockElem::new()
                .with_body(Some(BlockBody::Content(realized)))
                .pack()
                .spanned(self.span());
        }

        Ok(realized)
    }
}

impl ShowSet for Packed<RawElem> {
    fn show_set(&self, styles: StyleChain) -> Styles {
        let mut out = Styles::new();
        out.set(TextElem::set_overhang(false));
        out.set(TextElem::set_lang(Lang::ENGLISH));
        out.set(TextElem::set_hyphenate(Smart::Custom(false)));
        out.set(TextElem::set_size(TextSize(Em::new(0.8).into())));
        out.set(TextElem::set_font(FontList(vec![FontFamily::new("DejaVu Sans Mono")])));
        out.set(TextElem::set_cjk_latin_spacing(Smart::Custom(None)));
        if self.block(styles) {
            out.set(ParElem::set_justify(false));
        }
        out
    }
}

impl LocalName for Packed<RawElem> {
    const KEY: &'static str = "raw";
}

impl Figurable for Packed<RawElem> {}

impl PlainText for Packed<RawElem> {
    fn plain_text(&self, text: &mut EcoString) {
        text.push_str(&self.text.get());
    }
}

/// The content of the raw text.
#[derive(Debug, Clone, Hash, PartialEq)]
pub enum RawContent {
    /// From a string.
    Text(EcoString),
    /// From lines of text.
    Lines(EcoVec<(EcoString, Span)>),
}

impl RawContent {
    /// Returns or synthesizes the text content of the raw text.
    fn get(&self) -> EcoString {
        match self.clone() {
            RawContent::Text(text) => text,
            RawContent::Lines(lines) => {
                let mut lines = lines.into_iter().map(|(s, _)| s);
                if lines.len() <= 1 {
                    lines.next().unwrap_or_default()
                } else {
                    lines.collect::<Vec<_>>().join("\n").into()
                }
            }
        }
    }
}

cast! {
    RawContent,
    self => self.get().into_value(),
    v: EcoString => Self::Text(v),
}

/// A loaded syntax.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct RawSyntax(Arc<ManuallyHash<SyntaxSet>>);

impl RawSyntax {
    /// Load syntaxes from sources.
    fn load(
        world: Tracked<dyn World + '_>,
        sources: Spanned<OneOrMultiple<DataSource>>,
    ) -> SourceResult<Derived<OneOrMultiple<DataSource>, Vec<RawSyntax>>> {
        let data = sources.load(world)?;
        let list = sources
            .v
            .0
            .iter()
            .zip(&data)
            .map(|(source, data)| Self::decode(source, data))
            .collect::<StrResult<_>>()
            .at(sources.span)?;
        Ok(Derived::new(sources.v, list))
    }

    /// Decode a syntax from a loaded source.
    #[comemo::memoize]
    #[typst_macros::time(name = "load syntaxes")]
    fn decode(source: &DataSource, data: &Bytes) -> StrResult<RawSyntax> {
        let src = data.as_str().map_err(FileError::from)?;
        let syntax = SyntaxDefinition::load_from_str(src, false, None).map_err(
            |err| match source {
                DataSource::Path(path) => {
                    eco_format!("failed to parse syntax file `{path}` ({err})")
                }
                DataSource::Bytes(_) => {
                    eco_format!("failed to parse syntax ({err})")
                }
            },
        )?;

        let mut builder = SyntaxSetBuilder::new();
        builder.add(syntax);

        Ok(RawSyntax(Arc::new(ManuallyHash::new(
            builder.build(),
            typst_utils::hash128(data),
        ))))
    }

    /// Return the underlying syntax set.
    fn get(&self) -> &SyntaxSet {
        self.0.as_ref()
    }
}

/// A loaded syntect theme.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct RawTheme(Arc<ManuallyHash<synt::Theme>>);

impl RawTheme {
    /// Load a theme from a data source.
    fn load(
        world: Tracked<dyn World + '_>,
        source: Spanned<DataSource>,
    ) -> SourceResult<Derived<DataSource, Self>> {
        let data = source.load(world)?;
        let theme = Self::decode(&data).at(source.span)?;
        Ok(Derived::new(source.v, theme))
    }

    /// Decode a theme from bytes.
    #[comemo::memoize]
    fn decode(data: &Bytes) -> StrResult<RawTheme> {
        let mut cursor = std::io::Cursor::new(data.as_slice());
        let theme = synt::ThemeSet::load_from_reader(&mut cursor)
            .map_err(|err| eco_format!("failed to parse theme ({err})"))?;
        Ok(RawTheme(Arc::new(ManuallyHash::new(theme, typst_utils::hash128(data)))))
    }

    /// Get the underlying syntect theme.
    pub fn get(&self) -> &synt::Theme {
        self.0.as_ref()
    }
}

/// ハイライトされた生テキストの行。
///
/// これは[`raw`]要素によって合成される補助要素です。
///
/// 行番号、ハイライトされていない生テキスト、ハイライトされたテキスト、rawブロックの最初の行や最後の行であるかどうかなどの、行のさまざまなプロパティにアクセスすることができます。
#[elem(name = "line", title = "Raw Text / Code Line", Show, PlainText)]
pub struct RawLine {
    /// 1始まりのrawブロック内の行番号。
    #[required]
    pub number: i64,

    /// rawブロック内にある総行数。
    #[required]
    pub count: i64,

    /// 生テキストの行。
    #[required]
    pub text: EcoString,

    /// ハイライトされた生テキスト。
    #[required]
    pub body: Content,
}

impl Show for Packed<RawLine> {
    #[typst_macros::time(name = "raw.line", span = self.span())]
    fn show(&self, _: &mut Engine, _styles: StyleChain) -> SourceResult<Content> {
        Ok(self.body.clone())
    }
}

impl PlainText for Packed<RawLine> {
    fn plain_text(&self, text: &mut EcoString) {
        text.push_str(&self.text);
    }
}

/// Wrapper struct for the state required to highlight typst code.
struct ThemedHighlighter<'a> {
    /// The code being highlighted.
    code: &'a str,
    /// The current node being highlighted.
    node: LinkedNode<'a>,
    /// The highlighter.
    highlighter: synt::Highlighter<'a>,
    /// The current scopes.
    scopes: Vec<syntect::parsing::Scope>,
    /// The current highlighted line.
    current_line: Vec<Content>,
    /// The range of the current line.
    range: Range<usize>,
    /// The current line number.
    line: usize,
    /// The function to style a piece of text.
    style_fn: StyleFn<'a>,
    /// The function to append a line.
    line_fn: LineFn<'a>,
}

// Shorthands for highlighter closures.
type StyleFn<'a> =
    &'a mut dyn FnMut(usize, &LinkedNode, Range<usize>, synt::Style) -> Content;
type LineFn<'a> = &'a mut dyn FnMut(usize, Range<usize>, &mut Vec<Content>);

impl<'a> ThemedHighlighter<'a> {
    pub fn new(
        code: &'a str,
        top: LinkedNode<'a>,
        highlighter: synt::Highlighter<'a>,
        style_fn: StyleFn<'a>,
        line_fn: LineFn<'a>,
    ) -> Self {
        Self {
            code,
            node: top,
            highlighter,
            range: 0..0,
            scopes: Vec::new(),
            current_line: Vec::new(),
            line: 0,
            style_fn,
            line_fn,
        }
    }

    pub fn highlight(&mut self) {
        self.highlight_inner();

        if !self.current_line.is_empty() {
            (self.line_fn)(
                self.line,
                self.range.start..self.code.len(),
                &mut self.current_line,
            );

            self.current_line.clear();
        }
    }

    fn highlight_inner(&mut self) {
        if self.node.children().len() == 0 {
            let style = self.highlighter.style_for_stack(&self.scopes);
            let segment = &self.code[self.node.range()];

            let mut len = 0;
            for (i, line) in split_newlines(segment).into_iter().enumerate() {
                if i != 0 {
                    (self.line_fn)(
                        self.line,
                        self.range.start..self.range.end + len - 1,
                        &mut self.current_line,
                    );
                    self.range.start = self.range.end + len;
                    self.line += 1;
                }

                let offset = self.node.range().start + len;
                let token_range = offset..(offset + line.len());
                self.current_line.push((self.style_fn)(
                    self.line,
                    &self.node,
                    token_range,
                    style,
                ));

                len += line.len() + 1;
            }

            self.range.end += segment.len();
        }

        for child in self.node.children() {
            let mut scopes = self.scopes.clone();
            if let Some(tag) = typst_syntax::highlight(&child) {
                scopes.push(syntect::parsing::Scope::new(tag.tm_scope()).unwrap())
            }

            std::mem::swap(&mut scopes, &mut self.scopes);
            self.node = child;
            self.highlight_inner();
            std::mem::swap(&mut scopes, &mut self.scopes);
        }
    }
}

fn preprocess(
    text: &RawContent,
    styles: StyleChain,
    span: Span,
) -> EcoVec<(EcoString, Span)> {
    if let RawContent::Lines(lines) = text {
        if lines.iter().all(|(s, _)| !s.contains('\t')) {
            return lines.clone();
        }
    }

    let mut text = text.get();
    if text.contains('\t') {
        let tab_size = RawElem::tab_size_in(styles);
        text = align_tabs(&text, tab_size);
    }
    split_newlines(&text)
        .into_iter()
        .map(|line| (line.into(), span))
        .collect()
}

/// Style a piece of text with a syntect style.
fn styled(
    piece: &str,
    foreground: synt::Color,
    style: synt::Style,
    span: Span,
    span_offset: usize,
) -> Content {
    let mut body = TextElem::packed(piece).spanned(span);

    if span_offset > 0 {
        body = body.styled(TextElem::set_span_offset(span_offset));
    }

    if style.foreground != foreground {
        body = body.styled(TextElem::set_fill(to_typst(style.foreground).into()));
    }

    if style.font_style.contains(synt::FontStyle::BOLD) {
        body = body.strong().spanned(span);
    }

    if style.font_style.contains(synt::FontStyle::ITALIC) {
        body = body.emph().spanned(span);
    }

    if style.font_style.contains(synt::FontStyle::UNDERLINE) {
        body = body.underlined().spanned(span);
    }

    body
}

fn to_typst(synt::Color { r, g, b, a }: synt::Color) -> Color {
    Color::from_u8(r, g, b, a)
}

fn to_syn(color: Color) -> synt::Color {
    let [r, g, b, a] = color.to_rgb().to_vec4_u8();
    synt::Color { r, g, b, a }
}

/// Create a syntect theme item.
fn item(
    scope: &str,
    color: Option<&str>,
    font_style: Option<synt::FontStyle>,
) -> synt::ThemeItem {
    synt::ThemeItem {
        scope: scope.parse().unwrap(),
        style: synt::StyleModifier {
            foreground: color.map(|s| to_syn(s.parse::<Color>().unwrap())),
            background: None,
            font_style,
        },
    }
}

/// Replace tabs with spaces to align with multiples of `tab_size`.
fn align_tabs(text: &str, tab_size: usize) -> EcoString {
    let replacement = " ".repeat(tab_size);
    let divisor = tab_size.max(1);
    let amount = text.chars().filter(|&c| c == '\t').count();

    let mut res = EcoString::with_capacity(text.len() - amount + amount * tab_size);
    let mut column = 0;

    for grapheme in text.graphemes(true) {
        match grapheme {
            "\t" => {
                let required = tab_size - column % divisor;
                res.push_str(&replacement[..required]);
                column += required;
            }
            "\n" => {
                res.push_str(grapheme);
                column = 0;
            }
            _ => {
                res.push_str(grapheme);
                column += 1;
            }
        }
    }

    res
}

/// The syntect syntax definitions.
///
/// Syntax set is generated from the syntaxes from the `bat` project
/// <https://github.com/sharkdp/bat/tree/master/assets/syntaxes>
pub static RAW_SYNTAXES: LazyLock<syntect::parsing::SyntaxSet> =
    LazyLock::new(two_face::syntax::extra_no_newlines);

/// The default theme used for syntax highlighting.
pub static RAW_THEME: LazyLock<synt::Theme> = LazyLock::new(|| synt::Theme {
    name: Some("Typst Light".into()),
    author: Some("The Typst Project Developers".into()),
    settings: synt::ThemeSettings::default(),
    scopes: vec![
        item("comment", Some("#8a8a8a"), None),
        item("constant.character.escape", Some("#1d6c76"), None),
        item("markup.bold", None, Some(synt::FontStyle::BOLD)),
        item("markup.italic", None, Some(synt::FontStyle::ITALIC)),
        item("markup.underline", None, Some(synt::FontStyle::UNDERLINE)),
        item("markup.raw", Some("#818181"), None),
        item("string.other.math.typst", None, None),
        item("punctuation.definition.math", Some("#298e0d"), None),
        item("keyword.operator.math", Some("#1d6c76"), None),
        item("markup.heading, entity.name.section", None, Some(synt::FontStyle::BOLD)),
        item(
            "markup.heading.typst",
            None,
            Some(synt::FontStyle::BOLD | synt::FontStyle::UNDERLINE),
        ),
        item("punctuation.definition.list", Some("#8b41b1"), None),
        item("markup.list.term", None, Some(synt::FontStyle::BOLD)),
        item("entity.name.label, markup.other.reference", Some("#1d6c76"), None),
        item("keyword, constant.language, variable.language", Some("#d73a49"), None),
        item("storage.type, storage.modifier", Some("#d73a49"), None),
        item("constant", Some("#b60157"), None),
        item("string", Some("#298e0d"), None),
        item("entity.name, variable.function, support", Some("#4b69c6"), None),
        item("support.macro", Some("#16718d"), None),
        item("meta.annotation", Some("#301414"), None),
        item("entity.other, meta.interpolation", Some("#8b41b1"), None),
        item("meta.diff.range", Some("#8b41b1"), None),
        item("markup.inserted, meta.diff.header.to-file", Some("#298e0d"), None),
        item("markup.deleted, meta.diff.header.from-file", Some("#d73a49"), None),
    ],
});
