import { html, css, nothing } from 'lit';
import { BaseElement } from '../base-element';
import { property, state, query } from 'lit/decorators.js';
import { TabBadgeElement } from './tab-badge';
import { StatefulStashTab } from '@divicards/shared/poe.types';
import { League, permanentLeagues } from '@divicards/shared/types';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';

declare global {
	interface HTMLElementTagNameMap {
		'wc-tab-badge-group': TabBadgeGroupElement;
	}
}

const REMOVE_ONLY = '(Remove-only)';

const styles = css`
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
	}

	.page-controls {
		display: flex;
		gap: 0.4rem;
		align-items: center;
	}

	.page-controls > input {
		width: 5ch;
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

const filter = (
	stashes: StatefulStashTab[],
	nameQuery: string,
	shouldFilter: boolean,
	hideRemoveOnly: boolean
): StatefulStashTab[] => {
	if (!shouldFilter) return stashes;

	return stashes.filter(({ name }) => {
		if (hideRemoveOnly) {
			if (name.includes(REMOVE_ONLY)) return false;
		}
		return name.toLowerCase().includes(nameQuery.toLowerCase());
	});
};

const paginate = (stashes: StatefulStashTab[], page: number, perPage: number) => {
	const start = (page - 1) * perPage;
	const end = start + perPage;
	return stashes.slice(start, end);
};

const shouldUnlockHideRemoveOnly = (league: League, stashes: StatefulStashTab[]) => {
	return permanentLeagues.includes(league) && stashes.some(({ name }) => name.includes(REMOVE_ONLY));
};

export class TabBadgeGroupElement extends BaseElement {
	static define(tag = 'wc-tab-badge-group') {
		if (!customElements.get(tag)) {
			customElements.define(tag, TabBadgeGroupElement);
			TabBadgeElement.define();
		}
	}
	static htmlTag = 'wc-tab-badge-group';
	static styles = [this.baseStyles, styles];

	@property({ type: Array }) stashes: StatefulStashTab[] = [];
	@property({ reflect: true }) league: League = ACTIVE_LEAGUE;

	@property({ type: Number, reflect: true }) perPage = 50;
	@property({ type: Number, reflect: true }) page = 1;
	@property() nameQuery = '';

	@state() hideRemoveOnly = false;

	@query('input[type="checkbox"]') checkbox!: HTMLInputElement;
	@query('input#per-page') perPageInput!: HTMLInputElement;
	@query('input#page') pageInput!: HTMLInputElement;
	@query('input#filter-stashes-by-name') nameQueryInput!: HTMLInputElement;
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

	updated(changed: Map<string, unknown>) {
		if (changed.has('nameQuery') || changed.has('hideRemoveOnly') || changed.has('perPage')) {
			this.page = 1;
		}
	}

	render() {
		const ifWithHideRemoveOnly = this.withHideRemoveOnly
			? html` <div class="hide-remove-only">
					<label for="hide-remove-only">Hide remove-only</label>
					<input
						type="checkbox"
						id="hide-remove-only"
						v-model="hideRemoveOnly"
						.checked=${this.hideRemoveOnly}
						@change=${() => (this.hideRemoveOnly = this.checkbox.checked)}
					/>
			  </div>`
			: nothing;

		const filtersSection = this.shouldFilter
			? html`<div v-if="shouldFilter" style="display: flex; flex-wrap: wrap; align-items: center; gap: 2rem">
					<div>
						<div class="filter" v-if="shouldFilter">
							<label for="filter-stashes-by-name">Filter by name</label>
							<input
								type="text"
								id="filter-stashes-by-name"
								.value=${this.nameQuery}
								@input=${this.#onNameQueryInput}
							/>
						</div>
					</div>
					<div class="page-controls" v-if="shouldFilter">
						<button ?disabled=${this.page === 1} @click=${this.decreasePage}>prev</button>
						<input id="page" type="text" .value=${String(this.page)} @input=${this.#onPageInput} />
						<button @click=${this.increasePage}>next</button>
						<label for="per-page">per page</label>
						<input
							id="per-page"
							type="number"
							min="0"
							.value=${String(this.perPage)}
							@input=${this.#onPerPageInput}
						/>
					</div>

					${ifWithHideRemoveOnly}

					<div class="tabs-total"><span>${this.tabsTotal}</span> stash tabs</div>
			  </div>`
			: nothing;

		const paginatedTabs = html`<ul class="list">
			${this.paginated.map(tab => {
				return html`<li>
					<wc-tab-badge
						colour=${tab.metadata?.colour ?? '#fff'}
						name=${tab.name}
						.tabId=${tab.id}
						index=${tab.index}
						.selected=${tab.selected}
					></wc-tab-badge>
				</li>`;
			})}
		</ul>`;

		return html`<div class="tab-badge-group">${filtersSection} ${paginatedTabs}</div>`;
	}

	decreasePage() {
		if (this.page > 1) {
			this.page--;
		}
	}

	increasePage() {
		this.page++;
	}

	#onPageInput() {
		this.page = Number(this.pageInput.value);
	}

	#onPerPageInput() {
		this.perPage = Number(this.perPageInput.value);
	}

	#onNameQueryInput() {
		this.nameQuery = this.nameQueryInput.value;
	}
}
