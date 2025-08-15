import type { FC } from "hono/jsx";
import { menuTranslations } from "../../translations";
import { CloseIcon, HelpCircleIcon } from "../icons";
import { HtmlContent } from "./HtmlContent";

type TooltipProps = {
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
		label: string;
		desc: string;
		isShowLabel: boolean;
		bgColor: string;
		textColor: string;
	}
> = {
	element: {
		label: menuTranslations.elementFunction,
		desc: menuTranslations.elementFunctionDesc,
		isShowLabel: true,
		bgColor: "bg-blue-50",
		textColor: "text-blue-700",
	},
	contextual: {
		label: menuTranslations.contextFunction,
		desc: menuTranslations.contextFunctionDesc,
		isShowLabel: true,
		bgColor: "bg-indigo-50",
		textColor: "text-indigo-700",
	},
	definitions: {
		label: menuTranslations.definitionTooltip,
		desc: menuTranslations.definitionTooltipDesc,
		isShowLabel: false,
		bgColor: "bg-gray-100",
		textColor: "text-gray-700",
	},
	parameters: {
		label: menuTranslations.argument,
		desc: menuTranslations.argumentDesc,
		isShowLabel: false,
		bgColor: "bg-gray-100",
		textColor: "text-gray-700",
	},
	variadic: {
		label: menuTranslations.variadic,
		desc: menuTranslations.variadicDesc,
		isShowLabel: true,
		bgColor: "bg-green-50",
		textColor: "text-green-700",
	},
	settable: {
		label: menuTranslations.settable,
		desc: menuTranslations.settableDesc,
		isShowLabel: true,
		bgColor: "bg-amber-50",
		textColor: "text-amber-700",
	},
	positional: {
		label: menuTranslations.positional,
		desc: menuTranslations.positionalDesc,
		isShowLabel: true,
		bgColor: "bg-purple-50",
		textColor: "text-purple-700",
	},
	required: {
		label: menuTranslations.required,
		desc: menuTranslations.requiredDesc,
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
				aria-label={`${content.label}${menuTranslations.showInformation}`}
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
							aria-label={menuTranslations.close}
						>
							<div class="w-6 h-6">
								<CloseIcon />
							</div>
						</button>
					</div>
					<div class="p-4">
						<div class="text-sm font-normal text-gray-700">
							<HtmlContent html={content.desc} />
						</div>
					</div>
				</div>
			</div>
		</div>
	);
};
