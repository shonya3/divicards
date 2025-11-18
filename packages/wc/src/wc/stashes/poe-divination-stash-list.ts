import { LitElement, html, css, TemplateResult, CSSResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import type { TabWithItems, PoeItem } from 'poe-custom-elements/types.js';
import 'poe-custom-elements/item.js';
import type { IStashLoader } from '@divicards/shared/IStashLoader.js';
import '@shoelace-style/shoelace/dist/components/alert/alert.js';
import '@shoelace-style/shoelace/dist/components/icon/icon.js';
import '@shoelace-style/shoelace/dist/components/dialog/dialog.js';
import '../shared/e-json-viewer';

@customElement('poe-divination-stash-list')
export class PoeDivinationStashListElement extends LitElement {
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
      const cards: Array<{ name: string; chaos_value: number | null }> = await this.stashLoader.divinationCardPrices(this.league as any);
      this.debugData = { cards } as any;
      const next = new Map<string, number>();
      cards.forEach((r) => {
        if (!r || typeof r.name !== 'string') return;
        if (typeof r.chaos_value === 'number') {
          if (!next.has(r.name)) next.set(r.name, r.chaos_value);
        }
      });
      this.prices = next;
      this.errorMessage = null;
    } catch (err: unknown) {
      this.prices = new Map();
      try {
        const url = `https://poe.ninja/poe1/api/economy/stash/current/item/overview?league=${encodeURIComponent(this.league)}&type=DivinationCard`;
        const res = await fetch(url);
        if (res.ok) {
          const data = await res.json();
          const lines: Array<{ name: string; chaosValue: number }> = Array.isArray(data?.lines) ? data.lines : [];
          const next = new Map<string, number>();
          for (const v of lines) {
            if (typeof v?.name === 'string' && typeof v?.chaosValue === 'number') {
              if (!next.has(v.name)) next.set(v.name, v.chaosValue);
            }
          }
          this.prices = next;
          this.errorMessage = null;
          return;
        }
        this.errorMessage = `HTTP ${res.status} while fetching divination prices`;
      } catch (e: unknown) {
        const msg = typeof e === 'string' ? e : e instanceof Error ? e.message : 'Failed to fetch divination prices';
        this.errorMessage = `${msg}`;
      }
    }
  }

  protected render(): TemplateResult {
    const items = (this.tab?.items ?? []).filter(i => (i.frameType ?? 0) === 6);
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
    <sl-dialog label="Divination Prices JSON" .open=${this.viewPricesOpen} @sl-hide=${() => { this.viewPricesOpen = false; }} style="--width: 800px;">
      <e-json-viewer .data=${this.debugData}></e-json-viewer>
      <sl-button slot="footer" variant="primary" @click=${() => { this.viewPricesOpen = false; }}>Close</sl-button>
    </sl-dialog>`;
  }

  private renderHeader(cols: string[]): TemplateResult {
    const keys: Record<string, PoeDivinationStashListElement['sortBy']> = {
      Name: 'name', Tab: 'tab', Quantity: 'qty', Price: 'price', Total: 'total'
    };
    return html`<div class="header">
      ${cols.map(c => html`<button class="th" @click=${() => this.onSort(keys[c])}>${c}${this.sortBy === keys[c] ? (this.sortDir === 'asc' ? ' ▲' : ' ▼') : ''}</button>`)}
    </div>`;
  }

  private onSort(col: PoeDivinationStashListElement['sortBy']) {
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
    .tools { display: flex; justify-content: flex-end; padding-bottom: 6px; }
    sl-alert { position: sticky; top: 0; z-index: 1; }
    .header, .row { display: grid; grid-template-columns: 1fr 60px 80px 80px 100px; align-items: center; column-gap: 12px; }
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
    'poe-divination-stash-list': PoeDivinationStashListElement;
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
