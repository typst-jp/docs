import type { Func } from "../../types/model";

/**
 * Concatenates path segments.
 *
 * @param item - A Func object
 * @returns - The concatenated path string
 */
export const genPath = (item: Func): string => {
	return item.path.map((s) => `${s}.`).join("");
};
