import { describe, expect, it } from "vitest";
import { applyBasePath, joinPath } from "./path";

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
});

describe("applyBasePath", () => {
	it("should apply basePath to absolute path", () => {
		expect(applyBasePath("/base", "/foo")).toBe("/base/foo");
		expect(applyBasePath("/foo", "/bar/baz")).toBe("/foo/bar/baz");
	});

	it("should not apply basePath to relative path", () => {
		expect(applyBasePath("/base", "./index.html")).toBe("./index.html");
		expect(applyBasePath("/foo/bar", "baz/qux")).toBe("baz/qux");
	});

	it("should handle root basePath", () => {
		expect(applyBasePath("/", "/foo")).toBe("/foo");
	});

	it("should handle empty basePath", () => {
		expect(applyBasePath("", "/foo")).toBe("/foo");
	});
});
