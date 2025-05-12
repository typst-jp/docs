import type { FC } from "hono/jsx";
import type { HtmlBody, Page } from "../../types/model";
import BaseTemplate, { type BaseTemplateProps } from "./BaseTemplate";

export type HtmlTemplateProps = Omit<BaseTemplateProps, "page"> & {
	page: Omit<Page, "body"> & {
		body: HtmlBody;
	};
};

export const HtmlTemplate: FC<HtmlTemplateProps> = ({
	page,
	docs,
	path,
	previousPage,
	nextPage,
}) => {
	return (
		<BaseTemplate
			page={page}
			docs={docs}
			path={path}
			previousPage={previousPage}
			nextPage={nextPage}
		>
			<div dangerouslySetInnerHTML={{ __html: page.body.content as string }} />
		</BaseTemplate>
	);
};
