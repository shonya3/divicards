import type { Meta, StoryObj } from '@storybook/web-components';
import { PoeStashTabElement } from '../elements/poe-stash-tab.js';
import { html } from 'lit';
import '../elements/poe-stash-tab';
import { quadStd } from '../jsons/tabs/quadStd.js';
import { divination } from '../jsons/tabs/divination.js';
import { TabWithItems } from '../poe.types.js';
import premiumJson from '../jsons/PremiumStash.json' with {type: 'json'};
import { essence } from '../jsons/tabs/essence.js';
import { currencyTab } from '../jsons/tabs/currencyTab.js';
import { blight } from '../jsons/tabs/blight.js';
import { fragments } from '../jsons/tabs/myfragments.js';
const premium = premiumJson as TabWithItems;

type Args = { tab: TabWithItems };
type Story = StoryObj<Args>;

const meta: Meta<PoeStashTabElement> = {
	title: 'Components/poe-stash-tab',
	component: 'poe-stash-tab',
	render: (args: { tab: TabWithItems }) => html`<poe-stash-tab .tab=${args.tab}></poe-stash-tab>`,
};
export default meta;

export const Quad: Story = {
	args: { tab: quadStd },
};
export const Premium: Story = {
	args: { tab: premium },
};
export const Essence: Story = {
	args: { tab: essence },
};
export const Currency: Story = {
	args: { tab: currencyTab },
};
export const Blight: Story = {
	args: { tab: blight },
};
export const Fragment: Story = {
	args: { tab: fragments },
};
export const Divination: Story = {
	args: { tab: divination },
};
export const Normal: Story = {
	args: { tab: premium },
};
