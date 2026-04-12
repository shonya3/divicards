import { defineConfig } from "vite-plus";

export default defineConfig({
  staged: {
    "*": "vp check --fix",
  },
  lint: {
    plugins: ["typescript", "unicorn", "oxc"],
    categories: {
      correctness: "error",
    },
    rules: {
      "typescript/unbound-method": "off",
      "typescript/no-floating-promises": "off",
    },
    env: {
      builtin: true,
    },
    options: {
      typeAware: true,
      typeCheck: true,
    },
    ignorePatterns: ["crates/**/*.js", "**/.storybook/**"],
  },
  fmt: {
    ignorePatterns: ["**/dist/**", "**/gen/**"],
    printWidth: 120,
    sortImports: {
      sortSideEffects: true,
      customGroups: [
        {
          groupName: "lit",
          elementNamePattern: ["lit", "@lit-labs/**", "@lit/**", "lit/**", "signal-utils/**"],
        },
        {
          groupName: "shoelace",
          elementNamePattern: ["@shoelace-style/**"],
        },
      ],
      groups: [
        "lit",
        "shoelace",
        ["value-builtin", "value-external"],
        "value-internal",
        ["value-parent", "value-sibling", "value-index"],
        "unknown",
      ],
    },
  },
});
