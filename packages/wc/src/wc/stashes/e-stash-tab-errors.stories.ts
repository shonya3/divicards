import { Meta } from '@storybook/web-components';
import { html, TemplateResult } from 'lit';
import { StashTabErrorsElement } from './e-stash-tab-errors.js';
import './e-stash-tab-errors';

const meta: Meta<StashTabErrorsElement> = {
	title: 'Elements/stashes/e-stash-tab-errors',
};
export default meta;

const errors = [
	{
		noItemsTab: {
			id: '7f967993bb',
			index: 16,
			metadata: {
				colour: 'ff',
			},
			name: 'd (Remove-only)',
			type: 'PremiumStash',
		},
		message: 'Sample must have rain of chaos cards.',
	},
	{
		noItemsTab: {
			id: '12321321',
			index: 25,
			metadata: {
				colour: '7c5436',
			},
			name: 'Heist',
			type: 'PremiumStash',
		},
		message: 'Sample must have rain of chaos cards.',
	},
] as const;

export const Default = {
	render(): TemplateResult {
		return html`<e-stash-tab-errors .errors=${errors}></e-stash-tab-errors>`;
	},
};
