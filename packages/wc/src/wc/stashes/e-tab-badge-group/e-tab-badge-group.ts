import { html, nothing, LitElement, TemplateResult, CSSResult } from 'lit';
import { property, state, query, customElement } from 'lit/decorators.js';
import '../e-tab-badge/e-tab-badge.js';
import { type League, isPermanentLeague } from '@divicards/shared/types.js';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib.js';
import '@shoelace-style/shoelace/dist/components/input/input.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';
import '../../e-pagination/e-pagination.js';
import '../../e-help-tip.js';
import type { ErrorLabel, SelectedStashtabs } from '../types.js';
import { classMap } from 'lit/directives/class-map.js';
import { NoItemsTab } from 'poe-custom-elements/types.js';
import { PageChangeEvent } from '../../events/change/page.js';
import { PerPageChangeEvent } from '../../events/change/per_page.js';
import { SelectedTabsChangeEvent } from '../events.js';
import { MultiselectChangeEvent } from './events.js';
import { styles } from './e-tab-badge-group.styles.js';
import { TabSelectEvent } from '../e-tab-badge/events.js';
import '@shoelace-style/shoelace/dist/components/dropdown/dropdown.js';
import '@shoelace-style/shoelace/dist/components/menu/menu.js';
import '@shoelace-style/shoelace/dist/components/menu-item/menu-item.js';
import type SlMenuItem from '@shoelace-style/shoelace/dist/components/menu-item/menu-item.js';

declare global {
	interface HTMLElementTagNameMap {
		'e-tab-badge-group': TabBadgeGroupElement;
	}
}

export const REMOVE_ONLY = '(Remove-only)';

@customElement('e-tab-badge-group')
export class TabBadgeGroupElement extends LitElement {
	static override styles: Array<CSSResult> = [styles];

	@property({ type: Boolean, attribute: 'badges-disabled' }) badgesDisabled = false;
	@property({ type: Boolean }) multiselect = false;
	@property({ type: Array }) stashes: NoItemsTab[] = [];
	@property({ reflect: true }) league: League = ACTIVE_LEAGUE;
	@property({ type: Array }) errors: Array<ErrorLabel> = [];
	@property() hoveredErrorTabId: string | null = null;
	@property({ type: Number, reflect: true }) perPage = 20;
	@property({ type: Number, reflect: true }) page = 1;
	/** Query for searching stashtab by name */
	@property() stashtab_name_query = '';
	@property({ type: Object }) selected_tabs: SelectedStashtabs = new Map();

	@state() hideRemoveOnly = false;

	@query('sl-input#per-page') perPageInput!: HTMLInputElement;
	@query('sl-input#page') pageInput!: HTMLInputElement;
	@query('sl-input#filter-stashes-by-name') nameQueryInput!: HTMLInputElement;

	constructor() {
		super();
		this.addEventListener('stashes__tab-select', e => {
			this.#handle_tab_select(e);
			e.stopPropagation();
		});
	}
	get shouldFilter(): boolean {
		return this.stashes.length > 50;
	}
	get withHideRemoveOnly(): boolean {
		return shouldUnlockHideRemoveOnly(this.league, this.stashes);
	}
	get filtered(): Array<NoItemsTab> {
		return filter(this.stashes, this.stashtab_name_query, this.shouldFilter, this.hideRemoveOnly);
	}
	get paginated(): Array<NoItemsTab> {
		return paginate(this.filtered, this.page, this.perPage);
	}
	get tabsTotal(): number {
		return this.filtered.length;
	}

	willUpdate(changed: Map<string, unknown>): void {
		if (changed.has('nameQuery') || changed.has('hideRemoveOnly') || changed.has('perPage')) {
			this.page = 1;
		}
	}

