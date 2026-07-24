import { Meta, StoryObj } from '@storybook/web-components';
import { PoeItemInfoRequirementsElement } from '../../elements/item-info/poe-item-info-requirements.js';
import { Requirement } from '../../poe.types.js';
import { html } from 'lit';
import '../../elements/item-info/poe-item-info-requirements';

type Args = { requirements: Array<Requirement> };

const meta: Meta<PoeItemInfoRequirementsElement> = {
	title: 'Components/item-info/poe-item-info-requirements',
	component: 'poe-item-info-requirements',
	decorators: [story => html`<div style="padding:2rem;background-color: black">${story()}</div>`],
	render: (args: Args) =>
		html`<poe-item-info-requirements .requirements=${args.requirements}></poe-item-info-requirements>`,
};
export default meta;

export const Default: StoryObj<Args> = {
	args: {
		requirements: [
			{
				displayMode: 0,
				name: 'Level',
				suffix: '(gem)',
				type: 62,
				values: [['68', 0]],
			},
			{
				displayMode: 1,
				name: 'Str',
				suffix: '(gem)',
				type: 63,
				values: [['151', 0]],
			},
			{
				displayMode: 1,
				name: 'Dex',
				type: 64,
				values: [['68', 0]],
			},
			{
				displayMode: 1,
				name: 'Int',
				suffix: '(gem)',
				type: 65,
				values: [['39', 0]],
			},
		],
	},
};
