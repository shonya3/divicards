import { NoItemsTab } from 'poe-custom-elements/types.js';

declare global {
	interface HTMLElementEventMap {
		'stashes__tab-select': TabSelectEvent;
		'stashes__tab-click': TabClickEvent;
	}
}

export class TabSelectEvent extends Event {
	static readonly tag = 'stashes__tab-select';
	tab: NoItemsTab;
	selected: boolean;
	constructor(tab: NoItemsTab, selected: boolean, options?: EventInit) {
		super(TabSelectEvent.tag, options);
		this.tab = tab;
		this.selected = selected;
	}
}

export class TabClickEvent extends Event {
	static readonly tag = 'stashes__tab-click';
	tab: NoItemsTab;
	constructor(tab: NoItemsTab, options?: EventInit) {
		super(TabClickEvent.tag, options);
		this.tab = tab;
	}
}

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
