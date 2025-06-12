import { NoItemsTab } from 'poe-custom-elements/types.js';
import { EventMapFrom } from '../../../event-utils.js';

export type Events = [typeof TabSelectEvent, typeof TabClickEvent];

declare global {
	interface HTMLElementEventMap extends EventMapFrom<Events> {}
}

export class TabSelectEvent extends Event {
	static readonly tag = 'stashes__tab-select';

	constructor(readonly tab: NoItemsTab, readonly selected: boolean, options?: EventInit) {
		super(TabSelectEvent.tag, options);
	}
}

export class TabClickEvent extends Event {
	static readonly tag = 'stashes__tab-click';

	constructor(public readonly $tab: NoItemsTab, options?: EventInit) {
		super(TabClickEvent.tag, options);
	}
}
