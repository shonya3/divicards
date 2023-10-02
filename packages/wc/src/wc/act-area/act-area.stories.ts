import { html } from 'lit';

import { Meta } from '@storybook/web-components';
import { ActAreaElement } from './wc-act-area';

export default {
	title: 'Elements/act-area',
} satisfies Meta<ActAreaElement>;

export const Default = {
	render() {
		ActAreaElement.define();
		return html`<wc-act-area></wc-act-area>`;
	},
};
