import type { Page } from "../types/model";

/**
 * ドキュメントの階層構造を平坦化する
 * パンくずリストと前後のページ情報を取得するために使用する
 *
 * @param docs ページ情報の配列
 * @returns [平坦化されたページ情報のリスト, ページ情報のパス情報]
 */
export const flattenDocs = (docs: Page[]): [Page[], Page[][]] => {
  const flattenedPages: Page[] = []; // 平坦化されたページ情報のリスト
  const pagePaths: Page[][] = []; // ページ情報[i]のパス情報

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
