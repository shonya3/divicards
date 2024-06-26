import { Meta } from '@storybook/web-components';
import './e-stash-tab-container';
import { html } from 'lit';
import { StashTabContainerElement } from './e-stash-tab-container';
import stash from 'poe-custom-elements/jsons/QuadStashStd.json';

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
			.badge=${noItemsTab}
			status="complete"
			.tab=${stash}
		></e-stash-tab-container>`;
	},
};
