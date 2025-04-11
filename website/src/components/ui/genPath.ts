import { Func } from "../../types/model";

/**
 * pathを連結する
 *
 * @param item - Func
 * @returns - 連結されたpath
 */
export const genPath = (item: Func): string => {
  return item.path.map((s) => s + ".").join("");
};
