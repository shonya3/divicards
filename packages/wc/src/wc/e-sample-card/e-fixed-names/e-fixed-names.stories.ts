import { Meta } from '@storybook/web-components';
import { FixedNamesElement } from './e-fixed-names.js';
import './e-fixed-names';
import { html, TemplateResult } from 'lit';
import { fixedNames } from './data.js';
import { FixedName } from '@divicards/shared/types.js';

const meta: Meta<FixedNamesElement> = {
	title: 'Elements/e-sample-card/e-fixed-names',
	args: {
		fixedNames,
	},
};
export default meta;

export const Default = {
	render({ fixedNames }: { fixedNames: FixedName[] }): TemplateResult {
		return html`<e-fixed-names .fixedNames=${fixedNames}></e-fixed-names>`;
	},
};
