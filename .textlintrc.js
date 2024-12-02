// @ts-check

/** @type {import("@textlint/config-loader").TextlintConfigDescriptor} */
module.exports = {
  plugins: ["html"],
  filters: {
    comments: true,
  },
  rules: {
    "preset-jtf-style": {
      // 階層構造を表現する記号としての>の使用例があるため
      "4.3.7.山かっこ<>": false,
    },
  },
};
