import { basePath } from "../../../metadata";
import { Translation } from "../../../translation";

export const SiteTitle = () => {
	return (
		<a href={basePath} class="logo-box hover:opacity-80 transition-opacity">
			<div class="flex items-baseline">
				<span class="text-base font-bold text-teal-600">Typst</span>
				<span class="text-base font-medium text-gray-600 ml-1">
					<Translation translationKey="document" />
				</span>
				<span class="text-xs text-gray-600 ml-1">
					<Translation translationKey="langVersion" />
				</span>
			</div>
		</a>
	);
};
