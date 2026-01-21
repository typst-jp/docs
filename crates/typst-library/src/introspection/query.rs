use comemo::Tracked;

use crate::diag::HintedStrResult;
use crate::engine::Engine;
<<<<<<< HEAD
use crate::foundations::{func, Array, Context, LocatableSelector, Value};

/// 文書中の要素の検索。
///
/// `query`関数を用いると特定の型やラベルを持った要素を文書内から探すことができます。
/// 使用するにはまず[コンテキスト]($context)が利用可能であることを確かめる必要があります。
///

/// # 要素の探索
/// 以下の例では、[`outline`]を用いる代わりに手動で目次を作成しています。
///
/// このために、まず第1レベルの見出しで`outlined`がtrueなものを検索します。
/// この例において第1レベルの見出しのみを検索する目的は、第2レベル以下の見出しが目次に含まれないようにすることです。
/// `outlined`フィールドは"Table of Contents"という見出し自身を取り除くために使われます。
///
/// `query`関数を使用可能にするために、`context`を作成していることに注意してください。
=======
use crate::foundations::{Array, Context, LocatableSelector, Value, func};

/// Finds elements in the document.
///
/// The `query` function lets you search your document for elements of a
/// particular type or with a particular label. To use it, you first need to
/// ensure that [context] is available.
///

