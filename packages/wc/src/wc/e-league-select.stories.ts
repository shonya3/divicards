import { Meta } from '@storybook/web-components';
import { LeagueSelectElement } from './e-league-select.js';
import './e-league-select';
import { html, TemplateResult } from 'lit';
import { League, tradeLeagues } from '@divicards/shared/types.js';

const meta: Meta<LeagueSelectElement> = {
	title: 'Elements/e-league-select',
	args: {
		trade: false,
		league: 'Standard',
	},
	argTypes: {
		trade: { control: { control: 'boolean' } },
		league: {
			options: tradeLeagues,
			control: { type: 'select' },
		},
	},
};
export default meta;

export const Default = {
	render({ trade, league }: { trade: boolean; league: League }): TemplateResult {
		return html`<e-league-select ?trade=${trade} league=${league}></e-league-select>`;
	},
};
