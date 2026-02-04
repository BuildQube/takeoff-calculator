/**
 * @filename: lint-staged.config.js
 * @type {import('lint-staged').Configuration}
 */
export default {
    "*.@(js|ts|tsx)": [
        "oxlint --fix"
    ],
    "*.@(js|ts|tsx|yml|yaml|md|json)": [
        "prettier --write"
    ],
    "*.toml": [
        "taplo format"
    ],
    "*.rs": (_files) => "turbo run format:rs format:fix",
}