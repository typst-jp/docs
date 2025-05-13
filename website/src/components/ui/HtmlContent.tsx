import type { FC } from "hono/jsx";

export type HtmlContentProps = {
	html: string;
};

export const HtmlContent: FC<HtmlContentProps> = ({ html }) => {
	return (
		<div
			class="[&_img]:mx-auto [&_img]:block [&_img]:max-w-full"
			// biome-ignore lint/security/noDangerouslySetInnerHtml: typst-docsで生成されたHTMLを表示する
			dangerouslySetInnerHTML={{ __html: html }}
		/>
	);
};
