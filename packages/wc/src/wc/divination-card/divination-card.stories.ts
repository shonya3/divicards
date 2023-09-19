import { html } from 'lit';

import { Meta } from '@storybook/web-components';
import { DivinationCardElement } from './wc-divination-card';

export default {
	title: 'Elements/divination-card',
} satisfies Meta<DivinationCardElement>;

export const Default = {
	render() {
		DivinationCardElement.define();
		return html`<wc-divination-card name="The Doctor"></wc-divination-card>`;
	},
};
