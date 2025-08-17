import { rmSync, symlinkSync } from "node:fs";
import { resolve } from "node:path";
import { robotsTxtPlugin } from "@hono/ssg-plugins-essential/robots-txt";
import { sitemapPlugin } from "@hono/ssg-plugins-essential/sitemap";
import { defaultOptions } from "@hono/vite-dev-server";
import devServer from "@hono/vite-dev-server";
import ssg from "@hono/vite-ssg";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";

// typst-docsが生成したドキュメントのアセットをassets/docsにシンボリックリンクする
const assetsDocsPath = resolve(__dirname, "../assets/docs/");
const publicAssetsDocsPath = resolve(__dirname, "./public/assets/docs/");

rmSync(publicAssetsDocsPath, { recursive: true, force: true });
symlinkSync(assetsDocsPath, publicAssetsDocsPath, "dir");

export default defineConfig({
	plugins: [
		tailwindcss(),
		ssg({
			plugins: [
				sitemapPlugin({
					baseUrl: "https://typst-jp.github.io/",
				}),
				robotsTxtPlugin({
					rules: [{ userAgent: "*", allow: ["/"] }],
					sitemapUrl: "https://typst-jp.github.io/sitemap.xml",
				}),
			],
		}),
		devServer({
			entry: "src/index.tsx",
			exclude: [...defaultOptions.exclude, /^\/assets\/.+/, /^\/index\.html$/],
		}),
	],
	build: {
		rollupOptions: {
			input: ["src/globals.css"],
			output: {
				assetFileNames: "assets/[name].[ext]",
			},
		},
	},
	server: {
		host: process.env.VITE_LISTEN_ALL_ADDRESSES === "true",
	},
});
