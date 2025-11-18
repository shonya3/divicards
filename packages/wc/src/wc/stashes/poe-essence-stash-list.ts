import { LitElement, html, css, TemplateResult, CSSResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import type { TabWithItems, PoeItem } from 'poe-custom-elements/types.js';
import 'poe-custom-elements/item.js';
import type { IStashLoader } from '@divicards/shared/IStashLoader.js';

@customElement('poe-essence-stash-list')
export class PoeEssenceStashListElement extends LitElement {
  @property({ type: Object }) tab!: TabWithItems;
  @property() league: string = 'Standard';
  @property({ type: Object }) prices: Map<string, number> = new Map();
  @property() sortBy: 'name' | 'variant' | 'tab' | 'qty' | 'price' | 'total' = 'name';
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
      const rows = await this.stashLoader.essencePrices(this.league as any);
      const next = new Map<string, number>();
      rows.forEach(r => {
        const key = essenceKey(r.name, r.variant ?? undefined);
        if (typeof r.chaos_value === 'number') next.set(key, r.chaos_value);
      });
      this.prices = next;
      if (this.prices.size === 0) {
        const fbRows = await this.stashLoader.essencePrices('Standard' as any);
        const fb = new Map<string, number>();
        fbRows.forEach(r => {
          const key = essenceKey(r.name, r.variant ?? undefined);
          if (typeof r.chaos_value === 'number') fb.set(key, r.chaos_value);
        });
        if (fb.size > 0) this.prices = fb;
      }
    } catch {
      try {
        const rows = await this.stashLoader.essencePrices('Standard' as any);
        const next = new Map<string, number>();
        rows.forEach(r => {
          const key = essenceKey(r.name, r.variant ?? undefined);
          if (typeof r.chaos_value === 'number') next.set(key, r.chaos_value);
        });
        this.prices = next;
      } catch {}
    }
  }

  protected render(): TemplateResult {
    const items = this.tab?.items ?? [];
    const tabIndex = this.tab?.index ?? 0;
    const groups = groupEssences(items);
    const rows = Array.from(groups.values()).map(g => {
      const k = essenceKey(g.base, g.variant);
      const price = this.prices.get(k) ?? 0;
      const total = +(price * g.total).toFixed(1);
      return { name: g.base, variant: g.variant, qty: g.total, tab: tabIndex, price, total, sample: g.sample };
    });
    rows.sort((a, b) => {
      const mul = this.sortDir === 'asc' ? 1 : -1;
      switch (this.sortBy) {
        case 'name': return a.name.localeCompare(b.name) * mul;
        case 'variant': return (a.variant || '').localeCompare(b.variant || '') * mul;
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
        <div class="quality">${r.variant ?? '-'}</div>
        <div>${r.tab}</div>
        <div class="qty">${r.qty}</div>
        <div>${r.price ? `${r.price.toFixed(0)}c` : '-'}</div>
        <div>${r.total ? `${r.total.toFixed(0)}c` : '-'}</div>
      </div>`)}
    </div>`;
  }

  private renderHeader(cols: string[]): TemplateResult {
    const keys: Record<string, PoeEssenceStashListElement['sortBy']> = {
      Name: 'name', Tier: 'variant', Tab: 'tab', Quantity: 'qty', Price: 'price', Total: 'total'
    };
    return html`<div class="header">
      ${cols.map(c => html`<button class="th" @click=${() => this.onSort(keys[c])}>${c}${this.sortBy === keys[c] ? (this.sortDir === 'asc' ? ' ▲' : ' ▼') : ''}</button>`)}
    </div>`;
  }

  private onSort(col: PoeEssenceStashListElement['sortBy']) {
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
    .header, .row { display: grid; grid-template-columns: 1fr 80px 50px 80px 80px 100px; align-items: center; column-gap: 12px; }
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
    'poe-essence-stash-list': PoeEssenceStashListElement;
  }
}

function normalizeItem(item: PoeItem): PoeItem {
  return { ...item, w: 1, h: 1, x: 0, y: 0, identified: true } as PoeItem;
}

type Group = { base: string; variant?: string; total: number; sample: PoeItem };

function essenceKey(base: string, variant?: string): string { return `${base}__${variant ?? ''}`; }

function parseEssenceName(typeLine: string | undefined): { base: string; variant?: string } {
  const s = String(typeLine || '');
  const m = s.match(/^(\w+)\s+Essence\s+of\s+(.+)/);
  if (m) {
    const variant = m[1];
    const base = `Essence of ${m[2]}`;
    return { base, variant };
  }
  const n = s.includes('Essence of ') ? s.substring(s.indexOf('Essence of ')) : s;
  return { base: n };
}

function groupEssences(items: PoeItem[]): Map<string, Group> {
  const map = new Map<string, Group>();
  for (const it of items) {
    const { base, variant } = parseEssenceName(it.typeLine || it.baseType || it.name);
    const key = essenceKey(base, variant);
    const qty = it.stackSize ?? 1;
    const prev = map.get(key);
    if (prev) prev.total += qty; else map.set(key, { base, variant, total: qty, sample: it });
  }
  return map;
}
