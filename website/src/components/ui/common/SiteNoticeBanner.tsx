import {
	discordServerUrl,
	githubOrganizationUrl,
	githubRepositoryUrl,
	typstOfficialDocsUrl,
	version,
} from "../../../metadata";
import { InfoCircleIcon } from "../../icons";

export const SiteNoticeBanner = () => {
	return (
		<div
			x-data="{
        bannerVisible: false,
        bannerVisibleAfter: 300,
        checkBannerStatus() {
          const isBannerHidden = localStorage.getItem('typst-jp-banner-hidden') === 'true';
          if (!isBannerHidden) {
            setTimeout(() => {
              this.bannerVisible = true;
              this.$el.classList.remove('-translate-y-full');
            }, this.bannerVisibleAfter);
          }
        },
        hideBanner() {
          this.bannerVisible = false;
          localStorage.setItem('typst-jp-banner-hidden', 'true');
        }
      }"
			x-init="checkBannerStatus()"
			x-show="bannerVisible"
			x-transition:enter="transition ease-out duration-500"
			x-transition:enter-start="-translate-y-full"
			x-transition:enter-end="translate-y-0"
			x-transition:leave="transition ease-in duration-300"
			x-transition:leave-start="translate-y-0"
			x-transition:leave-end="-translate-y-full"
			class="fixed z-50 top-0 left-0 w-full h-auto py-2 duration-300 ease-out bg-white shadow-sm sm:py-4 -translate-y-full"
		>
			<div class="prose relative flex flex-col sm:flex-row items-start w-full px-3 sm:px-12 mx-auto max-w-7xl flex-wrap">
				<div class="flex flex-col sm:flex-row w-full text-xs leading-6 text-black duration-150 ease-out opacity-80 hover:opacity-100 gap-3">
					<span class="flex items-center flex-shrink-0 gap-2">
						<div class="w-4 h-4">
							<InfoCircleIcon />
						</div>
						<strong>情報 / Info</strong>
					</span>
					<span class="hidden sm:flex items-center">
						<span class="inline-block w-px h-12 bg-neutral-200 mx-3" />
					</span>
					<span class="block flex-1 pt-1 pb-2 leading-normal sm:inline sm:pt-0 sm:pb-0">
						当サイトは、Typst GmbHの許諾を得て、日本語コミュニティ「
						<a href={githubOrganizationUrl}>Typst Japan Community</a>」が
						<a href={typstOfficialDocsUrl}>
							Typst v{version}の公式ドキュメント
						</a>
						を翻訳したものです。誤訳や古い情報が含まれている可能性があるため、
						<a href={typstOfficialDocsUrl}>公式ドキュメント</a>
						との併用を推奨します。翻訳の改善やサイトの機能向上について、
						<a href={githubRepositoryUrl}>GitHub</a>でのIssueやPull
						Requestを歓迎します。コミュニティにご興味のある方は
						<a href={discordServerUrl}>Discordサーバー「くみはんクラブ」</a>
						にぜひご参加ください。
						<br />
						This site provides a Japanese translation of the{" "}
						<a href={typstOfficialDocsUrl}>Typst v{version} documentation</a>{" "}
						maintained by the "
						<a href={githubOrganizationUrl}>Typst Japan Community</a>" with
						permission from Typst GmbH. We recommend using this alongside the{" "}
						<a href={typstOfficialDocsUrl}>official documentation</a>. We
						welcome contributions through Issues and Pull Requests on{" "}
						<a href={githubRepositoryUrl}>our GitHub repository</a> for both
						translation improvements and website enhancements. Feel free to join{" "}
						<a href={discordServerUrl}>our Discord server "Kumihan Club"</a>.
					</span>
				</div>
			</div>
			<button
				type="button"
				x-on:click="hideBanner()"
				class="absolute top-2 right-4 flex items-center flex-shrink-0 translate-x-1 ease-out duration-150 justify-center w-6 h-6 p-1.5 text-black rounded-full hover:bg-neutral-100"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke-width="1.5"
					stroke="currentColor"
					class="w-full h-full"
				>
					<title>Close</title>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M6 18L18 6M6 6l12 12"
					/>
				</svg>
			</button>
		</div>
	);
};
