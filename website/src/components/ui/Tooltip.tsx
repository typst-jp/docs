import type { FC } from "hono/jsx";
import { Translation, translation } from "../../translation";
import { CloseIcon, HelpCircleIcon } from "../icons";

export type TooltipProps = {
	kind:
		| "element"
		| "contextual"
		| "definitions"
		| "parameters"
		| "variadic"
		| "settable"
		| "positional"
		| "required";
};

const tooltipContent: Record<
	TooltipProps["kind"],
	{
		label: ReturnType<typeof Translation>;
		desc: ReturnType<typeof Translation>;
		isShowLabel: boolean;
		bgColor: string;
		textColor: string;
	}
> = {
	element: {
		label: <Translation translationKey="elementFunction" />,
		desc: <Translation translationKey="elementFunctionDescription" />,
		isShowLabel: true,
		bgColor: "bg-blue-50",
		textColor: "text-blue-700",
	},
	contextual: {
		label: <Translation translationKey="contextFunction" />,
		desc: <Translation translationKey="contextFunctionDescription" />,
		isShowLabel: true,
		bgColor: "bg-indigo-50",
		textColor: "text-indigo-700",
	},
	definitions: {
		label: <Translation translationKey="definitionTooltip" />,
		desc: <Translation translationKey="definitionTooltipDescription" />,
		isShowLabel: false,
		bgColor: "bg-gray-100",
		textColor: "text-gray-700",
	},
	parameters: {
		label: <Translation translationKey="argument" />,
		desc: <Translation translationKey="argumentDescription" />,
		isShowLabel: false,
		bgColor: "bg-gray-100",
		textColor: "text-gray-700",
	},
	variadic: {
		label: <Translation translationKey="variadic" />,
		desc: <Translation translationKey="variadicDescription" />,
		isShowLabel: true,
		bgColor: "bg-green-50",
		textColor: "text-green-700",
	},
	settable: {
		label: <Translation translationKey="settable" />,
		desc: <Translation translationKey="settableDescription" />,
		isShowLabel: true,
		bgColor: "bg-amber-50",
		textColor: "text-amber-700",
	},
	positional: {
		label: <Translation translationKey="positional" />,
		desc: <Translation translationKey="positionalDescription" />,
		isShowLabel: true,
		bgColor: "bg-purple-50",
		textColor: "text-purple-700",
	},
	required: {
		label: <Translation translationKey="required" />,
		desc: <Translation translationKey="requiredDescription" />,
		isShowLabel: true,
		bgColor: "bg-rose-50",
		textColor: "text-rose-700",
	},
};

export const Tooltip: FC<TooltipProps> = ({ kind }) => {
	const content = tooltipContent[kind];

	return (
		<div
			className={
				content.isShowLabel
					? `tooltip-context px-2 py-1 ${content.bgColor} ${content.textColor} rounded-md text-xs font-medium flex items-center`
					: "tooltip-context relative inline-flex"
			}
			{...{ "x-data": "{ helpOpen: false }" }}
		>
			{content.isShowLabel && (
				<span class="text-xs font-medium mr-1">{content.label}</span>
			)}

			<button
				type="button"
				class="w-4 h-4 hover:bg-black/10 rounded focus:outline-none focus:ring-2 focus:ring-blue-500 cursor-pointer"
				aria-label={translation.showInformation({
					name: translation.tooltipKind({ kind }),
				})}
				tabindex={0}
				{...{ "x-on:click": "helpOpen = true" }}
				{...{ "x-on:keydown.enter": "helpOpen = true" }}
				{...{ "x-on:keydown.space": "helpOpen = true" }}
			>
				<HelpCircleIcon />
			</button>

			<div
				{...{ "x-show": "helpOpen" }}
				class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-start justify-center pt-16"
				{...{ "x-cloak": "" }}
				{...{ "x-transition:enter": "ease-out duration-300" }}
				{...{ "x-transition:enter-start": "opacity-0" }}
				{...{ "x-transition:enter-end": "opacity-100" }}
				{...{ "x-transition:leave": "ease-in duration-200" }}
				{...{ "x-transition:leave-start": "opacity-100" }}
				{...{ "x-transition:leave-end": "opacity-0" }}
				{...{ "x-on:click": "helpOpen = false" }}
				{...{ "x-on:keydown.escape": "helpOpen = false" }}
			>
				<div
					class="bg-white rounded-lg shadow-xl w-full max-w-md mx-4"
					{...{ "x-on:click.stop": "" }}
					{...{ "x-transition:enter": "ease-out duration-300" }}
					{...{ "x-transition:enter-start": "opacity-0 scale-95" }}
					{...{ "x-transition:enter-end": "opacity-100 scale-100" }}
					{...{ "x-transition:leave": "ease-in duration-200" }}
					{...{ "x-transition:leave-start": "opacity-100 scale-100" }}
					{...{ "x-transition:leave-end": "opacity-0 scale-95" }}
				>
					<div class="flex justify-between items-center p-4 border-b border-gray-200">
						<div
							class={`px-3 py-1 ${content.bgColor} ${content.textColor} rounded-md text-sm font-medium`}
						>
							{content.label}
						</div>
						<button
							type="button"
							class="text-gray-400 hover:text-gray-600 cursor-pointer"
							tabindex={0}
							{...{ "x-on:click": "helpOpen = false" }}
							{...{ "x-on:keydown.enter": "helpOpen = false" }}
							{...{ "x-on:keydown.space": "helpOpen = false" }}
							aria-label={translation.close()}
						>
							<div class="w-6 h-6">
								<CloseIcon />
							</div>
						</button>
					</div>
					<div class="p-4">
						<div class="text-sm font-normal text-gray-700">{content.desc}</div>
					</div>
				</div>
			</div>
		</div>
	);
};
