/**
 * Joins two paths by resolving any slash collisions between them.
 *
 * @param basePath - The base path to join.
 * @param path - The path to append to the basePath.
 * @returns The joined path with redundant slashes resolved.
 *
 * @example
 * ```ts
 * joinPath("/base/", "/foo") -> "/base/foo"
 * joinPath("/base", "foo") -> "/base/foo"
 * joinPath("/base", "/foo/bar") -> "/base/foo/bar"
 * joinPath("/foo/bar/", "/baz/qux") -> "/foo/bar/baz/qux"
 * joinPath("/foo/bar", "baz/qux") -> "/foo/bar/baz/qux"
 * ```
 */
export const joinPath = (basePath: string, path: string): string => {
	// Remove trailing slash from base.
	const baseClean = basePath !== "/" ? basePath.replace(/\/+$/, "") : basePath;
	// Remove leading slash from path.
	const pathClean = path.replace(/^\/+/, "");
	// Special case: if base is '/', avoid double slash
	if (baseClean === "/") {
		return `/${pathClean}`;
	}
	return `${baseClean}/${pathClean}`;
};

/**
 * Applies a base path to internal non-relative URLs (starting with `/`).
 * If the path is relative, it is returned as-is.
 *
 * @param basePath - The base path to apply.
 * @param path - The original path to modify.
 * @returns The modified path with the basePath applied if it was an absolute path.
 *
 * @example
 * ```ts
 * applyBasePath("/base", "/foo") -> "/base/foo"
 * applyBasePath("/base", "./index.html") -> "./index.html"
 * applyBasePath("/foo", "/bar/baz") -> "/foo/bar/baz"
 * applyBasePath("/foo/bar", "baz/qux") -> "baz/qux"
 * ```
 */
export const applyBasePath = (basePath: string, path: string): string => {
	const isRelative = !path.startsWith("/");
	return isRelative ? path : joinPath(basePath, path);
};
