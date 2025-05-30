import { describe, expect, it } from "vitest";
import type { Page } from "../types/model";
import { isPageOfKind } from "./isPageOfKind";

describe("isPageOfKind", () => {
	const createBasePage = (): Omit<Page, "body"> => ({
		route: "/test/",
		title: "Test Page",
		description: "Test description",
		part: null,
		outline: [],
		children: [],
	});

	it("htmlページを正しく判定する", () => {
		const htmlPage: Page = {
			...createBasePage(),
			body: {
				kind: "html",
				content: "<p>HTML content</p>",
			},
		};

		expect(isPageOfKind(htmlPage, "html")).toBe(true);
		expect(isPageOfKind(htmlPage, "func")).toBe(false);
		expect(isPageOfKind(htmlPage, "category")).toBe(false);
		expect(isPageOfKind(htmlPage, "group")).toBe(false);
		expect(isPageOfKind(htmlPage, "type")).toBe(false);
		expect(isPageOfKind(htmlPage, "symbols")).toBe(false);
	});

	it("funcページを正しく判定する", () => {
		const funcPage: Page = {
			...createBasePage(),
			body: {
				kind: "func",
				content: {
					path: ["test"],
					name: "testFunc",
					title: "Test Function",
					keywords: ["test"],
					oneliner: "Test function",
					element: false,
					contextual: false,
					deprecation: null,
					details: "<p>Function details</p>",
					example: null,
					self: false,
					params: [],
					returns: [],
					scope: [],
				},
			},
		};

		expect(isPageOfKind(funcPage, "func")).toBe(true);
		expect(isPageOfKind(funcPage, "html")).toBe(false);
		expect(isPageOfKind(funcPage, "category")).toBe(false);
		expect(isPageOfKind(funcPage, "group")).toBe(false);
		expect(isPageOfKind(funcPage, "type")).toBe(false);
		expect(isPageOfKind(funcPage, "symbols")).toBe(false);
	});

	it("categoryページを正しく判定する", () => {
		const categoryPage: Page = {
			...createBasePage(),
			body: {
				kind: "category",
				content: {
					name: "test-category",
					title: "Test Category",
					details: "Category details",
					items: [],
					shorthands: null,
				},
			},
		};

		expect(isPageOfKind(categoryPage, "category")).toBe(true);
		expect(isPageOfKind(categoryPage, "html")).toBe(false);
		expect(isPageOfKind(categoryPage, "func")).toBe(false);
		expect(isPageOfKind(categoryPage, "group")).toBe(false);
		expect(isPageOfKind(categoryPage, "type")).toBe(false);
		expect(isPageOfKind(categoryPage, "symbols")).toBe(false);
	});

	it("groupページを正しく判定する", () => {
		const groupPage: Page = {
			...createBasePage(),
			body: {
				kind: "group",
				content: {
					name: "test-group",
					title: "Test Group",
					details: "<p>Group details</p>",
					functions: [],
				},
			},
		};

		expect(isPageOfKind(groupPage, "group")).toBe(true);
		expect(isPageOfKind(groupPage, "html")).toBe(false);
		expect(isPageOfKind(groupPage, "func")).toBe(false);
		expect(isPageOfKind(groupPage, "category")).toBe(false);
		expect(isPageOfKind(groupPage, "type")).toBe(false);
		expect(isPageOfKind(groupPage, "symbols")).toBe(false);
	});

	it("typeページを正しく判定する", () => {
		const typePage: Page = {
			...createBasePage(),
			body: {
				kind: "type",
				content: {
					name: "test-type",
					title: "Test Type",
					keywords: ["type"],
					oneliner: "Test type",
					details: "<p>Type details</p>",
					constructor: null,
					scope: [],
				},
			},
		};

		expect(isPageOfKind(typePage, "type")).toBe(true);
		expect(isPageOfKind(typePage, "html")).toBe(false);
		expect(isPageOfKind(typePage, "func")).toBe(false);
		expect(isPageOfKind(typePage, "category")).toBe(false);
		expect(isPageOfKind(typePage, "group")).toBe(false);
		expect(isPageOfKind(typePage, "symbols")).toBe(false);
	});

	it("symbolsページを正しく判定する", () => {
		const symbolsPage: Page = {
			...createBasePage(),
			body: {
				kind: "symbols",
				content: {
					name: "test-symbols",
					title: "Test Symbols",
					details: "<p>Symbols details</p>",
					list: [],
				},
			},
		};

		expect(isPageOfKind(symbolsPage, "symbols")).toBe(true);
		expect(isPageOfKind(symbolsPage, "html")).toBe(false);
		expect(isPageOfKind(symbolsPage, "func")).toBe(false);
		expect(isPageOfKind(symbolsPage, "category")).toBe(false);
		expect(isPageOfKind(symbolsPage, "group")).toBe(false);
		expect(isPageOfKind(symbolsPage, "type")).toBe(false);
	});

	it("型ガードとして正しく動作する", () => {
		const htmlPage: Page = {
			...createBasePage(),
			body: {
				kind: "html",
				content: "<p>HTML content</p>",
			},
		};

		const funcPage: Page = {
			...createBasePage(),
			body: {
				kind: "func",
				content: {
					path: ["test"],
					name: "testFunc",
					title: "Test Function",
					keywords: ["test"],
					oneliner: "Test function",
					element: false,
					contextual: false,
					deprecation: null,
					details: "<p>Function details</p>",
					example: null,
					self: false,
					params: [],
					returns: [],
					scope: [],
				},
			},
		};

		if (isPageOfKind(htmlPage, "html")) {
			expect(htmlPage.body.content).toBe("<p>HTML content</p>");
		}

		if (isPageOfKind(funcPage, "func")) {
			expect(funcPage.body.content.name).toBe("testFunc");
		}

		if (isPageOfKind(htmlPage, "func")) {
			expect.fail("この分岐は実行されるべきではない");
		}
	});

	it("複数のページタイプを配列で処理する", () => {
		const pages: Page[] = [
			{
				...createBasePage(),
				route: "/html/",
				body: { kind: "html", content: "<p>HTML</p>" },
			},
			{
				...createBasePage(),
				route: "/func/",
				body: {
					kind: "func",
					content: {
						path: [],
						name: "func",
						title: "Function",
						keywords: [],
						oneliner: "",
						element: false,
						contextual: false,
						deprecation: null,
						details: "",
						example: null,
						self: false,
						params: [],
						returns: [],
						scope: [],
					},
				},
			},
			{
				...createBasePage(),
				route: "/category/",
				body: {
					kind: "category",
					content: {
						name: "cat",
						title: "Category",
						details: "",
						items: [],
						shorthands: null,
					},
				},
			},
		];

		const htmlPages = pages.filter((page) => isPageOfKind(page, "html"));
		const funcPages = pages.filter((page) => isPageOfKind(page, "func"));
		const categoryPages = pages.filter((page) =>
			isPageOfKind(page, "category"),
		);

		expect(htmlPages).toHaveLength(1);
		expect(funcPages).toHaveLength(1);
		expect(categoryPages).toHaveLength(1);

		expect(htmlPages[0].route).toBe("/html/");
		expect(funcPages[0].route).toBe("/func/");
		expect(categoryPages[0].route).toBe("/category/");
	});
});
