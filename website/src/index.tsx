import { serveStatic } from "@hono/node-server/serve-static";
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
import { basePath } from "./metadata";
import type { Body, Page } from "./types/model";
import { flattenDocs } from "./utils/flattenDocs";
import { isPageOfKind } from "./utils/isPageOfKind";
import { removeBasePath } from "./utils/path";
import { registerRoutes } from "./utils/translationStatus";

// typst-docsが生成したドキュメント
import docsJson from "../../docs.json";
const docs = docsJson as unknown as Page[];

const [flattenedPages, pagePaths] = flattenDocs(docs);

// 未知のページを未翻訳として登録する
const allRoutes = flattenedPages.map((page) => page.route);
registerRoutes(allRoutes);

const app = new Hono().basePath(import.meta.env.DEV ? basePath : "/");
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

	// Remove basePath from the route if it starts with basePath.
	let route = page.route;
	if (!route.startsWith(basePath)) {
		throw new Error(
			`'route' does not start with 'basePath': route='${route}', basePath='${basePath}'.
The 'basePath' must match the 'base' value used in typst-docs.
Please ensure both this site and typst-docs are configured with the same base path.`,
		);
	}
	route = removeBasePath(basePath, route);
	app.get(route, (c) => {
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

if (import.meta.env.DEV) {
	app.use(
		"*",
		serveStatic({
			root: "./public",
			rewriteRequestPath: (path) => {
				return path.slice(basePath.length);
			},
			onNotFound: (path, c) => {
				console.log(
					`${path} is not found while trying to serve a static asset`,
				);
			},
		}),
	);
}

export default app;
