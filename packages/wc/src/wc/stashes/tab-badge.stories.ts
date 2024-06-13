import { Meta } from '@storybook/web-components';
import { TabBadgeElement } from './tab-badge';
import { html } from 'lit';

export default {
	title: 'Elements/stashes/tab-badge',
} satisfies Meta<TabBadgeElement>;

const noItemsTab = {
	id: '7f967993bb',
	index: 16,
	metadata: {
		colour: 'ff',
	},
	name: 'd (Remove-only)',
	type: 'PremiumStash',
};

export const Default = {
	render() {
		TabBadgeElement.define();
		return html`<wc-tab-badge .tab=${noItemsTab}></wc-tab-badge>`;
	},
};
