import { LitElement, html, css, TemplateResult, CSSResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import type { TabWithItems, PoeItem } from 'poe-custom-elements/types.js';
import 'poe-custom-elements/item.js';
import type { IStashLoader } from '@divicards/shared/IStashLoader.js';
import '@shoelace-style/shoelace/dist/components/alert/alert.js';
import '@shoelace-style/shoelace/dist/components/icon/icon.js';
import '@shoelace-style/shoelace/dist/components/dialog/dialog.js';
import '@shoelace-style/shoelace/dist/components/input/input.js';
import '@shoelace-style/shoelace/dist/components/select/select.js';
import '@shoelace-style/shoelace/dist/components/option/option.js';
import '@shoelace-style/shoelace/dist/components/popup/popup.js';
import '../shared/e-json-viewer';

@customElement('poe-general-priced-list')
export class PoeGeneralPricedListElement extends LitElement {
  @property({ type: Object }) tab!: TabWithItems;
  @property() league: string = 'Standard';
  @property({ type: Object }) prices: Map<string, number> = new Map();
  @property() sortBy: 'name' | 'tab' | 'qty' | 'price' | 'total' = 'name';
  @property() sortDir: 'asc' | 'desc' = 'asc';
  @property({ attribute: false }) stashLoader!: IStashLoader;
  @property() errorMessage: string | null = null;
  @property({ type: Boolean }) viewPricesOpen: boolean = false;
  @property({ attribute: false }) debugData: Record<string, any[]> = {};
  @property({ type: Boolean }) aggregate: boolean = false;
  @property() filter: string = '';
  @property({ type: Boolean }) invalidRegex: boolean = false;
  @property({ type: Boolean }) filtersOpen: boolean = false;
  @property() category: string | null = null;
  @property({ type: Number }) qtyMin: number | null = null;
  @property({ type: Number }) qtyMax: number | null = null;
  @property({ type: Number }) priceMin: number | null = null;
  @property({ type: Number }) priceMax: number | null = null;
  @property({ type: Number }) totalMin: number | null = null;
  @property({ type: Number }) totalMax: number | null = null;
  private categoryIndex: Map<string, string> = new Map();

  private buildCategoryIndex(src: CategorySource): void {
    const map = new Map<string, string>();
    (Object.keys(src) as Array<keyof CategorySource>).forEach(key => {
      const arr = src[key] || [];
      const category = categoryFromKey(key);
      arr.forEach(r => { if (r && typeof (r as any).name === 'string') map.set(normalizeName((r as any).name), category); });
    });
    this.categoryIndex = map;
  }

  private categories(): string[] {
    const set = new Set<string>(Array.from(this.categoryIndex.values()));
    return Array.from(set.values());
  }

  private matchesAdvancedFilters(r: { name: string; qty: number; price: number; total: number }): boolean {
    if (this.aggregate) {
      if (this.category) {
        const cat = this.categoryIndex.get(normalizeName(r.name)) ?? 'Other';
        if (cat !== this.category) return false;
      }
      if (this.qtyMin !== null && r.qty < this.qtyMin) return false;
      if (this.qtyMax !== null && r.qty > this.qtyMax) return false;
      if (this.priceMin !== null && r.price < this.priceMin) return false;
      if (this.priceMax !== null && r.price > this.priceMax) return false;
      if (this.totalMin !== null && r.total < this.totalMin) return false;
      if (this.totalMax !== null && r.total > this.totalMax) return false;
    }
    return true;
  }

  async willUpdate(map: Map<PropertyKey, unknown>): Promise<void> {
    if (map.has('league') || this.prices.size === 0) {
      await this.loadPrices();
    }
  }

  protected async firstUpdated(): Promise<void> {
    if (this.prices.size === 0) await this.loadPrices();
  }

  private async loadPrices(): Promise<void> {
    try {
      const [currency, fragments, oils, incubators, fossils, resonators, deliriumOrbs, vials] = await Promise.all([
        this.stashLoader.currencyPrices(this.league as any),
        this.stashLoader.fragmentPrices(this.league as any),
        this.stashLoader.oilPrices(this.league as any),
        this.stashLoader.incubatorPrices(this.league as any),
        this.stashLoader.fossilPrices(this.league as any),
        this.stashLoader.resonatorPrices(this.league as any),
        this.stashLoader.deliriumOrbPrices(this.league as any),
        this.stashLoader.vialPrices(this.league as any),
      ]);
      this.debugData = { currency, fragments, oils, incubators, fossils, resonators, deliriumOrbs, vials } as any;
      const next = new Map<string, number>();
      const merge = (rows: Array<{ name: string; chaos_value: number | null }>) => {
        rows.forEach(r => {
          if (!r || typeof r.name !== 'string') return;
          if (typeof r.chaos_value === 'number') {
            if (!next.has(r.name)) next.set(r.name, r.chaos_value);
          }
        });
      };
      [currency, fragments, oils, incubators, fossils, resonators, deliriumOrbs, vials].forEach(merge);
      this.buildCategoryIndex({ currency, fragments, oils, incubators, fossils, resonators, deliriumOrbs, vials });
      this.prices = next;
      this.errorMessage = null;
    } catch (err: unknown) {
      this.prices = new Map();
      const msg = typeof err === 'string' ? err : err instanceof Error ? err.message : 'Failed to fetch prices';
      this.errorMessage = `${msg}`;
    }
  }

  protected render(): TemplateResult {
    const items = this.tab?.items ?? [];
    const tabIndex = this.tab?.index ?? 0;
    const groups = groupByName(items);
    const rows = Array.from(groups.values()).map(g => {
      const price = this.prices.get(g.name) ?? 0;
      const total = +(price * g.total).toFixed(1);
      return { name: g.name, qty: g.total, tab: tabIndex, price, total, sample: g.sample };
    });
    let regex: RegExp | null = null;
    this.invalidRegex = false;
    if (this.filter && this.filter.trim().length) {
      try {
        regex = new RegExp(this.filter.trim(), 'i');
      } catch (_) {
        this.invalidRegex = true;
      }
    }
    const filteredByRegex = regex ? rows.filter(r => regex!.test(r.name)) : rows;
    const filtered = filteredByRegex.filter(r => this.matchesAdvancedFilters(r));
    filtered.sort((a, b) => {
      const mul = this.sortDir === 'asc' ? 1 : -1;
      switch (this.sortBy) {
        case 'name': return a.name.localeCompare(b.name) * mul;
        case 'tab': return (a.tab - b.tab) * mul;
        case 'qty': return (a.qty - b.qty) * mul;
        case 'price': return (a.price - b.price) * mul;
        case 'total': return (a.total - b.total) * mul;
      }
    });

    const headerCols = this.aggregate ? ['Name', 'Quantity', 'Price', 'Total'] : ['Name', 'Tab', 'Quantity', 'Price', 'Total'];
    const filteredTotal = filtered.reduce((sum, r) => sum + (r.total || 0), 0);
    return html`<div class="list">
      <div class="tools">
        <sl-input size="small" placeholder="Filter (regex)" .value=${this.filter} @sl-input=${(e: any) => { this.filter = e.target.value; }}></sl-input>
        ${this.aggregate ? html`<sl-button size="small" id="filtersBtn" @click=${() => { this.filtersOpen = !this.filtersOpen; }}>Filters</sl-button>` : null}
        ${this.aggregate ? html`<div class="filtered-total">Filtered total: ${filteredTotal.toFixed(0)}c</div>` : null}
        <sl-button size="small" @click=${() => { this.viewPricesOpen = true; }}>View Prices JSON</sl-button>
      </div>
      ${this.aggregate ? html`<sl-popup .active=${this.filtersOpen} anchor="filtersBtn" placement="bottom-end" distance="8">
        <div class="filters-panel">
          <h4>Category</h4>
          <sl-select hoist .value=${this.category ?? ''} @sl-change=${(e: any) => { const v = e.target.value; this.category = v ? String(v) : null; }} placeholder="Filter by category">
            ${this.categories().map(c => html`<sl-option value=${c}>${c}</sl-option>`)}
          </sl-select>
          <div class="divider"></div>
          <h4>Quantity Range</h4>
          <div class="grid-2">
            <sl-input type="number" placeholder="No minimum" .value=${String(this.qtyMin ?? '')} @sl-input=${(e: any) => { const v = e.target.value; this.qtyMin = v === '' ? null : Number(v); }}></sl-input>
            <sl-input type="number" placeholder="No maximum" .value=${String(this.qtyMax ?? '')} @sl-input=${(e: any) => { const v = e.target.value; this.qtyMax = v === '' ? null : Number(v); }}></sl-input>
          </div>
          <div class="divider"></div>
          <h4>Item Price Range</h4>
          <div class="grid-2">
            <sl-input type="number" placeholder="No minimum" .value=${String(this.priceMin ?? '')} @sl-input=${(e: any) => { const v = e.target.value; this.priceMin = v === '' ? null : Number(v); }}></sl-input>
            <sl-input type="number" placeholder="No maximum" .value=${String(this.priceMax ?? '')} @sl-input=${(e: any) => { const v = e.target.value; this.priceMax = v === '' ? null : Number(v); }}></sl-input>
          </div>
          <div class="divider"></div>
          <h4>Total Price Range</h4>
          <div class="grid-2">
            <sl-input type="number" placeholder="No minimum" .value=${String(this.totalMin ?? '')} @sl-input=${(e: any) => { const v = e.target.value; this.totalMin = v === '' ? null : Number(v); }}></sl-input>
            <sl-input type="number" placeholder="No maximum" .value=${String(this.totalMax ?? '')} @sl-input=${(e: any) => { const v = e.target.value; this.totalMax = v === '' ? null : Number(v); }}></sl-input>
          </div>
        </div>
      </sl-popup>` : null}
      ${this.errorMessage ? html`<sl-alert variant="danger" closable @sl-after-hide=${() => (this.errorMessage = null)}>
        <sl-icon slot="icon" name="exclamation-octagon"></sl-icon>
        ${this.errorMessage}
      </sl-alert>` : null}
      ${this.invalidRegex ? html`<sl-alert variant="warning" closable @sl-after-hide=${() => (this.invalidRegex = false)}>
        <sl-icon slot="icon" name="exclamation-triangle"></sl-icon>
        Invalid regex: ${this.filter}
      </sl-alert>` : null}
      ${this.renderHeader(headerCols)}
      ${filtered.map(r => html`<div class="row ${this.aggregate ? 'agg' : ''}">
        <div class="name">
          <poe-item .item=${normalizeItem(r.sample)}></poe-item>
          <span>${r.name}</span>
        </div>
        ${this.aggregate ? null : html`<div class="tab">${r.tab}</div>`}
        <div class="qty">${r.qty}</div>
        <div class="price">${r.price ? `${r.price.toFixed(0)}c` : '-'}</div>
        <div class="total">${r.total ? `${r.total.toFixed(0)}c` : '-'}</div>
      </div>`)}
    </div>
    <sl-dialog label="Prices JSON" .open=${this.viewPricesOpen} @sl-hide=${() => { this.viewPricesOpen = false; }} style="--width: 800px;">
      <e-json-viewer .data=${this.debugData}></e-json-viewer>
      <sl-button slot="footer" variant="primary" @click=${() => { this.viewPricesOpen = false; }}>Close</sl-button>
    </sl-dialog>`;
  }

  private renderHeader(cols: string[]): TemplateResult {
    const keys: Record<string, PoeGeneralPricedListElement['sortBy']> = {
      Name: 'name', Tab: 'tab', Quantity: 'qty', Price: 'price', Total: 'total'
    };
    const numeric = new Set(['Quantity', 'Price', 'Total']);
    return html`<div class="header ${this.aggregate ? 'agg' : ''}">
      ${cols.map(c => html`<button class="th ${numeric.has(c) ? 'numeric' : ''}" @click=${() => this.onSort(keys[c])}>${c}${this.sortBy === keys[c] ? (this.sortDir === 'asc' ? ' ▲' : ' ▼') : ''}</button>`)}
    </div>`;
  }

  private onSort(col: PoeGeneralPricedListElement['sortBy']) {
    if (this.sortBy === col) {
      this.sortDir = this.sortDir === 'asc' ? 'desc' : 'asc';
    } else {
      this.sortBy = col;
      this.sortDir = 'asc';
    }
    this.requestUpdate();
  }

  static styles: CSSResult = css`
    :host { display: block; width: 100%; height: auto; }
    .list { width: 100%; padding: 8px; display: grid; grid-auto-rows: min-content; row-gap: 6px; overflow: auto; }
    .tools { display: flex; justify-content: flex-end; gap: 8px; padding-bottom: 6px; align-items: center; }
    .tools sl-input { min-width: 260px; }
    .filtered-total { font-weight: 600; opacity: 0.8; }
    sl-alert { position: sticky; top: 0; z-index: 1; }
    .header, .row { display: grid; grid-template-columns: 1fr 60px 80px 80px 100px; align-items: center; column-gap: 12px; }
    .header.agg, .row.agg { grid-template-columns: 1fr 80px 80px 100px; }
    .header { font-weight: 600; position: sticky; top: 0; background: var(--sl-color-gray-50); z-index: 2; padding: 6px 0; border-bottom: 1px solid var(--sl-color-gray-200); }
    .header .th { text-align: left; background: transparent; border: none; color: inherit; cursor: pointer; padding: 4px 0; }
    .header .th.numeric { text-align: right; }
    .name { display: flex; align-items: center; gap: 8px; }
    poe-item { --cell-size: 32px; --poe-item-size: 32px; --stack-size-font-size: 10px; }
    .qty { text-align: right; }
    .price, .total { text-align: right; }
    .row { border-bottom: 1px solid var(--sl-color-gray-200); padding: 6px 0; }
    .filters-panel { width: 360px; max-width: 80vw; padding: 12px; display: grid; row-gap: 10px; }
    .filters-panel h4 { margin: 0; font-size: 0.95rem; }
    .divider { height: 1px; background: var(--sl-color-gray-200); margin: 4px 0; }
    .grid-2 { display: grid; grid-template-columns: 1fr 1fr; column-gap: 8px; }
  `;
}

declare global {
  interface HTMLElementTagNameMap {
    'poe-general-priced-list': PoeGeneralPricedListElement;
  }
}

function normalizeItem(item: PoeItem): PoeItem {
  return { ...item, w: 1, h: 1, x: 0, y: 0, identified: true } as PoeItem;
}

type Group = { name: string; total: number; sample: PoeItem };

function groupByName(items: PoeItem[]): Map<string, Group> {
  const map = new Map<string, Group>();
  for (const it of items) {
    const name = it.typeLine || it.baseType || it.name;
    const qty = it.stackSize ?? 1;
    const prev = map.get(name);
    if (prev) prev.total += qty; else map.set(name, { name, total: qty, sample: it });
  }
  return map;
}

type CategorySource = {
  currency: Array<{ name: string }>,
  fragments: Array<{ name: string }>,
  oils: Array<{ name: string }>,
  incubators: Array<{ name: string }>,
  fossils: Array<{ name: string }>,
  resonators: Array<{ name: string }>,
  deliriumOrbs: Array<{ name: string }>,
  vials: Array<{ name: string }>
};

function normalizeName(n: string): string { return n.trim(); }

function categoryFromKey(k: keyof CategorySource): string {
  switch (k) {
    case 'currency': return 'Currency';
    case 'fragments': return 'Fragment';
    case 'oils': return 'Oil';
    case 'incubators': return 'Incubator';
    case 'fossils': return 'Fossil';
    case 'resonators': return 'Resonator';
    case 'deliriumOrbs': return 'Delirium Orb';
    case 'vials': return 'Vial';
    default: return 'Other';
  }
}
