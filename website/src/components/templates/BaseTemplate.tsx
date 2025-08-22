import { html } from "hono/html";
import type { FC, PropsWithChildren } from "hono/jsx";
import { basePath, originUrl, typstOfficialDocsUrl } from "../../metadata";
import type { Page } from "../../types/model";
import { joinPath, removeBasePath } from "../../utils/path";
import { getTranslationStatus } from "../../utils/translationStatus";
import {
	CaretRightCircleIcon,
	ChevronLeftIcon,
	ChevronRightIcon,
	CloseIcon,
	InfoCircleIcon,
	WorldIcon,
} from "../icons";
import {
	Breadcrumbs,
	Footer,
	Header,
	SearchWindow,
	SideNavigation,
	SiteNoticeBanner,
	TableOfContents,
	TranslationStatusAlert,
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
	const translationStatus = getTranslationStatus(route);
	const absoluteRouteUrl = new URL(route, originUrl).toString();
	const faviconUrl = new URL(
		joinPath(basePath, "/favicon.png"),
		originUrl,
	).toString();
	const typstOfficialRouteUrl = joinPath(
		typstOfficialDocsUrl,
		removeBasePath(basePath, route),
	);
	return (
		<html lang="ja" class="scroll-pt-24">
			<head>
				<meta charSet="utf-8" />
				<title>{title} – Typstドキュメント日本語版</title>
				<meta name="description" content={description} />
				<meta name="viewport" content="width=device-width,initial-scale=1" />
				<meta name="theme-color" content="#239dad" />
				<meta property="og:url" content={absoluteRouteUrl} />
				<meta
					property="og:title"
					content={`${title} – Typstドキュメント日本語版`}
				/>
				<meta property="og:site_name" content="Typst" />
				<meta property="og:description" content={description} />
				<meta property="og:type" content="" />
				<meta property="og:image" content={faviconUrl} />
				<meta property="og:image:width" content="1200" />
				<meta property="og:image:height" content="630" />
				<meta name="twitter:site" content="@typstapp" />
				<meta name="twitter:card" content="summary_large_image" />
				<link rel="canonical" href={absoluteRouteUrl} />
				<meta name="robots" content="index, follow" />
				<meta
					name="twitter:image:alt"
					content="The left side of a text editor with colorful cursors, as well as the text 'Compose papers faster, Typst'"
				/>
				<link
					rel="icon"
					type="image/png"
					sizes="32x32"
					href={joinPath(basePath, "/favicon.png")}
				/>
				<link
					rel="preload"
					href={joinPath(
						basePath,
						"/fonts/hanken-grotesk/HKGrotesk-Regular.woff2",
					)}
					as="font"
					type="font/woff2"
					crossOrigin="anonymous"
				/>
				<link
					rel="preload"
					href={joinPath(
						basePath,
						"/fonts/hanken-grotesk/HKGrotesk-Bold.woff2",
					)}
					as="font"
					type="font/woff2"
					crossOrigin="anonymous"
				/>
				<link
					rel="preload"
					href={joinPath(
						basePath,
						"/fonts/hanken-grotesk/HKGrotesk-SemiBold.woff2",
					)}
					as="font"
					type="font/woff2"
					crossOrigin="anonymous"
				/>
				<link
					rel="preload"
					href={joinPath(
						basePath,
						"/fonts/cascadia-code/CascadiaMono-Regular-Sub.woff2",
					)}
					as="font"
					type="font/woff2"
					crossOrigin="anonymous"
				/>
				<link
					href={
						import.meta.env.DEV
							? joinPath(basePath, "/src/globals.css")
							: joinPath(basePath, "/globals.css")
					}
					rel="stylesheet"
				/>
				<script
					defer
					src="https://cdn.jsdelivr.net/npm/alpinejs@3.14.8/dist/cdn.min.js"
				/>
				{/* NOTE: @hono/vite-dev-server does not respect the base setting in the Vite configuration. */}
				{import.meta.env.DEV &&
					html`
          <script>
			import("${joinPath(basePath, "/@vite/client")}")
          </script>
        `}
			</head>

			<body
				class="no-js docs has-outline min-h-screen flex flex-col"
				x-data="{ sidebarOpen: false, searchOpen: false }"
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

							<div class="flex flex-col gap-2 my-4">
								<TranslationStatusAlert status={translationStatus} />
							</div>

							{translationStatus !== "community" && (
								<div class="flex">
									<a
										href={typstOfficialRouteUrl}
										target="_blank"
										rel="noopener noreferrer"
										class="inline-flex items-center text-sm  underline text-gray-400 hover:text-gray-600 transition-colors"
									>
										<div class="w-4 h-4 mr-1 ">
											<WorldIcon />
										</div>
										原文（英語）を開く
									</a>
								</div>
							)}

							<div class="prose max-w-none w-full my-6 flex-grow">
								{children}
							</div>

							{translationStatus !== "community" && (
								<a
									href={typstOfficialRouteUrl}
									target="_blank"
									rel="noopener noreferrer"
									class="group inline-flex items-center px-3 py-2 rounded-md border border-gray-200 bg-white hover:border-gray-500 hover:bg-gray-50 transition-all duration-200 w-fit"
								>
									<div class="w-4 h-4 mr-2 text-gray-600 transition-colors">
										<WorldIcon />
									</div>
									<span class="text-sm font-medium text-gray-800 group-hover:text-gray-900 transition-colors">
										原文（英語）を開く
									</span>
									<div class="w-4 h-4 ml-2 text-gray-400 transition-colors">
										<ChevronRightIcon />
									</div>
								</a>
							)}

							{route === basePath ? (
								<div class="doc-categories grid grid-cols-1 md:grid-cols-2 gap-6 mt-8">
									<a
										class="doc-category flex flex-col p-6 bg-white border border-gray-200 rounded-lg hover:border-gray-500 hover:bg-gray-50 transition-all duration-200"
										href={joinPath(basePath, "/tutorial/")}
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
										href={joinPath(basePath, "/reference/")}
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

				<div
					class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-start justify-center pt-16"
					x-show="searchOpen"
					x-cloak
					x-transition:enter="ease-out duration-300"
					x-transition:enter-start="opacity-0"
					x-transition:enter-end="opacity-100"
					x-transition:leave="ease-in duration-200"
					x-transition:leave-start="opacity-100"
					x-transition:leave-end="opacity-0"
					x-on:click="searchOpen = false"
				>
					<div
						class="bg-white rounded-lg shadow-xl w-full max-w-2xl mx-4"
						x-on:click="$event.stopPropagation()"
						x-transition:enter="ease-out duration-300"
						x-transition:enter-start="opacity-0 scale-95"
						x-transition:enter-end="opacity-100 scale-100"
						x-transition:leave="ease-in duration-200"
						x-transition:leave-start="opacity-100 scale-100"
						x-transition:leave-end="opacity-0 scale-95"
					>
						<SearchWindow />
					</div>
				</div>

				<Footer />
			</body>
		</html>
	);
};

export default BaseTemplate;
