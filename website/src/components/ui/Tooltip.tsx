import type { FC } from "hono/jsx";
import { HelpCircleIcon } from "../icons";
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
		label: "Element",
		desc: "Element functions can be customized with <code>set</code> and <code>show</code> rules.",
		isShowLabel: true,
		bgColor: "bg-blue-50",
		textColor: "text-blue-700",
	},
	contextual: {
		label: "Contextual",
		desc: "Contextual functions can only be used when the context is known",
		isShowLabel: true,
		bgColor: "bg-indigo-50",
		textColor: "text-indigo-700",
	},
	definitions: {
		label: "Definitions",
		desc: "Functions and types and can have associated definitions. These are accessed by specifying the function or type, followed by a period, and then the definition's name.",
		isShowLabel: false,
		bgColor: "bg-gray-100",
		textColor: "text-gray-700",
	},
	parameters: {
		label: "Parameters",
		desc: "Parameters are the inputs to a function. They are specified in parentheses after the function name.",
		isShowLabel: false,
		bgColor: "bg-gray-100",
		textColor: "text-gray-700",
	},
	variadic: {
		label: "Variadic",
		desc: "Variadic parameters can be specified multiple times.",
		isShowLabel: true,
		bgColor: "bg-green-50",
		textColor: "text-green-700",
	},
	settable: {
		label: "Settable",
		desc: "Settable parameters can be customized for all following uses of the function with a <code>set</code> rule.",
		isShowLabel: true,
		bgColor: "bg-amber-50",
		textColor: "text-amber-700",
	},
	positional: {
		label: "Positional",
		desc: "Positional parameters are specified in order, without names.",
		isShowLabel: true,
		bgColor: "bg-purple-50",
		textColor: "text-purple-700",
	},
	required: {
		label: "Required",
		desc: "Required parameters must be specified when calling the function.",
		isShowLabel: true,
		bgColor: "bg-rose-50",
		textColor: "text-rose-700",
	},
};

export const Tooltip: FC<TooltipProps> = ({ kind }) => {
	const content = tooltipContent[kind];

	if (content.isShowLabel) {
		return (
			<div
				className={`tooltip-context px-2 py-1 ${content.bgColor} ${content.textColor} rounded-md text-xs font-medium flex items-center`}
			>
				<span class="text-xs font-medium mr-1">{content.label}</span>
				<div class="relative group">
					<div class="w-4 h-4">
						<HelpCircleIcon />
					</div>
					<div
						role="tooltip"
						tabIndex={-1}
						class="absolute invisible opacity-0 group-hover:visible group-hover:opacity-100
                  transition-opacity duration-200 bg-gray-900 text-white p-2 rounded shadow-lg
                  text-xs z-50 top-full mt-1 -left-4 w-64"
					>
						<HtmlContent html={content.desc} />
					</div>
				</div>
			</div>
		);
	}
	return (
		<div class="tooltip-context relative group inline-flex">
			<div class="w-4 h-4">
				<HelpCircleIcon />
			</div>
			<div
				role="tooltip"
				tabIndex={-1}
				class="absolute invisible opacity-0 group-hover:visible group-hover:opacity-100
                transition-opacity duration-200 bg-gray-900 text-white p-2 rounded shadow-lg
                text-xs z-50 top-full mt-1 -left-4 w-64"
			>
				<HtmlContent html={content.desc} />
			</div>
		</div>
	);
};
