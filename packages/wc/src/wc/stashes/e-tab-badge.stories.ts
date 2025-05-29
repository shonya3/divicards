import { Meta } from '@storybook/web-components';
import { TabBadgeElement } from './e-tab-badge.js';
import './e-tab-badge';
import { html } from 'lit';
import { NoItemsTab } from 'poe-custom-elements/types.js';

export default {
	title: 'Elements/stashes/e-tab-badge',
} satisfies Meta<TabBadgeElement>;

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
	render() {
		return html`<e-tab-badge .tab=${noItemsTab}></e-tab-badge>`;
	},
};
