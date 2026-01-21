<<<<<<< HEAD
use crate::foundations::{cast, func, Cast, Content, Str};
use crate::text::TextElem;

/// 文字列やコンテンツを小文字に変換。
///
/// # 例
=======
use crate::foundations::{Cast, Content, Str, cast, func};
use crate::text::TextElem;

/// Converts a string or content to lowercase.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// #lower("ABC") \
/// #lower[*My Text*] \
/// #lower[already low]
/// ```
#[func(title = "Lowercase")]
pub fn lower(
<<<<<<< HEAD
    /// 小文字に変換するテキスト。
=======
    /// The text to convert to lowercase.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    text: Caseable,
) -> Caseable {
    case(text, Case::Lower)
}

<<<<<<< HEAD
///  文字列やコンテンツを大文字に変換。
///
/// # 例
=======
/// Converts a string or content to uppercase.
///
/// # Example
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```example
/// #upper("abc") \
/// #upper[*my text*] \
/// #upper[ALREADY HIGH]
/// ```
#[func(title = "Uppercase")]
pub fn upper(
<<<<<<< HEAD
    /// 大文字に変換するテキスト。
=======
    /// The text to convert to uppercase.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    text: Caseable,
) -> Caseable {
    case(text, Case::Upper)
}

/// Change the case of text.
fn case(text: Caseable, case: Case) -> Caseable {
    match text {
        Caseable::Str(v) => Caseable::Str(case.apply(&v).into()),
<<<<<<< HEAD
        Caseable::Content(v) => {
            Caseable::Content(v.styled(TextElem::set_case(Some(case))))
        }
=======
        Caseable::Content(v) => Caseable::Content(v.set(TextElem::case, Some(case))),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }
}

/// A value whose case can be changed.
pub enum Caseable {
    Str(Str),
    Content(Content),
}

cast! {
    Caseable,
    self => match self {
        Self::Str(v) => v.into_value(),
        Self::Content(v) => v.into_value(),
    },
    v: Str => Self::Str(v),
    v: Content => Self::Content(v),
}

/// A case transformation on text.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum Case {
    /// Everything is lowercased.
    Lower,
    /// Everything is uppercased.
    Upper,
}

impl Case {
    /// Apply the case to a string.
    pub fn apply(self, text: &str) -> String {
        match self {
            Self::Lower => text.to_lowercase(),
            Self::Upper => text.to_uppercase(),
        }
    }
}
