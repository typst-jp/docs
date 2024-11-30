// @ts-check

/** @type {import("@textlint/config-loader").TextlintConfigDescriptor} */
module.exports = {
  plugins: ["html"],
  filters: {
    comments: true,
  },
  rules: {
    "preset-jtf-style": {
      // 和文が前提のルールであり、英文で誤検出があるため
      "3.3.かっこ類と隣接する文字の間のスペースの有無": {
        severity: "warning",
      },
      // 誤検出があるため
      "4.3.2.大かっこ［］": false,
      // 階層構造を表現する記号としての>の使用例があるため
      "4.3.7.山かっこ<>": false,
    },
  },
};
