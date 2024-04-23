import { html, css, nothing } from 'lit';
import { BaseElement } from '../base-element';
import { property, state, query } from 'lit/decorators.js';
import { TabBadgeElement } from './tab-badge';
import { NoItemsTab } from '@divicards/shared/poe.types';
import { League, isPermanentLeague, permanentLeagues } from '@divicards/shared/types';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';
import '@shoelace-style/shoelace/dist/components/input/input.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';
import '@shoelace-style/shoelace/dist/components/checkbox/checkbox.js';

declare global {
	interface HTMLElementTagNameMap {
		'wc-tab-badge-group': TabBadgeGroupElement;
	}
}

export const REMOVE_ONLY = '(Remove-only)';

const filter = (
	stashes: NoItemsTab[],
	nameQuery: string,
	shouldFilter: boolean,
	hideRemoveOnly: boolean
): NoItemsTab[] => {
	if (!shouldFilter) return stashes;

	return stashes.filter(({ name }) => {
		if (hideRemoveOnly) {
			if (name.includes(REMOVE_ONLY)) return false;
		}
		return name.toLowerCase().includes(nameQuery.toLowerCase());
	});
};

const paginate = (stashes: NoItemsTab[], page: number, perPage: number) => {
	const start = (page - 1) * perPage;
	const end = start + perPage;
	return stashes.slice(start, end);
};

const shouldUnlockHideRemoveOnly = (league: League, stashes: NoItemsTab[]) => {
	return isPermanentLeague(league) && stashes.some(({ name }) => name.includes(REMOVE_ONLY));
};

export interface Events {
	'upd:nameQuery': string;
	'upd:PerPage': number;
	'upd:page': number;
	'upd:selectedTabs': Map<TabBadgeElement['tabId'], { id: TabBadgeElement['tabId']; name: TabBadgeElement['name'] }>;

	/**  Event from TabBadgeElement */
	'tab-select': { tabId: TabBadgeElement['tabId']; name: TabBadgeElement['name']; selected: boolean };
}

export class TabBadgeGroupElement extends BaseElement {
	static override get defineList() {
		return [TabBadgeElement];
	}
	static override tag = 'wc-tab-badge-group';
	static override styles = [styles()];

	@property({ type: Array }) stashes: NoItemsTab[] = [];
	@property({ reflect: true }) league: League = ACTIVE_LEAGUE;
	@property({ type: Number, reflect: true }) perPage = 50;
	@property({ type: Number, reflect: true }) page = 1;
	@property() nameQuery = '';
	@property({ attribute: false }) selectedTabs: Map<
		TabBadgeElement['tabId'],
		{ id: TabBadgeElement['tabId']; name: TabBadgeElement['name'] }
	> = new Map();

	@state() hideRemoveOnly = false;

	@query('sl-checkbox') checkbox!: HTMLInputElement;
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
		return this.stashes.length;
	}

	willUpdate(changed: Map<string, unknown>) {
		if (changed.has('nameQuery') || changed.has('hideRemoveOnly') || changed.has('perPage')) {
			this.page = 1;
		}
	}

	#onPageInput() {
		this.page = Number(this.pageInput.value);
		this.emit<Events['upd:page']>('upd:page', this.page);
	}

	#onPerPageInput() {
		this.perPage = Number(this.perPageInput.value);
		this.emit<Events['upd:PerPage']>('upd:PerPage', this.perPage);
	}

	#onNameQueryInput() {
		this.nameQuery = this.nameQueryInput.value;
		this.emit<Events['upd:nameQuery']>('upd:nameQuery', this.nameQuery);
	}

	#onTabSelect(e: CustomEvent<Events['tab-select']>) {
		const { selected, tabId, name } = e.detail;
		selected ? this.selectedTabs.set(tabId, { id: tabId, name }) : this.selectedTabs.delete(tabId);
		this.selectedTabs = new Map(this.selectedTabs);
		this.emit<Events['upd:selectedTabs']>('upd:selectedTabs', this.selectedTabs);
	}

	protected override render() {
		const ifWithHideRemoveOnly = this.withHideRemoveOnly
			? html` <div class="hide-remove-only">
					<sl-checkbox
						id="hide-remove-only"
						@sl-change=${() => (this.hideRemoveOnly = this.checkbox.checked)}
						.checked=${this.hideRemoveOnly}
						>Hide remove-only</sl-checkbox
					>
				</div>`
			: nothing;

		const filtersSection = this.shouldFilter
			? html`<div style="display: flex; flex-wrap: wrap; align-items: center; gap: 2rem">
					<div>
						<div class="filter">
							<label for="filter-stashes-by-name">Filter by name</label>
							<sl-input
								type="text"
								id="filter-stashes-by-name"
								.value=${this.nameQuery}
								@input=${this.#onNameQueryInput}
							></sl-input>
						</div>
					</div>
					<div class="page-controls">
						<sl-button ?disabled=${this.page === 1} @click=${this.decreasePage}>prev</sl-button>
						<sl-input
							id="page"
							type="text"
							.value=${String(this.page)}
							@sl-input=${this.#onPageInput}
						></sl-input>
						<sl-button @click=${this.increasePage}>next</sl-button>
						<label for="per-page">per page</label>
						<sl-input
							id="per-page"
							type="number"
							min="0"
							.value=${String(this.perPage)}
							@sl-input=${this.#onPerPageInput}
						></sl-input>
					</div>

					${ifWithHideRemoveOnly}

					<div class="tabs-total"><span>${this.tabsTotal}</span> stash tabs</div>
				</div>`
			: nothing;

		return html`<div class="tab-badge-group">${filtersSection} ${this.paginatedTabs()}</div>`;
	}

	protected paginatedTabs() {
		return html`<ul class="list">
			${this.paginated.map(tab => {
				return html`<li>
					<wc-tab-badge
						hexish-color=${tab.metadata?.colour ?? '#fff'}
						name=${tab.name}
						.tabId=${tab.id}
						index=${tab.index}
						.selected=${this.selectedTabs.has(tab.id)}
					></wc-tab-badge>
				</li>`;
			})}
		</ul>`;
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
		.filter {
			display: flex;
			gap: 0.4rem;
			align-items: center;
		}

		.page-controls {
			display: flex;
			gap: 0.4rem;
			align-items: center;
		}

		.page-controls > sl-input {
			width: 9ch;
			text-align: center;
		}

		.hide-remove-only {
			display: flex;
			align-items: center;
			gap: 0.2rem;
		}

		.tabs-total {
			> span {
				color: yellow;
			}
		}

		.list {
			display: flex;
			flex-wrap: wrap;
			list-style: none;
			gap: 5px;
			margin-inline: 1rem;
		}
	`;
}
