use ecow::EcoString;

<<<<<<< HEAD
use crate::foundations::{ty, Repr};
=======
use crate::foundations::{Repr, ty};
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

/// A type with two states.
///
/// The boolean type has two values: `{true}` and `{false}`. It denotes whether
/// something is active or enabled.
///
/// # Example
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
