// @ts-check

/** @type {import("@textlint/config-loader").TextlintConfigDescriptor} */
module.exports = {
  plugins: ["html"],
  filters: {
    comments: true,
    allowlist: {
      allow: ["/<(code|pre)[^>]*>[\\s\\S]*?</(code|pre)>/"],
    },
  },
  rules: {
    "preset-jtf-style": {
      // デフォルトで無効のため
      "2.1.5.カタカナ": true,
      // デフォルトで無効のため
      "2.1.6.カタカナの長音": true,
      // デフォルトで無効のため
      "2.2.1.ひらがなと漢字の使い分け": true,
      // 階層構造を表現する記号としての>の使用例があるため
      "4.3.7.山かっこ<>": false,
    },
    prh: {
      rulePaths: ["./prh.yaml"],
      checkLink: false,
      checkBlockQuote: false,
      checkEmphasis: true,
      checkHeader: true,
      checkParagraph: true,
      checkCodeComment: [],
    },
  },
};
