import type { FC } from "hono/jsx";
import type { Func, FuncBody, Page } from "../../types/model";
import {
	FunctionDefinition,
	FunctionDisplay,
	FunctionParameters,
	Tooltip,
} from "../ui";
import { DeprecationWarning } from "../ui/DeprecationWarning";
import { HtmlContent } from "../ui/HtmlContent";
import BaseTemplate, { type BaseTemplateProps } from "./BaseTemplate";

export type FuncTemplateProps = Omit<BaseTemplateProps, "page"> & {
	page: Omit<Page, "body"> & {
		body: FuncBody;
	};
};

export const FuncTemplate: FC<FuncTemplateProps> = ({
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
			<h1 id="summary" class="flex items-center gap-2 mb-6">
				<code>{content.name}</code>
				<small class="flex items-center gap-1">
					{content.element && <Tooltip kind="element" />}
					{content.contextual && <Tooltip kind="contextual" />}
				</small>
			</h1>

			{content.deprecation && (
				<div className="mt-2">
					<DeprecationWarning message={content.deprecation} />
				</div>
			)}

			<div class="my-4 text-gray-700">
				<HtmlContent html={content.details} />
			</div>

			<h2 id="parameters" class="flex items-baseline gap-1">
				引数
				<Tooltip kind="parameters" />
			</h2>

			<div class="mb-6">
				<FunctionDefinition func={content} />
			</div>

			{content.example && (
				<div class="my-6 bg-gray-50 p-4 rounded-md border border-gray-200">
					<HtmlContent html={content.example} />
				</div>
			)}

			<div class="my-6">
				<FunctionParameters func={content} />
			</div>

			<ScopedDefinitions scope={content.scope} />
		</BaseTemplate>
	);
};

/**
 * If the definitions in `scope` are associated with the top scope on this page, then `parent` should be `undefined`.
 * Otherwise, `parent` should describe the parent of these definitions.
 */
function ScopedDefinitions({
	scope,
	parent,
}: { scope: Func[]; parent?: { name: string; id: string } | undefined }) {
	if (scope.length === 0) {
		return null;
	}

	/** `parent?.id` as a string */
	const parentId = parent ? `${parent.id}-` : "";

	// To be consistent with the official typst.app/docs,
	// the following heading levels will _not_ increase with the scope level.
	return (
		<div class="mt-8">
			<h2 id={`${parentId}definitions`} class="flex items-baseline gap-1">
				{parent ? (
					// Currently, the scope has at most two levels.
					// Therefore, it is sufficient to only annotate the direct `parent`.
					<>
						<code>{parent.name}</code>の定義
					</>
				) : (
					"定義"
				)}
				<Tooltip kind="definitions" />
			</h2>

			{scope.map((method, index) => {
				const methodId = `${parentId}definitions-${method.name}`;

				return (
					<div
						key={method.name}
						class="mb-8 pb-6 border-b border-gray-100 last:border-0"
					>
						<h3 id={methodId} class="method-head mb-3 flex items-center gap-2">
							<code
								class="text-base font-medium"
								style={
									method.deprecation
										? { textDecoration: "line-through" }
										: undefined
								}
							>
								{method.name}
							</code>

							<small class="flex items-center">
								{method.element && <Tooltip kind="element" />}
								{method.contextual && <Tooltip kind="contextual" />}
							</small>
						</h3>

						{method.deprecation && (
							<div className="mt-1">
								<DeprecationWarning message={method.deprecation} />
							</div>
						)}

						<div class="pl-2">
							<FunctionDisplay func={method} prefix={methodId} />
						</div>

						<ScopedDefinitions
							scope={method.scope}
							parent={{ name: method.name, id: methodId }}
						/>
					</div>
				);
			})}
		</div>
	);
}

export default FuncTemplate;
