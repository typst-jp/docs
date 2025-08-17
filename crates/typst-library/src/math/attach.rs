use crate::foundations::{elem, Content, Packed};
use crate::layout::{Length, Rel};
use crate::math::{EquationElem, Mathy};

/// オプションのアタッチメントを持つベースとなる関数。
///
/// ```example
/// $ attach(
///   Pi, t: alpha, b: beta,
///   tl: 1, tr: 2+3, bl: 4+5, br: 6,
/// ) $
/// ```
#[elem(Mathy)]
pub struct AttachElem {
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
                    if base.$content.is_none() && elem.$content.is_some() {
                        base.$content = elem.$content.clone();
                        elem.$content = None;
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

/// グループ化されたプライム記号。
///
/// ```example
/// $ a'''_b = a^'''_b $
/// ```
///
/// # 構文
/// この関数には専用の構文があり、primes関数の代わりにアポストロフィー記号を使います。
/// これらは自動的に前の要素に付加され、次の上付き文字のレベルに移動します。
#[elem(Mathy)]
pub struct PrimesElem {
    /// グループ化するプライム記号の数。
    #[required]
    pub count: usize,
}

/// アタッチメントを添え字として表示することをベースに強制。
///
/// ```example
/// $ scripts(sum)_1^2 != sum_1^2 $
/// ```
#[elem(Mathy)]
pub struct ScriptsElem {
    /// 添え字を取り付けるベース。
    #[required]
    pub body: Content,
}

/// アタッチメントをlimitsとして表示することをベースに強制。
///
/// ```example
/// $ limits(A)_1^2 != A_1^2 $
/// ```
#[elem(Mathy)]
pub struct LimitsElem {
    /// limitsを取り付けるベース。
    #[required]
    pub body: Content,

    /// インライン数式でもlimits表示を強制するかどうか。
    ///
    /// （例えばshowルールを用いて）limitsをグローバルに適用する場合、通常は無効にすることをおすすめします。
    #[default(true)]
    pub inline: bool,
}

/// 字形を伸縮します。
///
/// この関数は、上部及び下部アタッチメントがフィットするように、自動的にアタッチメントのベースを伸縮させることにも使えます。
///
/// 伸縮可能な字形は限られており、どの字形が伸縮可能かは使用する数式フォントに依存することに注意してください。
/// ただし、この点に関して多くの数式フォントで違いはありません。
///
/// ```example
/// $ H stretch(=)^"define" U + p V $
/// $ f : X stretch(->>, size: #150%)_"surjective" Y $
/// $ x stretch(harpoons.ltrb, size: #3em) y
///     stretch(\[, size: #150%) z $
/// ```
#[elem(Mathy)]
pub struct StretchElem {
    /// 伸縮させる字形。
    #[required]
    pub body: Content,

    /// 字形およびそのアタッチメントを基準とした伸縮の大きさ。
    #[resolve]
    #[default(Rel::one())]
    pub size: Rel<Length>,
}
