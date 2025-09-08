import type { FC } from "hono/jsx";
import { Translation } from "../../translation/";
import type { Func } from "../../types/model";
import { ChevronRightIcon } from "../icons";
import { FunctionDefinition } from "./FunctionDefinition";
import { FunctionParameters } from "./FunctionParameters";
import { HtmlContent } from "./HtmlContent";

type FunctionDisplayProps = {
	func: Func;
	/**
	 * The prefix for IDs of function parameters
	 *
	 * See `buildParamId`.
	 */
	prefix?: string | undefined;
	isExampleFolding?: boolean;
};

export const FunctionDisplay: FC<FunctionDisplayProps> = ({
	func,
	prefix = undefined,
	isExampleFolding = true,
}) => {
	return (
		<>
			<HtmlContent html={func.details} />

			<div class="my-4">
				<FunctionDefinition func={func} prefix={prefix} />
			</div>

			{func.example && isExampleFolding && (
				<details class="my-4 folding-example group">
					<summary class="flex items-center gap-1 text-sm font-medium text-blue-600 cursor-pointer hover:text-blue-800">
						<div class="w-4 h-4 text-gray-400 transform transition-transform duration-200 group-open:rotate-90">
							<ChevronRightIcon />
						</div>
						<Translation translationKey="showExample" />
					</summary>
					<div class="mt-2 bg-white p-3 rounded-md border border-gray-200 text-sm">
						<HtmlContent html={func.example} />
					</div>
				</details>
			)}

			{func.example && !isExampleFolding && (
				<div class="my-6 bg-gray-50 p-4 rounded-md border border-gray-200">
					<HtmlContent html={func.example} />
				</div>
			)}

			<div class="my-4">
				<FunctionParameters func={func} prefix={prefix} />
			</div>
		</>
	);
};
