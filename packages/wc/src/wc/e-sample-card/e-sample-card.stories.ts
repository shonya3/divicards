import { Meta } from '@storybook/web-components';
import { SampleCardElement } from './e-sample-card.js';
import './e-sample-card';
import { league, filename, selected, uuid, minimumCardPrice, sample } from './data.js';
import { html } from 'lit';

export default {
	title: 'Elements/e-sample-card/e-sample-card',
} satisfies Meta<SampleCardElement>;

export const Default = {
	render() {
		return html`<e-sample-card
			league=${league ?? 'Standard'}
			filename=${filename}
			?selected=${selected}
			uuid=${uuid}
			minimum-card-price=${minimumCardPrice}
			.sample=${sample}
		></e-sample-card>`;
	},
};
