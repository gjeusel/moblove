import prettierConfig from "eslint-config-prettier"
import eslintPluginSvelte from "eslint-plugin-svelte"

export default [
  ...eslintPluginSvelte.configs["flat/recommended"],
  {
    rules: {
      "svelte/valid-compile": "warn",
    },
  },
  // keep prettier config last
  prettierConfig,
]
