import { IDefaultStashLoader, IStashLoader } from '@divicards/shared/IStashLoader';
import {
	League,
	Order,
	DivinationCardRecord,
	Column,
	TablePreferences,
	TradeLeague,
	DivinationCardsSample,
	FixedName,
} from '@divicards/shared/types';
import { SlRange, SlAlert } from '@shoelace-style/shoelace';
import { TabWithItems, NoItemsTab } from 'poe-custom-elements/types.js';
import type { DefineComponent } from 'vue';
import { BasePopupElement } from '../wc/e-base-popup';
import { DivTableElement } from '../wc/sample-card/e-sample-table/e-sample-table';
import { To } from '../wc/sample-card/form-export-sample/form-export-sample';
import { LeagueSelectElement } from '../wc/e-league-select';
import { Size } from '../wc/e-order-triangle';
import { DownloadAs } from '../wc/stashes/e-stashes-view';
import { ErrorLabel } from '../wc/stashes/types';
import { ColorTheme } from '../wc/theme-toggle/theme-toggle';
import { ChangeEvent } from '../wc/stashes/e-tab-badge-group';

type BaseElementProps = {};

type BasePopupElementProps = {
	/** Instead of dialog's non-modal open, runs showModal() if true https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog#open */
	open?: boolean;
	/**  */
	onEscape?: string;
};

type DropFilesMessageElementProps = {};

type PaginationElementProps = {
	/**  */
	page?: number;
	/**  */
	'per-page'?: number;
	/** Number of items */
	n?: number;
	/**  */
	isLastPage?: boolean;
	/**  */
	onPageChange?: (e: CustomEvent<Event>) => void;
	/**  */
	onPerPageChange?: (e: CustomEvent<Event>) => void;
};

type HelpTipElementProps = {};

type SlConverterProps = {};

type LeagueSelectElementProps = {
	/**  */
	trade?: boolean;
	/**  */
	league?: League;
	/**  */
	privateLeague?: string;
	/**  */
	'with-private-league-input'?: boolean;
	/**  */
	select?: HTMLSelectElement;
	/**  */
	value?: string;
};

type OrderTriangleElementProps = {
	/**  */
	size?: Size;
	/**  */
	order?: Order;
	/**  */
	active?: boolean;
};

type PoeAuthElementProps = {
	/**  */
	name?: string;
	/**  */
	loggedIn?: boolean;
};

type DivTableStatElementProps = {};

type DivTableElementProps = {
	/**  */
	cards?: Readonly<DivinationCardRecord[]>;
	/**  */
	'min-price'?: number;
	/**  */
	column?: Column;
	/**  */
	order?: Order;
	/**  */
	_cards?: DivinationCardRecord[];
	/**  */
	nameQuery?: string;
	/**  */
	hideZeroSum?: boolean;
	/**  */
	filteredRecords?: DivinationCardRecord[];
	/**  */
	summary?: { amount: number; sum: number };
	/**  */
	checkboxHideZeroSum?: HTMLInputElement;
};

type FormExportSampleElementProps = {
	/**  */
	'spreadsheet-id'?: string;
	/**  */
	'sheet-title'?: string;
	/**  */
	order?: Order;
	/**  */
	orderedBy?: Column;
	/**  */
	cardsMustHaveAmount?: boolean;
	/**  */
	minPrice?: number;
	/**  */
	to?: To;
	/**  */
	columns?: Set<Column>;
	/**  */
	error?: string | null;
	/**  */
	tablePreferences?: TablePreferences;
};

type GoogleAuthElementProps = {
	/**  */
	name?: string;
	/**  */
	picture?: string;
	/**  */
	loggedIn?: boolean;
};

type SampleCardElementProps = {
	/**  */
	league?: TradeLeague;
	/**  */
	filename?: string;
	/**  */
	selected?: boolean | null;
	/**  */
	uuid?: string;
	/**  */
	'minimum-card-price'?: number;
	/**  */
	sample?: DivinationCardsSample;
	/**  */
	tablePopup?: BasePopupElement;
	/**  */
	selectedCheckbox?: HTMLInputElement;
	/**  */
	leagueSelect?: LeagueSelectElement;
	/**  */
	priceSlider?: HTMLInputElement;
	/**  */
	table?: DivTableElement;
	/**  */
	rangeEl?: SlRange;
	/**  */
	filteredCards?: string;
	/**  */
	filteredSummary?: string;
};

type StashTabContainerElementProps = {
	/** PoE API tab data https://www.pathofexile.com/developer/docs/reference#stashes-get */
	tab?: TabWithItems | null;
	/**  */
	status?: 'pending' | 'complete';
	/**  */
	scarabsSuccessAlert?: SlAlert;
	/** Emitted on "Extract cards sample" button click. */
	onExtractCards?: (e: CustomEvent<Event>) => void;
	/** Emitted on "X" button click. */
	onClose?: (e: CustomEvent<Event>) => void;
};

