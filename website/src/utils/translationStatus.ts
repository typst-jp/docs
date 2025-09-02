import fs from "node:fs";
import path from "node:path";

export type TranslationStatus =
	| "translated"
	| "partially_translated"
	| "untranslated"
	| "community";

export type TranslationStatusMap = {
	[route: string]: TranslationStatus;
};

const TRANSLATION_STATUS_FILE = path.resolve(
	process.cwd(),
	"translation-status.json",
);

/**
 * Loads the JSON file managing translation status for a page. If the file does not exist, returns an empty object.
 * @returns An object representing the translation status of the page.
 */
export const loadTranslationStatus = (): TranslationStatusMap => {
	if (fs.existsSync(TRANSLATION_STATUS_FILE)) {
		const content = fs.readFileSync(TRANSLATION_STATUS_FILE, "utf-8");
		return JSON.parse(content);
	}

	return {};
};

/**
 * Writes the page's translation status to the JSON file.
 * @param status An object representing the translation status of the page.
 */
export const saveTranslationStatus = (status: TranslationStatusMap): void => {
	const json = {
		$schema: "./translation-status.schema.json",
		...status,
	};
	fs.writeFileSync(
		TRANSLATION_STATUS_FILE,
		JSON.stringify(json, null, 2),
		"utf-8",
	);
};

/**
 * Registers new page routes as untranslated. Existing routes are ignored.
 * @param routes An array of page route paths to register.
 */
export const registerRoutes = (routes: string[]): void => {
	const status = loadTranslationStatus();
	let changed = false;

	for (const route of routes) {
		if (!(route in status)) {
			status[route] = "untranslated";
			changed = true;
		}
	}

	if (changed) {
		saveTranslationStatus(status);
	}
};

/**
 * Calculates the translation progress rate.
 * `translated` has a weight of 1.0, `partially_translated` has a weight of 0.5.
 * `original` routes are excluded from calculation as they are not translatable.
 * @returns A translation rate between 0.0 and 1.0.
 */
export const calculateTranslationProgressRate = (): number => {
	const status = loadTranslationStatus();
	const routes = Object.keys(status).filter((key) => key !== "$schema");
	const translationTargetRoutes = routes.filter(
		(route) => status[route] !== "community",
	);

	if (translationTargetRoutes.length === 0) {
		return 0;
	}

	let translationScore = 0;

	for (const route of translationTargetRoutes) {
		const currentStatus = status[route];
		if (currentStatus === "translated") {
			translationScore += 1;
		} else if (currentStatus === "partially_translated") {
			translationScore += 0.5;
		}
	}

	return translationScore / translationTargetRoutes.length;
};

let translationStatusCache: TranslationStatusMap | null = null;

/**
 * Retrieves the translation status for a specified route.
 * @param route The page route for which to retrieve the translation status.
 * @returns The translation status, or "untranslated" if the route does not exist.
 */
export const getTranslationStatus = (route: string): TranslationStatus => {
	if (translationStatusCache === null) {
		translationStatusCache = loadTranslationStatus();
	}
	return translationStatusCache[route] || "untranslated";
};
