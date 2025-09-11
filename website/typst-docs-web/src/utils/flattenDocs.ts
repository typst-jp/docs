import type { Page } from "../types/model";

/**
 * Flattens the hierarchical structure of documents.
 * Used to retrieve breadcrumb navigation and previous/next page information.
 *
 * @param docs An array of page objects containing document information.
 * @returns A tuple containing:
 *          - A flattened list of page objects
 *          - An array of arrays representing the path information for each page object
 */
export const flattenDocs = (docs: Page[]): [Page[], Page[][]] => {
	const flattenedPages: Page[] = []; // List to store flattened page objects.
	const pagePaths: Page[][] = []; // Path information for each page object [i].

	const _flattenDocs = (page: Page, pagePath: Page[]): void => {
		flattenedPages.push(page);
		pagePaths.push(pagePath);

		for (const childPage of page.children) {
			_flattenDocs(childPage, [...pagePath, childPage]);
		}
	};

	for (const page of docs) {
		_flattenDocs(page, [page]);
	}

	return [flattenedPages, pagePaths];
};
