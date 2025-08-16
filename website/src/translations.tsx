import type { JSX } from "hono/jsx";
import {
	discordServerUrl,
	githubOrganizationUrl,
	githubRepositoryUrl,
	typstOfficialDocsUrl,
	version,
} from "./metadata";

type MyMap = {
	[key: string]: string | (() => JSX.Element);
};

export const menuTranslations: MyMap = {
	lang: "ja",
	langVersion: "日本語版",
	definition: "定義",
	document: "ドキュメント",
	constructor: "コンストラクタ",
	definitionOf: "の定義",
	search: "検索",
	defaultValue: "デフォルト値",
	stringValues: "使用可能な文字列値",
	showExample: "例を表示",
	tableOfContents: "目次",
	nextPage: "次のページ",
	previousPage: "前のページ",
	reference:
		"Typstのあらゆる構文、概念、型、関数についての詳細なリファレンスです。",
	learnTypst: "一歩一歩、Typstの使い方を学びましょう。",
	tutorial: "チュートリアル",
	originalArticle: "原文（英語）を開く",
	documentationTitle: "Typstドキュメント日本語版",
	referenceTo: "リファレンス",
	typstOfficialDoc: "Typst公式ドキュメント",
	typstOfficialWebsite: "Typst公式サイト",
	translationRate: "翻訳率",
	untranslated: "未翻訳",
	untranslatedMessage:
		"このページはまだ翻訳されていません。原文の内容が表示されています。",
	originalVersion: "日本語版オリジナル",
	contentAddedByCommunity:
		"このページの内容は公式ドキュメントには含まれておらず、日本語コミュニティが独自に追加したものです。",
	partiallyTranslated: "部分的に翻訳済み",
	partiallyTranslatedMessage:
		"このページは部分的に翻訳されています。一部原文の内容が含まれています。",
	translated: "翻訳済み",
	translatedMessage: "このページは日本語に翻訳済みです。",
	elementFunction: "要素関数",
	elementFunctionDesc:
		"要素関数は<code>set</code>ルールや<code>show</code>ルールでカスタマイズできます。",
	contextFunction: "コンテキスト関数",
	contextFunctionDesc:
		"コンテキスト関数は、コンテキストが既知の場合にのみ使用できます。",
	definitionTooltip: "定義",
	definitionTooltipDesc:
		"これらの関数や型には、関連する定義を持たせることができます。定義にアクセスするには、対象の関数や型の名前を指定した後に、ピリオド区切りで定義名を記述します。",
	argument: "引数",
	argumentDesc:
		"引数は関数への入力値です。関数名の後に括弧で囲んで指定します。",
	variadic: "可変長引数",
	variadicDesc: "可変長引数は複数回指定することができます。",
	settable: "設定可能引数",
	settableDesc: () => {
		return (
			<>
				設定可能引数は、<code>set</code>
				ルールを用いて設定でき、それ以降で使用するデフォルト値を変更できます。,
			</>
		);
	},
	positional: "位置引数",
	positionalDesc:
		"位置引数は順序通りに指定することで、引数名を省略して設定できます。",
	required: "必須引数",
	requiredDesc: "必須引数は、関数を呼び出す際に必ず指定しなければなりません。",
	showInformation: "の詳細情報を表示",
	close: "閉じる",
	information: "情報",
	openSearch: "検索を開く",
	closeSearch: "検索を閉じる",
	openMenu: "メニューを開く",
	closeMenu: "メニューを閉じる",
	banner: () => {
		return (
			<>
				当サイトは、Typst GmbHの許諾を得て、日本語コミュニティ「
				<a href={githubOrganizationUrl}>Typst Japanese Community</a>」が
				<a href={typstOfficialDocsUrl}>Typst v{version}の公式ドキュメント</a>
				を翻訳したものです。誤訳や古い情報が含まれている可能性があるため、
				<a href={typstOfficialDocsUrl}>公式ドキュメント</a>
				との併用を推奨します。翻訳の改善やサイトの機能向上について、
				<a href={githubRepositoryUrl}>GitHub</a>
				でのIssueやPull Requestを歓迎します。コミュニティにご興味のある方は
				<a href={discordServerUrl}>Discordサーバー「くみはんクラブ」</a>
				にぜひご参加ください。
			</>
		);
	},
};

export const t = (key: string) => {
	if (typeof menuTranslations[key] === "string") {
		return menuTranslations[key];
	}
	if (Object.hasOwn(menuTranslations, key)) {
		return menuTranslations[key]();
	}
	throw new Error(`Not translation found for ${key}`);
};
