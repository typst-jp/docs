import { Translation } from "../../../translation/";
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
				<div class="flex flex-col sm:flex-row w-full text-xs leading-6 text-black duration-150 ease-out opacity-80 gap-3">
					<span class="flex items-center flex-shrink-0 gap-2">
						<div class="w-4 h-4">
							<InfoCircleIcon />
						</div>
						<strong>
							<Translation translationKey="siteNoticeBannerTitle" />
						</strong>
					</span>
					<span class="hidden sm:flex items-center">
						<span class="inline-block w-px h-12 bg-neutral-200 mx-3" />
					</span>
					<span class="block flex-1 pt-1 pb-2 leading-normal sm:inline sm:pt-0 sm:pb-0">
						<Translation translationKey="siteNoticeBannerDescription" />
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
