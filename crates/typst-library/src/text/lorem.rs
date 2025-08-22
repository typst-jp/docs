use crate::foundations::{func, Str};

/// ダミーテキストの作成。
///
/// この関数は与えられた単語数だけラテン語風のダミーテキストである _Lorem Ipsum_ を生成します。
/// この関数で生成される単語の並びはランダムに選ばれますが、生成のたびに同じものになります。
/// 通常のダミーテキストと同様に、意味のないテキストです。
/// レイアウトを試すプレースホルダーとして使用してください。
///
/// # 例
/// ```example
/// = Blind Text
/// #lorem(30)
///
/// = More Blind Text
/// #lorem(15)
/// ```
#[func(keywords = ["Blind Text"])]
pub fn lorem(
    /// ダミーテキストの単語数。
    words: usize,
) -> Str {
    lipsum::lipsum(words).replace("--", "–").into()
}
