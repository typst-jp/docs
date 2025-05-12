import type { Body, Page } from "../types/model";

/**
 * ページの種類を判定するための型ガード関数
 *
 * @param page - 判定するページ
 * @param kind - 判定する種類
 * @returns - ページが指定された種類であるかどうか
 **/
export const isPageOfKind = <K extends Body["kind"]>(
	page: Page,
	kind: K,
): page is Page & { body: Extract<Body, { kind: K }> } =>
	page.body.kind === kind;
