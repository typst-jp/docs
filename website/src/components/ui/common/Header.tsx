import {
	discordServerUrl,
	githubRepositoryUrl,
	typstOfficialDocsUrl,
	typstOfficialUrl,
	version,
} from "../../../metadata";
import { DiscordIcon, GitHubIcon, MenuIcon } from "../../icons";
import { SiteTitle } from "./SiteTitle";

const VersionBadge = () => (
	<span class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium bg-gray-200 text-gray-600 ml-2">
		v{version}
	</span>
);

export const Header = () => {
	return (
		<>
			<header class="boring sticky top-0 z-40 bg-white border-b border-gray-200 hidden lg:block">
				<div class="flex justify-between items-center py-3 px-6">
					<div class="flex items-center">
						<SiteTitle />
						<VersionBadge />
					</div>
					<nav class="ml-auto">
						<ul class="flex items-center gap-4">
							<li class="social">
								<a
									href={discordServerUrl}
									class="text-gray-600 hover:text-gray-800 transition-colors"
								>
									<div class="w-4 h-4">
										<DiscordIcon />
									</div>
								</a>
							</li>
							<li class="social">
								<a
									href={githubRepositoryUrl}
									class="text-gray-600 hover:text-gray-800 transition-colors"
								>
									<div class="w-4 h-4 text-gray-600 hover:text-gray-800 transition-colors">
										<GitHubIcon />
									</div>
								</a>
							</li>
							<li class="secondary">
								<a
									href={typstOfficialUrl}
									class="text-sm text-gray-600 hover:text-gray-800 transition-colors"
								>
									Typst公式サイト
								</a>
							</li>
							<li class="secondary">
								<a
									href={typstOfficialDocsUrl}
									class="text-sm text-gray-600 hover:text-gray-800 transition-colors"
								>
									Typst公式ドキュメント
								</a>
							</li>
						</ul>
					</nav>
				</div>
			</header>

			<header class="sticky top-0 z-30 bg-white border-b border-gray-200 flex lg:hidden items-center justify-between px-4">
				<div class="flex justify-between items-center py-3 w-full">
					<div class="flex items-center">
						<SiteTitle />
						<VersionBadge />
					</div>
					<button
						type="button"
						class="p-1 bg-white rounded-md border border-gray-200"
						x-on:click="sidebarOpen = !sidebarOpen"
						aria-label="メニューを開く"
					>
						<div class="w-6 h-6 text-gray-600 hover:text-gray-800 transition-colors">
							<MenuIcon />
						</div>
					</button>
				</div>
			</header>
		</>
	);
};
