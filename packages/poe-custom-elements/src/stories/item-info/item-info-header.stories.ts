import { html } from 'lit';
import { PoeItem } from '../../poe.types.js';
import { Meta, StoryObj } from '@storybook/web-components';
import { PoeItemInfoHeaderElement } from '../../elements/item-info/poe-item-info-header.js';
import '../../elements/item-info/poe-item-info-header.js';
import { influence } from '../../jsons/influence.js';

type Args = { item: PoeItem };

const meta: Meta<PoeItemInfoHeaderElement> = {
	title: 'Components/item-info/poe-item-info-header',
	component: 'poe-item-info-header',
	render: (args: Args) => html`<poe-item-info-header .item=${args.item}></poe-item-info-header>`,
};
export default meta;

export const Influence: StoryObj<Args> = {
	args: { item: influence },
};
