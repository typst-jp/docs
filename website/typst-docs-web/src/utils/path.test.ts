import { describe, expect, it } from "vitest";
import { joinPath, removeBasePath, shiftBase } from "./path";

describe("joinPath", () => {
	it("should join base and path with single slash", () => {
		expect(joinPath("/base/", "/foo")).toBe("/base/foo");
		expect(joinPath("/base", "foo")).toBe("/base/foo");
		expect(joinPath("/base", "/foo/bar")).toBe("/base/foo/bar");
		expect(joinPath("/foo/bar/", "/baz/qux")).toBe("/foo/bar/baz/qux");
		expect(joinPath("/foo/bar", "baz/qux")).toBe("/foo/bar/baz/qux");
	});

	it("should handle root base path correctly", () => {
		expect(joinPath("/", "/foo")).toBe("/foo");
		expect(joinPath("/", "foo")).toBe("/foo");
	});

	it("should handle empty path", () => {
		expect(joinPath("/base", "")).toBe("/base/");
		expect(joinPath("/base/", "")).toBe("/base/");
	});

	it("should handle empty base", () => {
		expect(joinPath("", "/foo")).toBe("/foo");
		expect(joinPath("", "foo")).toBe("/foo");
	});

	it("should join multiple parts", () => {
		expect(joinPath("/base", "foo", "bar")).toBe("/base/foo/bar");
		expect(joinPath("/", "foo", "bar")).toBe("/foo/bar");
		expect(joinPath("", "foo", "bar")).toBe("/foo/bar");
		expect(joinPath("/base/", "/foo/", "/bar/")).toBe("/base/foo/bar/");
	});

	it("should handle parts with trailing and leading slashes", () => {
		expect(joinPath("/base/", "/foo/", "/bar/")).toBe("/base/foo/bar/");
		expect(joinPath("/base/", "foo/", "/bar")).toBe("/base/foo/bar");
		expect(joinPath("/base", "foo/", "bar/")).toBe("/base/foo/bar/");
	});

	it("should handle parts with only slashes", () => {
		expect(joinPath("/", "/", "/")).toBe("/");
		expect(joinPath("/", "", "/foo")).toBe("/foo");
	});

	it("should preserve trailing slash if last part has it", () => {
		expect(joinPath("/base", "foo/", "bar/")).toBe("/base/foo/bar/");
		expect(joinPath("/base", "foo", "bar/")).toBe("/base/foo/bar/");
	});

	it("should handle special paths with leading slash", () => {
		expect(joinPath("/base", "/@")).toBe("/base/@");
		expect(joinPath("/base", "/node_modules")).toBe("/base/node_modules");
	});

	it("should remove duplicate slashes safely", () => {
		expect(joinPath("/base//", "/foo//bar/")).toBe("/base/foo/bar/");
		expect(joinPath("//", "/foo//", "/bar//baz//")).toBe("/foo/bar/baz/");
		expect(joinPath("/", "/", "/foo//bar//baz/")).toBe("/foo/bar/baz/");
		expect(joinPath("/base//", "//foo//", "//bar//")).toBe("/base/foo/bar/");
		expect(joinPath("base//", "/foo//bar/")).toBe("base/foo/bar/");
		expect(joinPath("base//", "foo//bar/")).toBe("base/foo/bar/");
		expect(joinPath("/base//", "foo//bar")).toBe("/base/foo/bar");
		expect(joinPath("base//", "foo//bar")).toBe("base/foo/bar");
	});

	it("should handle only slash and empty", () => {
		expect(joinPath("/")).toBe("/");
		expect(joinPath("", "/")).toBe("/");
		expect(joinPath("", "")).toBe("");
	});

	it("should handle mix of empty, slash, and normal parts", () => {
		expect(joinPath("", "/", "foo")).toBe("/foo");
		expect(joinPath("", "foo", "")).toBe("/foo/");
		expect(joinPath("", "foo", "/")).toBe("/foo/");
		expect(joinPath("/", "", "foo")).toBe("/foo");
		expect(joinPath("/", "foo", "")).toBe("/foo/");
	});

	it("should join when first part is http(s):// URL", () => {
		const paths = [
			"http://typst.app/docs",
			"http://typst.app/docs/",
			"https://typst.app/docs",
			"https://typst.app/docs/",
		];
		for (const base of paths) {
			const b = base.replace(/\/$/, "");
			expect(joinPath(base, "foo")).toBe(`${b}/foo`);
			expect(joinPath(base, "foo/")).toBe(`${b}/foo/`);
			expect(joinPath(base, "/foo")).toBe(`${b}/foo`);
			expect(joinPath(base, "/foo/")).toBe(`${b}/foo/`);
			expect(joinPath(base, "foo/bar")).toBe(`${b}/foo/bar`);
			expect(joinPath(base, "foo/bar/")).toBe(`${b}/foo/bar/`);
			expect(joinPath(base, "/foo/bar")).toBe(`${b}/foo/bar`);
			expect(joinPath(base, "/foo/bar/")).toBe(`${b}/foo/bar/`);
		}
	});
});

