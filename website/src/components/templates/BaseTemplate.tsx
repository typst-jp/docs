import type { FC, PropsWithChildren } from "hono/jsx";
import type { Page } from "../../types/model";
import {
	CaretRightCircleIcon,
	ChevronLeftIcon,
	ChevronRightIcon,
	CloseIcon,
	InfoCircleIcon,
} from "../icons";
import {
	Breadcrumbs,
	Footer,
	Header,
	SideNavigation,
	SiteNoticeBanner,
	TableOfContents,
} from "../ui/common/";

export type BaseTemplateProps = PropsWithChildren<{
	page: Page;
	docs: Page[];
	path: Page[];
	previousPage?: Page;
	nextPage?: Page;
}>;

export const BaseTemplate: FC<BaseTemplateProps> = ({
	children,
	page,
	docs,
	path,
	previousPage,
	nextPage,
}) => {
	const title = page.title;
	const description = page.description;
	const route = page.route;
	const outline = page.outline;
	return (
		<html lang="ja">
			<head>
				<meta charSet="utf-8" />
				<title>{title} – Typstドキュメント日本語版</title>
				<meta name="description" content={description} />
				<meta name="viewport" content="width=device-width,initial-scale=1" />
				<meta name="theme-color" content="#239dad" />
				<meta
					property="og:url"
					content={`https://typst-jp.github.io${route}`}
				/>
				<meta
					property="og:title"
					content={`${title} – Typstドキュメント日本語版`}
				/>
				<meta property="og:site_name" content="Typst" />
				<meta property="og:description" content={description} />
				<meta property="og:type" content="" />
				<meta
					property="og:image"
					content="https://typst-jp.github.io/assets/favicon.png"
				/>
				<meta property="og:image:width" content="1200" />
				<meta property="og:image:height" content="630" />
				<meta name="twitter:site" content="@typstapp" />
				<meta name="twitter:card" content="summary_large_image" />
				<link rel="canonical" href={`https://typst-jp.github.io${route}`} />
				<meta name="robots" content="index, follow" />
				<link rel="sitemap" type="application/xml" href="/sitemap.xml" />
				<meta
					name="twitter:image:alt"
					content="The left side of a text editor with colorful cursors, as well as the text 'Compose papers faster, Typst'"
				/>
				<link
					rel="icon"
					type="image/png"
					sizes="32x32"
					href="/assets/favicon.png"
				/>
				<link
					rel="preload"
					href="/assets/fonts/hanken-grotesk/HKGrotesk-Regular.woff2"
					as="font"
					type="font/woff2"
					crossOrigin="anonymous"
				/>
				<link
					rel="preload"
					href="/assets/fonts/hanken-grotesk/HKGrotesk-Bold.woff2"
					as="font"
					type="font/woff2"
					crossOrigin="anonymous"
				/>
				<link
					rel="preload"
					href="/assets/fonts/hanken-grotesk/HKGrotesk-SemiBold.woff2"
					as="font"
					type="font/woff2"
					crossOrigin="anonymous"
				/>
				<link
					rel="preload"
					href="/assets/fonts/cascadia-code/CascadiaMono-Regular-Sub.woff2"
					as="font"
					type="font/woff2"
					crossOrigin="anonymous"
				/>
				<link
					href={
						import.meta.env.DEV ? "/src/globals.css" : "/assets/globals.css"
					}
					rel="stylesheet"
				/>
				<script
					defer
					src="https://cdn.jsdelivr.net/npm/alpinejs@3.14.8/dist/cdn.min.js"
				/>
			</head>

			<body
				class="no-js docs has-outline min-h-screen flex flex-col"
				x-data="{ sidebarOpen: false }"
			>
				<SiteNoticeBanner />
				<Header />

				<div class="main-grid flex-1 flex bg-white relative">
					<div
						class="fixed inset-0 bg-black/50 backdrop-blur-sm z-30 transition-opacity duration-300"
						x-show="sidebarOpen"
						x-cloak
						x-transition:enter="ease-out duration-300"
						x-transition:enter-start="opacity-0"
						x-transition:enter-end="opacity-100"
						x-transition:leave="ease-in duration-200"
						x-transition:leave-start="opacity-100"
						x-transition:leave-end="opacity-0"
						x-on:click="sidebarOpen = false"
					/>
					<div class="container mx-auto max-w-8xl px-4 sm:px-6 lg:px-8 flex relative">
						<div
							x-cloak
							class="fixed inset-y-0 left-0 w-64 bg-white shadow-xl z-30 transform transition-transform duration-300 ease-in-out lg:hidden"
							x-bind:class="sidebarOpen ? 'translate-x-0' : '-translate-x-full'"
						>
							<div class="flex justify-end p-4 lg:hidden">
								<button
									type="button"
									class="text-gray-600"
									x-on:click="sidebarOpen = false"
									aria-label="メニューを閉じる"
								>
									<div class="w-6 h-6 text-gray-600 hover:text-gray-800 transition-colors">
										<CloseIcon />
									</div>
								</button>
							</div>
							<SideNavigation
								docs={docs}
								currentRoute={route}
								currentPath={path}
							/>
						</div>

						<div class="hidden lg:flex lg:flex-col lg:w-64 lg:mr-4">
							<SideNavigation
								docs={docs}
								currentRoute={route}
								currentPath={path}
							/>
						</div>

						<main class="flex-1 flex flex-col px-3.5 py-4 mb-8">
							<Breadcrumbs path={path} />

							<div class="prose max-w-none w-full mt-6 flex-grow">
								{children}
							</div>

							{route === "/docs/" ? (
								<div class="doc-categories grid grid-cols-1 md:grid-cols-2 gap-6 mt-8">
									<a
										class="doc-category flex flex-col p-6 bg-white border border-gray-200 rounded-lg hover:border-gray-500 hover:bg-gray-50 transition-all duration-200"
										href="/docs/tutorial"
									>
										<div class="flex items-center mb-3">
											<div class="w-6 h-6 text-gray-800 mr-2">
												<CaretRightCircleIcon />
											</div>
											<strong class="text-base font-semibold text-gray-800">
												チュートリアル
											</strong>
										</div>
										<p class="text-sm text-gray-600">
											一歩一歩、Typstの使い方を学びましょう。
										</p>
									</a>
									<a
										class="doc-category flex flex-col p-6 bg-white border border-gray-200 rounded-lg hover:border-gray-500 hover:bg-gray-50 transition-all duration-200"
										href="/docs/reference"
									>
										<div class="flex items-center mb-3">
											<div class="w-6 h-6 text-gray-800 mr-2">
												<InfoCircleIcon />
											</div>
											<strong class="text-base font-semibold text-gray-800">
												リファレンス
											</strong>
										</div>
										<p class="text-sm text-gray-600">
											Typstのあらゆる構文、概念、型、関数についての詳細なリファレンスです。
										</p>
									</a>
								</div>
							) : (
								previousPage &&
								nextPage && (
									<div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-12">
										<a
											href={previousPage.route}
											class="flex flex-col p-6 bg-white border border-gray-200 rounded-lg hover:border-gray-500 hover:bg-gray-50 transition-all duration-200"
										>
											<div class="flex items-center mb-3">
												<div class="w-6 h-6 text-gray-400 mr-2">
													<ChevronLeftIcon />
												</div>
												<strong class="text-base font-semibold text-gray-800">
													前のページ
												</strong>
											</div>
											<p class="text-sm text-gray-600">{previousPage.title}</p>
										</a>
										<a
											href={nextPage.route}
											class="flex flex-col p-6 bg-white border border-gray-200 rounded-lg hover:border-gray-500 hover:bg-gray-50 transition-all duration-200"
										>
											<div class="flex items-center mb-3 justify-between">
												<strong class="text-base font-semibold text-gray-800">
													次のページ
												</strong>
												<div class="w-6 h-6 text-gray-400">
													<ChevronRightIcon />
												</div>
											</div>
											<p class="text-sm text-gray-600">{nextPage.title}</p>
										</a>
									</div>
								)
							)}
						</main>

						<div class="flex-col w-full md:w-60 lg:w-72 ml-4 hidden xl:block">
							<TableOfContents outline={outline} />
						</div>
					</div>
				</div>

				<Footer />
			</body>
		</html>
	);
};

export default BaseTemplate;
