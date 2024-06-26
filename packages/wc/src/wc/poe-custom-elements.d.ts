declare global {
	interface HTMLElementTagNameMap {
		'poe-stash-tab': PoeStashTabElement;
	}
}
export declare class PoeStashTabElement extends LitElement {
	/** PoE API tab data https://www.pathofexile.com/developer/docs/reference#stashes-get */
	tab: TabWithItems;
	/** Mutable clone of tab */
	private tabState;
	get focusWithin(): boolean;
	get activeItemElement(): PoeItemElement | null;
	protected willUpdate(map: PropertyValueMap<this>): void;
	protected render(): TemplateResult;
	connectedCallback(): void;
	disconnectedCallback(): void;
	onKeyDown: (e: KeyboardEvent) => Promise<void>;
	static styles: import('lit').CSSResult;
}
