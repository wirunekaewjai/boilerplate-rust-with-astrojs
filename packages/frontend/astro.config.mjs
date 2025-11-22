// @ts-check
import { defineConfig } from "astro/config";

// https://astro.build/config
export default defineConfig({
  build: {
    assets: "assets",
    format: "file",
  },

  devToolbar: {
    enabled: false,
  },

  server: {
    port: 8081,
  },

  output: "static",

  vite: {
    build: {
      rollupOptions: {
        output: {
          hashCharacters: "base36",
        },
      },
    },

    server: {
      hmr: {
        port: 8082,
      },
    },
  },
});
