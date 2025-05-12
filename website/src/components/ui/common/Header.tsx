import {
	discordServerUrl,
	githubRepositoryUrl,
	typstOfficialDocsUrl,
	typstOfficialUrl,
} from "../../../metadata";
import { DiscordIcon, GitHubIcon } from "../../icons";
import { SiteTitle } from "./SiteTitle";

export const Header = () => {
	return (
		<header class="boring sticky top-0 z-40 bg-white border-b border-gray-200">
			<div class="flex justify-between items-center py-3 px-6">
				<SiteTitle />
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
	);
};
