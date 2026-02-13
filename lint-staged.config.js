/**
 * @filename: lint-staged.config.js
 * @type {import('lint-staged').Configuration}
 */
export default {
  '*.rs': (_files) => [
    'pnpm build',
    'turbo run format:rs format:fix',
    'cargo clippy --workspace --fix --allow-dirty',
  ],
  '*.{js,ts,cjs,mjs,d.cts,d.mts,jsx,tsx,json,jsonc}': [
    'biome check --write --no-errors-on-unmatched', // Format, sort imports, lint, and apply safe fixes
  ],
  '*.toml': ['taplo format'],
};
