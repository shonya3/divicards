import { Meta, StoryObj } from '@storybook/web-components';
import { PoeItemInfoContentElement } from '../../elements/item-info/poe-item-info-content.js';
import { influence } from '../../jsons/influence.js';
import { PoeItem } from '../../poe.types.js';
import { html } from 'lit';
import '../../elements/item-info/poe-item-info-content.js';

export type Args = { item: PoeItem };

const meta: Meta<PoeItemInfoContentElement> = {
	title: 'Components/item-info/poe-item-info-content',
	component: 'poe-item-info-content',
	render: (args: Args) => html`<poe-item-info-content .item=${args.item}></poe-item-info-content>`,
};
export default meta;

export const Default: StoryObj<Args> = {
	args: { item: influence },
};
