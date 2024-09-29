import type { DefineComponent } from "vue";

type BasePopupElementProps = {
  /** Instead of dialog's non-modal open, runs showModal() if true https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog#open */
  open?: boolean;
  /**  */
  onEscape?: string;
};

type DropFilesMessageElementProps = {};

type GoogleAuthElementProps = {
  /**  */
  name?: string;
  /**  */
  picture?: string;
  /**  */
  loggedIn?: boolean;
};

type HelpTipElementProps = {};

type LeagueSelectElementProps = {
  /**  */
  trade?: boolean;
  /**  */
  league?: League;
  /**  */
  privateLeague?: string;
  /**  */
  "with-private-league-input"?: boolean;
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

type PaginationElementProps = {
  /**  */
  page?: number;
  /**  */
  "per-page"?: number;
  /** Number of items */
  n?: number;
  /**  */
  is_last_page?: boolean;
  /**  */
  onpage?: (e: CustomEvent<PageChangeEvent>) => void;
  /**  */
  onper_page?: (e: CustomEvent<PerPageChangeEvent>) => void;
};

type PoeAuthElementProps = {
  /**  */
  name?: string;
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
  "minimum-card-price"?: number;
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
  table?: SampleTableElement;
  /**  */
  rangeEl?: SlRange;
  /**  */
  filteredCards?: string;
  /**  */
  filteredSummary?: string;
};

type ThemeToggleProps = {
  /**  */
  theme?: string;
  /**  */
  theme?: ColorTheme;
  /**  */
  $button?: HTMLButtonElement | null;
};

type StashTabContainerElementProps = {
  /** PoE API tab data https://www.pathofexile.com/developer/docs/reference#stashes-get */
  tab?: TabWithItems | null;
  /**  */
  status?: "pending" | "complete";
  /**  */
  scarabsSuccessAlert?: SlAlert;
  /** Emitted on "Extract cards sample" button click. */
  "onextract-cards"?: (e: CustomEvent<Event>) => void;
  /** Emitted on "X" button click. */
  onclose?: (e: CustomEvent<Event>) => void;
};

type StashTabErrorsElementProps = {
  /**  */
  errors?: Array<ErrorLabel>;
  /**  */
  hoveredErrorTabId?: ErrorLabel["noItemsTab"]["id"] | null;
  /** CustomEvent<Array<{ noItemsTab: NoItemsTab; message: string }>> - Emitted when the errors array changes due to user interaction. */
  "onupd:errors"?: (e: CustomEvent<CustomEvent>) => void;
  /** CustomEvent<string | null> - Emitted on Error block mouseenter or mouseleave */
  "onupd:hoveredErrorTabId"?: (e: CustomEvent<CustomEvent>) => void;
};

type StashesViewElementProps = {
  /**  */
  league?: League;
  /**  */
  downloadAs?: DownloadAs;
  /**  */
  multiselect?: boolean;
  /**  */
  selectedTabs?: Map<NoItemsTab["id"], { id: NoItemsTab["id"]; name: NoItemsTab["name"] }>;
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

type TabBadgeGroupElementProps = {
  /**  */
  "badges-disabled"?: boolean;
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
  selectedTabs?: Map<NoItemsTab["id"], { id: NoItemsTab["id"]; name: NoItemsTab["name"] }>;
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
  /**  */
  tab?: NoItemsTab;
  /**  */
  disabled?: boolean;
  /**  */
  selected?: boolean;
  /**  */
  color?: string | undefined;
  /**  */
  as?: "button" | "checkbox";
  /**  */
  tabState?: NoItemsTab;
  /**  */
  computedColor?: string;
  /**  */
  checkbox?: HTMLInputElement;
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

type FormExportSampleElementProps = {
  /**  */
  "spreadsheet-id"?: string;
  /**  */
  "sheet-title"?: string;
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

type NotCardsElementProps = {
  /**  */
  notCards?: string[];
  /**  */
  popup?: BasePopupElement;
};

type DivTableStatElementProps = {};

type SampleTableElementProps = {
  /**  */
  cards?: Readonly<DivinationCardRecord[]>;
  /**  */
  "min-price"?: number;
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

export type CustomElements = {
  /**
   *
   * ---
   *
   */
  "e-base-popup": DefineComponent<BasePopupElementProps>;

  /**
   * Message to drop files for main app screen
   * ---
   *
   */
  "e-drop-files-message": DefineComponent<DropFilesMessageElementProps>;

  /**
   *
   * ---
   *
   */
  "e-google-auth": DefineComponent<GoogleAuthElementProps>;

  /**
   * A questionmark logo with hoverable tip content
   * ---
   *
   *
   * ### **Slots:**
   *  - **The** - tip's main content
   */
  "e-help-tip": DefineComponent<HelpTipElementProps>;

  /**
   *
   * ---
   *
   */
  "e-league-select": DefineComponent<LeagueSelectElementProps>;

  /**
   *
   * ---
   *
   */
  "e-order-triangle": DefineComponent<OrderTriangleElementProps>;

  /**
   *
   * ---
   *
   *
   * ### **Events:**
   *  - **page**
   * - **per_page**
   */
  "e-pagination": DefineComponent<PaginationElementProps>;

  /**
   *
   * ---
   *
   */
  "e-poe-auth": DefineComponent<PoeAuthElementProps>;

  /**
   *
   * ---
   *
   */
  "e-sample-card": DefineComponent<SampleCardElementProps>;

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
  "e-theme-toggle": DefineComponent<ThemeToggleProps>;

  /**
   * Container for poe stash tab with header with actions.
   * ---
   *
   *
   * ### **Events:**
   *  - **extract-cards** - Emitted on "Extract cards sample" button click.
   * - **close** - Emitted on "X" button click.
   */
  "e-stash-tab-container": DefineComponent<StashTabContainerElementProps>;

  /**
   * Represents a block of possible stash tab errors during loading.
   * ---
   *
   *
   * ### **Events:**
   *  - **upd:errors** - CustomEvent<Array<{ noItemsTab: NoItemsTab; message: string }>> - Emitted when the errors array changes due to user interaction.
   * - **upd:hoveredErrorTabId** - CustomEvent<string | null> - Emitted on Error block mouseenter or mouseleave
   */
  "e-stash-tab-errors": DefineComponent<StashTabErrorsElementProps>;

  /**
   *
   * ---
   *
   */
  "e-stashes-view": DefineComponent<StashesViewElementProps>;

  /**
   *
   * ---
   *
   */
  "e-tab-badge-group": DefineComponent<TabBadgeGroupElementProps>;

  /**
   *
   * ---
   *
   */
  "e-tab-badge": DefineComponent<TabBadgeElementProps>;

  /**
   *
   * ---
   *
   */
  "e-fixed-icon": DefineComponent<FixedIconElementProps>;

  /**
   *
   * ---
   *
   */
  "e-fixed-names": DefineComponent<FixedNamesElementProps>;

  /**
   *
   * ---
   *
   */
  "e-form-export-sample": DefineComponent<FormExportSampleElementProps>;

  /**
   *
   * ---
   *
   */
  "e-not-cards": DefineComponent<NotCardsElementProps>;

  /**
   *
   * ---
   *
   */
  "e-sample-table-stat": DefineComponent<DivTableStatElementProps>;

  /**
   *
   * ---
   *
   */
  "e-sample-table": DefineComponent<SampleTableElementProps>;
};

declare module "vue" {
  // eslint-disable-next-line @typescript-eslint/no-empty-interface
  interface GlobalComponents extends CustomElements {}
}

declare global {
  namespace JSX {
    // eslint-disable-next-line @typescript-eslint/no-empty-interface
    interface IntrinsicElements extends CustomElements {}
  }
}
