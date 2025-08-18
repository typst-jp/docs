import { rmSync, symlinkSync } from "node:fs";
import { resolve } from "node:path";
import { robotsTxtPlugin } from "@hono/ssg-plugins-essential/robots-txt";
import { sitemapPlugin } from "@hono/ssg-plugins-essential/sitemap";
import { defaultOptions } from "@hono/vite-dev-server";
import devServer from "@hono/vite-dev-server";
import ssg from "@hono/vite-ssg";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import { basePath, originUrl } from "./src/metadata";

// typst-docsが生成したドキュメントのアセットをassets/にシンボリックリンクする
const assetsDocsPath = resolve(__dirname, "../assets/docs/");
const publicAssetsDocsPath = resolve(__dirname, "./public/assets/");

rmSync(publicAssetsDocsPath, { recursive: true, force: true });
symlinkSync(assetsDocsPath, publicAssetsDocsPath, "dir");

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
				new RegExp(`^${basePath.replace(/\/$/, "")}/@.+`),
				new RegExp(`^${basePath.replace(/\/$/, "")}/node_modules(?:/|$)`),
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
