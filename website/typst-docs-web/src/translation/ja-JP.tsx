import { Fragment } from "hono/jsx/jsx-runtime";
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
			return <Fragment>定義</Fragment>;
		case "constructor":
			return <Fragment>コンストラクタ</Fragment>;
		case "definitionOf":
			return (
				<Fragment>
					<code>{props.name}</code>の定義
				</Fragment>
			);
		case "search":
			return <Fragment>検索</Fragment>;
		case "defaultValue":
			return <Fragment>デフォルト値：</Fragment>;
		case "stringValues":
			return <Fragment>使用可能な文字列値：</Fragment>;
		case "showExample":
			return <Fragment>例を表示</Fragment>;
		case "tableOfContents":
			return <Fragment>目次</Fragment>;
		case "nextPage":
			return <Fragment>次のページ</Fragment>;
		case "previousPage":
			return <Fragment>前のページ</Fragment>;
		case "referenceDescription":
			return (
				<Fragment>
					Typstのあらゆる構文、概念、型、関数についての詳細なリファレンスです。
				</Fragment>
			);
		case "tutorialDescription":
			return <Fragment>一歩一歩、Typstの使い方を学びましょう。</Fragment>;
		case "tutorial":
			return <Fragment>チュートリアル</Fragment>;
		case "openOfficialDocs":
			return <Fragment>原文（英語）を開く</Fragment>;
		case "reference":
			return <Fragment>リファレンス</Fragment>;
		case "typstOfficialDocs":
			return <Fragment>Typst公式ドキュメント</Fragment>;
		case "typstOfficialWebsite":
			return <Fragment>Typst公式サイト</Fragment>;
		case "untranslated":
			return <Fragment>未翻訳</Fragment>;
		case "untranslatedMessage":
			return (
				<Fragment>
					このページはまだ翻訳されていません。原文の内容が表示されています。
				</Fragment>
			);
		case "communityContent":
			return <Fragment>日本語版オリジナル</Fragment>;
		case "contentAddedByCommunity":
			return (
				<Fragment>
					このページの内容は公式ドキュメントには含まれておらず、日本語コミュニティが独自に追加したものです。
				</Fragment>
			);
		case "partiallyTranslated":
			return <Fragment>部分的に翻訳済み</Fragment>;
		case "partiallyTranslatedMessage":
			return (
				<Fragment>
					このページは部分的に翻訳されています。一部原文の内容が含まれています。
				</Fragment>
			);
		case "translated":
			return <Fragment>翻訳済み</Fragment>;
		case "translatedMessage":
			return <Fragment>このページは日本語に翻訳済みです。</Fragment>;
		case "elementFunction":
			return <Fragment>要素関数</Fragment>;
		case "elementFunctionDescription":
			return (
				<Fragment>
					要素関数は<code>set</code>ルールや<code>show</code>
					ルールでカスタマイズできます。
				</Fragment>
			);
		case "contextFunction":
			return <Fragment>コンテキスト関数</Fragment>;
		case "contextFunctionDescription":
			return (
				<Fragment>
					コンテキスト関数は、コンテキストが既知の場合にのみ使用できます。
				</Fragment>
			);
		case "definitionTooltip":
			return <Fragment>定義</Fragment>;
		case "definitionTooltipDescription":
			return (
				<Fragment>
					これらの関数や型には、関連する定義を持たせることができます。定義にアクセスするには、対象の関数や型の名前を指定した後に、ピリオド区切りで定義名を記述します。
				</Fragment>
			);
		case "argument":
			return <Fragment>引数</Fragment>;
		case "argumentDescription":
			return (
				<Fragment>
					引数は関数への入力値です。関数名の後に括弧で囲んで指定します。
				</Fragment>
			);
		case "variadic":
			return <Fragment>可変長引数</Fragment>;
		case "variadicDescription":
			return <Fragment>可変長引数は複数回指定することができます。</Fragment>;

		case "positional":
			return <Fragment>位置引数</Fragment>;
		case "positionalDescription":
			return (
				<Fragment>
					位置引数は順序通りに指定することで、引数名を省略して設定できます。
				</Fragment>
			);
		case "required":
			return <Fragment>必須引数</Fragment>;
		case "requiredDescription":
			return (
				<Fragment>
					必須引数は、関数を呼び出す際に必ず指定しなければなりません。
				</Fragment>
			);
		case "document":
			return <Fragment>ドキュメント</Fragment>;
		case "langVersion":
			return <Fragment>日本語版</Fragment>;
		case "translationRate":
			return <Fragment>翻訳率</Fragment>;
		case "settable":
			return <Fragment>設定可能引数</Fragment>;
		case "settableDescription":
			return (
				<Fragment>
					設定可能引数は、<code>set</code>
					ルールを用いて設定でき、それ以降で使用するデフォルト値を変更できます。
				</Fragment>
			);
		case "siteNoticeBannerTitle":
			return <Fragment>情報 / Info</Fragment>;
		case "siteNoticeBannerDescription":
			return (
				<Fragment>
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
				</Fragment>
			);
		case "footer":
			return (
				<Fragment>
					Translated by{" "}
					<a href={githubOrganizationUrl}>Typst Japanese Community</a>
				</Fragment>
			);
		default:
			return null;
	}
};
