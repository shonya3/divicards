import { html } from 'lit';
import { PoeItem } from '../../poe.types.js';
import { Meta, StoryObj } from '@storybook/web-components';
import '../../elements/item-info/poe-item-info';
import { influence } from '../../jsons/influence.js';
import { PoeItemElement } from '../../elements/poe-item.js';

const meta: Meta<PoeItemElement> = {
	title: 'Components/item-info/poe-item-info',
	component: 'poe-item-info',
	render: (args: { item: PoeItem }) => html`<poe-item-info .item=${args.item}></poe-item-info>`,
};

export const Default: StoryObj = {
	args: { item: influence },
};

export default meta;
