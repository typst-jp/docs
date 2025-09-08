import type { FC } from "hono/jsx";
import type { TooltipProps } from "../components/ui/Tooltip";

/**
 * Translation dictionary for UI attributes and aria labels.
 *
 * @example
 * translation.closeMenu()
 * translation.showInformation({ name: "foo" })
 */
export type TranslationObject = {
	htmlLang: () => string;
	documentationTitle: () => string;
	close: () => string;
	closeMenu: () => string;
	closeSearch: () => string;
	openMenu: () => string;
	openSearch: () => string;
	showInformation: (props: { name: string }) => string;
	tooltipKind: (props: { kind: TooltipProps["kind"] }) => string;
};

type TranslationComponentKey =
	| "definition"
	| "constructor"
	| "tableOfContents"
	| "untranslated"
	| "untranslatedMessage"
	| "document"
	| "langVersion"
	| "elementFunction"
	| "elementFunctionDescription"
	| "contextFunction"
	| "contextFunctionDescription"
	| "definitionTooltip"
	| "definitionTooltipDescription"
	| "variadic"
	| "translationRate"
	| "variadicDescription"
	| "typstOfficialDocs"
	| "typstOfficialWebsite"
	| "communityContent"
	| "contentAddedByCommunity"
	| "partiallyTranslated"
	| "partiallyTranslatedMessage"
	| "translated"
	| "translatedMessage"
	| "siteNoticeBannerTitle"
	| "siteNoticeBannerDescription"
	| "tutorial"
	| "tutorialDescription"
	| "referenceDescription"
	| "reference"
	| "openOfficialDocs"
	| "search"
	| "argument"
	| "argumentDescription"
	| "required"
	| "requiredDescription"
	| "positional"
	| "positionalDescription"
	| "defaultValue"
	| "stringValues"
	| "showExample"
	| "settable"
	| "settableDescription"
	| "previousPage"
	| "nextPage"
	| "footer";

export type TranslationComponentProps =
	| { translationKey: TranslationComponentKey }
	| { translationKey: "definitionOf"; name: string };

/**
 * Translation component for UI text, descriptions, and other content to be embedded as JSX.
 *
 * @example
 * <Translation translationKey="definition" />
 */
export type TranslationComponent = FC<TranslationComponentProps>;

/**
 * Switch translation language here.
 */
export { Translation, translation } from "./ja-JP";
