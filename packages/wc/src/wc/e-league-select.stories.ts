import { Meta } from '@storybook/web-components';
import { LeagueSelectElement } from './e-league-select';
import './e-league-select';
import { html } from 'lit';
import { League, tradeLeagues } from '@divicards/shared/types';

export default {
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
} satisfies Meta<LeagueSelectElement>;

export const Default = {
	render({ trade, league }: { trade: boolean; league: League }) {
		return html`<e-league-select ?trade=${trade} league=${league}></e-league-select>`;
	},
};
