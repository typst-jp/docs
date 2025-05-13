import type { FC } from "hono/jsx";
import type { CategoryBody, Page } from "../../types/model";
import { HtmlContent } from "../ui/HtmlContent";
import BaseTemplate, { type BaseTemplateProps } from "./BaseTemplate";

export type CategoryTemplateProps = Omit<BaseTemplateProps, "page"> & {
	page: Omit<Page, "body"> & {
		body: CategoryBody;
	};
};

export const CategoryTemplate: FC<CategoryTemplateProps> = ({
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
			<h1 id="summary">{page.body.content.name}</h1>
			<HtmlContent html={page.body.content.details} />
			<h2 id="definitions">定義</h2>
			<ul class="subgridded">
				{page.body.content.items.map((item) => (
					<li key={item.route}>
						<div>
							<a href={item.route}>
								{item.code ? <code>{item.name}</code> : item.name}
							</a>
						</div>
						<div class="pl-4">{item.oneliner}</div>
					</li>
				))}
			</ul>
		</BaseTemplate>
	);
};
