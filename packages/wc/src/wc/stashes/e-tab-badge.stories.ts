import { Meta } from '@storybook/web-components';
import { TabBadgeElement } from './e-tab-badge.js';
import './e-tab-badge';
import { html, TemplateResult } from 'lit';
import { NoItemsTab } from 'poe-custom-elements/types.js';

const meta: Meta<TabBadgeElement> = {
	title: 'Elements/stashes/e-tab-badge',
};

export default meta;

const noItemsTab = {
	id: '7f967993bb',
	index: 16,
	metadata: {
		colour: 'ff',
	},
	name: 'd (Remove-only)',
	type: 'PremiumStash',
} as const satisfies NoItemsTab;

export const Default = {
	render(): TemplateResult {
		return html`<e-tab-badge .tab=${noItemsTab}></e-tab-badge>`;
	},
};
