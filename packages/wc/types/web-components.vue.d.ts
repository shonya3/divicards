import type { DefineComponent } from 'vue';
import { Size } from '../src/wc/order-triangle';
import { LeagueSelectElement } from '../src/wc/league-select';
import { DivTableElement } from '../src/wc/div-table/div-table';
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
import SlRange from '@shoelace-style/shoelace/dist/components/range/range.component.js';
import { TabBadgeElement } from '../src/wc/stashes/tab-badge';
import { NoItemsTab } from '@divicards/shared/poe.types';
import { IDefaultStashLoader, IStashLoader } from '@divicards/shared/IStashLoader';
import { BasePopupElement } from '../src/wc/base-popup';
import { To } from '../src/wc/form-export-sample/form-export-sample';
import { DownloadAs } from '../src/wc/stashes/stashes-view';

type BaseElementProps = {};

type BasePopupElementProps = {
	/** Instead of dialog's non-modal open, runs showModal() if true https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog#open */
	open?: boolean;
};

type DropFilesMessageElementProps = {};

type HelpTipElementProps = {};

type SlConverterProps = {};

type LeagueSelectElementProps = {
	/**  */
	trade?: boolean;
	/**  */
	league?: League;
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

type StashLoaderElementProps = {
	/**  */
	league?: League;
	/**  */
	customLeague?: string;
	/**  */
	selectedTabs?: Map<TabBadgeElement['tabId'], { id: TabBadgeElement['tabId']; name: TabBadgeElement['name'] }>;
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
	customLeague?: string;
	/**  */
	downloadAs?: DownloadAs;
	/**  */
	selectedTabs?: Map<TabBadgeElement['tabId'], { id: TabBadgeElement['tabId']; name: TabBadgeElement['name'] }>;
	/**  */
	stashes?: NoItemsTab[];
	/**  */
	noStashesMessage?: string;
	/**  */
	msg?: string;
	/**  */
	fetchingStash?: boolean;
	/**  */
	stashLoader?: IStashLoader;
	/**  */
	stashesButton?: HTMLButtonElement;
	/**  */
	getDataButton?: HTMLButtonElement;
};

type TabBadgeGroupElementProps = {
	/**  */
	stashes?: NoItemsTab[];
	/**  */
	league?: League;
	/**  */
	perPage?: number;
	/**  */
	page?: number;
	/**  */
	nameQuery?: string;
	/**  */
	selectedTabs?: Map<TabBadgeElement['tabId'], { id: TabBadgeElement['tabId']; name: TabBadgeElement['name'] }>;
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
};

type TabBadgeElementProps = {
	/** Color from Poe API. Examples: ff, 80b3ff, #f0f80, cc009a, 7c5436 */
	'hexish-color'?: string;
	/** Any valid CSS color */
	color?: string | undefined;
	/**  */
	name?: string;
	/**  */
	tabId?: string;
	/**  */
	selected?: boolean;
	/**  */
	index?: number;
	/**  */
	checkbox?: HTMLInputElement;
	/**  */
	computedColor?: string;
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
	'wc-base-popup': DefineComponent<BasePopupElementProps>;

	/**
	 * Message to drop files for main app screen
	 * ---
	 *
	 */
	'wc-drop-files-message': DefineComponent<DropFilesMessageElementProps>;

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
	'wc-div-table': DefineComponent<DivTableElementProps>;

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
	'wc-sample-card': DefineComponent<SampleCardElementProps>;

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
	 *
	 * ### **Methods:**
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
