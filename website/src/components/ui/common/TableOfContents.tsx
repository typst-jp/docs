import type { OutlineItem } from "../../../types/model";

export type TableOfContentsProps = {
	outline: OutlineItem[];
};

export const TableOfContents = ({ outline }: TableOfContentsProps) => {
	if (outline.length === 0) {
		return null;
	}

	return (
		<nav
			id="page-overview"
			class="flex-none w-full px-3.5 py-3 border border-neutral-200/60 rounded-md bg-white sticky top-[80px] mt-4 mb-8 h-[calc(100vh-80px-1rem)] overflow-auto"
		>
			<strong class="block mb-2 text-sm text-neutral-500 font-semibold tracking-wide">
				目次
			</strong>
			<ol class="space-y-1 text-sm text-neutral-700">
				{outline.map((item) => (
					<li key={item.id} data-assoc={item.id}>
						<a
							href={`#${item.id}`}
							class="block px-2 py-1 rounded hover:bg-neutral-100 transition-colors"
						>
							{item.name}
						</a>
					</li>
				))}
			</ol>
		</nav>
	);
};
