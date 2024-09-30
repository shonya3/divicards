import { html, css, nothing, LitElement } from 'lit';
import { property, state, query, customElement } from 'lit/decorators.js';
import './e-tab-badge';
import { League, isPermanentLeague } from '@divicards/shared/types';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';
import '@shoelace-style/shoelace/dist/components/input/input.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';
import '@shoelace-style/shoelace/dist/components/checkbox/checkbox.js';
import '../e-pagination';
import '../e-help-tip';
import { ErrorLabel } from './types';
import { classMap } from 'lit/directives/class-map.js';
import SlCheckbox from '@shoelace-style/shoelace/dist/components/checkbox/checkbox.js';
import { PageChangeEvent, PerPageChangeEvent } from '../e-pagination';
import { NoItemsTab } from 'poe-custom-elements/types.js';
import { emit } from '../../utils';

declare global {
	interface HTMLElementTagNameMap {
		'e-tab-badge-group': TabBadgeGroupElement;
	}
}

export const REMOVE_ONLY = '(Remove-only)';

export interface Events {
	'upd:nameQuery': string;
	'upd:PerPage': number;
	'upd:page': number;
	'upd:multiselect': boolean;
	'upd:selectedTabs': Map<NoItemsTab['id'], { id: NoItemsTab['id']; name: NoItemsTab['name'] }>;

	/**  Event from TabBadgeElement */
	'tab-select': { tabId: NoItemsTab['id']; name: NoItemsTab['name']; selected: boolean };
	'tab-click': { tabId: string; name: string };
}

export abstract class AbstractChange<
	CE, // Custom Element type
	T extends keyof CE = keyof CE, // Default T to keyof CE if not provided
	V = CE[T], // Infer the value type from the CE's field type
> extends Event {
	readonly field: T;
	readonly value: V;

	constructor(field: T, value: V, options?: EventInit) {
		super('event-field-change', options);
		this.field = field;
		this.value = value;
	}
}

export class ChangeEvent<
	T extends 'nameQuery' | 'perPage' | 'page' | 'multiselect' | 'selectedTabs', // Default to keyof TabBadgeGroupElement if no union provided
> extends AbstractChange<TabBadgeGroupElement, T> {}

@customElement('e-tab-badge-group')
export class TabBadgeGroupElement extends LitElement {
	static override styles = [styles()];

	@property({ type: Boolean, attribute: 'badges-disabled' }) badgesDisabled = false;
	@property({ type: Boolean }) multiselect = false;
	@property({ type: Array }) stashes: NoItemsTab[] = [];
	@property({ reflect: true }) league: League = ACTIVE_LEAGUE;
	@property({ type: Array }) errors: Array<ErrorLabel> = [];
	@property() hoveredErrorTabId: string | null = null;
	@property({ type: Number, reflect: true }) perPage = 50;
	@property({ type: Number, reflect: true }) page = 1;
	@property() nameQuery = '';
	@property({ attribute: false }) selectedTabs: Map<
		NoItemsTab['id'],
		{ id: NoItemsTab['id']; name: NoItemsTab['name'] }
	> = new Map();

	@state() hideRemoveOnly = false;

	@query('sl-checkbox#hide-remove-only') checkbox!: HTMLInputElement;
	@query('sl-input#per-page') perPageInput!: HTMLInputElement;
	@query('sl-input#page') pageInput!: HTMLInputElement;
	@query('sl-input#filter-stashes-by-name') nameQueryInput!: HTMLInputElement;

	constructor() {
		super();
		this.addEventListener('tab-select', e => {
			this.#onTabSelect(e as CustomEvent<Events['tab-select']>);
		});
	}
	get shouldFilter() {
		return this.stashes.length > 50;
	}
	get withHideRemoveOnly() {
		return shouldUnlockHideRemoveOnly(this.league, this.stashes);
	}
	get filtered() {
		return filter(this.stashes, this.nameQuery, this.shouldFilter, this.hideRemoveOnly);
	}
	get paginated() {
		return paginate(this.filtered, this.page, this.perPage);
	}
	get tabsTotal() {
		return this.filtered.length;
	}

