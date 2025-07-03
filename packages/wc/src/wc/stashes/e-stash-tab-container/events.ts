import { TabWithItems } from 'poe-custom-elements/types.js';
import { EventMapFrom } from '../../../event-utils.js';

declare global {
	interface HTMLElementEventMap extends EventMapFrom<Events> {}
}

export type Events = [typeof CloseEvent, typeof ExtractCardsEvent];

export class ExtractCardsEvent extends Event {
	static readonly tag = 'e-stash-tab-container__extract-cards';

	constructor(readonly $tab: TabWithItems) {
		super(ExtractCardsEvent.tag);
	}
}

export class CloseEvent extends Event {
	static readonly tag = 'e-stash-tab-container__close';

	constructor() {
		super(CloseEvent.tag);
	}
}
