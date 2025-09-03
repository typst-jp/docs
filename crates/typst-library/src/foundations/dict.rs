use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign};
use std::sync::Arc;

use ecow::{eco_format, EcoString};
use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use typst_syntax::is_ident;
use typst_utils::ArcExt;

use crate::diag::{Hint, HintedStrResult, StrResult};
use crate::foundations::{
    array, cast, func, repr, scope, ty, Array, Module, Repr, Str, Value,
};

/// Create a new [`Dict`] from key-value pairs.
#[macro_export]
#[doc(hidden)]
macro_rules! __dict {
    ($($key:expr => $value:expr),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut map = $crate::foundations::IndexMap::new();
        $(map.insert($key.into(), $crate::foundations::IntoValue::into_value($value));)*
        $crate::foundations::Dict::from(map)
    }};
}

#[doc(inline)]
pub use crate::__dict as dict;

/// 文字列をキーとする、値のマップです。
///
/// コンマ区切りの`キー: 値`ペアを括弧で囲むことで辞書を生成できます。
/// 値は同じ型である必要はありません。空の括弧`()`は空の配列を生成するため、
/// 空の辞書を作成するには特殊な`(:)`構文を使用する必要があります。
///
/// 辞書は概念的に配列と似ていますが、整数でインデックスがつけられる代わりに
/// 文字列が用いられます。辞書のエントリには`.at()`メソッドでアクセスしたり、
/// 作成したりできます。キーがあらかじめ分かっている場合は、代わりに
/// [フィールドアクセス記法]($scripting/#fields) (`.key`)を使って値にアクセス
/// することもできます。辞書は`+`演算子で加算したり、
/// [結合したり]($scripting/#blocks)できます。キーが辞書内に存在するか
/// どうかを確認するには、`in`キーワードを使用してください。
///
/// 辞書内のペアを[forループ]($scripting/#loops)を使って反復処理することも
/// できます。その場合、辞書のエントリは挿入または宣言された順に反復処理されます。
///
/// # 例
/// ```example
/// #let dict = (
///   name: "Typst",
///   born: 2019,
/// )
///
/// #dict.name \
/// #(dict.launch = 20)
/// #dict.len() \
/// #dict.keys() \
/// #dict.values() \
/// #dict.at("born") \
/// #dict.insert("city", "Berlin ")
/// #("name" in dict)
/// ```
#[ty(scope, cast, name = "dictionary")]
#[derive(Default, Clone, PartialEq)]
pub struct Dict(Arc<IndexMap<Str, Value>>);

impl Dict {
    /// Create a new, empty dictionary.
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether the dictionary is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Borrow the value at the given key.
    pub fn get(&self, key: &str) -> StrResult<&Value> {
        self.0.get(key).ok_or_else(|| missing_key(key))
    }

    /// Mutably borrow the value the given `key` maps to.
    pub fn at_mut(&mut self, key: &str) -> HintedStrResult<&mut Value> {
        Arc::make_mut(&mut self.0)
            .get_mut(key)
            .ok_or_else(|| missing_key(key))
            .hint("use `insert` to add or update values")
    }

    /// Remove the value if the dictionary contains the given key.
    pub fn take(&mut self, key: &str) -> StrResult<Value> {
        Arc::make_mut(&mut self.0)
            .shift_remove(key)
            .ok_or_else(|| missing_key(key))
    }

    /// Whether the dictionary contains a specific key.
    pub fn contains(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    /// Clear the dictionary.
    pub fn clear(&mut self) {
        if Arc::strong_count(&self.0) == 1 {
            Arc::make_mut(&mut self.0).clear();
        } else {
            *self = Self::new();
        }
    }

    /// Iterate over pairs of references to the contained keys and values.
    pub fn iter(&self) -> indexmap::map::Iter<Str, Value> {
        self.0.iter()
    }

    /// Check if there is any remaining pair, and if so return an
    /// "unexpected key" error.
    pub fn finish(&self, expected: &[&str]) -> StrResult<()> {
        let mut iter = self.iter().peekable();
        if iter.peek().is_none() {
            return Ok(());
        }
        let unexpected: Vec<&str> = iter.map(|kv| kv.0.as_str()).collect();

        Err(Self::unexpected_keys(unexpected, Some(expected)))
    }

    // Return an "unexpected key" error string.
    pub fn unexpected_keys(
        unexpected: Vec<&str>,
        hint_expected: Option<&[&str]>,
    ) -> EcoString {
        let format_as_list = |arr: &[&str]| {
            repr::separated_list(
                &arr.iter().map(|s| eco_format!("\"{s}\"")).collect::<Vec<_>>(),
                "and",
            )
        };

        let mut msg = String::from(match unexpected.len() {
            1 => "unexpected key ",
            _ => "unexpected keys ",
        });

        msg.push_str(&format_as_list(&unexpected[..]));

        if let Some(expected) = hint_expected {
            msg.push_str(", valid keys are ");
            msg.push_str(&format_as_list(expected));
        }

        msg.into()
    }
}

#[scope]
impl Dict {
    /// 値を辞書に変換します。
    ///
    /// この関数は、辞書形式の値を辞書に変換することのみを目的としており、個々の値ペアから
    /// 辞書を作成するためのものではありません。個々の値ペアから辞書を作成する場合は、
    /// 辞書の構文`(キー: 値)`を使用してください。


