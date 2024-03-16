import type { Preview } from "@storybook/web-components";


import * as styles from './dark.css'

console.log(styles)

const preview: Preview = {
    h: styles,
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
