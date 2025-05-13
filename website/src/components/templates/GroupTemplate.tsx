import type { FC } from "hono/jsx";
import type { GroupBody, Page } from "../../types/model";
import {
	FunctionDefinition,
	FunctionDisplay,
	FunctionParameters,
	Tooltip,
} from "../ui";
import { HtmlContent } from "../ui/HtmlContent";
import BaseTemplate, { type BaseTemplateProps } from "./BaseTemplate";

export type GroupTemplateProps = Omit<BaseTemplateProps, "page"> & {
	page: Omit<Page, "body"> & {
		body: GroupBody;
	};
};

export const GroupTemplate: FC<GroupTemplateProps> = ({
	page,
	docs,
	path,
	previousPage,
	nextPage,
}) => {
	const content = page.body.content;

	return (
		<BaseTemplate
			page={page}
			docs={docs}
			path={path}
			previousPage={previousPage}
			nextPage={nextPage}
		>
			<h1 id="summary">{content.title}</h1>
			<HtmlContent html={content.details} />

			{content.functions.length > 0 && (
				<>
					<h2 id="functions">Function</h2>

					{content.functions.map((method, index) => (
						<div key={method.name}>
							<h3 id={`definitions-${method.name}`} class="method-head">
								<code class="text-base font-medium">{method.name}</code>
								<div class="flex flex-wrap items-center gap-2 text-sm">
									{method.element && <Tooltip kind="element" />}
									{method.contextual && <Tooltip kind="contextual" />}
								</div>
							</h3>
							<FunctionDisplay
								func={method}
								prefix={`definitions-${method.name}`}
								isExampleFolding={false}
							/>
						</div>
					))}
				</>
			)}
		</BaseTemplate>
	);
};

export default GroupTemplate;
