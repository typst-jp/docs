import { robotsTxtPlugin } from "@hono/ssg-plugins-essential/robots-txt";
import { sitemapPlugin } from "@hono/ssg-plugins-essential/sitemap";
import devServer, { defaultOptions } from "@hono/vite-dev-server";
import ssg from "@hono/vite-ssg";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import { basePath, originUrl } from "./src/metadata";
import { joinPath } from "./src/utils/path";

const ssgPlugins =
	basePath === "/"
		? [
				sitemapPlugin({
					baseUrl: originUrl,
				}),
				robotsTxtPlugin({
					rules: [{ userAgent: "*", allow: ["/"] }],
					sitemapUrl: new URL("sitemap.xml", originUrl).href,
				}),
			]
		: [];

export default defineConfig({
	base: basePath,
	plugins: [
		tailwindcss(),
		ssg({
			plugins: ssgPlugins,
		}),
		devServer({
			entry: "src/index.tsx",
			exclude: [
				...defaultOptions.exclude,
				/^\/assets\/.+/,
				/^\/index\.html$/,
				// NOTE: @hono/vite-dev-server does not respect the base setting in the Vite configuration.
				new RegExp(`^${joinPath(basePath, "@")}`),
				new RegExp(`^${joinPath(basePath, "node_modules")}(?:/|$)`),
			],
		}),
	],
	build: {
		rollupOptions: {
			input: ["src/globals.css"],
			output: {
				assetFileNames: "[name].[ext]",
			},
		},
	},
	server: {
		host: process.env.VITE_LISTEN_ALL_ADDRESSES === "true",
	},
});
