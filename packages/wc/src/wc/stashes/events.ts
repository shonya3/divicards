import { NoItemsTab } from 'poe-custom-elements/types.js';

declare global {
	interface HTMLElementEventMap {
		'change:selected_tabs': SelectedTabsChangeEvent;
	}
}

export class SelectedTabsChangeEvent extends Event {
	static readonly tag = 'change:selected_tabs';
	readonly selected_tabs: Map<NoItemsTab['id'], { id: NoItemsTab['id']; name: NoItemsTab['name'] }>;
	constructor(
		selected_tabs: Map<NoItemsTab['id'], { id: NoItemsTab['id']; name: NoItemsTab['name'] }>,
		options?: EventInit
	) {
		super(SelectedTabsChangeEvent.tag, options);
		this.selected_tabs = selected_tabs;
	}
}
