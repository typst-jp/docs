import type { TooltipProps } from "../components/ui/Tooltip";
import {
	discordServerUrl,
	githubOrganizationUrl,
	githubRepositoryUrl,
	typstOfficialDocsUrl,
	version,
} from "../metadata";
import type { TranslationComponent, TranslationObject } from "./";

export const translation: TranslationObject = {
	htmlLang: () => "ja",
	documentationTitle: () => "Typstドキュメント日本語版",
	close: () => "閉じる",
	closeMenu: () => "メニューを閉じる",
	closeSearch: () => "検索を閉じる",
	openMenu: () => "メニューを開く",
	openSearch: () => "検索を開く",
	showInformation: (props: { name: string }) => `${props.name}の詳細情報を表示`,
	tooltipKind: (props: { kind: TooltipProps["kind"] }) => {
		switch (props.kind) {
			case "element":
				return "要素関数";
			case "contextual":
				return "コンテキスト関数";
			case "definitions":
				return "定義";
			case "parameters":
				return "引数";
			case "variadic":
				return "可変長引数";
			case "settable":
				return "設定可能引数";
			case "positional":
				return "位置引数";
			case "required":
				return "必須引数";
			default:
				return props.kind;
		}
	},
} as const;

export const Translation: TranslationComponent = (props) => {
	switch (props.translationKey) {
		case "definition":
			return <>定義</>;
		case "constructor":
			return <>コンストラクタ</>;
		case "definitionOf":
			return (
				<>
					<code>{props.name}</code>の定義
				</>
			);
		case "search":
			return <>検索</>;
		case "defaultValue":
			return <>デフォルト値：</>;
		case "stringValues":
			return <>使用可能な文字列値：</>;
		case "showExample":
			return <>例を表示</>;
		case "tableOfContents":
			return <>目次</>;
		case "nextPage":
			return <>次のページ</>;
		case "previousPage":
			return <>前のページ</>;
		case "referenceDescription":
			return (
				<>
					Typstのあらゆる構文、概念、型、関数についての詳細なリファレンスです。
				</>
			);
		case "tutorialDescription":
			return <>一歩一歩、Typstの使い方を学びましょう。</>;
		case "tutorial":
			return <>チュートリアル</>;
		case "openOfficialDocs":
			return <>原文（英語）を開く</>;
		case "reference":
			return <>リファレンス</>;
		case "typstOfficialDocs":
			return <>Typst公式ドキュメント</>;
		case "typstOfficialWebsite":
			return <>Typst公式サイト</>;
		case "untranslated":
			return <>未翻訳</>;
		case "untranslatedMessage":
			return (
				<>このページはまだ翻訳されていません。原文の内容が表示されています。</>
			);
		case "communityContent":
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
		case "elementFunctionDescription":
			return (
				<>
					要素関数は<code>set</code>ルールや<code>show</code>
					ルールでカスタマイズできます。
				</>
			);
		case "contextFunction":
			return <>コンテキスト関数</>;
		case "contextFunctionDescription":
			return (
				<>コンテキスト関数は、コンテキストが既知の場合にのみ使用できます。</>
			);
		case "definitionTooltip":
			return <>定義</>;
		case "definitionTooltipDescription":
			return (
				<>
					これらの関数や型には、関連する定義を持たせることができます。定義にアクセスするには、対象の関数や型の名前を指定した後に、ピリオド区切りで定義名を記述します。
				</>
			);
		case "argument":
			return <>引数</>;
		case "argumentDescription":
			return (
				<>引数は関数への入力値です。関数名の後に括弧で囲んで指定します。</>
			);
		case "variadic":
			return <>可変長引数</>;
		case "variadicDescription":
			return <>可変長引数は複数回指定することができます。</>;

		case "positional":
			return <>位置引数</>;
		case "positionalDescription":
			return (
				<>位置引数は順序通りに指定することで、引数名を省略して設定できます。</>
			);
		case "required":
			return <>必須引数</>;
		case "requiredDescription":
			return <>必須引数は、関数を呼び出す際に必ず指定しなければなりません。</>;
		case "document":
			return <>ドキュメント</>;
		case "langVersion":
			return <>日本語版</>;
		case "translationRate":
			return <>翻訳率</>;
		case "settable":
			return <>設定可能引数</>;
		case "settableDescription":
			return (
				<>
					設定可能引数は、<code>set</code>
					ルールを用いて設定でき、それ以降で使用するデフォルト値を変更できます。
				</>
			);
		case "siteNoticeBannerTitle":
			return <>情報 / Info</>;
		case "siteNoticeBannerDescription":
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
					<br />
					This site provides a Japanese translation of the{" "}
					<a href={typstOfficialDocsUrl}>Typst v{version} documentation</a>{" "}
					maintained by the "
					<a href={githubOrganizationUrl}>Typst Japanese Community</a>" with
					permission from Typst GmbH. We recommend using this alongside the{" "}
					<a href={typstOfficialDocsUrl}>official documentation</a>. We welcome
					contributions through Issues and Pull Requests on{" "}
					<a href={githubRepositoryUrl}>our GitHub repository</a> for both
					translation improvements and website enhancements. Feel free to join{" "}
					<a href={discordServerUrl}>our Discord server "Kumihan Club"</a>.
				</>
			);
		case "footer":
			return (
				<>
					Translated by{" "}
					<a href={githubOrganizationUrl}>Typst Japanese Community</a>
				</>
			);
		default:
			return null;
	}
};
