import { Meta } from '@storybook/web-components';
import { FixedNamesElement } from './fixed-names';
import { html } from 'lit';
import { fixedNames } from './data';
import { FixedName } from '@divicards/shared/types';

export default {
	title: 'Elements/file-card/fixed-names-list',
	args: {
		fixedNames,
	},
} satisfies Meta<FixedNamesElement>;

export const Default = {
	render({ fixedNames }: { fixedNames: FixedName[] }) {
		FixedNamesElement.define();
		return html`<wc-fixed-names .fixedNames=${fixedNames}></wc-fixed-names>`;
	},
};
