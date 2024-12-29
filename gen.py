"""typst-docsが出力したJSONファイルを読み込んでHTMLファイルを出力する"""

import json
import shutil
from pathlib import Path
from typing import Any, TypedDict

import jinja2


class OutlineItemDict(TypedDict):
    """アウトライン情報"""

    id: str
    name: str
    children: list["OutlineItemDict"]


class BodyDict(TypedDict):
    """本文情報"""

    kind: str
    # NOTE: dict[str, Any]の正確な型はtypst-docsの出力を参照してください
    content: str | dict[str, Any]


class PageDict(TypedDict):
    """ページ情報"""

    route: str
    title: str
    description: str
    part: str | None
    outline: list[OutlineItemDict]
    body: BodyDict
    children: list["PageDict"]


def type2href(parameter_type: str) -> str | None:
    """型名からリンクを取得する"""
    foundation_set = set(
        (
            "arguments",
            "array",
            "auto",
            "bool",
            "bytes",
            "content",
            "datetime",
            "decimal",
            "dictionary",
            "duration",
            "float",
            "function",
            "int",
            "label",
            "module",
            "none",
            "plugin",
            "regex",
            "selector",
            "str",
            "type",
            "version",
        )
    )
    layout_set = set(
        (
            "alignment",
            "angle",
            "direction",
            "fraction",
            "length",
            "ratio",
            "relative",
        )
    )
    visualize_set = set(
        (
            "color",
            "gradient",
            "pattern",
            "stroke",
        )
    )
    introspection_set = set(
        (
            "counter",
            "location",
            "state",
        )
    )
    if parameter_type in foundation_set:
        return f"foundations/{parameter_type}"
    elif parameter_type in layout_set:
        return f"layout/{parameter_type}"
    elif parameter_type in visualize_set:
        return f"visualize/{parameter_type}"
    elif parameter_type in introspection_set:
        return f"introspection/{parameter_type}"
    else:
        return None


def type2class(parameter_type: str) -> str:
    """型名からCSSのクラス名を取得する"""
    type2class_map: dict[str, str] = {
        "none": "pill-kw",
        "auto": "pill-kw",
        "function": "pill-fn",
        "string": "pill-str",
        "str": "pill-str",
        "content": "pill-con",
        "color": "pill-col",
        "bool": "pill-bool",
        "boolean": "pill-bool",
        "integer": "pill-num",
        "int": "pill-num",
        "ratio": "pill-num",
        "length": "pill-num",
        "relative length": "pill-num",
        "float": "pill-num",
        "angle": "pill-num",
        "fraction": "pill-num",
    }
    return type2class_map.get(parameter_type, "pill-obj")


def gen_path(item: dict[str, Any]) -> str:
    """pathを連結する"""
    return "".join([s + "." for s in item["path"]])


def render_jinja_html(
    template_loc: str,
    file_name: str,
    **context: Any,  # noqa: ANN401
) -> str:
    """Jinja2テンプレートをレンダリングする"""
    return (
        jinja2.Environment(
            loader=jinja2.FileSystemLoader(f"{template_loc}/"),
            autoescape=False,  # noqa: S701
        )
        .get_template(file_name)
        .render(context)
    )


if __name__ == "__main__":
    # 出力先のディレクトリを初期化する
    if Path("./dist").exists():
        shutil.rmtree("./dist")

    shutil.copytree("./static", "./dist")

    if Path("./dist/assets/docs").exists():
        shutil.rmtree("./dist/assets/docs")

    shutil.copytree("./assets/docs", "./dist/assets/docs")

    # typst-docsが出力したJSONファイルを読み込む
    docs: list[PageDict] = []
    with Path("./assets/docs.json").open(encoding="utf-8") as f:
        docs = json.load(f)

    # ドキュメントの階層構造を平坦化する
    # パンくずリストと前後のページ情報を取得するため
    flattened_pages: list[PageDict] = []  # 平坦化されたページ情報のリスト
    page_paths: list[list[PageDict]] = []  # ページ情報[i]のパス情報

    def _flatten_docs(page: PageDict, page_path: list[PageDict]) -> None:
        """ドキュメントの階層構造を平坦化する

        Args:
            page (PageDict): ページ情報
            page_path (list[PageDict]): ページ情報のパス
        """
        flattened_pages.append(page)
        page_paths.append(page_path)
        for child_page in page["children"]:
            _flatten_docs(child_page, [*page_path, child_page])

    for page in docs:
        _flatten_docs(page, [page])

    # ドキュメントをHTMLファイルに出力する
    def _render_to_file(
        page: PageDict,
        path: list[PageDict],
        page_index: int,
    ) -> None:
        """ページをHTMLファイルに出力する

        Args:
            page (PageDict): ページ情報
            path (list[PageDict]): パンくずリストを生成するためのページ情報のリスト
            page_index (int): 前後のページ情報を取得するためのページのインデックス
        """
        previous_page: PageDict | None = (
            flattened_pages[page_index - 1] if page_index > 0 else None
        )
        next_page: PageDict | None = (
            flattened_pages[page_index + 1]
            if page_index < len(flattened_pages) - 1
            else None
        )
        if not Path(f"./dist{page['route']}").exists():
            Path(f"./dist{page['route']}").mkdir(parents=True)
        with Path(
            f"./dist{page['route']}{'/' if not page['route'].endswith('/') else ''}index.html",  # noqa: E501
        ).open(
            mode="w",
            encoding="utf-8",
        ) as f:
            f.write(
                render_jinja_html(
                    "./templates/",
                    f"{page['body']['kind']}_template.html.j2",
                    docs=docs,
                    path=path,
                    prev=previous_page,
                    next=next_page,
                    type2href=type2href,
                    type2class=type2class,
                    gen_path=gen_path,
                    **page,
                ),
            )
            print(f"Generated: {page['route']}")  # noqa: T201

    for i, (page, path) in enumerate(zip(flattened_pages, page_paths, strict=True)):
        _render_to_file(page, path, i)
