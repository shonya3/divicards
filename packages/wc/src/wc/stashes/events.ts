import { League, DivinationCardsSample } from '@divicards/shared/types.js';
import { NoItemsTab, TabWithItems } from 'poe-custom-elements/types.js';
import { EventMapFrom } from '../../event-utils.js';
import { SelectedStashtabs } from './types.js';
import { TabClickEvent } from './e-tab-badge/events.js';

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
	typeof TabClickEvent,
	typeof BulkLoadAllTabsEvent
];

export class CloseEvent extends Event {
	static readonly tag = 'stashes__close';

	constructor(options?: EventInit) {
		super(CloseEvent.tag, options);
	}
}

export class StashtabsBadgesFetchedEvent extends Event {
	static readonly tag = 'stashes__stashtabs-badges-fetched';

	constructor(readonly $stashtabs_badges: Array<NoItemsTab>, options?: EventInit) {
		super(StashtabsBadgesFetchedEvent.tag, options);
	}
}

export class ExtractCardsEvent extends Event {
	static readonly tag = 'stashes__extract-cards';

	constructor(readonly $tab: TabWithItems, readonly $league: League, options?: EventInit) {
		super(ExtractCardsEvent.tag, options);
	}
}

export class SampleFromStashtabEvent extends Event {
	static readonly tag = 'stashes__sample-from-stashtab';

	constructor(
		readonly $stashtab_name: string,
		readonly $sample: DivinationCardsSample,
		readonly $league: League,
		options?: EventInit
	) {
		super(SampleFromStashtabEvent.tag, options);
	}
}

export class StashtabFetchedEvent extends Event {
	static readonly tag = 'stashes__stashtab-fetched';

	constructor(readonly $stashtab: TabWithItems, readonly $league: League, options?: EventInit) {
		super(StashtabFetchedEvent.tag, options);
	}
}

export class SelectedTabsChangeEvent extends Event {
	static readonly tag = 'change:selected_tabs';

	constructor(readonly $selected_tabs: SelectedStashtabs, options?: EventInit) {
		super(SelectedTabsChangeEvent.tag, options);
	}
}

export class BulkLoadAllTabsEvent extends Event {
    static readonly tag = 'stashes__bulk-load-all';

    constructor(options?: EventInit) {
        super(BulkLoadAllTabsEvent.tag, options);
    }
}
