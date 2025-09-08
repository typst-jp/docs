import type { FC } from "hono/jsx";
import { Translation } from "../../translation/";
import type { Page, TypeBody } from "../../types/model";
import { FunctionDisplay, Tooltip } from "../ui";
import { HtmlContent } from "../ui/HtmlContent";
import { TypeIcon } from "../ui/TypeIcon";
import BaseTemplate, { type BaseTemplateProps } from "./BaseTemplate";

export type TypeTemplateProps = Omit<BaseTemplateProps, "page"> & {
	page: Omit<Page, "body"> & {
		body: TypeBody;
	};
};

export const TypeTemplate: FC<TypeTemplateProps> = ({
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
			<h1 id="summary">
				<TypeIcon type={content.name} isHeading={true} />
			</h1>

			<HtmlContent html={content.details} />

			{content.constructor && (
				<>
					<h2 id="constructor" class="flex items-center gap-1">
						<Translation translationKey="constructor" />
						<Tooltip kind="parameters" />
					</h2>

					<FunctionDisplay
						func={content.constructor}
						prefix="constructor"
						isExampleFolding={false}
					/>
				</>
			)}

			{content.scope.length > 0 && (
				<>
					<h2 id="definitions" class="flex items-center gap-1">
						<Translation translationKey="definition" />
						<Tooltip kind="definitions" />
					</h2>

					{content.scope.map((method, _index) => (
						<div key={method.name}>
							<h3
								id={`definitions-${method.name}`}
								class="method-head flex items-center gap-2 mb-3"
							>
								<code class="text-base font-medium">{method.name}</code>
								<div class="flex flex-wrap items-center gap-2">
									{method.element && <Tooltip kind="element" />}
									{method.contextual && <Tooltip kind="contextual" />}
								</div>
							</h3>

							<FunctionDisplay
								func={method}
								prefix={`definitions-${method.name}`}
							/>
						</div>
					))}
				</>
			)}
		</BaseTemplate>
	);
};

export default TypeTemplate;
