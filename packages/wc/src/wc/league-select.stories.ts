import { Meta } from '@storybook/web-components';
import { LeagueSelectElement } from './league-select';
import { html } from 'lit';
import { League, tradeLeagues } from '@divicards/shared/types';

export default {
	title: 'Elements/league-select',
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
		LeagueSelectElement.define();
		return html`<wc-league-select ?trade=${trade} league=${league}></wc-league-select>`;
	},
};