type StashTabErrorsElementProps = {
	/**  */
	errors?: Array<ErrorLabel>;
	/**  */
	hoveredErrorTabId?: ErrorLabel['noItemsTab']['id'] | null;
	/** CustomEvent<Array<{ noItemsTab: NoItemsTab; message: string }>> - Emitted when the errors array changes due to user interaction. */
	onUpderrors?: (e: CustomEvent<CustomEvent>) => void;
	/** CustomEvent<string | null> - Emitted on Error block mouseenter or mouseleave */
	onUpdhoverederrortabid?: (e: CustomEvent<CustomEvent>) => void;
};

type StashLoaderElementProps = {
	/**  */
	league?: League;
	/**  */
	customLeague?: string;
	/**  */
	selectedTabs?: Map<NoItemsTab['id'], { id: NoItemsTab['id']; name: NoItemsTab['name'] }>;
	/**  */
	stashes?: NoItemsTab[];
	/**  */
	noStashesMessage?: string;
	/**  */
	msg?: string;
	/**  */
	fetchingStash?: boolean;
	/**  */
	stashLoader?: IDefaultStashLoader;
	/**  */
	stashesButton?: HTMLButtonElement;
	/**  */
	getDataButton?: HTMLButtonElement;
};

type StashesViewElementProps = {
	/**  */
	league?: League;
	/**  */
	downloadAs?: DownloadAs;
	/**  */
	multiselect?: boolean;
	/**  */
	selectedTabs?: Map<NoItemsTab['id'], { id: NoItemsTab['id']; name: NoItemsTab['name'] }>;
	/**  */
	stashes?: NoItemsTab[];
	/**  */
	noStashesMessage?: string;
	/**  */
	msg?: string;
	/**  */
	fetchingStashTab?: boolean;
	/**  */
	fetchingStash?: boolean;
	/**  */
	stashLoader?: IStashLoader;
	/**  */
	errors?: Array<ErrorLabel>;
	/**  */
	stashLoadsAvailable?: number;
	/**  */
	availableInTenSeconds?: number;
	/**  */
	hoveredErrorTabId?: string | null;
	/**  */
	downloadedStashTabs?: Array<TabWithItems>;
	/**  */
	openedTabId?: string | null;
	/**  */
	openedTab?: NoItemsTab | null;
	/**  */
	stashesButton?: HTMLButtonElement;
	/**  */
	getDataButton?: HTMLButtonElement;
};

type UpdateEventProps = {
	/**  */
	field?: string;
};

type TabBadgeGroupElementProps = {
	/**  */
	'badges-disabled'?: boolean;
	/**  */
	multiselect?: boolean;
	/**  */
	stashes?: NoItemsTab[];
	/**  */
	league?: League;
	/**  */
	errors?: Array<ErrorLabel>;
	/**  */
	hoveredErrorTabId?: string | null;
	/**  */
	perPage?: number;
	/**  */
	page?: number;
	/**  */
	nameQuery?: string;
	/**  */
	selectedTabs?: Map<NoItemsTab['id'], { id: NoItemsTab['id']; name: NoItemsTab['name'] }>;
	/**  */
	hideRemoveOnly?: boolean;
	/**  */
	checkbox?: HTMLInputElement;
	/**  */
	perPageInput?: HTMLInputElement;
	/**  */
	pageInput?: HTMLInputElement;
	/**  */
	nameQueryInput?: HTMLInputElement;
	/**  */
	shouldFilter?: string;
	/**  */
	withHideRemoveOnly?: string;
	/**  */
	filtered?: string;
	/**  */
	paginated?: string;
	/**  */
	tabsTotal?: string;

	onChange?: ChangeEvent;
};

type TabSelectEventProps = {
	/**  */
	tab?: NoItemsTab;
	/**  */
	selected?: boolean;
};

type TabClickEventProps = {
	/**  */
	tab?: NoItemsTab;
};

type TabBadgeElementProps = {
	/**  */
	tab?: NoItemsTab;
	/**  */
	disabled?: boolean;
	/**  */
	selected?: boolean;
	/**  */
	color?: string | undefined;
	/**  */
	as?: 'button' | 'checkbox';
	/**  */
	tabState?: NoItemsTab;
	/**  */
	computedColor?: string;
	/**  */
	checkbox?: HTMLInputElement;
};

type ThemeToggleProps = {
	/**  */
	theme?: ColorTheme & string;
	/**  */
	$button?: HTMLButtonElement | null;
};

type FixedIconElementProps = {
	/**  */
	width?: number;
	/**  */
	height?: number;
};

