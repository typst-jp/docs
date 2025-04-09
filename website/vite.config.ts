import { defineConfig } from "vite";
import ssg from "@hono/vite-ssg";
import { defaultOptions } from "@hono/vite-dev-server";
import devServer from "@hono/vite-dev-server";
import { rmSync, symlinkSync } from "node:fs";
import { resolve } from "node:path";

// typst-docsが生成したドキュメントのアセットをassets/docsにシンボリックリンクする
const assetsDocsPath = resolve(__dirname, "../assets/docs/");
const publicAssetsDocsPath = resolve(__dirname, "./public/assets/docs/");

rmSync(publicAssetsDocsPath, { recursive: true, force: true });
symlinkSync(assetsDocsPath, publicAssetsDocsPath, "dir");

export default defineConfig({
  plugins: [
    ssg(),
    devServer({
      entry: "src/index.tsx",
      exclude: [
        ...defaultOptions.exclude,
        /^\/assets\/.+/,
        /^\/scripts\/.+/,
        /^\/styles\/.+/,
        /^\/index\.html$/,
      ],
    }),
  ],
});
