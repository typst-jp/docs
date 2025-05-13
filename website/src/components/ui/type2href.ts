/**
 * 型名からリンクを取得する
 *
 * @param parameterType 型名
 * @returns リンク
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
		return `foundations/${parameterType}`;
	}
	if (layoutSet.has(parameterType)) {
		return `layout/${parameterType}`;
	}
	if (visualizeSet.has(parameterType)) {
		return `visualize/${parameterType}`;
	}
	if (introspectionSet.has(parameterType)) {
		return `introspection/${parameterType}`;
	}
	return null;
};
