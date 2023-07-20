import { Meta } from '@storybook/web-components';
import { FileCardElement } from './file-card';
import {
	league,
	filename,
	href,
	selected,
	id,
	valid,
	error,
	minimumCardPrice,
	sample,
	isReady,
	fileCardProps,
} from './data';
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
			href=${href}
			?selected=${selected}
			uuid=${id}
			?valid=${valid}
			.error=${error}
			minimum-card-price=${minimumCardPrice}
			.sample=${sample}
			?is-ready=${isReady}
		></wc-file-card>`;
	},
};
