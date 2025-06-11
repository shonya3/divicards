declare global {
	interface HTMLElementEventMap {
		'change:page': PageChangeEvent;
	}
}

export class PageChangeEvent extends Event {
	static readonly tag = 'change:page';
	constructor(readonly page: number, options?: EventInit) {
		super(PageChangeEvent.tag, options);
	}
}