describe("removeBasePath", () => {
	it("should remove basePath with trailing slash", () => {
		expect(removeBasePath("/docs/", "/docs/foo/bar")).toBe("/foo/bar");
		expect(removeBasePath("/base/", "/base/foo")).toBe("/foo");
	});
	it("should remove basePath without trailing slash", () => {
		expect(removeBasePath("/docs", "/docs/foo/bar")).toBe("/foo/bar");
		expect(removeBasePath("/base", "/base/foo")).toBe("/foo");
	});
	it("should return route unchanged if it does not start with basePath", () => {
		expect(removeBasePath("/docs", "/other/foo")).toBe("/other/foo");
		expect(removeBasePath("/base", "/docs/foo")).toBe("/docs/foo");
	});
	it("should handle root basePath", () => {
		expect(removeBasePath("/", "/foo/bar")).toBe("/foo/bar");
	});
	it("should handle empty basePath", () => {
		expect(removeBasePath("", "/foo/bar")).toBe("/foo/bar");
	});
});

describe("shiftBase", () => {
	describe("normal usages", () => {
		it("should handle non-root oldBasePath and full newBaseUrl", () => {
			for (const oldBasePath of ["/base", "/base/"]) {
				for (const newBaseUrl of [
					"https://typst.app/docs",
					"https://typst.app/docs/",
				]) {
					expect(shiftBase("/base/foo/bar/", oldBasePath, newBaseUrl)).toBe(
						"https://typst.app/docs/foo/bar/",
					);
					expect(shiftBase("/base/", oldBasePath, newBaseUrl)).toBe(
						"https://typst.app/docs/",
					);
				}
			}
		});

		it("should handle root oldBasePath and full newBaseUrl", () => {
			for (const oldBasePath of ["/", ""]) {
				for (const newBaseUrl of [
					"https://typst.app/docs",
					"https://typst.app/docs/",
				]) {
					expect(shiftBase("/foo/bar/", oldBasePath, newBaseUrl)).toBe(
						"https://typst.app/docs/foo/bar/",
					);
					expect(shiftBase("/", oldBasePath, newBaseUrl)).toBe(
						"https://typst.app/docs/",
					);
				}
			}
		});

		it("should handle non-root oldBasePath and newBaseUrl without origin", () => {
			for (const oldBasePath of ["/ja-JP/docs", "/ja-JP/docs/"]) {
				for (const newBaseUrl of ["/en-US/docs", "/en-US/docs/"]) {
					expect(
						shiftBase("/ja-JP/docs/foo/bar/", oldBasePath, newBaseUrl),
					).toBe("/en-US/docs/foo/bar/");
					expect(shiftBase("/ja-JP/docs/", oldBasePath, newBaseUrl)).toBe(
						"/en-US/docs/",
					);
				}
			}
		});
	});

	// The following tests are generated by AI automatically. They describe the behaviors in edge cases.
	// However, these behaviors are not actually used, and their result and may not meet actual needs.
	// Therefore, the following usages should be avoided in practice.
	describe("generated usages", () => {
		it("should handle edge cases with empty oldBasePath and newBaseUrl", () => {
			expect(shiftBase("/foo/bar", "", "")).toBe("/foo/bar");
			expect(shiftBase("/", "", "")).toBe("/");
			expect(shiftBase("", "", "")).toBe("");
		});

		it("should handle edge cases with empty route", () => {
			expect(shiftBase("", "/base", "/new")).toBe("/new/");
			expect(shiftBase("", "/", "/new")).toBe("/new/");
			expect(shiftBase("", "", "/new")).toBe("/new/");
		});

		it("should handle edge cases with only slashes", () => {
			expect(shiftBase("/", "/", "/new")).toBe("/new/");
			expect(shiftBase("/", "/base", "/new")).toBe("/new/");
			expect(shiftBase("/base", "/base", "/new")).toBe("/new/");
		});

		it("should handle routes with duplicate slashes", () => {
			expect(shiftBase("/base//foo//bar", "/base", "/new")).toBe(
				"/new/foo/bar",
			);
			expect(shiftBase("//base//foo//", "/base", "/new")).toBe(
				"/new/base/foo/",
			);
			expect(shiftBase("//base//", "/base", "/new")).toBe("/new/base/");
		});

		it("should handle routes with special characters", () => {
			expect(shiftBase("/base/@/foo", "/base", "/new")).toBe("/new/@/foo");
			expect(shiftBase("/base/#/foo", "/base", "/new")).toBe("/new/#/foo");
			expect(shiftBase("/base/!$/foo", "/base", "/new")).toBe("/new/!$/foo");
		});

		it("should handle routes with trailing slashes in newBaseUrl", () => {
			expect(shiftBase("/base/foo", "/base", "/new/")).toBe("/new/foo");
			expect(shiftBase("/base/foo/", "/base", "/new/")).toBe("/new/foo/");
			expect(shiftBase("/base/", "/base", "/new/")).toBe("/new/");
		});

		it("should handle routes with trailing slashes in oldBasePath", () => {
			expect(shiftBase("/base/foo", "/base/", "/new")).toBe("/new/foo");
			expect(shiftBase("/base/foo/", "/base/", "/new")).toBe("/new/foo/");
			expect(shiftBase("/base/", "/base/", "/new")).toBe("/new/");
		});

		it("should handle routes with mixed slashes and empty parts", () => {
			expect(shiftBase("/base//foo", "/base", "/new")).toBe("/new/foo");
			expect(shiftBase("/base/foo//", "/base", "/new")).toBe("/new/foo/");
			expect(shiftBase("/base//", "/base", "/new")).toBe("/new/");
		});

		it("should handle newBaseUrl with https://", () => {
			expect(shiftBase("/base/foo", "/base", "https://example.com/new")).toBe(
				"https://example.com/new/foo",
			);
			expect(shiftBase("/base/foo/", "/base", "https://example.com/new")).toBe(
				"https://example.com/new/foo/",
			);
			expect(shiftBase("/base/", "/base", "https://example.com/new")).toBe(
				"https://example.com/new/",
			);
			expect(shiftBase("/base", "/base", "https://example.com/new")).toBe(
				"https://example.com/new/",
			);
		});

		it("should handle newBaseUrl with //example.com", () => {
			expect(shiftBase("/base/foo", "/base", "//example.com/new")).toBe(
				"//example.com/new/foo",
			);
			expect(shiftBase("/base/foo/", "/base", "//example.com/new")).toBe(
				"//example.com/new/foo/",
			);
			expect(shiftBase("/base/", "/base", "//example.com/new")).toBe(
				"//example.com/new/",
			);
			expect(shiftBase("/base", "/base", "//example.com/new")).toBe(
				"//example.com/new/",
			);
		});

		it("should handle newBaseUrl with only origin and no path", () => {
			expect(shiftBase("/base/foo", "/base", "https://example.com")).toBe(
				"https://example.com/foo",
			);
			expect(shiftBase("/base/foo/", "/base", "https://example.com")).toBe(
				"https://example.com/foo/",
			);
			expect(shiftBase("/base/", "/base", "https://example.com")).toBe(
				"https://example.com/",
			);
			expect(shiftBase("/base", "/base", "https://example.com")).toBe(
				"https://example.com/",
			);
		});
	});
});
