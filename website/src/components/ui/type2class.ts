/**
 * 型名からCSSのクラス名を取得する
 *
 * @param parameterType - 型名
 * @returns CSSのクラス名
 */
export const type2class = (parameterType: string): string => {
  const type2classMap: Record<string, string> = {
    none: "pill-kw",
    auto: "pill-kw",
    function: "pill-fn",
    string: "pill-str",
    str: "pill-str",
    content: "pill-con",
    color: "pill-col",
    bool: "pill-bool",
    boolean: "pill-bool",
    integer: "pill-num",
    int: "pill-num",
    ratio: "pill-num",
    length: "pill-num",
    "relative length": "pill-num",
    float: "pill-num",
    angle: "pill-num",
    fraction: "pill-num",
  };

  return type2classMap[parameterType] || "pill-obj";
};
