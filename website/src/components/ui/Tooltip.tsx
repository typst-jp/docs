import type { FC } from "hono/jsx";
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
		label: "要素関数",
		desc: "要素関数は<code>set</code>ルールや<code>show</code>ルールでカスタマイズできます。",
		isShowLabel: true,
		bgColor: "bg-blue-50",
		textColor: "text-blue-700",
	},
	contextual: {
		label: "コンテキスト関数",
		desc: "コンテキスト関数は、コンテキストが既知の場合にのみ使用できます。",
		isShowLabel: true,
		bgColor: "bg-indigo-50",
		textColor: "text-indigo-700",
	},
	definitions: {
		label: "定義",
		desc: "これらの関数や型には、関連する定義を持たせることができます。定義にアクセスするには、対象の関数や型の名前を指定した後に、ピリオド区切りで定義名を記述します。",
		isShowLabel: false,
		bgColor: "bg-gray-100",
		textColor: "text-gray-700",
	},
	parameters: {
		label: "引数",
		desc: "引数は関数への入力値です。関数名の後に括弧で囲んで指定します。",
		isShowLabel: false,
		bgColor: "bg-gray-100",
		textColor: "text-gray-700",
	},
	variadic: {
		label: "可変長引数",
		desc: "可変長引数は複数回指定することができます。",
		isShowLabel: true,
		bgColor: "bg-green-50",
		textColor: "text-green-700",
	},
	settable: {
		label: "設定可能引数",
		desc: "設定可能引数は、<code>set</code>ルールを用いて設定でき、それ以降で使用するデフォルト値を変更できます。",
		isShowLabel: true,
		bgColor: "bg-amber-50",
		textColor: "text-amber-700",
	},
	positional: {
		label: "位置引数",
		desc: "位置引数は順序通りに指定することで、引数名を省略して設定できます。",
		isShowLabel: true,
		bgColor: "bg-purple-50",
		textColor: "text-purple-700",
	},
	required: {
		label: "必須引数",
		desc: "必須引数は、関数を呼び出す際に必ず指定しなければなりません。",
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
				aria-label={`${content.label}の詳細情報を表示`}
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
							aria-label="閉じる"
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
