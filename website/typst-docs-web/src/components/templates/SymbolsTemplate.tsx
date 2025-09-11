import type { FC } from "hono/jsx";
import { basePath, typstOfficialDocsUrl } from "../../metadata";
import type { Page, SymbolsBody } from "../../types/model";
import { shiftBase } from "../../utils/path";
import type { BaseTemplateProps } from "./BaseTemplate";

export type SymbolsTemplateProps = Omit<BaseTemplateProps, "page"> & {
	page: Omit<Page, "body"> & {
		body: SymbolsBody;
	};
};

export const SymbolsTemplate: FC<SymbolsTemplateProps> = ({ page }) => {
	const redirectUrl = shiftBase(page.route, basePath, typstOfficialDocsUrl);

	return (
		<html lang="ja">
			<head>
				<meta httpEquiv="refresh" content={`0;url=${redirectUrl}`} />
			</head>
		</html>
	);
};

export default SymbolsTemplate;
