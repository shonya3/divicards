import { LitElement, html, css, TemplateResult, CSSResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import type { TabWithItems } from 'poe-custom-elements/types.js';
import 'poe-custom-elements/item.js';

@customElement('poe-simple-stash-tab')
export class PoeSimpleStashTabElement extends LitElement {
  @property({ type: Object }) tab!: TabWithItems;

  protected render(): TemplateResult {
    const items = (this.tab?.items ?? [])
      .slice()
      .sort((a, b) => (b.stackSize ?? 0) - (a.stackSize ?? 0));

    return html`<div class="simple-tab">
      <div class="simple-grid">
        ${items.map(item => html`<poe-item .item=${normalizeItem(item)}></poe-item>`)}
      </div>
    </div>`;
  }

  static styles: CSSResult = css`
    :host { display: block; width: var(--size, 569px); height: var(--size, 569px); }
    .simple-tab { width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; }
    .simple-grid { display: grid; grid-template-columns: repeat(9, 58px); grid-auto-rows: 58px; gap: 4px; }
    poe-item { --cell-size: 58px; --stack-size-font-size: 12px; }
  `;
}

declare global {
  interface HTMLElementTagNameMap {
    'poe-simple-stash-tab': PoeSimpleStashTabElement;
  }
}

function normalizeItem(item: any): any {
  return { ...item, w: 1, h: 1, x: 0, y: 0, identified: true };
}

