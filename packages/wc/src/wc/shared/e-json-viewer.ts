import { LitElement, html, css, TemplateResult, CSSResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import '@shoelace-style/shoelace/dist/components/details/details.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';
import '@shoelace-style/shoelace/dist/components/input/input.js';
import '@shoelace-style/shoelace/dist/components/copy-button/copy-button.js';

@customElement('e-json-viewer')
export class EJsonViewer extends LitElement {
  @property({ attribute: false }) data: unknown = null;
  @property({ type: Boolean }) expandAll: boolean = false;
  @property() filter: string = '';

  protected render(): TemplateResult {
    const json = this.data ?? null;
    return html`<div class="tools">
      <sl-input placeholder="Filter keys" size="small" @sl-input=${(e: any) => { this.filter = e.target.value ?? ''; }}></sl-input>
      <sl-button size="small" @click=${() => { this.expandAll = true; }}>Expand all</sl-button>
      <sl-button size="small" @click=${() => { this.expandAll = false; }}>Collapse all</sl-button>
      <sl-copy-button .value=${JSON.stringify(json, null, 2)}></sl-copy-button>
    </div>
    <div class="tree">${this.renderNode('root', json)}</div>`;
  }

  private renderNode(key: string, value: any): TemplateResult {
    const match = (k: string) => {
      if (!this.filter) return true;
      return k.toLowerCase().includes(this.filter.toLowerCase());
    };

    if (Array.isArray(value)) {
      return html`<sl-details summary="${key} [${value.length}]" .open=${this.expandAll}>
        ${value.map((v, i) => this.renderNode(String(i), v))}
      </sl-details>`;
    }
    if (value && typeof value === 'object') {
      const entries = Object.entries(value as Record<string, unknown>);
      return html`<sl-details summary="${key}" .open=${this.expandAll}>
        ${entries
          .filter(([k]) => match(k))
          .map(([k, v]) => html`<div class="kv">${this.renderNode(k, v)}</div>`)}
      </sl-details>`;
    }
    return html`<div class="leaf"><span class="k">${key}:</span> <code class="v">${formatPrimitive(value)}</code></div>`;
  }

  static styles: CSSResult = css`
    :host { display: block; width: 100%; }
    .tools { display: flex; gap: 8px; align-items: center; padding-bottom: 8px; position: sticky; top: 0; background: var(--sl-color-gray-50); z-index: 1; }
    .tree { display: grid; grid-auto-rows: min-content; row-gap: 6px; max-height: 60vh; overflow: auto; }
    sl-details::part(summary) { font-weight: 600; }
    .leaf { padding: 4px 0; }
    .k { color: var(--sl-color-gray-700); }
    .v { color: var(--sl-color-primary-700); }
  `;
}

declare global {
  interface HTMLElementTagNameMap {
    'e-json-viewer': EJsonViewer;
  }
}

function formatPrimitive(v: unknown): string {
  if (typeof v === 'string') return v;
  if (typeof v === 'number') return String(v);
  if (typeof v === 'boolean') return v ? 'true' : 'false';
  if (v === null || v === undefined) return 'null';
  try { return JSON.stringify(v); } catch { return String(v); }
}
