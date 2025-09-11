import type { Body, Page } from "../types/model";

/**
 * Type guard function to determine if a page belongs to a specific kind.
 *
 * @param page - The page to check.
 * @param kind - The kind to check against.
 * @returns - A type predicate indicating whether the page is of the specified kind.
 **/
export const isPageOfKind = <K extends Body["kind"]>(
	page: Page,
	kind: K,
): page is Page & { body: Extract<Body, { kind: K }> } =>
	page.body.kind === kind;
