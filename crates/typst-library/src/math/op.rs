use ecow::EcoString;

use crate::foundations::{elem, Content, NativeElement, Scope, SymbolElem};
use crate::layout::HElem;
use crate::math::{upright, Mathy, THIN};
use crate::text::TextElem;

/// 数式中のテキスト演算子。
///
/// # 例
/// ```example
/// $ tan x = (sin x)/(cos x) $
/// $ op("custom",
///      limits: #true)_(n->oo) n $
/// ```
///
/// # 定義済み演算子 { #predefined }
/// Typstではあらかじめ以下の演算子が定義されています。
/// `arccos`、`arcsin`、`arctan`、`arg`、`cos`、`cosh`、`cot`、`coth`、`csc`、`csch`、`ctg`、`deg`、`det`、`dim`、`exp`、`gcd`、`lcm`、`hom`、`id`、`im`、`inf`、`ker`、`lg`、`lim`、`liminf`、`limsup`、`ln`、`log`、`max`、`min`、`mod`、`Pr`、`sec`、`sech`、`sin`、`sinc`、`sinh`、`sup`、`tan`、`tanh`、`tg`、`tr`。
#[elem(title = "Text Operator", Mathy)]
pub struct OpElem {
    /// 演算子のテキスト。
    #[required]
    pub text: Content,

    /// ディスプレイモードのときに演算子のアタッチメントをlimitsのように表示するかどうか。
    #[default(false)]
    pub limits: bool,
}

macro_rules! ops {
    ($($name:ident $(: $value:literal)? $(($tts:tt))?),* $(,)?) => {
        pub(super) fn define(math: &mut Scope) {
            $({
                let operator = EcoString::from(ops!(@name $name $(: $value)?));
                math.define(
                    stringify!($name),
                    // Latex also uses their equivalent of `TextElem` here.
                    OpElem::new(TextElem::new(operator).into())
                        .with_limits(ops!(@limit $($tts)*))
                        .pack()
                );
            })*

            let dif = |d| {
                HElem::new(THIN.into()).with_weak(true).pack()
                    + upright(SymbolElem::packed(d))
            };
            math.define("dif", dif('d'));
            math.define("Dif", dif('D'));
        }
    };
    (@name $name:ident) => { stringify!($name) };
    (@name $name:ident: $value:literal) => { $value };
    (@limit limits) => { true };
    (@limit) => { false };
}

ops! {
    arccos,
    arcsin,
    arctan,
    arg,
    cos,
    cosh,
    cot,
    coth,
    csc,
    csch,
    ctg,
    deg,
    det (limits),
    dim,
    exp,
    gcd (limits),
    lcm (limits),
    hom,
    id,
    im,
    inf (limits),
    ker,
    lg,
    lim (limits),
    liminf: "lim inf" (limits),
    limsup: "lim sup" (limits),
    ln,
    log,
    max (limits),
    min (limits),
    mod,
    Pr (limits),
    sec,
    sech,
    sin,
    sinc,
    sinh,
    sup (limits),
    tan,
    tanh,
    tg,
    tr,
}
