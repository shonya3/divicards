import { LitElement, html, css, TemplateResult, CSSResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import type { TabWithItems, PoeItem, NoItemsTab } from 'poe-custom-elements/types.js';
import 'poe-custom-elements/item.js';
import type { IStashLoader } from '@divicards/shared/IStashLoader.js';

@customElement('poe-map-stash-list')
export class PoeMapStashListElement extends LitElement {
  @property({ type: Object }) tab!: TabWithItems;
  @property() league: string = 'Standard';
  @property({ type: Object }) prices: Map<string, number> = new Map();
  @property() sortBy: 'name' | 'tier' | 'tab' | 'qty' | 'price' | 'total' = 'name';
  @property() sortDir: 'asc' | 'desc' = 'asc';
  @property({ attribute: false }) stashLoader!: IStashLoader;

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
      const rows = await this.stashLoader.mapPrices(this.league as any);
      const next = new Map<string, number>();
      rows.forEach((r: { name: string; tier: number; chaos_value: number | null }) => {
        const key = `${r.name}__${r.tier}`;
        if (typeof r.chaos_value === 'number') next.set(key, r.chaos_value);
      });
      this.prices = next;
      if (this.prices.size === 0) {
        const fallbackRows = await this.stashLoader.mapPrices('Standard' as any);
        const fb = new Map<string, number>();
        fallbackRows.forEach((r: { name: string; tier: number; chaos_value: number | null }) => {
          const key = `${r.name}__${r.tier}`;
          if (typeof r.chaos_value === 'number') fb.set(key, r.chaos_value);
        });
        if (fb.size > 0) this.prices = fb;
      }
    } catch (err) {
      try {
        const rows = await this.stashLoader.mapPrices('Standard' as any);
        const next = new Map<string, number>();
        rows.forEach((r: { name: string; tier: number; chaos_value: number | null }) => {
          const key = `${r.name}__${r.tier}`;
          if (typeof r.chaos_value === 'number') next.set(key, r.chaos_value);
        });
        this.prices = next;
      } catch {}
    }
  }

  protected render(): TemplateResult {
    const items = this.tab?.items ?? [];
    const tabIndex = this.tab?.index ?? 0;
    if (items.length > 0) {
      const groups = groupByNameTier(items);
      const rows = Array.from(groups.values()).map(g => {
        const key = `${g.name}__${g.tier}`;
        const price = this.prices.get(key) ?? 0;
        const total = +(price * g.total).toFixed(1);
        return { name: g.name, tier: g.tier, qty: g.total, tab: tabIndex, price, total, sample: g.sample };
      });
      rows.sort((a, b) => {
        const mul = this.sortDir === 'asc' ? 1 : -1;
        switch (this.sortBy) {
          case 'name': return a.name.localeCompare(b.name) * mul;
          case 'tier': return (a.tier - b.tier) * mul;
          case 'tab': return (a.tab - b.tab) * mul;
          case 'qty': return (a.qty - b.qty) * mul;
          case 'price': return (a.price - b.price) * mul;
          case 'total': return (a.total - b.total) * mul;
        }
      });

      return html`<div class="list">
        ${this.renderHeader(['Name', 'Tier', 'Tab', 'Quantity', 'Price', 'Total'])}
        ${rows.map(r => html`<div class="row">
          <div class="name">
            <poe-item .item=${normalizeItem(r.sample)}></poe-item>
            <span>${r.name}</span>
          </div>
          <div class="quality">${r.tier}</div>
          <div>${r.tab}</div>
          <div class="qty">${r.qty}</div>
          <div>${r.price ? `${r.price.toFixed(0)}c` : '-'}</div>
          <div>${r.total ? `${r.total.toFixed(0)}c` : '-'}</div>
        </div>`)}
      </div>`;
    }

    const childRows = renderFromChildren(this.tab?.children ?? [], tabIndex, this.prices, this.sortBy, this.sortDir);
    return html`<div class="list">
      ${this.renderHeader(['Name', 'Tier', 'Tab', 'Quantity', 'Price', 'Total'])}
      ${childRows}
    </div>`;
  }

  private renderHeader(cols: string[]): TemplateResult {
    const keys: Record<string, PoeMapStashListElement['sortBy']> = {
      Name: 'name', Tier: 'tier', Tab: 'tab', Quantity: 'qty', Price: 'price', Total: 'total'
    };
    return html`<div class="header">
      ${cols.map(c => html`<button class="th" @click=${() => this.onSort(keys[c])}>${c}${this.sortBy === keys[c] ? (this.sortDir === 'asc' ? ' ▲' : ' ▼') : ''}</button>`)}
    </div>`;
  }

  private onSort(col: PoeMapStashListElement['sortBy']) {
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
    .header, .row { display: grid; grid-template-columns: 1fr 70px 50px 80px 80px 100px; align-items: center; column-gap: 12px; }
    .header { font-weight: 600; position: sticky; top: 0; background: var(--sl-color-gray-50); z-index: 1; padding: 6px 0; border-bottom: 1px solid var(--sl-color-gray-200); }
    .header .th { text-align: left; background: transparent; border: none; color: inherit; cursor: pointer; padding: 4px 0; }
    .name { display: flex; align-items: center; gap: 8px; }
    .name img.icon { width: 24px; height: 24px; object-fit: contain; }
    poe-item { --cell-size: 32px; --poe-item-size: 32px; --stack-size-font-size: 10px; }
    .quality, .qty { text-align: right; }
    .row { border-bottom: 1px solid var(--sl-color-gray-200); padding: 6px 0; }
  `;
}

declare global {
  interface HTMLElementTagNameMap {
    'poe-map-stash-list': PoeMapStashListElement;
  }
}

function normalizeItem(item: PoeItem): PoeItem {
  return { ...item, w: 1, h: 1, x: 0, y: 0, identified: true };
}

type GroupTier = { name: string; tier: number; total: number; sample: PoeItem };


function groupByNameTier(items: PoeItem[]): Map<string, GroupTier> {
  const map = new Map<string, GroupTier>();
  for (const it of items) {
    const name = it.typeLine || it.baseType || it.name;
    const tier = getMapTier(it);
    const key = `${name}__${tier}`;
    const qty = it.stackSize ?? 1;
    const prev = map.get(key);
    if (prev) {
      prev.total += qty;
    } else {
      map.set(key, { name, tier, total: qty, sample: it });
    }
  }
  return map;
}

function getMapTier(item: PoeItem): number {
  const p = item.properties || [];
  for (const prop of p) {
    if (prop.name === 'Map Tier' && prop.values?.[0]?.[0]) {
      const v = String(prop.values[0][0]);
      const m = v.match(/(\d+)/);
      if (m) return parseInt(m[1], 10);
    }
  }
  return 0;
}


function renderFromChildren(children: NoItemsTab[], tabIndex: number, prices: Map<string, number>, sortBy: PoeMapStashListElement['sortBy'], sortDir: PoeMapStashListElement['sortDir']): TemplateResult[] {
  const rows = children
    .filter(c => c.metadata?.items && (c.metadata as any)?.map?.name)
    .map(c => {
      const metaMap = (c.metadata as any)?.map ?? {};
      const name = metaMap.name as string;
      const tier = (metaMap.tier ?? 0) as number;
      const qty = (c.metadata!.items ?? 0) as number;
      const img = metaMap.image as string | undefined;
      const key = `${name}__${tier}`;
      const price = prices.get(key) ?? 0;
      const total = +(price * qty).toFixed(1);
      return { name, tier, qty, img, tab: tabIndex, price, total };
    });

  rows.sort((a, b) => {
    const mul = sortDir === 'asc' ? 1 : -1;
    switch (sortBy) {
      case 'name': return a.name.localeCompare(b.name) * mul;
      case 'tier': return (a.tier - b.tier) * mul;
      case 'tab': return (a.tab - b.tab) * mul;
      case 'qty': return (a.qty - b.qty) * mul;
      case 'price': return (a.price - b.price) * mul;
      case 'total': return (a.total - b.total) * mul;
    }
  });

  return rows.map(r => html`<div class="row">
    <div class="name">
      ${r.img ? html`<img class="icon" src=${r.img} alt=${r.name} />` : null}
      <span>${r.name}</span>
    </div>
    <div class="quality">${r.tier}</div>
    <div>${r.tab}</div>
    <div class="qty">${r.qty}</div>
    <div>${r.price ? `${r.price.toFixed(0)}c` : '-'}</div>
    <div>${r.total ? `${r.total.toFixed(0)}c` : '-'}</div>
  </div>`);
}
