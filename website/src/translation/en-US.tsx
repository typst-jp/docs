import type { TooltipProps } from "../components/ui/Tooltip";
import { githubOrganizationUrl } from "../metadata";
import type { TranslationComponent, TranslationObject } from "./";

export const translation: TranslationObject = {
	htmlLang: () => "en",
	documentationTitle: () => "Typst Documentation (English)",
	close: () => "Close",
	closeMenu: () => "Close menu",
	closeSearch: () => "Close search",
	openMenu: () => "Open menu",
	openSearch: () => "Open search",
	showInformation: (props: { name: string }) =>
		`Show details for ${props.name}`,
	tooltipKind: (props: { kind: TooltipProps["kind"] }) => {
		switch (props.kind) {
			case "element":
				return "Element";
			case "contextual":
				return "Contextual";
			case "definitions":
				return "Definition";
			case "parameters":
				return "Parameter";
			case "variadic":
				return "Variadic";
			case "settable":
				return "Settable";
			case "positional":
				return "Positional";
			case "required":
				return "Required";
			default:
				return props.kind;
		}
	},
} as const;

export const Translation: TranslationComponent = (props) => {
	switch (props.translationKey) {
		case "definition":
			return <>Definition</>;
		case "constructor":
			return <>Constructor</>;
		case "definitionOf":
			return (
				<>
					<code>{props.name}</code> Definition
				</>
			);
		case "search":
			return <>Search</>;
		case "defaultValue":
			return <>Default value:</>;
		case "stringValues":
			return <>Available string values:</>;
		case "showExample":
			return <>Show example</>;
		case "tableOfContents":
			return <>On this page</>;
		case "nextPage":
			return <>Next page</>;
		case "previousPage":
			return <>Previous page</>;
		case "referenceDescription":
			return (
				<>
					Detailed reference for all Typst syntax, concepts, types, and
					functions.
				</>
			);
		case "tutorialDescription":
			return <>Learn how to use Typst step by step.</>;
		case "tutorial":
			return <>Tutorial</>;
		case "openOfficialDocs":
			return <>Open official docs</>;
		case "reference":
			return <>Reference</>;
		case "typstOfficialDocs":
			return <>Typst official docs</>;
		case "typstOfficialWebsite":
			return <>Typst official website</>;
		case "untranslated":
			return <>Untranslated</>;
		case "untranslatedMessage":
			return (
				<>
					This page has not been translated yet. The original content is shown.
				</>
			);
		case "communityContent":
			return <>Community original content</>;
		case "contentAddedByCommunity":
			return (
				<>
					This page contains content that is not part of the official
					documentation, added independently by the community.
				</>
			);
		case "partiallyTranslated":
			return <>Partially translated</>;
		case "partiallyTranslatedMessage":
			return (
				<>
					This page is partially translated. Some original content is included.
				</>
			);
		case "translated":
			return <>Translated</>;
		case "translatedMessage":
			return <>This page has been translated into English.</>;
		case "elementFunction":
			return <>Element</>;
		case "elementFunctionDescription":
			return (
				<>
					Element functions can be customized with <code>set</code> and{" "}
					<code>show</code> rules.
				</>
			);
		case "contextFunction":
			return <>Context</>;
		case "contextFunctionDescription":
			return <>Context functions can only be used when the context is known.</>;
		case "definitionTooltip":
			return <>Definition</>;
		case "definitionTooltipDescription":
			return (
				<>
					These functions and types can have related definitions. To access a
					definition, specify the name of the function or type, followed by the
					definition name separated by a period.
				</>
			);
		case "argument":
			return <>Parameter</>;
		case "argumentDescription":
			return (
				<>
					Parameters are input values for functions. Specify them in parentheses
					after the function name.
				</>
			);
		case "variadic":
			return <>Variadic</>;
		case "variadicDescription":
			return <>Variadic parameters can be specified multiple times.</>;
		case "positional":
			return <>Positional</>;
		case "positionalDescription":
			return (
				<>
					Positional parameters can be set by specifying them in order, omitting
					the parameter name.
				</>
			);
		case "required":
			return <>Required</>;
		case "requiredDescription":
			return (
				<>Required parameters must be specified when calling the function.</>
			);
		case "document":
			return <>Document</>;
		case "langVersion":
			return <>English</>;
		case "translationRate":
			return <>Translated</>;
		case "settable":
			return <>Settable</>;
		case "settableDescription":
			return (
				<>
					Settable parameters can be set using the <code>set</code> rule,
					changing the default value used thereafter.
				</>
			);
		case "siteNoticeBannerTitle":
			return <>Info</>;
		case "siteNoticeBannerDescription":
			return (
				<>
					This site is generated using the static site generator developed by
					the <a href="https://github.com/typst-community">Typst Community</a>.
					Please adjust the text content of this banner according to your usage
					requirements. At Typst GmbH's request, when publishing documentation,
					you must clearly indicate that it is non-official and display the
					version of Typst being documented. For details, refer to{" "}
					<a href="https://github.com/typst/typst/issues/874#issuecomment-2273854138">
						Issue #874 on typst/typst
					</a>
					.
				</>
			);
		case "footer":
			return (
				<>
					Translated by <a href={githubOrganizationUrl}>Typst Community</a>
				</>
			);
		default:
			return null;
	}
};
