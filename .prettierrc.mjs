export default {
  printWidth: 100,
  semi: false,
  useTabs: false,
  tabWidth: 2,
  singleQuote: false,
  trailingComma: "all",
  arrowParens: "always",
  plugins: ["prettier-plugin-svelte", "prettier-plugin-tailwindcss"],
  overrides: [
    {
      files: ["*.svelte"],
      options: {
        parser: "svelte",
        svelteStrictMode: true,
        svelteAllowShorthand: true,
        svelteIndentScriptAndStyle: true,
      },
    },
    {
      files: ["*.ts", "*.tsx"],
      options: { parser: "typescript" },
    },
  ],
}
