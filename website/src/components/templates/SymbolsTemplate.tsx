import type { FC } from "hono/jsx";
import type { Page, SymbolsBody } from "../../types/model";
import type { BaseTemplateProps } from "./BaseTemplate";

export type SymbolsTemplateProps = Omit<BaseTemplateProps, "page"> & {
	page: Omit<Page, "body"> & {
		body: SymbolsBody;
	};
};

export const SymbolsTemplate: FC<SymbolsTemplateProps> = ({ page }) => {
	const redirectUrl = `https://typst.app${page.route}`;

	return (
		<html lang="ja">
			<head>
				<meta httpEquiv="refresh" content={`0;url=${redirectUrl}`} />
			</head>
		</html>
	);
};

export default SymbolsTemplate;
