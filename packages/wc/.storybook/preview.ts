import { withThemeToggle } from './withThemeToggle';
import type { Preview } from '@storybook/web-components-vite';
import './styles.css';
import '@shoelace-style/shoelace/dist/themes/dark.css';
import '@shoelace-style/shoelace/dist/themes/light.css';

const preview: Preview = {
	decorators: [withThemeToggle],
	parameters: {
		actions: { argTypesRegex: '^on[A-Z].*' },
		controls: {
			matchers: {
				color: /(background|color)$/i,
				date: /Date$/,
			},
		},
	},
};

export default preview;

export const globalTypes = {
	theme: {
		name: 'Theme',
		description: 'Global theme',
		defaultValue: 'light',
		toolbar: {
			icon: 'contrast',
			items: [
				{ value: 'light', title: 'Light', icon: 'sun' },
				{ value: 'dark', title: 'Dark', icon: 'moon' },
			],
			showName: true,
			dynamicTitle: true,
		},
	},
};
