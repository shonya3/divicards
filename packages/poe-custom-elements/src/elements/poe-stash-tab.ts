import { LitElement, html, css, TemplateResult, PropertyValueMap, CSSResult } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';
import type { PoeItem, StashType, TabWithItems } from '../poe.types.js';
import './poe-item';
import { styleMap } from 'lit/directives/style-map.js';
import { appendFontinStyle } from '../lib/internal.js';
import { PoeItemElement } from './poe-item.js';
import { basePath } from '../lib/base_path.js';
import { classMap } from 'lit/directives/class-map.js';

type Direction = 'down' | 'right' | 'up' | 'left';
const SUPPORTED_STASH_TYPES = [
	'NormalStash',
	'PremiumStash',
	'QuadStash',
	'EssenceStash',
	'CurrencyStash',
	'FragmentStash',
	'BlightStash',
	'DivinationCardStash',
];

/**
 * PoE stash tab
 */
@customElement('poe-stash-tab')
export class PoeStashTabElement extends LitElement {
	/** PoE API tab data https://www.pathofexile.com/developer/docs/reference#stashes-get */
	@property({ type: Object }) tab!: TabWithItems;
	/** The state of search input for DivinationStashType */
	@state() search_query_for_divination_stash = '';
	/** Mutable clone of tab */
	#tab!: TabWithItems;
	get #focused_item_element(): PoeItemElement | null {
		return this.shadowRoot?.querySelector('poe-item:focus') ?? null;
	}

	protected willUpdate(map: PropertyValueMap<this>): void {
		if (map.has('tab') && this.tab) {
			this.#tab = structuredClone(this.tab);
			const cells = get_stash_tab_side_cell_count(this.#tab.type);
			adjust_item_x_y_for_custom_tab(this.#tab, cells);
			this.#tab.items = orderItems(this.#tab.items);
			this.style.setProperty('--cells-side-count', cells.toString());
			this.style.setProperty('--background-image', `url(${tabImageSrc(this.#tab.type)})`);
		}
	}

	protected render(): TemplateResult {
		if (!this.tab) {
			this.style.setProperty('border', '2px solid red');
			return html`<p style="color: red">No Poe Api stash tab data (.tab)</p>`;
		}
		if (!SUPPORTED_STASH_TYPES.includes(this.tab.type)) {
			this.style.setProperty('border', '2px solid red');
			return html`<p style="color: red; font-size: 24px">
				StashType ( ${this.tab.type} ) is not supported ( yet? ).
			</p>`;
		}
		return html`
			<ul>
				${this.#tab.items.map(
					item =>
						html`<li
							style=${styleMap({
								'grid-column': `${item.x + 1} / span ${item.w}`,
								'grid-row': `${item.y + 1} / span ${item.h}`,
							})}
						>
							<poe-item
								data-x=${item.x}
								data-y=${item.y}
								tabindex="0"
								placed
								style="--cell-size: ${sizeOfCellPixels(this.#tab.type)}"
								.item=${item}
								class=${classMap({
									'item--visible': item.baseType
										.toLowerCase()
										.includes(this.search_query_for_divination_stash),
								})}
							></poe-item>
						</li>`
				)}
			</ul>
			${this.#tab.type === 'DivinationCardStash'
				? html`<input
						placeholder="Search cards"
						autocomplete="off"
						id="search-divination-stash-tab"
						type="text"
						.value=${this.search_query_for_divination_stash}
						@input=${this.#handleDivinationCardsQueryInput}
				  />`
				: null}
		`;
	}

	#handleDivinationCardsQueryInput(e: InputEvent) {
		if (e.target instanceof HTMLInputElement) {
			this.search_query_for_divination_stash = e.target.value.trim().toLowerCase();
		}
	}

	connectedCallback(): void {
		super.connectedCallback();
		appendFontinStyle();
		window.addEventListener('keydown', this.#navigate_items_with_arrow_keys);
	}

	disconnectedCallback(): void {
		super.disconnectedCallback();
		window.removeEventListener('keydown', this.#navigate_items_with_arrow_keys);
	}

	#navigate_items_with_arrow_keys = async (e: KeyboardEvent): Promise<void> => {
		if (!['ArrowDown', 'ArrowRight', 'ArrowUp', 'ArrowLeft'].includes(e.code)) {
			return;
		}

		const active_item = this.#focused_item_element?.item;
		if (!active_item) {
			return;
		}

		// Prevent outer scrolling when the arrow keys are pressed while focus is inside the stash tab.
		e.preventDefault();

		const direction = e.code.slice(5).toLowerCase() as Direction;
		const item = await find_closest_item_element({
			active_item,
			direction,
			items: this.#tab.items,
			stash_tab_element: this,
			tab_cells_side_count: get_stash_tab_side_cell_count(this.#tab.type),
		});
		if (!item) {
			return;
		}

		item.focus();
	};

	static styles: CSSResult = css`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}
		:host {
			display: block;
			--size: 569px;
			--size-of-all-inner-borders: 5px;
			--cells-side-count: '(computed) Number of cells';
			--background-image: '(computed)';
			width: var(--size);
			height: var(--size);
			background-image: var(--background-image);
			font-family: fontin;
			position: relative;
		}

		ul {
			width: var(--size);
			height: var(--size);
			list-style: none;
			display: grid;
			grid-template-rows: repeat(var(--cells-side-count), 1fr);
			grid-template-columns: repeat(var(--cells-side-count), 1fr);
			gap: calc(var(--size-of-all-inner-borders) / var(--cells-side-count));
		}

		poe-item:focus,
		poe-item:focus-visible {
			outline: 3px solid rgb(39, 186, 253);
		}
		:host(:focus-within) {
			outline: 3px solid rgb(39, 186, 253);
		}

		poe-item {
			visibility: hidden;
		}

		.item--visible {
			visibility: visible;
		}

		#search-divination-stash-tab {
			position: absolute;
			bottom: 1.2rem;
			right: 1.2rem;
			font: inherit;
			border: 1px solid currentColor;
			color: #e2e2e2;
			transition: border 0.1s ease;
			background-color: transparent;
			border-radius: 2px;
			padding: 3px 7px;
			width: 30ch;
		}
	`;
}

/**
 * Adjusts the (x, y) coordinates of items in stash tabs that have a custom, non-grid layout,
 * rearranging them to fit into a grid layout.
 *
 * @param tab - The stash tab containing the items.
 * @param cells_side_count - The number of cells on one side of the grid layout.
 */
function adjust_item_x_y_for_custom_tab(tab: TabWithItems, cells_side_count: number): void {
	if (!cells_side_count) {
		return;
	}
	if (tab.type === 'FragmentStash') {
		let current_x_for_y1_group = 0;
		const STARTING_Y_FOR_Y1_GROUP = 12;
		const Y_OFFSET_FOR_Y2_GROUP = 13;
		tab!.items.forEach(item => {
			switch (item.y) {
				case 0: {
					item.y = Math.floor(item.x / cells_side_count);
					item.x = item.x % cells_side_count;
					break;
				}
				case 1: {
					item.y = STARTING_Y_FOR_Y1_GROUP + Math.floor(current_x_for_y1_group / cells_side_count);
					item.x = current_x_for_y1_group % cells_side_count;
					current_x_for_y1_group++;
					break;
				}
				case 2: {
					item.y += Y_OFFSET_FOR_Y2_GROUP;
					break;
				}
				default: {
					console.warn(`Fragments stash unexpected item Y-coordinate. Expected 0|1|2, got ${item.y}`);
				}
			}
		});
	}
	if (
		tab.type === 'EssenceStash' ||
		tab.type === 'CurrencyStash' ||
		tab.type === 'BlightStash' ||
		tab.type === 'DivinationCardStash'
	) {
		if (tab.type === 'DivinationCardStash') {
			let x = 0;
			tab!.items.forEach(item => {
				item.x = x++;
			});
		}

		tab!.items.forEach(item => {
			item.y = Math.floor(item.x / cells_side_count);
			item.x = item.x % cells_side_count;
		});
	}
}

/**
 * Returns the number of cells on one side of the stash tab based on the stash type.
 *
 * @param stash_type - The type of the stash tab (e.g., PremiumStash, QuadStash, etc.).
 * @returns The number of cells on one side of the stash tab.
 */
function get_stash_tab_side_cell_count(stash_type: StashType): number {
	switch (stash_type) {
		case 'PremiumStash':
		case 'NormalStash':
		case 'BlightStash':
			return 12;
		case 'QuadStash':
		case 'FragmentStash':
		case 'DivinationCardStash':
			return 24;
		default:
			return 12;
	}
}

function tabImageSrc(stash_type: StashType): string {
	switch (stash_type) {
		case 'PremiumStash':
		case 'NormalStash':
		case 'EssenceStash':
		case 'CurrencyStash':
		case 'BlightStash':
			return `${basePath()}/poe-images/StashPanelGrid.png`;
		case 'QuadStash':
		case 'FragmentStash':
		case 'DivinationCardStash':
			return `${basePath()}/poe-images/QuadStashPanelGrid.png`;
		default:
			return `${basePath()}/poe-images/StashPanelGrid.png`;
	}
}

function sizeOfCellPixels(stash_type: StashType): `${number}px` {
	return `${564 / get_stash_tab_side_cell_count(stash_type)}px`;
}

function orderItems(items: Array<PoeItem>): Array<PoeItem> {
	return Object.values(Object.groupBy(items, ({ y }) => y)).flatMap((itemsByRow = []) => {
		itemsByRow.sort((a, b) => a.x - b.x);
		return itemsByRow;
	});
}

/**
 * Finds the item element closest to the active item based on the provided direction.
 */
async function find_closest_item_element({
	active_item,
	direction,
	items,
	stash_tab_element,
	tab_cells_side_count,
}: {
	active_item: PoeItem;
	direction: Direction;
	items: Array<PoeItem>;
	stash_tab_element: PoeStashTabElement;
	tab_cells_side_count: number;
}): Promise<PoeItemElement | null> {
	if (direction === 'down') {
		let current_x = active_item.x;
		let current_y = active_item.y + active_item.h;
		let item = null;

		if (current_y + 1 > tab_cells_side_count) {
			return null;
		}
		while (!item) {
			for (let dx = 0; dx < active_item.w; dx++) {
				item = await item_element_from_x_y({
					stash_tab_element,
					items,
					coordinates: { x: current_x + dx, y: current_y },
				});

				if (item) {
					break;
				}
			}

			if (current_y === tab_cells_side_count - 1 && current_x === tab_cells_side_count - 1) {
				break;
			}

			if (current_y === tab_cells_side_count - 1) {
				current_y = active_item.y;
				current_x++;
			} else {
				current_y++;
			}
		}

		return item;
	} else if (direction === 'right') {
		let current_x = active_item.x + active_item.w;
		let current_y = active_item.y;
		let item = null;

		if (current_x + 1 > tab_cells_side_count) {
			return null;
		}
		while (!item) {
			for (let dy = 0; dy < active_item.h; dy++) {
				item = await item_element_from_x_y({
					stash_tab_element,
					items,
					coordinates: { x: current_x, y: current_y + dy },
				});

				if (item) {
					break;
				}
			}

			if (current_y === tab_cells_side_count - 1 && current_x === tab_cells_side_count - 1) {
				break;
			}

			if (current_x === tab_cells_side_count - 1) {
				current_x = active_item.x + active_item.w;
				current_y++;
			} else {
				current_x++;
			}
		}

		return item;
	} else if (direction === 'up') {
		let current_x = active_item.x;
		let current_y = active_item.y - 1;
		let item = null;

		if (current_y < 0) {
			return null;
		}
		while (!item) {
			for (let dx = 0; dx < active_item.w; dx++) {
				item = await item_element_from_x_y({
					stash_tab_element,
					items,
					coordinates: { x: current_x + dx, y: current_y },
				});

				if (item) {
					break;
				}
			}

			if (current_y === 0 && current_x === tab_cells_side_count - 1) {
				break;
			}

			if (current_y === 0) {
				current_y = active_item.y;
				current_x = current_x + 1;
			} else {
				current_y--;
			}
		}

		return item;
	} else if (direction === 'left') {
		let current_x = active_item.x - 1;
		let current_y = active_item.y;
		let item = null;

		if (current_x < 0) {
			return null;
		}
		while (!item) {
			for (let dy = 0; dy < active_item.h; dy++) {
				item = await item_element_from_x_y({
					stash_tab_element,
					items,
					coordinates: { x: current_x, y: current_y + dy },
				});

				if (item) {
					break;
				}
			}

			if (current_x === 0 && current_y === tab_cells_side_count - 1) {
				break;
			}

			if (current_x === 0) {
				current_x = active_item.x;
				current_y = current_y + 1;
			} else {
				current_x--;
			}
		}

		return item;
	}

	return null;
}

function item_from_x_y({
	coordinates,
	items,
}: {
	items: Array<PoeItem>;
	coordinates: { x: number; y: number };
}): PoeItem | null {
	const itemOnExactXY = items.find(item => item.x === coordinates.x && item.y === coordinates.y);
	if (itemOnExactXY) {
		return itemOnExactXY;
	}

	for (const item of items) {
		for (let y = item.y; y < item.y + item.h; y++) {
			for (let x = item.x; x < item.x + item.w; x++) {
				if (x === coordinates.x && y === coordinates.y) {
					return item;
				}
			}
		}
	}
	return null;
}

async function item_element_from_x_y({
	stash_tab_element,
	coordinates,
	items,
}: {
	stash_tab_element: PoeStashTabElement;
	coordinates: { x: number; y: number };
	items: Array<PoeItem>;
}): Promise<PoeItemElement | null> {
	const item = item_from_x_y({
		coordinates,
		items,
	});
	if (!item) {
		return null;
	}

	if (!stash_tab_element.shadowRoot!.querySelector('poe-item')) {
		await stash_tab_element.updateComplete;
	}

	return (
		stash_tab_element.shadowRoot?.querySelector<PoeItemElement>(
			`poe-item[data-x="${item.x}"][data-y="${item.y}"]`
		) ?? null
	);
}

declare global {
	interface HTMLElementTagNameMap {
		'poe-stash-tab': PoeStashTabElement;
	}
}