    ///
    /// ```example
    /// #dictionary(sys).at("version")
    /// ```
    #[func(constructor)]
    pub fn construct(
        /// 辞書に変換する値。
        value: ToDict,
    ) -> Dict {
        value.0
    }

    /// 辞書に含まれるペアの個数。
    #[func(title = "Length")]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// 指定されたキーに対応する辞書内の値を返します。キーがすでに辞書内に存在する場合、
    /// 代入演算子の左辺で使用できます。キーが辞書に存在しない場合、デフォルト値を返し、
    /// デフォルト値が指定されていない場合はエラーになります。
    #[func]
    pub fn at(
        &self,
        /// 辞書項目を取得するためのキー。
        key: Str,
        /// キーが辞書内にない場合に返されるデフォルト値。
        #[named]
        default: Option<Value>,
    ) -> StrResult<Value> {
        self.0
            .get(&key)
            .cloned()
            .or(default)
            .ok_or_else(|| missing_key_no_default(&key))
    }

    /// 新しいペアを辞書に挿入します。すでにこのキー辞書にが含まれている場合、
    /// 値は更新されます。
    #[func]
    pub fn insert(
        &mut self,
        /// 挿入するペアのキー。
        key: Str,
        /// 挿入するペアの値。
        value: Value,
    ) {
        Arc::make_mut(&mut self.0).insert(key, value);
    }

    /// キーを指定して辞書からペアを削除し、その値を返します。
    #[func]
    pub fn remove(
        &mut self,
        /// 削除するペアのキー。
        key: Str,
        /// キーが辞書内にない場合に返されるデフォルト値。
        #[named]
        default: Option<Value>,
    ) -> StrResult<Value> {
        Arc::make_mut(&mut self.0)
            .shift_remove(&key)
            .or(default)
            .ok_or_else(|| missing_key(&key))
    }

    /// 辞書のキーを、挿入された順序で配列として返します。
    #[func]
    pub fn keys(&self) -> Array {
        self.0.keys().cloned().map(Value::Str).collect()
    }

    /// 辞書の値を、挿入された順序で配列として返します。
    #[func]
    pub fn values(&self) -> Array {
        self.0.values().cloned().collect()
    }

    /// 辞書のキーと値を、ペアの配列として返します。各ペアは
    /// 長さ2の配列として表現されます。
    #[func]
    pub fn pairs(&self) -> Array {
        self.0
            .iter()
            .map(|(k, v)| Value::Array(array![k.clone(), v.clone()]))
            .collect()
    }
}

/// A value that can be cast to dictionary.
pub struct ToDict(Dict);

cast! {
    ToDict,
    v: Module => Self(v
        .scope()
        .iter()
        .map(|(k, b)| (Str::from(k.clone()), b.read().clone()))
        .collect()
    ),
}

impl Debug for Dict {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.0.iter()).finish()
    }
}

impl Repr for Dict {
    fn repr(&self) -> EcoString {
        if self.is_empty() {
            return "(:)".into();
        }

        let max = 40;
        let mut pieces: Vec<_> = self
            .iter()
            .take(max)
            .map(|(key, value)| {
                if is_ident(key) {
                    eco_format!("{key}: {}", value.repr())
                } else {
                    eco_format!("{}: {}", key.repr(), value.repr())
                }
            })
            .collect();

        if self.len() > max {
            pieces.push(eco_format!(".. ({} pairs omitted)", self.len() - max));
        }

        repr::pretty_array_like(&pieces, false).into()
    }
}

impl Add for Dict {
    type Output = Self;

    fn add(mut self, rhs: Dict) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for Dict {
    fn add_assign(&mut self, rhs: Dict) {
        match Arc::try_unwrap(rhs.0) {
            Ok(map) => self.extend(map),
            Err(rc) => self.extend(rc.iter().map(|(k, v)| (k.clone(), v.clone()))),
        }
    }
}

impl Hash for Dict {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.0.len());
        for item in self {
            item.hash(state);
        }
    }
}

impl Serialize for Dict {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Dict {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(IndexMap::<Str, Value>::deserialize(deserializer)?.into())
    }
}

impl Extend<(Str, Value)> for Dict {
    fn extend<T: IntoIterator<Item = (Str, Value)>>(&mut self, iter: T) {
        Arc::make_mut(&mut self.0).extend(iter);
    }
}

impl FromIterator<(Str, Value)> for Dict {
    fn from_iter<T: IntoIterator<Item = (Str, Value)>>(iter: T) -> Self {
        Self(Arc::new(iter.into_iter().collect()))
    }
}

impl IntoIterator for Dict {
    type Item = (Str, Value);
    type IntoIter = indexmap::map::IntoIter<Str, Value>;

    fn into_iter(self) -> Self::IntoIter {
        Arc::take(self.0).into_iter()
    }
}

impl<'a> IntoIterator for &'a Dict {
    type Item = (&'a Str, &'a Value);
    type IntoIter = indexmap::map::Iter<'a, Str, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl From<IndexMap<Str, Value>> for Dict {
    fn from(map: IndexMap<Str, Value>) -> Self {
        Self(Arc::new(map))
    }
}

/// The missing key access error message.
#[cold]
fn missing_key(key: &str) -> EcoString {
    eco_format!("dictionary does not contain key {}", key.repr())
}

/// The missing key access error message when no default was given.
#[cold]
fn missing_key_no_default(key: &str) -> EcoString {
    eco_format!(
        "dictionary does not contain key {} \
         and no default value was specified",
        key.repr()
    )
}
