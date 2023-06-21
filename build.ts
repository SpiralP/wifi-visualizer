import { build } from "esbuild";
import path from "path";

const NODE_ENV = process.env.NODE_ENV || "development";

(async () => {
  await build({
    entryPoints: [path.join(__dirname, "web/index.tsx")],
    bundle: true,
    minify: NODE_ENV === "production",
    sourcemap: true,
    target: ["chrome102", "firefox102"],
    outfile: path.join(__dirname, "dist/index.js"),
    define: {
      "process.env.NODE_ENV": JSON.stringify(NODE_ENV),
      "process.env": JSON.stringify({ NODE_ENV }),
    },
    platform: "browser",
    loader: {
      ".ttf": "file",
      ".eot": "file",
      ".woff2": "file",
      ".woff": "file",
      ".svg": "file",
    },
  });
})();
