import type { FC } from "hono/jsx";
import { twMerge } from "tailwind-merge";
import type { TranslationStatus } from "../../../utils/translationStatus";
import { LanguageIcon } from "../../icons";
import { menuTranslations } from "../../../translations";

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
				label: menuTranslations.translated,
				message: menuTranslations.translatedMessage,
			};
		case "partially_translated":
			return {
				bgColor: "bg-yellow-50",
				borderColor: "border-yellow-200",
				textColor: "text-yellow-800",
				iconColor: "text-yellow-600",
				label: menuTranslations.partiallyTranslated,
				message: menuTranslations.partiallyTranslatedMessage
					,
			};
		case "untranslated":
			return {
				bgColor: "bg-red-50",
				borderColor: "border-red-200",
				textColor: "text-red-800",
				iconColor: "text-red-600",
				label: menuTranslations.untranslated,
				message: menuTranslations.untranslatedMessage,
			};
		case "community":
			return {
				bgColor: "bg-cyan-50",
				borderColor: "border-cyan-200",
				textColor: "text-cyan-800",
				iconColor: "text-cyan-600",
				label: menuTranslations.originalVersion,
				message: menuTranslations.contentAddedByCommunity,
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
				"border rounded-md p-4",
				config.bgColor,
				config.borderColor,
				config.textColor
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
