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
  } else if (layoutSet.has(parameterType)) {
    return `layout/${parameterType}`;
  } else if (visualizeSet.has(parameterType)) {
    return `visualize/${parameterType}`;
  } else if (introspectionSet.has(parameterType)) {
    return `introspection/${parameterType}`;
  } else {
    return null;
  }
};
