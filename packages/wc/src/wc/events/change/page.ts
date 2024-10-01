declare global {
	interface HTMLElementEventMap {
		'change:page': PageChangeEvent;
	}
}

export class PageChangeEvent extends Event {
	static readonly tag = 'change:page';
	readonly page: number;
	constructor(page: number, options?: EventInit) {
		super(PageChangeEvent.tag, options);
		this.page = page;
	}
}
