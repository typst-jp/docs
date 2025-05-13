import type { Page } from "../../../types/model";
import { ChevronRightIcon, HomeIcon } from "../../icons";

export type BreadcrumbsProps = {
	path: Page[];
};

export const Breadcrumbs = ({ path }: BreadcrumbsProps) => {
	return (
		<nav class="flex justify-between px-3.5 py-1 border border-neutral-200/60 rounded-md">
			<ol class="inline-flex flex-wrap items-center space-x-1 text-sm text-gray-600">
				<li class="flex items-center h-full">
					<a href="/docs/" class="py-1 hover:text-gray-800 transition-colors">
						<div class="w-4 h-4">
							<HomeIcon />
						</div>
					</a>
				</li>
				{path.map((item, idx) => (
					<li key={item.route} class="flex items-center">
						<div class="w-4 h-4 text-gray-400">
							<ChevronRightIcon />
						</div>
						{idx === path.length - 1 ? (
							<span class="inline-flex items-center py-1 font-semibold text-gray-800 cursor-default">
								{item.title}
							</span>
						) : (
							<a
								href={item.route}
								class="inline-flex items-center py-1 hover:text-gray-800 transition-colors focus:outline-none"
							>
								{item.title}
							</a>
						)}
					</li>
				))}
			</ol>
		</nav>
	);
};
