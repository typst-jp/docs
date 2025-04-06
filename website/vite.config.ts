import { defineConfig } from "vite";
import ssg from "@hono/vite-ssg";
import devServer from "@hono/vite-dev-server";

export default defineConfig({
  plugins: [
    ssg(),
    devServer({
      entry: "src/index.tsx",
    }),
  ],
});
