import type { FC } from "hono/jsx";
import { basePath } from "../../metadata";
import type { Func } from "../../types/model";
import { joinPath } from "../../utils/path";
import { genPath } from "./genPath";
import { TypeIcon } from "./TypeIcon";
import { buildParamId, type2href } from "./type2href";

type FunctionDefinitionProps = {
	func: Func;
	/**
	 * The prefix for parameter IDs
	 *
	 * See `buildParamId`.
	 */
	prefix?: string | undefined;
};

export const FunctionDefinition: FC<FunctionDefinitionProps> = ({
	func,
	prefix = undefined,
}) => {
	return (
		<div class="bg-gray-50 p-4 rounded-md border border-gray-100 overflow-x-auto">
			<div class="code code-definition font-mono text-base">
				<span class="text-gray-500">{func.self ? "self." : genPath(func)}</span>
				<span class="typ-func font-semibold text-blue-500">{func.name}</span>
				<span class="text-gray-700">(</span>
				<div class="arguments pl-4 md:pl-6 flex flex-col">
					{func.params.map((param, index) => (
						<div
							key={param.name}
							class="overview-param flex flex-row items-center py-0.5"
						>
							{!param.positional && (
								<div class="flex-shrink-0">
									<a
										href={`#${buildParamId(param.name, prefix)}`}
										class="text-gray-800 hover:text-blue-500 transition-colors mr-1"
									>
										<span class="font-medium">{param.name}</span>
										<span class="text-gray-500">:</span>
									</a>
								</div>
							)}
							<div class="flex flex-row flex-wrap gap-1">
								{param.types.map((t) => {
									const href = type2href(t);
									return (
										<TypeIcon
											key={t}
											type={t}
											href={
												href ? joinPath(basePath, "reference", href) : undefined
											}
										/>
									);
								})}
							</div>
							{index < func.params.length - 1 && (
								<span class="text-gray-500 ml-1">,</span>
							)}
						</div>
					))}
				</div>
				<span class="text-gray-700">)</span>

				{func.returns.length > 0 && (
					<>
						<span class="text-gray-500 mx-1">-&gt;</span>
						<div class="inline-flex flex-wrap gap-1">
							{func.returns.map((ret, i) => {
								const href = type2href(ret);
								return (
									<>
										<TypeIcon
											key={ret}
											type={ret}
											href={
												href ? joinPath(basePath, "reference", href) : undefined
											}
										/>
										{i < func.returns.length - 1 && (
											<span class="text-gray-500 mx-1">,</span>
										)}
									</>
								);
							})}
						</div>
					</>
				)}
			</div>
		</div>
	);
};
