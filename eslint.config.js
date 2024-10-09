import prettierConfig from "eslint-config-prettier"
import eslintPluginSvelte from "eslint-plugin-svelte"

export default [
  ...eslintPluginSvelte.configs["flat/recommended"],
  {
    files: [
      "**/*.svelte",
      "*.svelte",
      "**/*.svelte.ts",
      "*.svelte.ts",
      "**/*.svelte.js",
      "*.svelte.js",
    ],
    languageOptions: {
      parserOptions: {
        warnOnUnsupportedTypeScriptVersion: false, // stop warning
        parser: {
          ts: "@typescript-eslint/parser",
          js: "espree",
          typescript: "@typescript-eslint/parser",
        },
      },
    },
  },
  {
    rules: {
      // "svelte/valid-compile": "warn",
    },
  },
  // keep prettier config last
  prettierConfig,
]
