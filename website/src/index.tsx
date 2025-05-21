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

// typst-docsが生成したドキュメント
import docsJson from "../../assets/docs.json";
const docs = docsJson as unknown as Page[];

const [flattenedPages, pagePaths] = flattenDocs(docs);

const app = new Hono();
app.use(appendTrailingSlash());
app.use(trimTrailingSlash());
app.get("/", (c) => {
	return c.redirect("/docs");
});

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

app.get("/sitemap.xml", (c) => {
	const routes = ["/", ...flattenedPages.map((page) => page.route)];
	const today = new Date();
	const formattedDate = `${today.getFullYear()}-${String(today.getMonth() + 1).padStart(2, "0")}-${String(today.getDate()).padStart(2, "0")}`;

	const sitemap = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${routes
	.map(
		(route) => `	<url>
		<loc>https://typst-jp.github.io${route}</loc>
		<lastmod>${formattedDate}</lastmod>
	</url>`,
	)
	.join("\n")}
</urlset>
`;

	return c.text(sitemap, 200, {
		"Content-Type": "application/xml",
	});
});

export default app;
