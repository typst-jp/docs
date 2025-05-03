import {
  discordServerUrl,
  githubOrganizationUrl,
  githubRepositoryUrl,
} from "../../../metadata";
import { DiscordIcon, GitHubIcon } from "../../icons";

export const Footer = () => {
  return (
    <footer>
      <section class="bg-white">
        <hr class="border-t border-gray-200" />
        <div class="prose max-w-screen-xl px-4 py-12 mx-auto space-y-8 overflow-hidden sm:px-6 lg:px-8">
          <div class="flex justify-center mt-8 space-x-6">
            <a
              href={githubRepositoryUrl}
              class="text-gray-400 hover:text-gray-500"
            >
              <span class="sr-only">GitHub</span>
              <div class="w-6 h-6">
                <GitHubIcon />
              </div>
            </a>
            <a
              href={discordServerUrl}
              class="text-gray-400 hover:text-gray-500"
            >
              <span class="sr-only">Discord</span>
              <div class="w-6 h-6">
                <DiscordIcon />
              </div>
            </a>
          </div>
          <p class="mt-8 text-base leading-6 text-center text-gray-400">
            Translated by{" "}
            <a href={githubOrganizationUrl}>Typst Japan Community</a>
          </p>
        </div>
      </section>
    </footer>
  );
};
