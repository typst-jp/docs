/**
 * Join multiple URL path segments safely.
 *
 * @param parts - The path segments to join.
 * @returns The joined path with redundant slashes resolved.
 *
 * @example
 * ```ts
 * joinPath("/base/", "/foo") -> "/base/foo"
 * joinPath("/base", "foo") -> "/base/foo"
 * joinPath("/base", "/foo/bar") -> "/base/foo/bar"
 * joinPath("/foo/bar/", "/baz/qux") -> "/foo/bar/baz/qux"
 * joinPath("/foo/bar", "baz/qux") -> "/foo/bar/baz/qux"
 * joinPath("/base", "foo", "bar") -> "/base/foo/bar"
 * joinPath("/", "foo", "bar") -> "/foo/bar"
 * ```
 */
export const joinPath = (...parts: string[]): string => {
	if (parts.length === 0) return "";
	if (parts.length === 1) return parts[0];
	if (parts.every((p) => !p)) return "";

	const schemeMatch = parts[0].match(/^(https?:\/\/)(.*)$/);
	let joined: string;
	if (schemeMatch) {
		const scheme = schemeMatch[1];
		const rest = schemeMatch[2];
		joined = joinPath(rest, ...parts.slice(1));
		joined = scheme + joined;
	} else {
		let needsLeadingSlash = false;
		const firstNonEmpty = parts.find((p) => p !== "") || "";
		if (parts[0].startsWith("/")) {
			needsLeadingSlash = true;
		} else if (parts[0] === "") {
			if (firstNonEmpty.startsWith("/")) needsLeadingSlash = true;
		}
		let needsTrailingSlash = false;
		if (parts[parts.length - 1].endsWith("/")) needsTrailingSlash = true;
		const cleaned = parts.map((p, i) => {
			if (i === 0) return p.replace(/\/+$/, "");
			if (i === parts.length - 1) return p.replace(/^\/+/, "");
			return p.replace(/^\/+|\/+$/g, "");
		});
		joined = cleaned.filter((v, i) => v !== "" || parts[i] === "").join("/");
		if (needsLeadingSlash && !joined.startsWith("/")) joined = `/${joined}`;
		if (needsTrailingSlash && !joined.endsWith("/")) joined = `${joined}/`;
	}
	return joined.replace(/([^:])\/+/g, "$1/");
};

/**
 * Removes the basePath prefix from a route string.
 *
 * @param basePath - The base path to remove.
 * @param route - The route string to process.
 * @returns The route string with basePath removed from the start.
 *
 * @example
 * ```ts
 * removeBasePath("/docs/", "/docs/foo/bar") -> "/foo/bar"
 * removeBasePath("/docs", "/docs/foo/bar") -> "/foo/bar"
 * ```
 */
export const removeBasePath = (basePath: string, route: string): string => {
	if (!route.startsWith(basePath)) return route;
	const offset = basePath.length - (basePath.endsWith("/") ? 1 : 0);
	return route.slice(offset);
};

/**
 * Replace the oldBasePath in a page route with a newBaseUrl.
 *
 * @param route - The route string to process.
 * @param oldBasePath - The old base path to be removed from the route.
 * @param newBaseUrl - The new base URL (may include origin) to be prepended to the route.
 * @returns The route string with its base replaced.
 *
 * @example
 * ```ts
 * shiftBase("/base/foo/bar/", "/base/", "https://typst.app/docs/") // -> "https://typst.app/docs/foo/bar/"
 * ```
 */
export const shiftBase = (
	route: string,
	oldBasePath: string,
	newBaseUrl: string,
): string => joinPath(newBaseUrl, removeBasePath(oldBasePath, route));
