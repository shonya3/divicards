import type { Preview } from "@storybook/web-components-vite";
import  '@shoelace-style/shoelace/dist/themes/dark.css'

const preview: Preview = {
  parameters: {
    actions: { argTypesRegex: "^on[A-Z].*" },
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/,
      },
    },
  },
};

export default preview;
