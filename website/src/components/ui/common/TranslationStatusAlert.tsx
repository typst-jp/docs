import type { FC } from "hono/jsx";
import { twMerge } from "tailwind-merge";
import type { TranslationStatus } from "../../../utils/translationStatus";
import { LanguageIcon } from "../../icons";

type StatusConfig = {
	bgColor: string;
	borderColor: string;
	textColor: string;
	iconColor: string;
	label: string;
	message: string;
};

const getStatusConfig = (status: TranslationStatus): StatusConfig => {
	switch (status) {
		case "translated":
			return {
				bgColor: "bg-green-50",
				borderColor: "border-green-200",
				textColor: "text-green-800",
				iconColor: "text-green-600",
				label: "翻訳済み",
				message: "このページは日本語に翻訳済みです。",
			};
		case "partially_translated":
			return {
				bgColor: "bg-yellow-50",
				borderColor: "border-yellow-200",
				textColor: "text-yellow-800",
				iconColor: "text-yellow-600",
				label: "部分的に翻訳済み",
				message:
					"このページは部分的に翻訳されています。一部原文の内容が含まれています。",
			};
		case "untranslated":
			return {
				bgColor: "bg-red-50",
				borderColor: "border-red-200",
				textColor: "text-red-800",
				iconColor: "text-red-600",
				label: "未翻訳",
				message:
					"このページはまだ翻訳されていません。原文の内容が表示されています。",
			};
	}
};

export type TranslationStatusAlertProps = {
	status: TranslationStatus;
};

export const TranslationStatusAlert: FC<TranslationStatusAlertProps> = ({
	status,
}) => {
	const config = getStatusConfig(status);
	return (
		<div
			class={twMerge(
				"border rounded-md p-4 mb-6",
				config.bgColor,
				config.borderColor,
				config.textColor,
			)}
		>
			<div class="flex items-start">
				<div class={twMerge("w-5 h-5 mr-3 flex-shrink-0", config.iconColor)}>
					<LanguageIcon />
				</div>
				<div class="flex-1">
					<div class="text-sm font-bold mb-1">{config.label}</div>
					<p class="text-sm">{config.message}</p>
				</div>
			</div>
		</div>
	);
};
