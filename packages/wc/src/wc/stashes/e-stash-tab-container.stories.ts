import { Meta } from '@storybook/web-components';
import './e-stash-tab-container';
import { html } from 'lit';
import { StashTabContainerElement } from './e-stash-tab-container';
import quadStash from './json/QuadStashStd.json';
import fragmentsTab from './json/fragmentsTab.json';
import { TabWithItems } from 'poe-custom-elements/types.js';

export default {
	title: 'Elements/stashes/e-stash-tab-container',
} satisfies Meta<StashTabContainerElement>;

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
		return html`<e-stash-tab-container
			status="complete"
			.tab=${quadStash as TabWithItems}
		></e-stash-tab-container>`;
	},
};

export const Fragments = {
	render() {
		return html`<e-stash-tab-container
			status="complete"
			.tab=${fragmentsTab as TabWithItems}
		></e-stash-tab-container>`;
	},
};
