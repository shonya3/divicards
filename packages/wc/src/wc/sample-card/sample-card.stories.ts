import { Meta } from '@storybook/web-components';
import { SampleCardElement } from './sample-card';
import { league, filename, selected, uuid, minimumCardPrice, sample } from './data';
import { html } from 'lit';

export default {
	title: 'Elements/sample-card/sample-card',
} satisfies Meta<SampleCardElement>;

export const Default = {
	render() {
		SampleCardElement.define();
		return html`<wc-sample-card
			league=${league}
			filename=${filename}
			?selected=${selected}
			uuid=${uuid}
			minimum-card-price=${minimumCardPrice}
			.sample=${sample}
		></wc-sample-card>`;
	},
};
