import { describe, expect, it } from "vitest";
import type { Page } from "../types/model";
import { flattenDocs } from "./flattenDocs";

describe("flattenDocs", () => {
	const createMockPage = (
		route: string,
		title: string,
		children: Page[] = [],
	): Page => ({
		route,
		title,
		description: `Description for ${title}`,
		part: null,
		outline: [],
		body: {
			kind: "html",
			content: `<p>${title} content</p>`,
		},
		children,
	});

	it("空の配列を渡すと空の結果を返す", () => {
		const [flattenedPages, pagePaths] = flattenDocs([]);

		expect(flattenedPages).toEqual([]);
		expect(pagePaths).toEqual([]);
	});

	it("子を持たない単一ページを平坦化する", () => {
		const page = createMockPage("/docs/", "Documentation");
		const [flattenedPages, pagePaths] = flattenDocs([page]);

		expect(flattenedPages).toEqual([page]);
		expect(pagePaths).toEqual([[page]]);
	});

	it("複数の子を持たないページを平坦化する", () => {
		const page1 = createMockPage("/docs/", "Documentation");
		const page2 = createMockPage("/tutorial/", "Tutorial");
		const [flattenedPages, pagePaths] = flattenDocs([page1, page2]);

		expect(flattenedPages).toEqual([page1, page2]);
		expect(pagePaths).toEqual([[page1], [page2]]);
	});

	it("子を持つページを平坦化する", () => {
		const childPage = createMockPage("/docs/tutorial/", "Tutorial");
		const parentPage = createMockPage("/docs/", "Documentation", [childPage]);
		const [flattenedPages, pagePaths] = flattenDocs([parentPage]);

		expect(flattenedPages).toEqual([parentPage, childPage]);
		expect(pagePaths).toEqual([[parentPage], [parentPage, childPage]]);
	});

	it("複数階層のページを平坦化する", () => {
		const grandChildPage = createMockPage("/docs/tutorial/basics/", "Basics");
		const childPage = createMockPage("/docs/tutorial/", "Tutorial", [
			grandChildPage,
		]);
		const parentPage = createMockPage("/docs/", "Documentation", [childPage]);
		const [flattenedPages, pagePaths] = flattenDocs([parentPage]);

		expect(flattenedPages).toEqual([parentPage, childPage, grandChildPage]);
		expect(pagePaths).toEqual([
			[parentPage],
			[parentPage, childPage],
			[parentPage, childPage, grandChildPage],
		]);
	});

	it("複数の子を持つページを平坦化する", () => {
		const child1 = createMockPage("/docs/tutorial/", "Tutorial");
		const child2 = createMockPage("/docs/reference/", "Reference");
		const parentPage = createMockPage("/docs/", "Documentation", [
			child1,
			child2,
		]);
		const [flattenedPages, pagePaths] = flattenDocs([parentPage]);

		expect(flattenedPages).toEqual([parentPage, child1, child2]);
		expect(pagePaths).toEqual([
			[parentPage],
			[parentPage, child1],
			[parentPage, child2],
		]);
	});

	it("複雑な階層構造を平坦化する", () => {
		// docs/
		//   ├── tutorial/
		//   │   ├── basics/
		//   │   └── advanced/
		//   └── reference/
		//       └── functions/
		const basics = createMockPage("/docs/tutorial/basics/", "Basics");
		const advanced = createMockPage("/docs/tutorial/advanced/", "Advanced");
		const tutorial = createMockPage("/docs/tutorial/", "Tutorial", [
			basics,
			advanced,
		]);

		const functions = createMockPage("/docs/reference/functions/", "Functions");
		const reference = createMockPage("/docs/reference/", "Reference", [
			functions,
		]);

		const docs = createMockPage("/docs/", "Documentation", [
			tutorial,
			reference,
		]);

		const [flattenedPages, pagePaths] = flattenDocs([docs]);

		expect(flattenedPages).toEqual([
			docs,
			tutorial,
			basics,
			advanced,
			reference,
			functions,
		]);
		expect(pagePaths).toEqual([
			[docs],
			[docs, tutorial],
			[docs, tutorial, basics],
			[docs, tutorial, advanced],
			[docs, reference],
			[docs, reference, functions],
		]);
	});

	it("複数のルートページを持つ階層構造を平坦化する", () => {
		const tutorialChild = createMockPage("/tutorial/basics/", "Basics");
		const tutorial = createMockPage("/tutorial/", "Tutorial", [tutorialChild]);

		const docsChild = createMockPage("/docs/reference/", "Reference");
		const docs = createMockPage("/docs/", "Documentation", [docsChild]);

		const [flattenedPages, pagePaths] = flattenDocs([tutorial, docs]);

		expect(flattenedPages).toEqual([tutorial, tutorialChild, docs, docsChild]);
		expect(pagePaths).toEqual([
			[tutorial],
			[tutorial, tutorialChild],
			[docs],
			[docs, docsChild],
		]);
	});

	it("パス情報が正しく設定される", () => {
		const grandChild = createMockPage("/a/b/c/", "C");
		const child = createMockPage("/a/b/", "B", [grandChild]);
		const parent = createMockPage("/a/", "A", [child]);

		const [flattenedPages, pagePaths] = flattenDocs([parent]);

		expect(pagePaths[0]).toEqual([parent]);
		expect(pagePaths[1]).toEqual([parent, child]);
		expect(pagePaths[2]).toEqual([parent, child, grandChild]);

		expect(pagePaths[0][0].route).toBe("/a/");
		expect(pagePaths[1][1].route).toBe("/a/b/");
		expect(pagePaths[2][2].route).toBe("/a/b/c/");
	});
});
