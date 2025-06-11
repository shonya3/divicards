import { League, DivinationCardsSample } from '@divicards/shared/types.js';
import { NoItemsTab, TabWithItems } from 'poe-custom-elements/types.js';
import { EventMapFrom } from '../../event-utils.js';
import { TabClickEvent } from './e-tab-badge/e-tab-badge.js';

declare global {
	interface HTMLElementEventMap extends EventMapFrom<Events> {}
}

export type Events = [
	typeof CloseEvent,
	typeof SampleFromStashtabEvent,
	typeof StashtabFetchedEvent,
	typeof ExtractCardsEvent,
	typeof SelectedTabsChangeEvent,
	typeof StashtabsBadgesFetchedEvent,
	typeof TabClickEvent
];

export class CloseEvent extends Event {
	static readonly tag = 'stashes__close';
	constructor(options?: EventInit) {
		super(CloseEvent.tag, options);
	}
}

export class StashtabsBadgesFetchedEvent extends Event {
	static readonly tag = 'stashes__stashtabs-badges-fetched';
	readonly stashtabs_badges: Array<NoItemsTab>;
	constructor(stashtabs_badges: Array<NoItemsTab>, options?: EventInit) {
		super(StashtabsBadgesFetchedEvent.tag, options);
		this.stashtabs_badges = stashtabs_badges;
	}
}

export class ExtractCardsEvent extends Event {
	static readonly tag = 'stashes__extract-cards';
	readonly $tab: TabWithItems;
	readonly $league: League;
	constructor(tab: TabWithItems, league: League, options?: EventInit) {
		super(ExtractCardsEvent.tag, options);
		this.$tab = tab;
		this.$league = league;
	}
}

export class SampleFromStashtabEvent extends Event {
	static readonly tag = 'stashes__sample-from-stashtab';
	readonly $stashtab_name: string;
	readonly $sample: DivinationCardsSample;
	readonly $league: League;
	constructor(
		{
			stashtab_name,
			sample,
			league,
		}: {
			stashtab_name: string;
			sample: DivinationCardsSample;
			league: League;
		},
		options?: EventInit
	) {
		super(SampleFromStashtabEvent.tag, options);
		this.$stashtab_name = stashtab_name;
		this.$sample = sample;
		this.$league = league;
	}
}

export class StashtabFetchedEvent extends Event {
	static readonly tag = 'stashes__stashtab-fetched';
	readonly $stashtab: TabWithItems;
	readonly $league: League;

	constructor(stashtab: TabWithItems, league: League, options?: EventInit) {
		super(StashtabFetchedEvent.tag, options);
		this.$stashtab = stashtab;
		this.$league = league;
	}
}

export class SelectedTabsChangeEvent extends Event {
	static readonly tag = 'change:selected_tabs';
	readonly $selected_tabs: Map<NoItemsTab['id'], { id: NoItemsTab['id']; name: NoItemsTab['name'] }>;
	constructor(
		selected_tabs: Map<NoItemsTab['id'], { id: NoItemsTab['id']; name: NoItemsTab['name'] }>,
		options?: EventInit
	) {
		super(SelectedTabsChangeEvent.tag, options);
		this.$selected_tabs = selected_tabs;
	}
}
