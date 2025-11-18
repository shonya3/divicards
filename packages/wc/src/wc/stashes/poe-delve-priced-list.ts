import { LitElement, html, css, TemplateResult, CSSResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import type { TabWithItems, PoeItem } from 'poe-custom-elements/types.js';
import 'poe-custom-elements/item.js';
import type { IStashLoader } from '@divicards/shared/IStashLoader.js';
import '@shoelace-style/shoelace/dist/components/alert/alert.js';
import '@shoelace-style/shoelace/dist/components/icon/icon.js';
import '@shoelace-style/shoelace/dist/components/dialog/dialog.js';
import '../shared/e-json-viewer';

@customElement('poe-delve-priced-list')
export class PoeDelvePricedListElement extends LitElement {
  @property({ type: Object }) tab!: TabWithItems;
  @property() league: string = 'Standard';
  @property({ type: Object }) prices: Map<string, number> = new Map();
  @property() sortBy: 'name' | 'tab' | 'qty' | 'price' | 'total' = 'name';
  @property() sortDir: 'asc' | 'desc' = 'asc';
  @property({ attribute: false }) stashLoader!: IStashLoader;
  @property() errorMessage: string | null = null;
  @property({ type: Boolean }) viewPricesOpen: boolean = false;
  @property({ attribute: false }) debugData: Record<string, any[]> = {};

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
      const [fossils, resonators] = await Promise.all([
        this.stashLoader.fossilPrices(this.league as any),
        this.stashLoader.resonatorPrices(this.league as any),
      ]);
      this.debugData = { fossils, resonators } as any;
      const next = new Map<string, number>();
      const merge = (rows: Array<{ name: string; chaos_value: number | null }>) => {
        rows.forEach(r => {
          if (!r || typeof r.name !== 'string') return;
          if (typeof r.chaos_value === 'number') {
            if (!next.has(r.name)) next.set(r.name, r.chaos_value);
          }
        });
      };
      [fossils, resonators].forEach(merge);
      this.prices = next;
      this.errorMessage = null;
    } catch (err: unknown) {
      this.prices = new Map();
      const msg = typeof err === 'string' ? err : err instanceof Error ? err.message : 'Failed to fetch delve prices';
      this.errorMessage = `${msg}`;
    }
  }

  protected render(): TemplateResult {
    const items = (this.tab?.items ?? [])
      .filter(i => /Fossil|Resonator/i.test(i.baseType ?? i.typeLine ?? i.name ?? ''));
    const tabIndex = this.tab?.index ?? 0;
    const groups = groupByName(items);
    const rows = Array.from(groups.values()).map(g => {
      const price = this.prices.get(g.name) ?? 0;
      const total = +(price * g.total).toFixed(1);
      return { name: g.name, qty: g.total, tab: tabIndex, price, total, sample: g.sample };
    });
    rows.sort((a, b) => {
      const mul = this.sortDir === 'asc' ? 1 : -1;
      switch (this.sortBy) {
        case 'name': return a.name.localeCompare(b.name) * mul;
        case 'tab': return (a.tab - b.tab) * mul;
        case 'qty': return (a.qty - b.qty) * mul;
        case 'price': return (a.price - b.price) * mul;
        case 'total': return (a.total - b.total) * mul;
      }
    });

    return html`<div class="list">
      <div class="tools">
        <sl-button size="small" @click=${() => { this.viewPricesOpen = true; }}>View Prices JSON</sl-button>
      </div>
      ${this.errorMessage ? html`<sl-alert variant="danger" closable @sl-after-hide=${() => (this.errorMessage = null)}>
        <sl-icon slot="icon" name="exclamation-octagon"></sl-icon>
        ${this.errorMessage}
      </sl-alert>` : null}
      ${this.renderHeader(['Name', 'Tab', 'Quantity', 'Price', 'Total'])}
      ${rows.map(r => html`<div class="row">
        <div class="name">
          <poe-item .item=${normalizeItem(r.sample)}></poe-item>
          <span>${r.name}</span>
        </div>
        <div>${r.tab}</div>
        <div class="qty">${r.qty}</div>
        <div>${r.price ? `${r.price.toFixed(0)}c` : '-'}</div>
        <div>${r.total ? `${r.total.toFixed(0)}c` : '-'}</div>
      </div>`)}
    </div>
    <sl-dialog label="Delve Prices JSON" .open=${this.viewPricesOpen} @sl-hide=${() => { this.viewPricesOpen = false; }} style="--width: 800px;">
      <e-json-viewer .data=${this.debugData}></e-json-viewer>
      <sl-button slot="footer" variant="primary" @click=${() => { this.viewPricesOpen = false; }}>Close</sl-button>
    </sl-dialog>`;
  }

  private renderHeader(cols: string[]): TemplateResult {
    const keys: Record<string, PoeDelvePricedListElement['sortBy']> = {
      Name: 'name', Tab: 'tab', Quantity: 'qty', Price: 'price', Total: 'total'
    };
    return html`<div class="header">
      ${cols.map(c => html`<button class="th" @click=${() => this.onSort(keys[c])}>${c}${this.sortBy === keys[c] ? (this.sortDir === 'asc' ? ' ▲' : ' ▼') : ''}</button>`)}
    </div>`;
  }

  private onSort(col: PoeDelvePricedListElement['sortBy']) {
    if (this.sortBy === col) {
      this.sortDir = this.sortDir === 'asc' ? 'desc' : 'asc';
    } else {
      this.sortBy = col;
      this.sortDir = 'asc';
    }
    this.requestUpdate();
  }

  static styles: CSSResult = css`
    :host { display: block; width: var(--size, 569px); height: var(--size, 569px); }
    .list { width: 100%; height: 100%; padding: 8px; display: grid; grid-auto-rows: min-content; row-gap: 6px; overflow: auto; }
    .tools { display: flex; justify-content: flex-end; padding-bottom: 6px; }
    sl-alert { position: sticky; top: 0; z-index: 1; }
    .header, .row { display: grid; grid-template-columns: 1fr 50px 80px 80px 100px; align-items: center; column-gap: 12px; }
    .header { font-weight: 600; position: sticky; top: 0; background: var(--sl-color-gray-50); z-index: 1; padding: 6px 0; border-bottom: 1px solid var(--sl-color-gray-200); }
    .header .th { text-align: left; background: transparent; border: none; color: inherit; cursor: pointer; padding: 4px 0; }
    .name { display: flex; align-items: center; gap: 8px; }
    poe-item { --cell-size: 32px; --poe-item-size: 32px; --stack-size-font-size: 10px; }
    .qty { text-align: right; }
    .row { border-bottom: 1px solid var(--sl-color-gray-200); padding: 6px 0; }
  `;
}

declare global {
  interface HTMLElementTagNameMap {
    'poe-delve-priced-list': PoeDelvePricedListElement;
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