/// # Finding elements
/// In the example below, we manually create a table of contents instead of
/// using the [`outline`] function.
///
/// To do this, we first query for all headings in the document at level 1 and
/// where `outlined` is true. Querying only for headings at level 1 ensures
/// that, for the purpose of this example, sub-headings are not included in the
/// table of contents. The `outlined` field is used to exclude the "Table of
/// Contents" heading itself.
///
/// Note that we open a `context` to be able to use the `query` function.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// >>> #set page(
/// >>>  width: 240pt,
/// >>>  height: 180pt,
/// >>>  margin: (top: 20pt, bottom: 35pt)
/// >>> )
/// #set page(numbering: "1")
///
/// #heading(outlined: false)[
///   Table of Contents
/// ]
/// #context {
///   let chapters = query(
///     heading.where(
///       level: 1,
///       outlined: true,
///     )
///   )
///   for chapter in chapters {
///     let loc = chapter.location()
///     let nr = numbering(
///       loc.page-numbering(),
///       ..counter(page).at(loc),
///     )
///     [#chapter.body #h(1fr) #nr \ ]
///   }
/// }
///
/// = Introduction
/// #lorem(10)
/// #pagebreak()
///
/// == Sub-Heading
/// #lorem(8)
///
/// = Discussion
/// #lorem(18)
/// ```
///
<<<<<<< HEAD
/// ページ番号を取得するために、まず[`location`]($content.location)メソッドを用いて`query`が返す要素のロケーションを取得します。
/// 続けて、その位置にある[ページの番号付け]($location.page-numbering)と[ページカウンター]($counter/#page-counter)を取得し、カウンターに番号付けを適用します。
///
/// # 注意事項 { #caution }
/// 全てのクエリを解決するために、Typstは文書の評価とレイアウトを複数回行います。
/// しかしながら、実際にクエリが完全に解決されるかは保証されません。
/// 注意しないとクエリ自身に影響しうるクエリを書いてしまい、結果が決して収束しなくなります。
///
/// 以下の例では、文書中の全ての見出しを検索し、同じ数だけ見出しを生成しています。
/// 最初は`Real`という見出しが1つだけあります。
/// したがって、`count`は`1`で、`Fake`という見出しが作成されます。
/// Typstはクエリの結果が変わったことに気づき、再度処理を行います。
/// このとき`count`は`2`で、 2つの`Fake`見出しが作成されます。
/// これが延々と続きます。
/// ご覧の通り、出力には有限個の見出ししかありません。
/// これは単にTypstが数回試行した後に諦めるためです。
///
/// 一般に、クエリ自身に影響を与えるようなクエリを書こうとしてはいけません。
/// [カウンター]($counter)や[状態]($state)などの他の内省機能にも同じ注意が必要です。
=======
/// To get the page numbers, we first get the location of the elements returned
/// by `query` with [`location`]($content.location). We then also retrieve the
/// [page numbering]($location.page-numbering) and [page
/// counter]($counter/#page-counter) at that location and apply the numbering to
/// the counter.
///
/// # A word of caution { #caution }
/// To resolve all your queries, Typst evaluates and layouts parts of the
/// document multiple times. However, there is no guarantee that your queries
/// can actually be completely resolved. If you aren't careful a query can
/// affect itself—leading to a result that never stabilizes.
///
/// In the example below, we query for all headings in the document. We then
/// generate as many headings. In the beginning, there's just one heading,
/// titled `Real`. Thus, `count` is `1` and one `Fake` heading is generated.
/// Typst sees that the query's result has changed and processes it again. This
/// time, `count` is `2` and two `Fake` headings are generated. This goes on and
/// on. As we can see, the output has a finite amount of headings. This is
/// because Typst simply gives up after a few attempts.
///
/// In general, you should try not to write queries that affect themselves. The
/// same words of caution also apply to other introspection features like
/// [counters]($counter) and [state].
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```example
/// = Real
/// #context {
///   let elems = query(heading)
///   let count = elems.len()
///   count * [= Fake]
/// }
/// ```
///
<<<<<<< HEAD
/// # コマンドラインクエリ
/// `typst query`コマンドを用いてコマンドラインからクエリを実行することもできます。
/// このコマンドは文書上で任意のクエリを実行し、シリアライズされた形で結果の要素を返します。
/// 以下の何らかの不可視の[メタデータ]($metadata)を含んだ`example.typ`ファイルを考えます。
=======
/// # Command line queries
/// You can also perform queries from the command line with the `typst query`
/// command. This command executes an arbitrary query on the document and
/// returns the resulting elements in serialized form. Consider the following
/// `example.typ` file which contains some invisible [metadata]:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```typ
/// #metadata("This is a note") <note>
/// ```
///
<<<<<<< HEAD
/// Typst CLIを用いて以下のようにこのファイルに対してクエリを実行できます。
=======
/// You can execute a query on it as follows using Typst's CLI:
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// ```sh
/// $ typst query example.typ "<note>"
/// [
///   {
///     "func": "metadata",
///     "value": "This is a note",
///     "label": "<note>"
///   }
/// ]
/// ```
///
<<<<<<< HEAD
/// 結果となる要素の特定の1つのフィールドにのみ興味があることが多いです。
/// `metadata`要素の場合、`value`フィールドが興味の対象です。
/// `--field`引数を用いてこのフィールドのみを抽出できます。
=======
/// ## Retrieving a specific field
///
/// Frequently, you're interested in only one specific field of the resulting
/// elements. In the case of the `metadata` element, the `value` field is the
/// interesting one. You can extract just this field with the `--field`
/// argument.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```sh
/// $ typst query example.typ "<note>" --field value
/// ["This is a note"]
/// ```
///
<<<<<<< HEAD
/// 単一の要素にのみ興味がある場合は、`--one`フラグを用いてその要素のみを抽出できます。
=======
/// If you are interested in just a single element, you can use the `--one`
/// flag to extract just it.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
///
/// ```sh
/// $ typst query example.typ "<note>" --field value --one
/// "This is a note"
/// ```
<<<<<<< HEAD
=======
///
/// ## Querying for a specific export target
///
/// In case you need to query a document when exporting for a specific target,
/// you can use the `--target` argument. Valid values are `paged`, and `html`
/// (if the [`html`] feature is enabled).
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
#[func(contextual)]
pub fn query(
    engine: &mut Engine,
    context: Tracked<Context>,
<<<<<<< HEAD
    /// - `heading`や`figure`のような要素関数
    /// - `{<label>}`
    /// - `{heading.where(level: 1)}`のような、より複雑なセレクター
    /// - `{selector(heading).before(here())}`
    ///
    /// が可能です。
    ///
    /// [ロケータブル]($location/#locatable)要素関数がサポートされています。
=======
    /// Can be
    /// - an element function like a `heading` or `figure`,
    /// - a `{<label>}`,
    /// - a more complex selector like `{heading.where(level: 1)}`,
    /// - or `{selector(heading).before(here())}`.
    ///
    /// Only [locatable]($location/#locatable) element functions are supported.
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    target: LocatableSelector,
) -> HintedStrResult<Array> {
    context.introspect()?;
    let vec = engine.introspector.query(&target.0);
    Ok(vec.into_iter().map(Value::Content).collect())
}
