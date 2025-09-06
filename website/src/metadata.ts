import { version } from "../package.json";

// TODO: The metadata will be configurable via a JSON configuration file.
/** The version of the documentation, without a leading `v`. */
export { version };
/** The official Typst website URL. */
export const typstOfficialUrl = "https://typst.app";
/** The official Typst documentation base URL. */
export const typstOfficialDocsUrl: `http://${string}/` | `https://${string}/` =
	"https://typst.app/docs/";
/** The GitHub organization URL. */
export const githubOrganizationUrl = "https://github.com/typst-jp";
/** The GitHub repository URL. */
export const githubRepositoryUrl = "https://github.com/typst-jp/docs";
/** The Discord server invite URL. */
export const discordServerUrl = "https://discord.gg/9xF7k4aAuH";
/** The origin URL of the deployed site, used for metadata. Note that the base path should not be included. */
export const originUrl = "https://typst-jp.github.io/";
/** The base public path for deployment. This must match the value used in typst-docs. */
export const basePath: "/" | `/${string}/` = "/docs/";
/** Indicates whether to display the translation status on the site. Community content is always displayed. */
export const displayTranslationStatus: boolean = true;