	willUpdate(changed: Map<string, unknown>) {
		if (changed.has('nameQuery') || changed.has('hideRemoveOnly') || changed.has('perPage')) {
			this.page = 1;
		}
	}

	protected override render() {
		return html`</div>
			<div class="tab-badge-group">
				${
					this.shouldFilter
						? html`<header class="header">
								<sl-input
									type="text"
									id="filter-stashes-by-name"
									.value=${this.nameQuery}
									@input=${this.#onNameQueryInput}
									.helpText=${`Search tab by name`}
								></sl-input>
								<e-pagination
									.n=${this.tabsTotal}
									.page=${this.page}
									.per_page=${this.perPage}
									@e-pagination--page-change=${this.#onPageChange}
									@e-pagination--per-page-change=${this.#onPerPageChange}
								></e-pagination>
								<div class="header__right">
									<div class="multiselect">
										<sl-checkbox
											@sl-change=${this.#onMultiselectChange}
											.checked=${this.multiselect}
											>Multiselect</sl-checkbox
										>
										<e-help-tip>Select multiple tabs at once to download it in one go.</e-help-tip>
									</div>
									${this.withHideRemoveOnly
										? html` <div class="hide-remove-only">
												<sl-checkbox
													id="hide-remove-only"
													@sl-change=${this.#onHideRemoveOnlyChange}
													.checked=${this.hideRemoveOnly}
													>Hide remove-only</sl-checkbox
												>
											</div>`
										: nothing}
									<div class="tabs-total">
										<span class="tabs-total__count">${this.tabsTotal}</span> stash tabs
									</div>
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
								.selected=${this.selectedTabs.has(tab.id)}
								.disabled=${this.badgesDisabled}
							></e-tab-badge>
						</li>`;
					})}
				</ul>
			</div>`;
	}

	#onHideRemoveOnlyChange() {
		this.hideRemoveOnly = this.checkbox.checked;
	}
	#onPageChange({ page }: PageChangeEvent) {
		this.page = page;
		emit<Events['upd:page']>(this, 'upd:page', page);
	}
	#onPerPageChange({ per_page }: PerPageChangeEvent): void {
		this.perPage = per_page;
		emit<Events['upd:PerPage']>(this, 'upd:PerPage', per_page);
	}
	#onNameQueryInput() {
		this.nameQuery = this.nameQueryInput.value;
		emit<Events['upd:nameQuery']>(this, 'upd:nameQuery', this.nameQuery);
	}
	#onTabSelect(e: CustomEvent<Events['tab-select']>) {
		const { selected, tabId, name } = e.detail;
		selected ? this.selectedTabs.set(tabId, { id: tabId, name }) : this.selectedTabs.delete(tabId);
		this.selectedTabs = new Map(this.selectedTabs);
		emit<Events['upd:selectedTabs']>(this, 'upd:selectedTabs', this.selectedTabs);
	}
	#onMultiselectChange(e: InputEvent) {
		this.multiselect = (e.target as SlCheckbox).checked;
		emit<Events['upd:multiselect']>(this, 'upd:multiselect', this.multiselect);
	}
	decreasePage() {
		if (this.page > 1) {
			this.page--;
		}
	}
	increasePage() {
		this.page++;
	}
}

function styles() {
	return css`
		:host {
			display: inline-block;
		}
		.tab-badge-group {
			display: grid;
			gap: 1rem;
		}

		.header {
			padding-inline: 2rem;
			display: flex;
			flex-wrap: wrap;
			justify-content: space-between;
			align-items: center;
			gap: 2rem;

			.header__right {
				margin-top: 12px;
				display: flex;
				align-items: center;
				gap: 1rem;
			}

			> sl-input {
				margin-top: 1rem;
			}
		}

		.hide-remove-only {
			display: flex;
			align-items: center;
			gap: 0.2rem;
		}

		.tabs-total__count {
			color: var(--sl-color-amber-800);
		}

		.multiselect {
			display: flex;
			align-items: center;
			gap: 4px;
		}

		.list {
			display: flex;
			flex-wrap: wrap;
			list-style: none;
			gap: 5px;
			margin-inline: 1rem;
		}

		li {
			border: 1px solid transparent;
			border-radius: 4px;
		}

		.hovered-error {
			border-color: red;
		}
	`;
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
