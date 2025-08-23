import { describe, expect, it } from "vitest";
import { joinPath, removeBasePath } from "./path";

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
