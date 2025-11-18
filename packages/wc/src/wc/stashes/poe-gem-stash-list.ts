import { LitElement, html, css, TemplateResult, CSSResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import type { TabWithItems, PoeItem } from 'poe-custom-elements/types.js';
import 'poe-custom-elements/item.js';
import type { IStashLoader } from '@divicards/shared/IStashLoader.js';
import '@shoelace-style/shoelace/dist/components/alert/alert.js';
import '@shoelace-style/shoelace/dist/components/icon/icon.js';

@customElement('poe-gem-stash-list')
export class PoeGemStashListElement extends LitElement {
  @property({ type: Object }) tab!: TabWithItems;
  @property() league: string = 'Standard';
  @property({ type: Object }) prices: Map<string, number> = new Map();
  @property() sortBy: 'name' | 'level' | 'quality' | 'tab' | 'qty' | 'price' | 'total' = 'name';
  @property() sortDir: 'asc' | 'desc' = 'asc';
  @property({ attribute: false }) stashLoader!: IStashLoader;
  @property() errorMessage: string | null = null;
  @property({ type: Boolean }) loading: boolean = false;
  @property() loadedLeague: string | null = null;

  async willUpdate(map: Map<PropertyKey, unknown>): Promise<void> {
    if (map.has('league')) {
      if (this.loadedLeague !== this.league) await this.loadPrices();
    } else if (this.prices.size === 0 && !this.loading) {
      await this.loadPrices();
    }
  }

  protected async firstUpdated(): Promise<void> {
    if (this.prices.size === 0) await this.loadPrices();
  }

  private async loadPrices(): Promise<void> {
    if (this.loading) return;
    this.loading = true;
    try {
      const rows = await this.stashLoader.gemPrices(this.league as any);
      const next = new Map<string, number>();
      rows.forEach((r: { name: string; level: number; quality: number; chaos_value: number | null }) => {
        const key = gemKey(r.name, r.level ?? 0, r.quality ?? 0);
        if (typeof r.chaos_value === 'number') next.set(key, r.chaos_value);
      });
      this.prices = next;
      this.loadedLeague = String(this.league);
      this.errorMessage = null;
    } catch (err: unknown) {
      this.prices = new Map();
      const msg = typeof err === 'string' ? err : err instanceof Error ? err.message : 'Failed to fetch gem prices';
      this.errorMessage = `${msg}`;
    } finally {
      this.loading = false;
    }
  }

  protected render(): TemplateResult {
    const items = this.tab?.items ?? [];
    const tabIndex = this.tab?.index ?? 0;
    const groups = groupByNameLevelQuality(items);
    const rows = Array.from(groups.values()).map(g => {
      const key = gemKey(g.name, g.level, g.quality);
      const price = this.prices.get(key) ?? 0;
      const total = +(price * g.total).toFixed(1);
      return { name: g.name, level: g.level, quality: g.quality, qty: g.total, tab: tabIndex, price, total, sample: g.sample };
    });
    rows.sort((a, b) => {
      const mul = this.sortDir === 'asc' ? 1 : -1;
      switch (this.sortBy) {
        case 'name': return a.name.localeCompare(b.name) * mul;
        case 'level': return (a.level - b.level) * mul;
        case 'quality': return (a.quality - b.quality) * mul;
        case 'tab': return (a.tab - b.tab) * mul;
        case 'qty': return (a.qty - b.qty) * mul;
        case 'price': return (a.price - b.price) * mul;
        case 'total': return (a.total - b.total) * mul;
      }
    });

    return html`<div class="list">
      ${this.errorMessage ? html`<sl-alert variant="danger" closable @sl-after-hide=${() => (this.errorMessage = null)}>
        <sl-icon slot="icon" name="exclamation-octagon"></sl-icon>
        ${this.errorMessage}
      </sl-alert>` : null}
      ${this.renderHeader(['Name', 'Level', 'Quality', 'Tab', 'Quantity', 'Price', 'Total'])}
      ${rows.map(r => html`<div class="row">
        <div class="name">
          <poe-item .item=${normalizeItem(r.sample)}></poe-item>
          <span>${r.name}</span>
        </div>
        <div class="quality">${r.level}</div>
        <div class="quality">${r.quality}</div>
        <div>${r.tab}</div>
        <div class="qty">${r.qty}</div>
        <div>${r.price ? `${r.price.toFixed(0)}c` : '-'}</div>
        <div>${r.total ? `${r.total.toFixed(0)}c` : '-'}</div>
      </div>`)}
    </div>`;
  }

  private renderHeader(cols: string[]): TemplateResult {
    const keys: Record<string, PoeGemStashListElement['sortBy']> = {
      Name: 'name', Level: 'level', Quality: 'quality', Tab: 'tab', Quantity: 'qty', Price: 'price', Total: 'total'
    };
    return html`<div class="header">
      ${cols.map(c => html`<button class="th" @click=${() => this.onSort(keys[c])}>${c}${this.sortBy === keys[c] ? (this.sortDir === 'asc' ? ' ▲' : ' ▼') : ''}</button>`)}
    </div>`;
  }

  private onSort(col: PoeGemStashListElement['sortBy']) {
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
    sl-alert { position: sticky; top: 0; z-index: 1; }
    .header, .row { display: grid; grid-template-columns: 1fr 60px 60px 50px 80px 80px 100px; align-items: center; column-gap: 12px; }
    .header { font-weight: 600; position: sticky; top: 0; background: var(--sl-color-gray-50); z-index: 1; padding: 6px 0; border-bottom: 1px solid var(--sl-color-gray-200); }
    .header .th { text-align: left; background: transparent; border: none; color: inherit; cursor: pointer; padding: 4px 0; }
    .name { display: flex; align-items: center; gap: 8px; }
    poe-item { --cell-size: 32px; --poe-item-size: 32px; --stack-size-font-size: 10px; }
    .quality, .qty { text-align: right; }
    .row { border-bottom: 1px solid var(--sl-color-gray-200); padding: 6px 0; }
  `;
}

declare global {
  interface HTMLElementTagNameMap {
    'poe-gem-stash-list': PoeGemStashListElement;
  }
}

function normalizeItem(item: PoeItem): PoeItem {
  return { ...item, w: 1, h: 1, x: 0, y: 0, identified: true } as PoeItem;
}

type GroupGem = { name: string; level: number; quality: number; total: number; sample: PoeItem };

function gemKey(name: string, level: number, quality: number): string { return `${name}__${level}__${quality}`; }

function getGemLevel(item: PoeItem): number {
  const p = item.properties || [];
  for (const prop of p) {
    if ((prop as any).name === 'Gem Level' || (prop as any).name === 'Level') {
      const val = Array.isArray((prop as any).values) && (prop as any).values?.[0]?.[0];
      if (val !== undefined && val !== null) {
        const v = String(val);
        const m = v.match(/(\d+)/);
        if (m) return parseInt(m[1], 10);
      }
    }
  }
  return 0;
}

function getGemQuality(item: PoeItem): number {
  const p = item.properties || [];
  for (const prop of p) {
    if ((prop as any).name === 'Quality') {
      const val = Array.isArray((prop as any).values) && (prop as any).values?.[0]?.[0];
      if (val !== undefined && val !== null) {
        const v = String(val);
        const m = v.match(/(\d+)/);
        if (m) return parseInt(m[1], 10);
      }
    }
  }
  return 0;
}

function groupByNameLevelQuality(items: PoeItem[]): Map<string, GroupGem> {
  const map = new Map<string, GroupGem>();
  for (const it of items) {
    const name = it.typeLine || it.baseType || it.name;
    const level = getGemLevel(it);
    const quality = getGemQuality(it);
    const key = gemKey(name, level, quality);
    const qty = it.stackSize ?? 1;
    const prev = map.get(key);
    if (prev) {
      prev.total += qty;
    } else {
      map.set(key, { name, level, quality, total: qty, sample: it });
    }
  }
  return map;
}
