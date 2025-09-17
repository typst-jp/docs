/**
 * Retrieve a link from a type name.
 *
 * @param parameterType The type name.
 * @returns The link.
 */
export const type2href = (parameterType: string): string | null => {
	const foundationSet = new Set([
		"arguments",
		"array",
		"auto",
		"bool",
		"bytes",
		"content",
		"datetime",
		"decimal",
		"dictionary",
		"duration",
		"float",
		"function",
		"int",
		"label",
		"module",
		"none",
		"plugin",
		"regex",
		"selector",
		"str",
		"type",
		"version",
	]);

	const layoutSet = new Set([
		"alignment",
		"angle",
		"direction",
		"fraction",
		"length",
		"ratio",
		"relative",
	]);

	const visualizeSet = new Set(["color", "gradient", "pattern", "stroke"]);

	const introspectionSet = new Set(["counter", "location", "state"]);

	if (foundationSet.has(parameterType)) {
		return `foundations/${parameterType}/`;
	}
	if (layoutSet.has(parameterType)) {
		return `layout/${parameterType}/`;
	}
	if (visualizeSet.has(parameterType)) {
		return `visualize/${parameterType}/`;
	}
	if (introspectionSet.has(parameterType)) {
		return `introspection/${parameterType}/`;
	}
	return null;
};

/**
 * Build the ID of a parameter with prefix
 *
 * If the parameter belongs to a top-level function on a page, leave `prefix` empty;
 * Otherwise, set it to an appropriate prefix.
 *
 * ## Example values of `prefix`
 *
 * | Page (kind)         | Function         | Parameter  | `prefix`                |
 * | ------------------- | ---------------- | ---------- | ----------------------- |
 * | `figure` (function) | `figure`         | `body`     | `undefined`             |
 * | `figure` (function) | `figure.caption` | `body`     | `"definitions-caption"` |
 * | `calc` (group)      | `calc.abs`       | `value`    | `"functions-abs"`       |
 * | `array` (type)      | `array.at`       | `index`    | `"definitions-at"`      |
 * | `array` (type)      | Constructor      | `value`    | `"constructor"`      |
 */
export function buildParamId(
	parameterName: string,
	prefix: string | undefined,
): string {
	return prefix === undefined
		? `parameters-${parameterName}`
		: `${prefix}-${parameterName}`;
}
