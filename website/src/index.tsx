import { Hono } from "hono";
import { appendTrailingSlash, trimTrailingSlash } from "hono/trailing-slash";
import {
	CategoryTemplate,
	FuncTemplate,
	GroupTemplate,
	HtmlTemplate,
	SymbolsTemplate,
	TypeTemplate,
} from "./components/templates";
import type { Body, Page } from "./types/model";
import { flattenDocs } from "./utils/flattenDocs";
import { isPageOfKind } from "./utils/isPageOfKind";
import { registerRoutes } from "./utils/translationStatus";

// typst-docsが生成したドキュメント
import docsJson from "../../assets/docs.json";
const docs = docsJson as unknown as Page[];

const [flattenedPages, pagePaths] = flattenDocs(docs);

// 未知のページを未翻訳として登録する
const allRoutes = flattenedPages.map((page) => page.route);
registerRoutes(allRoutes);

const app = new Hono();
app.use(appendTrailingSlash());
app.use(trimTrailingSlash());

flattenedPages.forEach((page, pageIndex) => {
	const path = pagePaths[pageIndex];

	const previousPage: Page | null =
		pageIndex > 0 ? flattenedPages[pageIndex - 1] : null;
	const nextPage: Page | null =
		pageIndex < flattenedPages.length - 1
			? flattenedPages[pageIndex + 1]
			: null;

	const commonProps = {
		docs,
		path,
		previousPage: previousPage || undefined,
		nextPage: nextPage || undefined,
	};

	app.get(page.route, (c) => {
		if (isPageOfKind(page, "html")) {
			return c.html(<HtmlTemplate page={page} {...commonProps} />);
		}
		if (isPageOfKind(page, "category")) {
			return c.html(<CategoryTemplate page={page} {...commonProps} />);
		}
		if (isPageOfKind(page, "func")) {
			return c.html(<FuncTemplate page={page} {...commonProps} />);
		}
		if (isPageOfKind(page, "group")) {
			return c.html(<GroupTemplate page={page} {...commonProps} />);
		}
		if (isPageOfKind(page, "type")) {
			return c.html(<TypeTemplate page={page} {...commonProps} />);
		}
		if (isPageOfKind(page, "symbols")) {
			return c.html(<SymbolsTemplate page={page} {...commonProps} />);
		}

		return c.notFound();
	});
});

export default app;
