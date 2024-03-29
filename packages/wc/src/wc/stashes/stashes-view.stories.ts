import { html } from 'lit';
import { StashesViewElement } from './stashes-view';
import { MockStashLoader } from './data';

export default {
	title: 'Elements/stashes/stashes-view',
};

export const Default = {
	render() {
		StashesViewElement.define();

		return html`<wc-stashes-view .stashLoader=${new MockStashLoader()}></wc-stashes-view>`;
	},
};
