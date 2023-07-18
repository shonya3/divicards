import { Meta } from '@storybook/web-components';
import { TabBadgeElement } from './tab-badge';
import { html } from 'lit';

export default {
	title: 'Elements/stashes/tab-badge',
} satisfies Meta<TabBadgeElement>;

export const Default = {
	render(d: { color: string; name: string; tabId: string; selected: boolean; index: number }) {
		TabBadgeElement.define();
		return html`<wc-tab-badge tabId="e07f5f2946" colour="7c5436" index="5"></wc-tab-badge>`;
	},
};
