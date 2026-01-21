//! Documentation provider for Typst.

mod contribs;
mod html;
mod link;
mod model;

pub use self::contribs::*;
pub use self::html::*;
pub use self::model::*;

<<<<<<< HEAD
use std::collections::HashSet;

use ecow::{eco_format, EcoString};
use heck::ToTitleCase;
use serde::Deserialize;
use serde_yaml as yaml;
use std::sync::LazyLock;
use typst::diag::{bail, StrResult};
=======
use ecow::{EcoString, eco_format};
use heck::ToTitleCase;
use rustc_hash::FxHashSet;
use serde::Deserialize;
use serde_yaml as yaml;
use std::sync::LazyLock;
use typst::diag::{StrResult, bail};
use typst::foundations::Deprecation;
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
use typst::foundations::{
    AutoValue, Binding, Bytes, CastInfo, Func, Module, NoneValue, ParamInfo, Repr, Scope,
    Smart, Type, Value,
};
use typst::layout::{Abs, Margin, PageElem, PagedDocument};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
<<<<<<< HEAD
use typst::{Category, Feature, Library, LibraryBuilder};
=======
use typst::{Category, Feature, Library, LibraryExt};
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
use unicode_math_class::MathClass;

macro_rules! load {
    ($path:literal) => {
        include_str!(concat!("../", $path))
    };
}

