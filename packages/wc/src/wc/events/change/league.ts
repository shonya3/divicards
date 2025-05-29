import { League } from '@divicards/shared/types.js';

declare global {
	interface HTMLElementEventMap {
		'change:league': LeagueChangeEvent;
	}
}

export class LeagueChangeEvent extends Event {
	static readonly tag = 'change:league';
	readonly league: League;
	constructor(league: League, options?: EventInit) {
		super(LeagueChangeEvent.tag, options);
		this.league = league;
	}
}
