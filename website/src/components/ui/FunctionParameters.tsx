import type { FC } from "hono/jsx";
import type { Func } from "../../types/model";
import { ChevronRightIcon } from "../icons";
import { HtmlContent } from "./HtmlContent";
import { Tooltip } from "./Tooltip";
import { TypeIcon } from "./TypeIcon";
import { type2href } from "./type2href";

type FunctionParametersProps = {
	func: Func;
	prefix?: string;
};

export const FunctionParameters: FC<FunctionParametersProps> = ({
	func,
	prefix = "",
}) => {
	return (
		<div class="space-y-6">
			{func.params.map((param, index) => (
				<div
					key={param.name}
					class="bg-gray-50 rounded-md p-4 border border-gray-100"
				>
					<h4
						id={`${prefix}-${func.name}-parameters-${param.name}`}
						class="flex flex-wrap items-center gap-2 mb-3"
					>
						<code class="text-base font-medium">{param.name}</code>
						<div class="flex flex-wrap items-center gap-2 text-sm">
							<div class="flex flex-wrap gap-1">
								{param.types.map((t) => {
									const href = type2href(t);
									return (
										<TypeIcon
											key={t}
											type={t}
											href={href ? `/docs/reference/${href}` : undefined}
										/>
									);
								})}
							</div>

							{param.required && <Tooltip kind="required" />}
							{param.positional && <Tooltip kind="positional" />}
							{param.variadic && <Tooltip kind="variadic" />}
							{param.settable && <Tooltip kind="settable" />}
						</div>
					</h4>

					<div class="mb-3 text-gray-700">
						<HtmlContent html={param.details} />
					</div>

					{param.strings.length > 0 && (
						<div class="mt-3">
							<h5 class="text-sm font-medium text-gray-700 mb-2">
								使用可能な文字列値:
							</h5>
							<ul class="type-args space-y-2 pl-4">
								{param.strings.map((string) => (
									<li key={string.string}>
										<div class="break-box">
											<div class="mb-1">
												<code class="text-gray-800">{string.string}</code>
											</div>
											<div class="text-sm text-gray-700">
												<HtmlContent html={string.details} />
											</div>
										</div>
									</li>
								))}
							</ul>
						</div>
					)}

					{param.default && (
						<p class="mt-3 text-sm">
							<span class="font-medium">デフォルト値:</span>{" "}
							<span class="text-gray-700">
								<HtmlContent html={param.default} />
							</span>
						</p>
					)}

					{param.example && (
						<details class="mt-4 folding-example group">
							<summary class="flex items-center gap-1 text-sm font-medium text-blue-600 cursor-pointer hover:text-blue-800">
								<div class="w-4 h-4 text-gray-400 transform transition-transform duration-200 group-open:rotate-90">
									<ChevronRightIcon />
								</div>
								例を表示
							</summary>
							<div class="mt-2 bg-white p-3 rounded-md border border-gray-200 text-sm">
								<HtmlContent html={param.example} />
							</div>
						</details>
					)}
				</div>
			))}
		</div>
	);
};