static GROUPS: LazyLock<Vec<GroupData>> = LazyLock::new(|| {
    let mut groups: Vec<GroupData> =
        yaml::from_str(load!("reference/groups.yml")).unwrap();
    for group in &mut groups {
<<<<<<< HEAD
        if group.filter.is_empty() {
=======
        if group.filter.is_empty() && group.name != "std" {
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            group.filter = group
                .module()
                .scope()
                .iter()
                .filter(|(_, b)| matches!(b.read(), Value::Func(_)))
                .map(|(k, _)| k.clone())
                .collect();
        }
<<<<<<< HEAD
=======
        if group.name == "typed" {
            group.filter = typst_assets::html::ELEMS
                .iter()
                .map(|elem| elem.name.into())
                .collect();
        }
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }
    groups
});

static LIBRARY: LazyLock<LazyHash<Library>> = LazyLock::new(|| {
<<<<<<< HEAD
    let mut lib = LibraryBuilder::default()
        .with_features([Feature::Html].into_iter().collect())
=======
    let mut lib = Library::builder()
        .with_features([Feature::Html, Feature::A11yExtras].into_iter().collect())
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        .build();
    let scope = lib.global.scope_mut();

    // Add those types, so that they show up in the docs.
    scope.start_category(Category::Foundations);
    scope.define_type::<NoneValue>();
    scope.define_type::<AutoValue>();
    scope.reset_category();

    // Adjust the default look.
<<<<<<< HEAD
    lib.styles
        .set(PageElem::set_width(Smart::Custom(Abs::pt(240.0).into())));
    lib.styles.set(PageElem::set_height(Smart::Auto));
    lib.styles.set(PageElem::set_margin(Margin::splat(Some(Smart::Custom(
        Abs::pt(15.0).into(),
    )))));
=======
    lib.styles.set(PageElem::width, Smart::Custom(Abs::pt(240.0).into()));
    lib.styles.set(PageElem::height, Smart::Auto);
    lib.styles
        .set(PageElem::margin, Margin::splat(Some(Smart::Custom(Abs::pt(15.0).into()))));
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

    LazyHash::new(lib)
});

static FONTS: LazyLock<(LazyHash<FontBook>, Vec<Font>)> = LazyLock::new(|| {
    let fonts: Vec<_> = typst_assets::fonts()
        .chain(typst_dev_assets::fonts())
        .flat_map(|data| Font::iter(Bytes::new(data)))
        .collect();
    let book = FontBook::from_fonts(&fonts);
    (LazyHash::new(book), fonts)
});

/// Build documentation pages.
pub fn provide(resolver: &dyn Resolver) -> Vec<PageModel> {
    let base = resolver.base();
    vec![
        md_page(resolver, base, load!("overview.md")).with_route(base),
        tutorial_pages(resolver),
        reference_pages(resolver),
        guide_pages(resolver),
        changelog_pages(resolver),
<<<<<<< HEAD
        japanese_pages(resolver),
        md_page(resolver, base, load!("glossary.md")),
=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ]
}

/// Resolve consumer dependencies.
pub trait Resolver {
    /// Try to resolve a link. If this returns `None`, the system will try to
    /// resolve the link itself.
    fn link(&self, link: &str) -> Option<String>;

    /// Produce an URL for an image file.
    fn image(&self, filename: &str, data: &[u8]) -> String;

    /// Produce HTML for an example.
    fn example(&self, hash: u128, source: Option<Html>, document: &PagedDocument)
<<<<<<< HEAD
        -> Html;
=======
    -> Html;
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

    /// Determine the commits between two tags.
    fn commits(&self, from: &str, to: &str) -> Vec<Commit>;

    /// Get the base URL for the routes and links. This must end with a slash.
    fn base(&self) -> &str;
}

/// Create a page from a markdown file.
#[track_caller]
fn md_page(resolver: &dyn Resolver, parent: &str, md: &str) -> PageModel {
<<<<<<< HEAD
    assert!(parent.starts_with('/') && parent.ends_with('/'));
    let html = Html::markdown(resolver, md, Some(0));
    let title = html.title().expect("chapter lacks a title");
=======
    md_page_with_title(resolver, parent, md, None)
}

/// Create a page from a markdown file.
#[track_caller]
fn md_page_with_title(
    resolver: &dyn Resolver,
    parent: &str,
    md: &str,
    title: Option<&str>,
) -> PageModel {
    assert!(parent.starts_with('/') && parent.ends_with('/'));
    let html = Html::markdown(resolver, md, Some(0));
    let title = title.or(html.title()).expect("chapter lacks a title");
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    PageModel {
        route: eco_format!("{parent}{}/", urlify(title)),
        title: title.into(),
        description: html.description().expect("chapter lacks a description"),
        part: None,
        outline: html.outline(),
        body: BodyModel::Html(html),
        children: vec![],
    }
}

/// Build the tutorial.
fn tutorial_pages(resolver: &dyn Resolver) -> PageModel {
    let mut page = md_page(resolver, resolver.base(), load!("tutorial/welcome.md"));
    let base = format!("{}tutorial/", resolver.base());
    page.children = vec![
        md_page(resolver, &base, load!("tutorial/1-writing.md")),
        md_page(resolver, &base, load!("tutorial/2-formatting.md")),
        md_page(resolver, &base, load!("tutorial/3-advanced.md")),
        md_page(resolver, &base, load!("tutorial/4-template.md")),
    ];
    page
}

/// Build the reference.
fn reference_pages(resolver: &dyn Resolver) -> PageModel {
    let mut page = md_page(resolver, resolver.base(), load!("reference/welcome.md"));
    let base = format!("{}reference/", resolver.base());
    page.children = vec![
        md_page(resolver, &base, load!("reference/language/syntax.md"))
            .with_part("Language"),
        md_page(resolver, &base, load!("reference/language/styling.md")),
        md_page(resolver, &base, load!("reference/language/scripting.md")),
        md_page(resolver, &base, load!("reference/language/context.md")),
        category_page(resolver, Category::Foundations).with_part("Library"),
        category_page(resolver, Category::Model),
        category_page(resolver, Category::Text),
        category_page(resolver, Category::Math),
        category_page(resolver, Category::Symbols),
        category_page(resolver, Category::Layout),
        category_page(resolver, Category::Visualize),
        category_page(resolver, Category::Introspection),
        category_page(resolver, Category::DataLoading),
        category_page(resolver, Category::Pdf).with_part("Export"),
        category_page(resolver, Category::Html),
        category_page(resolver, Category::Png),
        category_page(resolver, Category::Svg),
    ];
    page
}

/// Build the guides section.
fn guide_pages(resolver: &dyn Resolver) -> PageModel {
    let mut page = md_page(resolver, resolver.base(), load!("guides/welcome.md"));
    let base = format!("{}guides/", resolver.base());
<<<<<<< HEAD
    page.title = "ガイド".into();
    page.children = vec![
        md_page(resolver, &base, load!("guides/guide-for-latex-users.md")),
        md_page(resolver, &base, load!("guides/page-setup.md")),
        md_page(resolver, &base, load!("guides/tables.md")),
=======
    page.children = vec![
        md_page_with_title(
            resolver,
            &base,
            load!("guides/guide-for-latex-users.md"),
            Some("For LaTeX Users"),
        ),
        md_page_with_title(
            resolver,
            &base,
            load!("guides/page-setup.md"),
            Some("Page Setup"),
        ),
        md_page_with_title(resolver, &base, load!("guides/tables.md"), Some("Tables")),
        md_page_with_title(
            resolver,
            &base,
            load!("guides/accessibility.md"),
            Some("Accessibility"),
        ),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ];
    page
}

/// Build the changelog section.
fn changelog_pages(resolver: &dyn Resolver) -> PageModel {
    let mut page = md_page(resolver, resolver.base(), load!("changelog/welcome.md"));
    let base = format!("{}changelog/", resolver.base());
<<<<<<< HEAD
    page.title = "変更履歴".into();
    page.children = vec![
=======
    page.children = vec![
        md_page(resolver, &base, load!("changelog/0.14.0.md")),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        md_page(resolver, &base, load!("changelog/0.13.1.md")),
        md_page(resolver, &base, load!("changelog/0.13.0.md")),
        md_page(resolver, &base, load!("changelog/0.12.0.md")),
        md_page(resolver, &base, load!("changelog/0.11.1.md")),
        md_page(resolver, &base, load!("changelog/0.11.0.md")),
        md_page(resolver, &base, load!("changelog/0.10.0.md")),
        md_page(resolver, &base, load!("changelog/0.9.0.md")),
        md_page(resolver, &base, load!("changelog/0.8.0.md")),
        md_page(resolver, &base, load!("changelog/0.7.0.md")),
        md_page(resolver, &base, load!("changelog/0.6.0.md")),
        md_page(resolver, &base, load!("changelog/0.5.0.md")),
        md_page(resolver, &base, load!("changelog/0.4.0.md")),
        md_page(resolver, &base, load!("changelog/0.3.0.md")),
        md_page(resolver, &base, load!("changelog/0.2.0.md")),
        md_page(resolver, &base, load!("changelog/0.1.0.md")),
        md_page(resolver, &base, load!("changelog/earlier.md")),
<<<<<<< HEAD
    ]
    .into_iter()
    .map(|child| {
        let route = eco_format!("{base}{}/", urlify(child.title.as_str()));
        PageModel { route, ..child }
    })
    .collect();

    page
}

/// Build the japanese section.
fn japanese_pages(resolver: &dyn Resolver) -> PageModel {
    let mut page = md_page(resolver, resolver.base(), load!("japanese/welcome.md"));
    let base = format!("{}japanese/", resolver.base());
    page.children = vec![
        md_page(resolver, &base, load!("japanese/templates.md")),
        md_page(resolver, &base, load!("japanese/packages.md")),
        md_page(resolver, &base, load!("japanese/articles.md")),
=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    ];
    page
}

/// Create a page for a category.
#[track_caller]
fn category_page(resolver: &dyn Resolver, category: Category) -> PageModel {
    let route = eco_format!("{}reference/{}/", resolver.base(), category.name());
    let mut children = vec![];
    let mut items = vec![];
    let mut shorthands = None;
    let mut markup = vec![];
    let mut math = vec![];

    let docs = category_docs(category);
    let (module, path): (&Module, &[&str]) = match category {
        Category::Math => (&LIBRARY.math, &["math"]),
        Category::Pdf => (get_module(&LIBRARY.global, "pdf").unwrap(), &["pdf"]),
        Category::Html => (get_module(&LIBRARY.global, "html").unwrap(), &["html"]),
        _ => (&LIBRARY.global, &[]),
    };

    // Add groups.
    for group in GROUPS.iter().filter(|g| g.category == category).cloned() {
        if matches!(group.name.as_str(), "sym" | "emoji") {
            let subpage = symbols_page(resolver, &route, &group);
            let BodyModel::Symbols(model) = &subpage.body else { continue };
            let list = &model.list;
            markup.extend(
                list.iter()
                    .filter(|symbol| symbol.markup_shorthand.is_some())
                    .cloned(),
            );
            math.extend(
                list.iter().filter(|symbol| symbol.math_shorthand.is_some()).cloned(),
            );

            items.push(CategoryItem {
                name: group.name.clone(),
                route: subpage.route.clone(),
<<<<<<< HEAD
                oneliner: oneliner(docs).into(),
=======
                oneliner: oneliner(docs),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
                code: true,
            });
            children.push(subpage);
            continue;
        }

        let (child, item) = group_page(resolver, &route, &group);
        children.push(child);
        items.push(item);
    }

    // Add symbol pages. These are ordered manually.
    if category == Category::Symbols {
        shorthands = Some(ShorthandsModel { markup, math });
    }

<<<<<<< HEAD
    let mut skip = HashSet::new();
    if category == Category::Math {
        skip = GROUPS
            .iter()
            .filter(|g| g.category == category)
            .flat_map(|g| &g.filter)
            .map(|s| s.as_str())
            .collect();

=======
    let mut skip: FxHashSet<&str> = GROUPS
        .iter()
        .filter(|g| g.category == category)
        .flat_map(|g| &g.filter)
        .map(|s| s.as_str())
        .collect();

    if category == Category::Math {
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        // Already documented in the text category.
        skip.insert("text");
    }

    // Tiling would be duplicate otherwise.
    if category == Category::Visualize {
        skip.insert("pattern");
    }

<<<<<<< HEAD
=======
    // PDF attach would be duplicate otherwise.
    if category == Category::Pdf {
        skip.insert("embed");
    }

>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    // Add values and types.
    let scope = module.scope();
    for (name, binding) in scope.iter() {
        if binding.category() != Some(category) {
            continue;
        }

        if skip.contains(name.as_str()) {
            continue;
        }

        match binding.read() {
            Value::Func(func) => {
                let name = func.name().unwrap();
                let subpage =
                    func_page(resolver, &route, func, path, binding.deprecation());
                items.push(CategoryItem {
                    name: name.into(),
                    route: subpage.route.clone(),
<<<<<<< HEAD
                    oneliner: oneliner(func.docs().unwrap_or_default()).into(),
=======
                    oneliner: oneliner(func.docs().unwrap_or_default()),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
                    code: true,
                });
                children.push(subpage);
            }
            Value::Type(ty) => {
                let subpage = type_page(resolver, &route, ty);
                items.push(CategoryItem {
                    name: ty.short_name().into(),
                    route: subpage.route.clone(),
<<<<<<< HEAD
                    oneliner: oneliner(ty.docs()).into(),
=======
                    oneliner: oneliner(ty.docs()),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
                    code: true,
                });
                children.push(subpage);
            }
            _ => {}
        }
    }

    if category != Category::Symbols {
        children.sort_by_cached_key(|child| child.title.clone());
        items.sort_by_cached_key(|item| item.name.clone());
    }

<<<<<<< HEAD
    let _title = EcoString::from(match category {
=======
    let title = EcoString::from(match category {
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        Category::Pdf | Category::Html | Category::Png | Category::Svg => {
            category.name().to_uppercase()
        }
        _ => category.name().to_title_case(),
    });
<<<<<<< HEAD
    let translated_title = EcoString::from(match category {
        Category::Foundations => "基礎",
        Category::Model => "モデル",
        Category::Text => "文章",
        Category::Math => "数式",
        Category::Symbols => "記号",
        Category::Layout => "レイアウト",
        Category::Visualize => "視覚化",
        Category::Introspection => "内省",
        Category::DataLoading => "データの読み込み",
        Category::Pdf => "PDF",
        Category::Html => "HTML",
        Category::Svg => "SVG",
        Category::Png => "PNG",
    });
    let details = Html::markdown(resolver, docs, Some(1));
    let mut outline = vec![OutlineItem {
        id: "summary".into(),
        name: "概要".into(),
        children: vec![],
    }];
    outline.extend(details.outline());
    if !items.is_empty() {
        outline.push(OutlineItem {
            id: "definitions".into(),
            name: "定義".into(),
            children: vec![],
        });
=======

    let details = Html::markdown(resolver, docs, Some(1));
    let mut outline = vec![OutlineItem::from_name("Summary")];
    outline.extend(details.outline());
    if !items.is_empty() {
        outline.push(OutlineItem::from_name("Definitions"));
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }
    if shorthands.is_some() {
        outline.push(OutlineItem::from_name("Shorthands"));
    }

    PageModel {
        route,
<<<<<<< HEAD
        title: translated_title.clone().into(),
        description: eco_format!(
            "Typstにおける{translated_title}に関連する関数のドキュメント"
=======
        title: title.clone(),
        description: eco_format!(
            "Documentation for functions related to {title} in Typst."
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        ),
        part: None,
        outline,
        body: BodyModel::Category(CategoryModel {
            name: category.name(),
<<<<<<< HEAD
            title: translated_title,
=======
            title,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            details,
            items,
            shorthands,
        }),
        children,
    }
}

/// Retrieve the docs for a category.
fn category_docs(category: Category) -> &'static str {
    match category {
        Category::Foundations => load!("reference/library/foundations.md"),
        Category::Introspection => load!("reference/library/introspection.md"),
        Category::Layout => load!("reference/library/layout.md"),
        Category::DataLoading => load!("reference/library/data-loading.md"),
        Category::Math => load!("reference/library/math.md"),
        Category::Model => load!("reference/library/model.md"),
        Category::Symbols => load!("reference/library/symbols.md"),
        Category::Text => load!("reference/library/text.md"),
        Category::Visualize => load!("reference/library/visualize.md"),
        Category::Pdf => load!("reference/export/pdf.md"),
        Category::Html => load!("reference/export/html.md"),
        Category::Svg => load!("reference/export/svg.md"),
        Category::Png => load!("reference/export/png.md"),
    }
}

/// Create a page for a function.
fn func_page(
    resolver: &dyn Resolver,
    parent: &str,
    func: &Func,
    path: &[&str],
<<<<<<< HEAD
    deprecation: Option<&'static str>,
=======
    deprecation: Option<&Deprecation>,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
) -> PageModel {
    let model = func_model(resolver, func, path, false, deprecation);
    let name = func.name().unwrap();
    PageModel {
        route: eco_format!("{parent}{}/", urlify(name)),
        title: func.title().unwrap().into(),
        description: eco_format!("Documentation for the `{name}` function."),
        part: None,
        outline: func_outline(&model, ""),
        body: BodyModel::Func(model),
        children: vec![],
    }
}

/// Produce a function's model.
fn func_model(
    resolver: &dyn Resolver,
    func: &Func,
    path: &[&str],
    nested: bool,
<<<<<<< HEAD
    deprecation: Option<&'static str>,
=======
    deprecation: Option<&Deprecation>,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
) -> FuncModel {
    let name = func.name().unwrap();
    let scope = func.scope().unwrap();
    let docs = func.docs().unwrap();

    let mut self_ = false;
    let mut params = func.params().unwrap();
    if params.first().is_some_and(|first| first.name == "self") {
        self_ = true;
        params = &params[1..];
    }

    let mut returns = vec![];
    let mut strings = vec![];
    casts(resolver, &mut returns, &mut strings, func.returns().unwrap());
    if !strings.is_empty() && !returns.contains(&"str") {
        returns.push("str");
    }
    returns.sort_by_key(|ty| type_index(ty));
    if returns == ["none"] {
        returns.clear();
    }

    let nesting = if nested { None } else { Some(1) };
<<<<<<< HEAD
    let (details, example) =
        if nested { split_details_and_example(docs) } else { (docs, None) };
=======
    let items =
        if nested { details_blocks(docs) } else { vec![RawDetailsBlock::Markdown(docs)] };

    let Some(first_md) = items.iter().find_map(|item| {
        if let RawDetailsBlock::Markdown(md) = item { Some(md) } else { None }
    }) else {
        panic!("function lacks any details")
    };

    let mut params = params.to_vec();
    if func.keywords().contains(&"typed-html") {
        params.retain(|param| !is_global_html_attr(param.name));
    }
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

    FuncModel {
        path: path.iter().copied().map(Into::into).collect(),
        name: name.into(),
        title: func.title().unwrap(),
        keywords: func.keywords(),
<<<<<<< HEAD
        oneliner: oneliner(details),
        element: func.element().is_some(),
        contextual: func.contextual().unwrap_or(false),
        deprecation,
        details: Html::markdown(resolver, details, nesting),
        example: example.map(|md| Html::markdown(resolver, md, None)),
=======
        oneliner: oneliner(first_md),
        element: func.element().is_some(),
        contextual: func.contextual().unwrap_or(false),
        deprecation_message: deprecation.map(Deprecation::message),
        deprecation_until: deprecation.and_then(Deprecation::until),
        details: items
            .into_iter()
            .map(|proto| proto.into_model(resolver, nesting))
            .collect(),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        self_,
        params: params.iter().map(|param| param_model(resolver, param)).collect(),
        returns,
        scope: scope_models(resolver, name, scope),
    }
}

/// Produce a parameter's model.
fn param_model(resolver: &dyn Resolver, info: &ParamInfo) -> ParamModel {
<<<<<<< HEAD
    let (details, example) = split_details_and_example(info.docs);

=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    let mut types = vec![];
    let mut strings = vec![];
    casts(resolver, &mut types, &mut strings, &info.input);
    if !strings.is_empty() && !types.contains(&"str") {
        types.push("str");
    }
    types.sort_by_key(|ty| type_index(ty));

    ParamModel {
        name: info.name,
<<<<<<< HEAD
        details: Html::markdown(resolver, details, None),
        example: example.map(|md| Html::markdown(resolver, md, None)),
=======
        details: details_blocks(info.docs)
            .into_iter()
            .map(|proto| proto.into_model(resolver, None))
            .collect(),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        types,
        strings,
        default: info.default.map(|default| {
            let node = typst::syntax::parse_code(&default().repr());
            Html::new(typst::syntax::highlight_html(&node))
        }),
        positional: info.positional,
        named: info.named,
        required: info.required,
        variadic: info.variadic,
        settable: info.settable,
    }
}

<<<<<<< HEAD
/// Split up documentation into details and an example.
fn split_details_and_example(docs: &str) -> (&str, Option<&str>) {
    let mut details = docs;
    let mut example = None;
    if let Some(mut i) = docs.find("```") {
        while docs[..i].ends_with('`') {
            i -= 1;
        }
        details = &docs[..i];
        example = Some(&docs[i..]);
    }
    (details, example)
=======
/// A details block that has not yet been processed.
enum RawDetailsBlock<'a> {
    /// Raw Markdown.
    Markdown(&'a str),
    /// An example with an optional title.
    Example { body: &'a str, title: Option<&'a str> },
}

impl<'a> RawDetailsBlock<'a> {
    fn into_model(self, resolver: &dyn Resolver, nesting: Option<usize>) -> DetailsBlock {
        match self {
            RawDetailsBlock::Markdown(md) => {
                DetailsBlock::Html(Html::markdown(resolver, md, nesting))
            }
            RawDetailsBlock::Example { body, title } => DetailsBlock::Example {
                body: Html::markdown(resolver, body, None),
                title: title.map(Into::into),
            },
        }
    }
}

/// Split up documentation into Markdown blocks and examples.
fn details_blocks(docs: &str) -> Vec<RawDetailsBlock<'_>> {
    let mut i = 0;
    let mut res = Vec::new();

    while i < docs.len() {
        match find_fence_start(&docs[i..]) {
            Some((found, fence_len)) => {
                let fence_idx = i + found;

                // Find the language tag of the fence, if any.
                let lang_tag_end = docs[fence_idx + fence_len..]
                    .find('\n')
                    .map(|end| fence_idx + fence_len + end)
                    .unwrap_or(docs.len());

                let tag = &docs[fence_idx + fence_len..lang_tag_end].trim();
                let title = ExampleArgs::from_tag(tag).title;

                // First, push non-fenced content.
                if found > 0 {
                    res.push(RawDetailsBlock::Markdown(&docs[i..fence_idx]));
                }

                // Then, find the end of the fence.
                let offset = fence_idx + fence_len;
                let Some(fence_end) = docs[offset..]
                    .find(&"`".repeat(fence_len))
                    .map(|end| offset + end + fence_len)
                else {
                    panic!(
                        "unclosed code fence in docs at position {}: {}",
                        fence_idx,
                        &docs[fence_idx..]
                    );
                };

                res.push(RawDetailsBlock::Example {
                    body: &docs[fence_idx..fence_end],
                    title,
                });
                i = fence_end;
            }
            None => {
                res.push(RawDetailsBlock::Markdown(&docs[i..]));
                break;
            }
        }
    }

    res
}

/// Returns the start of a code fence and how many backticks it uses.
fn find_fence_start(md: &str) -> Option<(usize, usize)> {
    let start = md.find("```")?;
    let mut count = 3;
    while md[start + count..].starts_with('`') {
        count += 1;
    }
    Some((start, count))
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

/// Process cast information into types and strings.
fn casts(
    resolver: &dyn Resolver,
    types: &mut Vec<&'static str>,
    strings: &mut Vec<StrParam>,
    info: &CastInfo,
) {
    match info {
        CastInfo::Any => types.push("any"),
        CastInfo::Value(Value::Str(string), docs) => strings.push(StrParam {
            string: string.clone().into(),
            details: Html::markdown(resolver, docs, None),
        }),
        CastInfo::Value(..) => {}
        CastInfo::Type(ty) => types.push(ty.short_name()),
        CastInfo::Union(options) => {
            for option in options {
                casts(resolver, types, strings, option);
            }
        }
    }
}

/// Produce models for a function's scope.
fn scope_models(resolver: &dyn Resolver, name: &str, scope: &Scope) -> Vec<FuncModel> {
    scope
        .iter()
        .filter_map(|(_, binding)| {
            let Value::Func(func) = binding.read() else { return None };
            Some(func_model(resolver, func, &[name], true, binding.deprecation()))
        })
        .collect()
}

/// Produce an outline for a function page.
fn func_outline(model: &FuncModel, id_base: &str) -> Vec<OutlineItem> {
    let mut outline = vec![];

    if id_base.is_empty() {
<<<<<<< HEAD
        outline.push(OutlineItem {
            id: "summary".into(),
            name: "概要".into(),
            children: vec![],
        });
        outline.extend(model.details.outline());
=======
        outline.push(OutlineItem::from_name("Summary"));
        for block in &model.details {
            if let DetailsBlock::Html(html) = block {
                outline.extend(html.outline());
            }
        }
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

        if !model.params.is_empty() {
            outline.push(OutlineItem {
                id: "parameters".into(),
<<<<<<< HEAD
                name: "引数".into(),
=======
                name: "Parameters".into(),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
                children: model
                    .params
                    .iter()
                    .map(|param| OutlineItem {
                        id: eco_format!("parameters-{}", urlify(param.name)),
                        name: param.name.into(),
                        children: vec![],
                    })
                    .collect(),
            });
        }
    } else {
        outline.extend(model.params.iter().map(|param| OutlineItem {
            id: eco_format!("{id_base}-{}", urlify(param.name)),
            name: param.name.into(),
            children: vec![],
        }));
    }

    outline.extend(scope_outline(&model.scope, id_base));

    outline
}

/// Produce an outline for a function scope.
fn scope_outline(scope: &[FuncModel], id_base: &str) -> Option<OutlineItem> {
    if scope.is_empty() {
        return None;
    }

    let dash = if id_base.is_empty() { "" } else { "-" };
    let id = eco_format!("{id_base}{dash}definitions");

    let children = scope
        .iter()
        .map(|func| {
            let id = urlify(&eco_format!("{id}-{}", func.name));
            let children = func_outline(func, &id);
            OutlineItem { id, name: func.title.into(), children }
        })
        .collect();

<<<<<<< HEAD
    Some(OutlineItem { id, name: "定義".into(), children })
=======
    Some(OutlineItem { id, name: "Definitions".into(), children })
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

/// Create a page for a group of functions.
fn group_page(
    resolver: &dyn Resolver,
    parent: &str,
    group: &GroupData,
) -> (PageModel, CategoryItem) {
    let mut functions = vec![];
<<<<<<< HEAD
    let mut outline = vec![OutlineItem {
        id: "summary".into(),
        name: "概要".into(),
        children: vec![],
    }];
=======
    let mut outline = vec![OutlineItem::from_name("Summary")];
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

    let path: Vec<_> = group.path.iter().map(|s| s.as_str()).collect();
    let details = Html::markdown(resolver, &group.details, Some(1));
    outline.extend(details.outline());

    let mut outline_items = vec![];
    for name in &group.filter {
        let binding = group.module().scope().get(name).unwrap();
        let Ok(ref func) = binding.read().clone().cast::<Func>() else {
            panic!("not a function")
        };
        let func = func_model(resolver, func, &path, true, binding.deprecation());
        let id_base = urlify(&eco_format!("functions-{}", func.name));
        let children = func_outline(&func, &id_base);
        outline_items.push(OutlineItem {
            id: id_base,
            name: func.title.into(),
            children,
        });
        functions.push(func);
    }

    outline.push(OutlineItem {
        id: "functions".into(),
        name: "Functions".into(),
        children: outline_items,
    });

<<<<<<< HEAD
=======
    let global_attributes = if group.name == "typed" {
        let div = group.module().scope().get("div").unwrap();
        let func = div.read().clone().cast::<Func>().unwrap();
        func.params()
            .unwrap()
            .iter()
            .filter(|param| is_global_html_attr(param.name))
            .map(|info| param_model(resolver, info))
            .collect()
    } else {
        vec![]
    };

    if !global_attributes.is_empty() {
        let id = "global-attributes";
        outline.push(OutlineItem {
            id: id.into(),
            name: "Global Attributes".into(),
            children: global_attributes
                .iter()
                .map(|param| OutlineItem {
                    id: eco_format!("{id}-{}", urlify(param.name)),
                    name: param.name.into(),
                    children: vec![],
                })
                .collect(),
        });
    }

>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    let model = PageModel {
        route: eco_format!("{parent}{}/", group.name),
        title: group.title.clone(),
        description: eco_format!("Documentation for the {} functions.", group.name),
        part: None,
        outline,
        body: BodyModel::Group(GroupModel {
            name: group.name.clone(),
            title: group.title.clone(),
            details,
            functions,
<<<<<<< HEAD
=======
            global_attributes,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        }),
        children: vec![],
    };

    let item = CategoryItem {
        name: group.name.clone(),
        route: model.route.clone(),
<<<<<<< HEAD
        oneliner: oneliner(&group.details).into(),
=======
        oneliner: oneliner(&group.details),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        code: false,
    };

    (model, item)
}

<<<<<<< HEAD
=======
/// Whether the given `name` is one of a global HTML attribute (shared by all
/// elements).
fn is_global_html_attr(name: &str) -> bool {
    use typst_assets::html as data;
    data::ATTRS[..data::ATTRS_GLOBAL]
        .iter()
        .any(|global| global.name == name)
}

>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
/// Create a page for a type.
fn type_page(resolver: &dyn Resolver, parent: &str, ty: &Type) -> PageModel {
    let model = type_model(resolver, ty);
    PageModel {
        route: eco_format!("{parent}{}/", urlify(ty.short_name())),
        title: ty.title().into(),
        description: eco_format!("Documentation for the {} type.", ty.title()),
        part: None,
        outline: type_outline(&model),
        body: BodyModel::Type(model),
        children: vec![],
    }
}

/// Produce a type's model.
fn type_model(resolver: &dyn Resolver, ty: &Type) -> TypeModel {
    TypeModel {
        name: ty.short_name(),
        title: ty.title(),
        keywords: ty.keywords(),
        oneliner: oneliner(ty.docs()),
        details: Html::markdown(resolver, ty.docs(), Some(1)),
        constructor: ty
            .constructor()
            .ok()
            .map(|func| func_model(resolver, &func, &[], true, None)),
        scope: scope_models(resolver, ty.short_name(), ty.scope()),
    }
}

/// Produce an outline for a type page.
fn type_outline(model: &TypeModel) -> Vec<OutlineItem> {
<<<<<<< HEAD
    let mut outline = vec![OutlineItem {
        id: "summary".into(),
        name: "概要".into(),
        children: vec![],
    }];
=======
    let mut outline = vec![OutlineItem::from_name("Summary")];
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    outline.extend(model.details.outline());

    if let Some(func) = &model.constructor {
        outline.push(OutlineItem {
            id: "constructor".into(),
<<<<<<< HEAD
            name: "コンストラクタ".into(),
=======
            name: "Constructor".into(),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            children: func_outline(func, "constructor"),
        });
    }

    outline.extend(scope_outline(&model.scope, ""));
    outline
}

/// Create a page for symbols.
fn symbols_page(resolver: &dyn Resolver, parent: &str, group: &GroupData) -> PageModel {
    let model = symbols_model(resolver, group);
    PageModel {
        route: eco_format!("{parent}{}/", group.name),
        title: group.title.clone(),
        description: eco_format!("Documentation for the `{}` module.", group.name),
        part: None,
        outline: vec![],
        body: BodyModel::Symbols(model),
        children: vec![],
    }
}

/// Produce a symbol list's model.
fn symbols_model(resolver: &dyn Resolver, group: &GroupData) -> SymbolsModel {
    let mut list = vec![];
    for (name, binding) in group.module().scope().iter() {
        let Value::Symbol(symbol) = binding.read() else { continue };
<<<<<<< HEAD
        let complete = |variant: &str| {
            if variant.is_empty() {
                name.clone()
            } else {
                eco_format!("{}.{}", name, variant)
            }
        };

        for (variant, c) in symbol.variants() {
            let shorthand = |list: &[(&'static str, char)]| {
                list.iter().copied().find(|&(_, x)| x == c).map(|(s, _)| s)
            };

            let name = complete(variant);
            let deprecation = match name.as_str() {
                "integral.sect" => {
                    Some("`integral.sect` is deprecated, use `integral.inter` instead")
                }
                _ => binding.deprecation(),
            };
=======
        let complete = |variant: codex::ModifierSet<&str>| {
            if variant.is_empty() {
                name.clone()
            } else {
                eco_format!("{}.{}", name, variant.as_str())
            }
        };

        for (variant, value, deprecation_message) in symbol.variants() {
            let value_char = value.parse::<char>().ok();

            let shorthand = |list: &[(&'static str, char)]| {
                value_char.and_then(|c| {
                    list.iter().copied().find(|&(_, x)| x == c).map(|(s, _)| s)
                })
            };

            let name = complete(variant);
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

            list.push(SymbolModel {
                name,
                markup_shorthand: shorthand(typst::syntax::ast::Shorthand::LIST),
                math_shorthand: shorthand(typst::syntax::ast::MathShorthand::LIST),
<<<<<<< HEAD
                math_class: typst_utils::default_math_class(c).map(math_class_name),
                codepoint: c as _,
                accent: typst::math::Accent::combine(c).is_some(),
                alternates: symbol
                    .variants()
                    .filter(|(other, _)| other != &variant)
                    .map(|(other, _)| complete(other))
                    .collect(),
                deprecation,
=======
                // Matches `typst_layout::math::GlyphFragment::new`
                math_class: value.chars().next().and_then(|c| {
                    typst_utils::default_math_class(c).map(math_class_name)
                }),
                value: value.into(),
                // Matches casting `Symbol` to `Accent`
                accent: value_char
                    .is_some_and(|c| typst::math::Accent::combine(c).is_some()),
                alternates: symbol
                    .variants()
                    .filter(|(other, _, _)| other != &variant)
                    .map(|(other, _, _)| complete(other))
                    .collect(),
                deprecation_message: deprecation_message
                    .or_else(|| binding.deprecation().map(Deprecation::message)),
                deprecation_until: binding.deprecation().and_then(Deprecation::until),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            });
        }
    }

    SymbolsModel {
        name: group.name.clone(),
        title: group.title.clone(),
        details: Html::markdown(resolver, &group.details, Some(1)),
        list,
    }
}

/// Extract a module from another module.
#[track_caller]
fn get_module<'a>(parent: &'a Module, name: &str) -> StrResult<&'a Module> {
    match parent.scope().get(name).map(Binding::read) {
        Some(Value::Module(module)) => Ok(module),
        _ => bail!("module doesn't contain module `{name}`"),
    }
}

/// Turn a title into an URL fragment.
pub fn urlify(title: &str) -> EcoString {
<<<<<<< HEAD
    match title {
        "チュートリアル" => "tutorial".into(),
        "Typstで執筆するには" => "writing-in-typst".into(),
        "書式を設定する" => "formatting".into(),
        "高度なスタイリング" => "advanced-styling".into(),
        "テンプレートを作成する" => "making-a-template".into(),
        "リファレンス" => "reference".into(),
        "構文" => "syntax".into(),
        "スタイル設定" => "styling".into(),
        "スクリプト記述" => "scripting".into(),
        "コンテキスト" => "context".into(),
        "ガイド" => "guides".into(),
        "LaTeXユーザー向けガイド" => "guide-for-latex-users".into(),
        "ページ設定ガイド" => "page-setup-guide".into(),
        "表ガイド" => "table-guide".into(),
        "更新日志" => "changelog".into(),
        "路线图" => "roadmap".into(),
        "社区" => "community".into(),
        "変更履歴" => "changelog".into(),
        "初期バージョン" => "earlier".into(),
        "日本語組版情報" => "japanese".into(),
        "日本語テンプレート" => "templates".into(),
        "日本語向けパッケージ" => "packages".into(),
        "日本語記事" => "articles".into(),
        "用語集" => "glossary".into(),
        _ => title
            .chars()
            .map(|c| c.to_ascii_lowercase())
            .map(|c| match c {
                'a'..='z' | '0'..='9' | '.' => c,
                _ => '-',
            })
            .collect(),
    }
}

/// Extract the first line of documentation.
fn oneliner(docs: &str) -> &str {
    docs.lines().next().unwrap_or_default()
=======
    title
        .chars()
        .map(|c| c.to_ascii_lowercase())
        .map(|c| match c {
            'a'..='z' | '0'..='9' | '.' => c,
            _ => '-',
        })
        .collect()
}

/// Extract the first line of documentation.
fn oneliner(docs: &str) -> EcoString {
    let paragraph = docs.split("\n\n").next().unwrap_or_default();
    let mut depth = 0;
    let mut period = false;
    let mut end = paragraph.len();
    for (i, c) in paragraph.char_indices() {
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            '.' if depth == 0 => period = true,
            c if period && c.is_whitespace() && !docs[..i].ends_with("e.g.") => {
                end = i;
                break;
            }
            _ => period = false,
        }
    }
    EcoString::from(&docs[..end]).replace("\r\n", " ").replace("\n", " ")
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

/// The order of types in the documentation.
fn type_index(ty: &str) -> usize {
    TYPE_ORDER.iter().position(|&v| v == ty).unwrap_or(usize::MAX)
}

const TYPE_ORDER: &[&str] = &[
    "any",
    "none",
    "auto",
    "bool",
    "int",
    "float",
    "length",
    "angle",
    "ratio",
    "relative",
    "fraction",
    "color",
    "gradient",
    "datetime",
    "duration",
    "str",
    "bytes",
    "regex",
    "label",
    "content",
    "array",
    "dict",
    "func",
    "args",
    "selector",
    "location",
    "direction",
    "alignment",
    "alignment2d",
    "stroke",
];

fn math_class_name(class: MathClass) -> &'static str {
    match class {
        MathClass::Normal => "Normal",
        MathClass::Alphabetic => "Alphabetic",
        MathClass::Binary => "Binary",
        MathClass::Closing => "Closing",
        MathClass::Diacritic => "Diacritic",
        MathClass::Fence => "Fence",
        MathClass::GlyphPart => "Glyph Part",
        MathClass::Large => "Large",
        MathClass::Opening => "Opening",
        MathClass::Punctuation => "Punctuation",
        MathClass::Relation => "Relation",
        MathClass::Space => "Space",
        MathClass::Unary => "Unary",
        MathClass::Vary => "Vary",
        MathClass::Special => "Special",
    }
}

/// Data about a collection of functions.
#[derive(Debug, Clone, Deserialize)]
struct GroupData {
    name: EcoString,
    title: EcoString,
    category: Category,
    #[serde(default)]
    path: Vec<EcoString>,
    #[serde(default)]
    filter: Vec<EcoString>,
    details: EcoString,
}

impl GroupData {
    fn module(&self) -> &'static Module {
        let mut focus = &LIBRARY.global;
        for path in &self.path {
            focus = get_module(focus, path).unwrap();
        }
        focus
    }
}

#[cfg(test)]
mod tests {
    use super::*;
<<<<<<< HEAD
    use md5;
    use std::io::Write;
    use std::path::Path;
    use typst::layout::PagedDocument;

    #[test]
    fn test_docs() {
        // remove all files in ../assets/docs
        let _ = std::fs::remove_dir_all("../assets/docs");
        // create ../assets/docs directory
        let _ = std::fs::create_dir_all("../assets/docs");
        // convert all pages to html and generate example images to ../assets/docs
        let pages = provide(&TestResolver);
        // convert pages to JSON and save to ../assets/docs.json
        let json = serde_json::to_string_pretty(&pages).unwrap();
        let mut file = std::fs::File::create("../assets/docs.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
=======

    #[test]
    fn test_docs() {
        provide(&TestResolver);
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }

    struct TestResolver;

    impl Resolver for TestResolver {
        fn link(&self, _: &str) -> Option<String> {
            None
        }

<<<<<<< HEAD
        fn example(
            &self,
            _: u128,
            source: Option<Html>,
            document: &PagedDocument,
        ) -> Html {
            let page = document.pages.first().unwrap();
            // convert frames to a png
            let ppi = 2.0;
            // the first frame is the main frame
            let pixmap = typst_render::render(page, ppi);
            // Get a random filename by md5
            match source {
                Some(source) => {
                    let filename = format!("{:x}.png", md5::compute(source.as_str()));
                    let path = Path::new("../assets/docs").join(filename.clone());
                    let _ = pixmap.save_png(path).map_err(|_| "failed to write PNG file");
                    Html::new(format!(
                        r#"<div class="previewed-code"><pre>{}</pre><div class="preview"><img src="/assets/docs/{}" alt="Preview" width="480" height="190"/></div></div>"#,
                        source.as_str(),
                        filename
                    ))
                }
                _ => Html::new(String::new()),
            }
        }

        fn image(&self, filename: &str, data: &[u8]) -> String {
            // Set the output path
            let output_path = Path::new("../assets/docs").join(filename);

            // Create parent directories if they don't exist
            if let Some(parent) = output_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }

            // Write the file
            std::fs::write(&output_path, data).ok();

            // return /assets/docs/<filename>
            format!("/assets/docs/{}", filename)
=======
        fn example(&self, _: u128, _: Option<Html>, _: &PagedDocument) -> Html {
            Html::new(String::new())
        }

        fn image(&self, _: &str, _: &[u8]) -> String {
            String::new()
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        }

        fn commits(&self, _: &str, _: &str) -> Vec<Commit> {
            vec![]
        }

        fn base(&self) -> &str {
<<<<<<< HEAD
            "/docs/"
=======
            "/"
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        }
    }
}
