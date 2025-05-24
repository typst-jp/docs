use crate::diag::bail;
use crate::foundations::{cast, elem, func, Content, NativeElement, SymbolElem};
use crate::layout::{Length, Rel};
use crate::math::Mathy;

/// 対象の要素にアクセント記号を付ける。
///
/// # 例
/// ```example
/// $grave(a) = accent(a, `)$ \
/// $arrow(a) = accent(a, arrow)$ \
/// $tilde(a) = accent(a, \u{0303})$
/// ```
#[elem(Mathy)]
pub struct AccentElem {
    /// アクセント記号が適用される対象の要素。
    /// 複数の文字から構成される場合もあります。
    ///
    /// ```example
    /// $arrow(A B C)$
    /// ```
    #[required]
    pub base: Content,

    /// 対象の要素に適用するアクセント記号。
    ///
    /// サポートされているアクセント記号には以下のものがあります。
    ///
    /// | アクセント記号           | 名称                  | コードポイント                  |
    /// | ------------------------ | --------------------- | ------------------------------- |
    /// | グレイブ                 | `grave`               | <code>&DiacriticalGrave;</code> |
    /// | アキュート               | `acute`               | `´`                             |
    /// | サーカムフレックス       | `hat`                 | `^`                             |
    /// | チルダ                   | `tilde`               | `~`                             |
    /// | マクロン                 | `macron`              | `¯`                             |
    /// | ダッシュ                 | `dash`                | `‾`                             |
    /// | ブレーヴェ               | `breve`               | `˘`                             |
    /// | ドット                   | `dot`                 | `.`                             |
    /// | 2点ドット、 ダイエリシス | `dot.double`, `diaer` | `¨`                             |
    /// | 3点ドット                | `dot.triple`          | <code>&tdot;</code>             |
    /// | 4点ドット                | `dot.quad`            | <code>&DotDot;</code>           |
    /// | 丸                       | `circle`              | `∘`                             |
    /// | ダブルアキュート         | `acute.double`        | `˝`                             |
    /// | キャロン                 | `caron`               | `ˇ`                             |
    /// | 右向き矢印               | `arrow`, `->`         | `→`                             |
    /// | 左向き矢印               | `arrow.l`, `<-`       | `←`                             |
    /// | 左右矢印                 | `arrow.l.r`           | `↔`                             |
    /// | 右向きハープーン         | `harpoon`             | `⇀`                             |
    /// | 左向きハープーン         | `harpoon.lt`          | `↼`                             |
    #[required]
    pub accent: Accent,

    /// 対象の要素の幅に対するアクセント記号の相対的な大きさ。
    #[resolve]
    #[default(Rel::one())]
    pub size: Rel<Length>,
}

/// An accent character.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Accent(pub char);

impl Accent {
    /// Normalize a character into an accent.
    pub fn new(c: char) -> Self {
        Self(Self::combine(c).unwrap_or(c))
    }
}

/// This macro generates accent-related functions.
///
/// ```ignore
/// accents! {
///     '\u{0300}' | '`' => grave,
/// //  ^^^^^^^^^    ^^^    ^^^^^
/// //  |            |      |
/// //  |            |      +-- The name of the function.
/// //  |            +--------- The alternative characters that represent the accent.
/// //  +---------------------- The primary character that represents the accent.
/// }
/// ```
///
/// When combined with the `Accent::combine` function, accent characters can be normalized
/// to the primary character.
macro_rules! accents {
    ($($primary:literal $(| $alt:literal)* => $name:ident),* $(,)?) => {
        impl Accent {
            /// Normalize an accent to a combining one.
            pub fn combine(c: char) -> Option<char> {
                Some(match c {
                    $($primary $(| $alt)* => $primary,)*
                    _ => return None,
                })
            }
        }

        $(
            /// The accent function for callable symbol definitions.
            #[func]
            pub fn $name(
                /// The base to which the accent is applied.
                base: Content,
                /// The size of the accent, relative to the width of the base.
                #[named]
                size: Option<Rel<Length>>,
            ) -> Content {
                let mut accent = AccentElem::new(base, Accent::new($primary));
                if let Some(size) = size {
                    accent = accent.with_size(size);
                }
                accent.pack()
            }
        )+
    };
}

// Keep it synced with the documenting table above.
accents! {
    '\u{0300}' | '`' => grave,
    '\u{0301}' | '´' => acute,
    '\u{0302}' | '^' | 'ˆ' => hat,
    '\u{0303}' | '~' | '∼' | '˜' => tilde,
    '\u{0304}' | '¯' => macron,
    '\u{0305}' | '-' | '‾' | '−' => dash,
    '\u{0306}' | '˘' => breve,
    '\u{0307}' | '.' | '˙' | '⋅' => dot,
    '\u{0308}' | '¨' => dot_double,
    '\u{20db}' => dot_triple,
    '\u{20dc}' => dot_quad,
    '\u{030a}' | '∘' | '○' => circle,
    '\u{030b}' | '˝' => acute_double,
    '\u{030c}' | 'ˇ' => caron,
    '\u{20d6}' | '←' => arrow_l,
    '\u{20d7}' | '→' | '⟶' => arrow,
    '\u{20e1}' | '↔' | '⟷' => arrow_l_r,
    '\u{20d0}' | '↼' => harpoon_lt,
    '\u{20d1}' | '⇀' => harpoon,
}

cast! {
    Accent,
    self => self.0.into_value(),
    v: char => Self::new(v),
    v: Content => match v.to_packed::<SymbolElem>() {
        Some(elem) => Self::new(elem.text),
        None => bail!("expected a symbol"),
    },
}
