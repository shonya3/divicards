import { html } from 'lit';
import './e-stashes-view';
import { MockStashLoader } from './data';

export default {
	title: 'Elements/stashes/stashes-view',
};

export const Default = {
	render() {
		return html`<e-stashes-view .stashLoader=${new MockStashLoader()}></e-stashes-view>`;
	},
};
