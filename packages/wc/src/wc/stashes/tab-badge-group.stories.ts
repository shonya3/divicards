import { html } from 'lit';
import { TabBadgeGroupElement } from './tab-badge-group';
import { league, stashes } from './dummy';

export default {
	title: 'Elements/stashes/tab-badge-group',
};

export const Default = {
	render() {
		TabBadgeGroupElement.define();
		return html`<wc-tab-badge-group .stashes=${stashes} .league=${league}></wc-tab-badge-group>`;
	},
};
