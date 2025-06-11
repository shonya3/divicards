import { EventMapFrom } from '../../../event-utils.js';
import { PageChangeEvent } from '../../events/change/page.js';
import { PerPageChangeEvent } from '../../events/change/per_page.js';
import { TabClickEvent } from '../e-tab-badge/e-tab-badge.js';
import { SelectedTabsChangeEvent } from '../events.js';

declare global {
	interface HTMLElementEventMap extends EventMapFrom<Events> {}
}

export type Events = [
	typeof MultiselectChangeEvent,

	// pagination
	typeof PageChangeEvent,
	typeof PerPageChangeEvent,
	typeof SelectedTabsChangeEvent,

	// composed from e-tab-badge
	typeof TabClickEvent
];

declare global {
	interface HTMLElementEventMap {
		'change:multiselect': MultiselectChangeEvent;
	}
}
export class MultiselectChangeEvent extends Event {
	static readonly tag = 'change:multiselect';

	constructor(readonly $multiselect: boolean, options?: EventInit) {
		super(MultiselectChangeEvent.tag, options);
	}
}
