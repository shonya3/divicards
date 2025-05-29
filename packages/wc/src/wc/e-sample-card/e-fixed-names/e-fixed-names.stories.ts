import { Meta } from '@storybook/web-components';
import { FixedNamesElement } from './e-fixed-names.js';
import './e-fixed-names';
import { html } from 'lit';
import { fixedNames } from './data.js';
import { FixedName } from '@divicards/shared/types.js';

export default {
	title: 'Elements/e-sample-card/e-fixed-names',
	args: {
		fixedNames,
	},
} satisfies Meta<FixedNamesElement>;

export const Default = {
	render({ fixedNames }: { fixedNames: FixedName[] }) {
		return html`<e-fixed-names .fixedNames=${fixedNames}></e-fixed-names>`;
	},
};