	protected override render(): TemplateResult {
		return html`</div>
			<div class="tab-badge-group">
				${
					this.shouldFilter
						? html`<header class="header">
								<div class="header__left">
									<sl-input
										size="small"
										type="text"
										id="filter-stashes-by-name"
										.value=${this.stashtab_name_query}
										@input=${this.#change_query}
										.helpText=${`Search tab by name`}
									></sl-input>
									<e-pagination
										.n=${this.tabsTotal}
										.page=${this.page}
										.per_page=${this.perPage}
										@change:page=${this.#handle_page_change}
										@change:per_page=${this.#handle_per_page_change}
									></e-pagination>
								</div>
								<div class="header__right">
									<sl-dropdown>
										<sl-button slot="trigger" caret size="small">Options</sl-button>
										<sl-menu @sl-select=${this.#handle_menu_item_select}>
											<sl-menu-item
												value="multiselect"
												type="checkbox"
												?checked=${this.multiselect}
											>
												Multiselect
												<e-help-tip slot="suffix"
													>Select multiple tabs at once to download it in one go.</e-help-tip
												>
											</sl-menu-item>
											${this.withHideRemoveOnly
												? html`
														<sl-menu-item
															value="hideRemoveOnly"
															type="checkbox"
															?checked=${this.hideRemoveOnly}
															>Hide remove-only</sl-menu-item
														>
												  `
												: nothing}
										</sl-menu>
									</sl-dropdown>
								</div>
						  </header>`
						: nothing
				}
				<ul class="list">
					${this.paginated.map(tab => {
						return html`<li
							class=${classMap({
								error: this.errors.some(({ noItemsTab }) => noItemsTab.id === tab.id),
								'hovered-error': this.hoveredErrorTabId === tab.id,
							})}
						>
							<e-tab-badge
								.as=${this.multiselect ? 'checkbox' : 'button'}
								.tab=${tab}
								.selected=${this.selected_tabs.has(tab.id)}
								.disabled=${this.badgesDisabled}
							></e-tab-badge>
						</li>`;
					})}
				</ul>
			</div>`;
	}

	#handle_page_change({ page }: PageChangeEvent): void {
		this.page = page;
		this.dispatchEvent(new PageChangeEvent(page));
	}
	#handle_per_page_change({ per_page }: PerPageChangeEvent): void {
		this.perPage = per_page;
		this.dispatchEvent(new PerPageChangeEvent(per_page));
	}
	#change_query(e: InputEvent): void {
		this.stashtab_name_query = (e.target as HTMLInputElement).value;
	}
	#handle_tab_select({ tab, selected }: TabSelectEvent): void {
		const { id, name } = tab;
		selected ? this.selected_tabs.set(id, { id, name }) : this.selected_tabs.delete(id);
		this.selected_tabs = new Map(this.selected_tabs);
		this.dispatchEvent(new SelectedTabsChangeEvent(this.selected_tabs));
	}

	#handle_menu_item_select(e: CustomEvent): void {
		const selectedItem = e.detail.item as SlMenuItem;

		if (selectedItem.value === 'multiselect') {
			this.multiselect = selectedItem.checked;
			this.dispatchEvent(new MultiselectChangeEvent(this.multiselect));
		} else if (selectedItem.value === 'hideRemoveOnly') {
			this.hideRemoveOnly = selectedItem.checked;
		}
	}

	decreasePage(): void {
		if (this.page > 1) {
			this.page--;
		}
	}
	increasePage(): void {
		this.page++;
	}
}

function filter(
	stashes: NoItemsTab[],
	nameQuery: string,
	shouldFilter: boolean,
	hideRemoveOnly: boolean
): NoItemsTab[] {
	if (!shouldFilter) return stashes;

	return stashes.filter(({ name }) => {
		if (hideRemoveOnly) {
			if (name.includes(REMOVE_ONLY)) return false;
		}
		return name.toLowerCase().includes(nameQuery.toLowerCase());
	});
}

function paginate(stashes: NoItemsTab[], page: number, perPage: number) {
	const start = (page - 1) * perPage;
	const end = start + perPage;
	return stashes.slice(start, end);
}

function shouldUnlockHideRemoveOnly(league: League, stashes: NoItemsTab[]) {
	return isPermanentLeague(league) && stashes.some(({ name }) => name.includes(REMOVE_ONLY));
}
