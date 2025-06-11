import { Meta } from '@storybook/web-components-vite';
import { SampleCardElement } from './e-sample-card.js';
import './e-sample-card';
import { league, filename, selected, uuid, minimumCardPrice, sample, csvDataForDrag } from './data.js';
import { html, TemplateResult } from 'lit';

const meta: Meta<SampleCardElement> = {
	title: 'Elements/e-sample-card/e-sample-card',
};
export default meta;

export const Default = {
	render(): TemplateResult {
		return html`<e-sample-card
			league=${league ?? 'Standard'}
			filename=${filename}
			?selected=${selected}
			uuid=${uuid}
			minimum-card-price=${minimumCardPrice}
			.sample=${sample}
			.csvDataForDrag=${csvDataForDrag}
		></e-sample-card>`;
	},
};
