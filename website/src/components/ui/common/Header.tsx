import {
	discordServerUrl,
	githubRepositoryUrl,
	typstOfficialDocsUrl,
	typstOfficialUrl,
	version,
} from "../../../metadata";
import { Translation, translation } from "../../../translation";
import { calculateTranslationProgressRate } from "../../../utils/translationStatus";
import {
	DiscordIcon,
	GitHubIcon,
	LanguageIcon,
	MenuIcon,
	SearchIcon,
} from "../../icons";
import { SiteTitle } from "./SiteTitle";

const VersionBadge = () => (
	<span class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium bg-gray-200 text-gray-600">
		v{version}
	</span>
);

const TranslationCoverageBadge = () => {
	const rate = calculateTranslationProgressRate() * 100;
	return (
		<div class="inline-flex items-center gap-2 px-2 py-1 border border-gray-200 rounded-md bg-gray-50">
			<div class="flex items-center gap-1">
				<div class="w-4 h-4 text-gray-600">
					<LanguageIcon />
				</div>
				<span class="text-xs text-gray-600 font-medium">
					<Translation translationKey="translationRate" />
				</span>
			</div>
			<div class="flex items-center gap-1">
				<div class="w-12 h-1.5 bg-gray-200 rounded-full overflow-hidden">
					<div
						class="h-full bg-green-500 rounded-full transition-all duration-300"
						style={`width: ${rate}%`}
					/>
				</div>
				<span class="text-xs text-gray-600 font-medium">
					{rate.toFixed(0)}%
				</span>
			</div>
		</div>
	);
};

export const Header = () => {
	return (
		<>
			<header class="boring sticky top-0 z-40 bg-white border-b border-gray-200 hidden lg:block">
				<div class="relative flex items-center justify-between h-16 px-6">
					<div class="flex items-center gap-2 flex-shrink-0">
						<SiteTitle />
						<VersionBadge />
						<TranslationCoverageBadge />
					</div>

					<div class="xl:absolute xl:left-1/2 xl:transform xl:-translate-x-1/2">
						<button
							type="button"
							class="flex items-center gap-2 px-4 py-2 text-sm text-gray-600 hover:text-gray-800 border border-gray-200 rounded-md hover:border-gray-300 transition-colors xl:w-64 w-48 h-10"
							x-on:click="searchOpen = true"
						>
							<div class="w-4 h-4 text-gray-600">
								<SearchIcon />
							</div>
							<span class="text-left flex-1">
								<Translation translationKey="search" />
								...
							</span>
						</button>
					</div>

					<div class="flex items-center gap-4 flex-shrink-0">
						<nav>
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
										<Translation translationKey="typstOfficialWebsite" />
									</a>
								</li>
								<li class="secondary">
									<a
										href={typstOfficialDocsUrl}
										class="text-sm text-gray-600 hover:text-gray-800 transition-colors"
									>
										<Translation translationKey="typstOfficialDocs" />
									</a>
								</li>
							</ul>
						</nav>
					</div>
				</div>
			</header>

			<header class="sticky top-0 z-30 bg-white border-b border-gray-200 flex lg:hidden items-center justify-between px-4">
				<div class="flex justify-between items-center py-3 w-full min-h-16">
					<div class="flex flex-col gap-1 min-w-0">
						<div class="flex items-center gap-2">
							<SiteTitle />
							<VersionBadge />
						</div>
						<div>
							<TranslationCoverageBadge />
						</div>
					</div>
					<div class="flex items-center gap-2 flex-shrink-0 self-center">
						<button
							type="button"
							class="p-2 text-gray-600 hover:text-gray-800 transition-colors"
							x-on:click="searchOpen = true"
							aria-label={translation.openSearch()}
						>
							<div class="w-6 h-6 text-gray-600 hover:text-gray-800 transition-colors">
								<SearchIcon />
							</div>
						</button>
						<button
							type="button"
							class="p-1 bg-white rounded-md border border-gray-200"
							x-on:click="sidebarOpen = !sidebarOpen"
							aria-label={translation.openMenu()}
						>
							<div class="w-6 h-6 text-gray-600 hover:text-gray-800 transition-colors">
								<MenuIcon />
							</div>
						</button>
					</div>
				</div>
			</header>
		</>
	);
};
