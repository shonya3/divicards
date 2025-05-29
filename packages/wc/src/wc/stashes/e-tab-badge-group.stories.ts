import { html } from 'lit';
import './e-tab-badge-group';
import { league, stashes } from './data.js';

export default {
	title: 'Elements/stashes/e-tab-badge-group',
};

export const Default = {
	render() {
		return html`<e-tab-badge-group .stashes=${stashes} .league=${league}></e-tab-badge-group>`;
	},
};
