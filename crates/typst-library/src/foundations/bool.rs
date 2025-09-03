use ecow::EcoString;

use crate::foundations::{ty, Repr};

/// 2つの状態を持つ型です。
///
/// ブーリアン型には、`{true}`と`{false}`という2つの値があります。
/// これは、何かの状態がオンであるのかや、有効であるのかなどを示します。
///
/// # 例
/// ```example
/// #false \
/// #true \
/// #(1 < 2)
/// ```
#[ty(cast, title = "Boolean")]
type bool;

impl Repr for bool {
    fn repr(&self) -> EcoString {
        match self {
            true => "true".into(),
            false => "false".into(),
        }
    }
}
