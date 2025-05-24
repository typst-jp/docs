import type { FC } from "hono/jsx";
import { CloseIcon } from "../../icons";

export const SearchWindow: FC = () => {
	return (
		<div class="flex flex-col max-h-[80vh]">
			<link href="/pagefind/pagefind-ui.css" rel="stylesheet" />
			<script src="/pagefind/pagefind-ui.js" />
			<div class="flex justify-between items-center p-4 border-b border-gray-200 flex-shrink-0">
				<h2 class="text-lg font-semibold">検索</h2>
				<button
					type="button"
					class="text-gray-400 hover:text-gray-600"
					x-on:click="searchOpen = false"
					aria-label="検索を閉じる"
				>
					<div class="w-6 h-6 text-gray-600 hover:text-gray-800 transition-colors">
						<CloseIcon />
					</div>
				</button>
			</div>
			<div class="p-4 overflow-y-auto flex-1">
				<div id="search" />
			</div>
			<script
				// biome-ignore lint/security/noDangerouslySetInnerHtml: pagefindで生成されたスクリプトを実行する
				dangerouslySetInnerHTML={{
					__html: `window.addEventListener('DOMContentLoaded', (event) => {
	new PagefindUI({ element: "#search", showSubResults: true });
});
`,
				}}
			/>
		</div>
	);
};
