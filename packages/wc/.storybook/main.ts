import type { StorybookConfig } from '@storybook/web-components-vite';
const config: StorybookConfig = {
    stories: ['../src/**/*.mdx', '../src/**/*.stories.@(js|jsx|ts|tsx)'],
    addons: ['@storybook/addon-links', '@chromatic-com/storybook'],

    framework: {
		name: '@storybook/web-components-vite',
		options: {},
	}
};
export default config;
