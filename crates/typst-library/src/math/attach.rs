<<<<<<< HEAD
use crate::foundations::{elem, Content, Packed};
use crate::layout::{Length, Rel};
use crate::math::{EquationElem, Mathy};

/// オプションのアタッチメントを持つベースとなる関数。
=======
use crate::foundations::{Content, Packed, elem};
use crate::layout::{Length, Rel};
use crate::math::{EquationElem, Mathy};

/// A base with optional attachments.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// $ attach(
///   Pi, t: alpha, b: beta,
///   tl: 1, tr: 2+3, bl: 4+5, br: 6,
/// ) $
/// ```
#[elem(Mathy)]
pub struct AttachElem {
<<<<<<< HEAD
    /// アタッチメントを取り付けるベース。
    #[required]
    pub base: Content,

    /// 右上かベースの上にスマート配置された上部アタッチメント。
    ///
    /// ベースを`{limits()}`か`{scripts()}`でラップするとスマート配置を上書きできます。
    pub t: Option<Content>,

    /// 右下かベースの下にスマート配置された下部アタッチメント。
    ///
    /// ベースを`{limits()}`か`{scripts()}`でラップするとスマート配置を上書きできます。
    pub b: Option<Content>,

    /// 左上のアタッチメント（ベースの前）。
    pub tl: Option<Content>,

    /// 左下のアタッチメント（ベースの前）。
    pub bl: Option<Content>,

    /// 右上のアタッチメント（ベースの後）。
    pub tr: Option<Content>,

    /// 右下のアタッチメント（ベースの後）。
=======
    /// The base to which things are attached.
    #[required]
    pub base: Content,

    /// The top attachment, smartly positioned at top-right or above the base.
    ///
    /// You can wrap the base in `{limits()}` or `{scripts()}` to override the
    /// smart positioning.
    pub t: Option<Content>,

    /// The bottom attachment, smartly positioned at the bottom-right or below
    /// the base.
    ///
    /// You can wrap the base in `{limits()}` or `{scripts()}` to override the
    /// smart positioning.
    pub b: Option<Content>,

    /// The top-left attachment (before the base).
    pub tl: Option<Content>,

    /// The bottom-left attachment (before base).
    pub bl: Option<Content>,

    /// The top-right attachment (after the base).
    pub tr: Option<Content>,

    /// The bottom-right attachment (after the base).
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    pub br: Option<Content>,
}

impl Packed<AttachElem> {
    /// If an AttachElem's base is also an AttachElem, merge attachments into the
    /// base AttachElem where possible.
    pub fn merge_base(&self) -> Option<Self> {
        // Extract from an EquationElem.
        let mut base = &self.base;
        while let Some(equation) = base.to_packed::<EquationElem>() {
            base = &equation.body;
        }

        // Move attachments from elem into base where possible.
        if let Some(base) = base.to_packed::<AttachElem>() {
            let mut elem = self.clone();
            let mut base = base.clone();

            macro_rules! merge {
                ($content:ident) => {
<<<<<<< HEAD
                    if base.$content.is_none() && elem.$content.is_some() {
                        base.$content = elem.$content.clone();
                        elem.$content = None;
=======
                    if !base.$content.is_set() && elem.$content.is_set() {
                        base.$content = elem.$content.clone();
                        elem.$content.unset();
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
                    }
                };
            }

            merge!(t);
            merge!(b);
            merge!(tl);
            merge!(tr);
            merge!(bl);
            merge!(br);

            elem.base = base.pack();
            return Some(elem);
        }

        None
    }
}

<<<<<<< HEAD
/// グループ化されたプライム記号。
=======
/// Grouped primes.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// $ a'''_b = a^'''_b $
/// ```
///
<<<<<<< HEAD
/// # 構文
/// この関数には専用の構文があり、primes関数の代わりにアポストロフィー記号を使います。
/// これらは自動的に前の要素に付加され、次の上付き文字のレベルに移動します。
#[elem(Mathy)]
pub struct PrimesElem {
    /// グループ化するプライム記号の数。
=======
/// # Syntax
/// This function has dedicated syntax: use apostrophes instead of primes. They
/// will automatically attach to the previous element, moving superscripts to
/// the next level.
#[elem(Mathy)]
pub struct PrimesElem {
    /// The number of grouped primes.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[required]
    pub count: usize,
}

<<<<<<< HEAD
/// アタッチメントを添え字として表示することをベースに強制。
=======
/// Forces a base to display attachments as scripts.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// $ scripts(sum)_1^2 != sum_1^2 $
/// ```
#[elem(Mathy)]
pub struct ScriptsElem {
<<<<<<< HEAD
    /// 添え字を取り付けるベース。
=======
    /// The base to attach the scripts to.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[required]
    pub body: Content,
}

<<<<<<< HEAD
/// アタッチメントをlimitsとして表示することをベースに強制。
=======
/// Forces a base to display attachments as limits.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// $ limits(A)_1^2 != A_1^2 $
/// ```
#[elem(Mathy)]
pub struct LimitsElem {
<<<<<<< HEAD
    /// limitsを取り付けるベース。
    #[required]
    pub body: Content,

    /// インライン数式でもlimits表示を強制するかどうか。
    ///
    /// （例えばshowルールを用いて）limitsをグローバルに適用する場合、通常は無効にすることをおすすめします。
=======
    /// The base to attach the limits to.
    #[required]
    pub body: Content,

    /// Whether to also force limits in inline equations.
    ///
    /// When applying limits globally (e.g., through a show rule), it is
    /// typically a good idea to disable this.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[default(true)]
    pub inline: bool,
}

<<<<<<< HEAD
/// 字形を伸縮します。
///
/// この関数は、上部および下部アタッチメントがフィットするように、自動的にアタッチメントのベースを伸縮させることにも使えます。
///
/// 伸縮可能な字形は限られており、どの字形が伸縮可能かは使用する数式フォントに依存することに注意してください。
/// ただし、この点に関して多くの数式フォントで違いはありません。
=======
/// Stretches a glyph.
///
/// This function can also be used to automatically stretch the base of an
/// attachment, so that it fits the top and bottom attachments.
///
/// Note that only some glyphs can be stretched, and which ones can depend on
/// the math font being used. However, most math fonts are the same in this
/// regard.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// $ H stretch(=)^"define" U + p V $
/// $ f : X stretch(->>, size: #150%)_"surjective" Y $
/// $ x stretch(harpoons.ltrb, size: #3em) y
///     stretch(\[, size: #150%) z $
/// ```
#[elem(Mathy)]
pub struct StretchElem {
<<<<<<< HEAD
    /// 伸縮させる字形。
    #[required]
    pub body: Content,

    /// 字形およびそのアタッチメントを基準とした伸縮の大きさ。
    #[resolve]
=======
    /// The glyph to stretch.
    #[required]
    pub body: Content,

    /// The size to stretch to, relative to the maximum size of the glyph and
    /// its attachments.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    #[default(Rel::one())]
    pub size: Rel<Length>,
}