type FixedNamesElementProps = {
	/**  */
	width?: number;
	/**  */
	height?: number;
	/**  */
	fixedNames?: FixedName[];
	/**  */
	popup?: BasePopupElement;
};

type NotCardsElementProps = {
	/**  */
	notCards?: string[];
	/**  */
	popup?: BasePopupElement;
};

export type CustomElements = {
	/**
	 *
	 * ---
	 *
	 */
	'wc-base-element': DefineComponent<BaseElementProps>;

	/**
	 *
	 * ---
	 *
	 */
	'e-base-popup': DefineComponent<BasePopupElementProps>;

	/**
	 * Message to drop files for main app screen
	 * ---
	 *
	 */
	'wc-drop-files-message': DefineComponent<DropFilesMessageElementProps>;

	/**
	 *
	 * ---
	 *
	 *
	 * ### **Events:**
	 *  - **page-change**
	 * - **per-page-change**
	 */
	'e-pagination': DefineComponent<PaginationElementProps>;

	/**
	 * A questionmark logo with hoverable tip content
	 * ---
	 *
	 *
	 * ### **Slots:**
	 *  - **The** - tip's main content
	 */
	'wc-help-tip': DefineComponent<HelpTipElementProps>;

	/**
	 *
	 * ---
	 *
	 */
	'wc-league-select': DefineComponent<LeagueSelectElementProps>;

	/**
	 *
	 * ---
	 *
	 */
	'wc-order-triangle': DefineComponent<OrderTriangleElementProps>;

	/**
	 *
	 * ---
	 *
	 */
	'wc-poe-auth': DefineComponent<PoeAuthElementProps>;

	/**
	 *
	 * ---
	 *
	 */
	'e-sample-table-stat': DefineComponent<DivTableStatElementProps>;

	/**
	 *
	 * ---
	 *
	 */
	'e-sample-table': DefineComponent<DivTableElementProps>;

	/**
	 *
	 * ---
	 *
	 */
	'wc-form-export-sample': DefineComponent<FormExportSampleElementProps>;

	/**
	 *
	 * ---
	 *
	 */
	'wc-google-auth': DefineComponent<GoogleAuthElementProps>;

	/**
	 *
	 * ---
	 *
	 */
	'wc-sample-card': DefineComponent<SampleCardElementProps>;

	/**
	 * Container for poe stash tab with header with actions.
	 * ---
	 *
	 *
	 * ### **Events:**
	 *  - **extract-cards** - Emitted on "Extract cards sample" button click.
	 * - **close** - Emitted on "X" button click.
	 */
	'e-stash-tab-container': DefineComponent<StashTabContainerElementProps>;

	/**
	 * Represents a block of possible stash tab errors during loading.
	 * ---
	 *
	 *
	 * ### **Events:**
	 *  - **upd:errors** - CustomEvent<Array<{ noItemsTab: NoItemsTab; message: string }>> - Emitted when the errors array changes due to user interaction.
	 * - **upd:hoveredErrorTabId** - CustomEvent<string | null> - Emitted on Error block mouseenter or mouseleave
	 */
	'e-stash-tab-errors': DefineComponent<StashTabErrorsElementProps>;

	/**
	 *
	 * ---
	 *
	 *
	 * ### **Methods:**
	 *
	 */
	'wc-stash-loader': DefineComponent<StashLoaderElementProps>;

	/**
	 *
	 * ---
	 *
	 */
	'wc-stashes-view': DefineComponent<StashesViewElementProps>;

	/**
	 *
	 * ---
	 *
	 */
	'wc-tab-badge-group': DefineComponent<TabBadgeGroupElementProps>;

	/**
	 *
	 * ---
	 *
	 */
	'wc-tab-badge': DefineComponent<TabBadgeElementProps>;

	/**
	 *
	 * ---
	 *
	 *
	 * ### **CSS Properties:**
	 *  - **--size** - undefined _(default: undefined)_
	 * - **--icon-fill** - undefined _(default: undefined)_
	 * - **--icon-fill** - undefined _(default: undefined)_
	 */
	'wc-theme-toggle': DefineComponent<ThemeToggleProps>;

	/**
	 *
	 * ---
	 *
	 */
	'wc-fixed-icon': DefineComponent<FixedIconElementProps>;

	/**
	 *
	 * ---
	 *
	 */
	'wc-fixed-names': DefineComponent<FixedNamesElementProps>;

	/**
	 *
	 * ---
	 *
	 */
	'wc-not-cards': DefineComponent<NotCardsElementProps>;
};

declare module 'vue' {
	// eslint-disable-next-line @typescript-eslint/no-empty-interface
	interface GlobalComponents extends CustomElements {}
}

declare global {
	namespace JSX {
		// eslint-disable-next-line @typescript-eslint/no-empty-interface
		interface IntrinsicElements extends CustomElements {}
	}
}
