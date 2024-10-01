declare global {
	interface HTMLElementEventMap {
		'change:per_page': PerPageChangeEvent;
	}
}

export class PerPageChangeEvent extends Event {
	static readonly tag = 'change:per_page';
	readonly per_page: number;
	constructor(per_page: number, options?: EventInit) {
		super(PerPageChangeEvent.tag, options);
		this.per_page = per_page;
	}
}
