import type { Preview } from '@storybook/web-components';
import { setCustomElementsManifest } from '@storybook/web-components';
import customElementsManifest from '../custom-elements.json';
import { appendFontinStyle } from '../src/lib/internal';

/**
 * https://github.com/storybookjs/storybook/issues/15436
 * @param customElements
 * @param options
 * @returns
 */
export const setCustomElementsManifestWithOptions = (
	customElements: any,
	options: { privateFields?: boolean }
): void => {
	let { privateFields = true } = options;
	if (!privateFields) {
		customElements?.modules?.forEach((module: { declarations: any[] }) => {
			module?.declarations?.forEach(declaration => {
				Object.keys(declaration).forEach(key => {
					if (Array.isArray(declaration[key])) {
						declaration[key] = declaration[key].filter(
							(member: { privacy: string | string[] }) => !member.privacy?.includes('private')
						);
					}
				});
			});
		});
	}
	return setCustomElementsManifest(customElements);
};

setCustomElementsManifestWithOptions(customElementsManifest, {
	privateFields: true,
});
appendFontinStyle();

const preview: Preview = {
	tags: ['autodocs'],
	parameters: {
		controls: {
			matchers: {
				color: /(background|color)$/i,
				date: /Date$/i,
			},
		},
	},
};

export default preview;
