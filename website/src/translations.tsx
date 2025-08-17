import type { FC, JSX, PropsWithChildren } from "hono/jsx";
import {
	discordServerUrl,
	githubOrganizationUrl,
	githubRepositoryUrl,
	typstOfficialDocsUrl,
} from "./metadata";

type MyMap = {
	[key: string]: string;
};

export const menuTranslations: MyMap = {
	lang: "ja",
	ariaCloseMenu: "メニューを閉じる",
	documentationTitle: "Typstドキュメント日本語版",
	ariaShowInformation: "の詳細情報を表示",
	ariaClose: "閉じる",
	ariaOpenSearch: "検索を開く",
	ariaOpenMenu: "メニューを開く",
	ariaCloseSearch: "検索を閉じる",
};

export const t = (key: keyof MyMap) => {
	if (typeof menuTranslations[key] === "string") {
		return menuTranslations[key];
	}
	throw new Error(`Not translation found for ${key}`);
};

type TranslationKey =
	| "definition"
	| "definitionOf"
	| "constructor"
	| "tableOfContents"
	| "untranslated"
	| "untranslatedMessage"
	| "tableOfContents"
	| "tableOfContents"
	| "document"
	| "langVersion"
	| "elementFunction"
	| "elementFunctionDesc"
	| "contextFunction"
	| "contextFunctionDesc"
	| "definitionTooltip"
	| "definitionTooltipDesc"
	| "variadic"
	| "translationRate"
	| "variadicDesc"
	| "typstOfficialDoc"
	| "typstOfficialWebsite"
	| "originalVersion"
	| "contentAddedByCommunity"
	| "partiallyTranslated"
	| "partiallyTranslatedMessage"
	| "translated"
	| "translatedMessage"
	| "contextFunctionDesc"
	| "information"
	| "tutorial"
	| "learnTypst"
	| "reference"
	| "referenceTo"
	| "originalArticle"
	| "search"
	| "argument"
	| "argumentDesc"
	| "required"
	| "requiredDesc"
	| "positional"
	| "positionalDesc"
	| "defaultValue"
	| "stringValues"
	| "showExample"
	| "settable"
	| "settableDesc"
	| "previousPage"
	| "nextPage";

export type TranslationProps =
	| { translationKey: TranslationKey }
	| PropsWithChildren<{ translationKey: "banner"; version: string }>;

/*
This is inferred as the following type:

type TranslationProps = {
    translationKey: "lang";
} |
	...
| {
    translationKey: "banner";
    children?: string | number | boolean | Promise<string> | JSXNode | Child[] | null;
}
*/

export const Translation: FC<TranslationProps> = (props) => {
	switch (props.translationKey) {
		case "definition":
			return <>定義</>;
		case "constructor":
			return <>コンストラクタ</>;
		case "definitionOf":
			return <>の定義</>;
		case "search":
			return <>検索</>;
		case "defaultValue":
			return <>デフォルト値</>;
		case "stringValues":
			return <>使用可能な文字列値</>;
		case "showExample":
			return <>例を表示</>;
		case "tableOfContents":
			return <>目次</>;
		case "nextPage":
			return <>次のページ</>;
		case "previousPage":
			return <>前のページ</>;
		case "reference":
			return (
				<>
					Typstのあらゆる構文、概念、型、関数についての詳細なリファレンスです。
				</>
			);
		case "learnTypst":
			return <>一歩一歩、Typstの使い方を学びましょう。</>;
		case "tutorial":
			return <>チュートリアル</>;
		case "originalArticle":
			return <>原文（英語）を開く</>;
		case "referenceTo":
			return <>リファレンス</>;
		case "typstOfficialDoc":
			return <>Typst公式ドキュメント</>;
		case "typstOfficialWebsite":
			return <>Typst公式サイト</>;
		case "untranslated":
			return <>未翻訳</>;
		case "untranslatedMessage":
			return (
				<>このページはまだ翻訳されていません。原文の内容が表示されています。</>
			);
		case "originalVersion":
			return <>日本語版オリジナル</>;
		case "contentAddedByCommunity":
			return (
				<>
					このページの内容は公式ドキュメントには含まれておらず、日本語コミュニティが独自に追加したものです。
				</>
			);
		case "partiallyTranslated":
			return <>部分的に翻訳済み</>;
		case "partiallyTranslatedMessage":
			return (
				<>
					このページは部分的に翻訳されています。一部原文の内容が含まれています。
				</>
			);
		case "translated":
			return <>翻訳済み</>;
		case "translatedMessage":
			return <>このページは日本語に翻訳済みです。</>;
		case "elementFunction":
			return <>要素関数</>;
		case "elementFunctionDesc":
			return (
				<>
					要素関数は<code>set</code>ルールや<code>show</code>
					ルールでカスタマイズできます。
				</>
			);
		case "contextFunction":
			return <>コンテキスト関数</>;
		case "contextFunctionDesc":
			return (
				<>コンテキスト関数は、コンテキストが既知の場合にのみ使用できます。</>
			);
		case "definitionTooltip":
			return <>定義</>;
		case "definitionTooltipDesc":
			return (
				<>
					これらの関数や型には、関連する定義を持たせることができます。定義にアクセスするには、対象の関数や型の名前を指定した後に、ピリオド区切りで定義名を記述します。
				</>
			);
		case "argument":
			return <>引数</>;
		case "argumentDesc":
			return (
				<>引数は関数への入力値です。関数名の後に括弧で囲んで指定します。</>
			);
		case "variadic":
			return <>可変長引数</>;
		case "variadicDesc":
			return <>可変長引数は複数回指定することができます。</>;

		case "positional":
			return <>位置引数</>;
		case "positionalDesc":
			return (
				<>位置引数は順序通りに指定することで、引数名を省略して設定できます。</>
			);
		case "required":
			return <>必須引数</>;
		case "requiredDesc":
			return <>必須引数は、関数を呼び出す際に必ず指定しなければなりません。</>;
		case "document":
			return <>ドキュメント</>;
		case "langVersion":
			return <>日本語版</>;
		case "translationRate":
			return <>翻訳率</>;
		case "settable":
			return <>設定可能引数</>;
		case "settableDesc":
			return (
				<>
					設定可能引数は、<code>set</code>
					ルールを用いて設定でき、それ以降で使用するデフォルト値を変更できます。,
				</>
			);
		case "banner":
			return (
				<>
					当サイトは、Typst GmbHの許諾を得て、日本語コミュニティ「
					<a href={githubOrganizationUrl}>Typst Japanese Community</a>」が
					<a href={typstOfficialDocsUrl}>
						Typst v{props.version}の公式ドキュメント
					</a>
					を翻訳したものです。誤訳や古い情報が含まれている可能性があるため、
					<a href={typstOfficialDocsUrl}>公式ドキュメント</a>
					との併用を推奨します。翻訳の改善やサイトの機能向上について、
					<a href={githubRepositoryUrl}>GitHub</a>
					でのIssueやPull Requestを歓迎します。コミュニティにご興味のある方は
					<a href={discordServerUrl}>Discordサーバー「くみはんクラブ」</a>
					にぜひご参加ください。
				</>
			);
		case "information":
			return <>情報</>;
		default:
			throw new Error(`No translationKey found for Translation Element`);
	}
};
