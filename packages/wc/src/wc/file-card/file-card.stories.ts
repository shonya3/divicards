import { Meta } from '@storybook/web-components';
import { FileCardElement } from './file-card';
import { league, filename, selected, uuid, minimumCardPrice, sample, fileCardProps } from './data';
import { html } from 'lit';

export default {
	title: 'Elements/file-card/file-card',
} satisfies Meta<FileCardElement>;

export const Default = {
	render() {
		FileCardElement.define();
		console.log({
			fileCardProps,
		});
		return html`<wc-file-card
			league=${league}
			filename=${filename}
			?selected=${selected}
			uuid=${uuid}
			minimum-card-price=${minimumCardPrice}
			.sample=${sample}
		></wc-file-card>`;
	},
};
