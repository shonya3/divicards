import { LitElement, html, nothing, TemplateResult, CSSResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';
import '@shoelace-style/shoelace/dist/components/input/input.js';
import '@shoelace-style/shoelace/dist/components/icon-button/icon-button.js';
import { PageChangeEvent } from '../events/change/page.js';
import { PerPageChangeEvent } from '../events/change/per_page.js';
import { styles } from './e-pagination.styles.js';

export type Events = [PageChangeEvent, PerPageChangeEvent];

export type Props = {
	page?: number;
	per_page?: number;
	n: number;
	size?: Size;
};

@customElement('e-pagination')
export class PaginationElement extends LitElement {
	static styles: Array<CSSResult> = [styles];

	@property({ reflect: true, type: Number }) page = 1;
	@property({ reflect: true, type: Number }) per_page = 10;
	@property({ type: Number }) n: number = 0;
	@property() size: Size = 'small';

	protected render(): TemplateResult {
		const range = this.active_items_range();
		return html`
			<div class="page-controls">
				<div class="buttons">
					<sl-icon-button
						label="previous"
						name="chevron-left"
						?disabled=${this.page === 1}
						@click=${this.#h_prev_click}
					></sl-icon-button>
					<sl-input
						.size=${this.size}
						class="page-input"
						id="page"
						type="number"
						.value=${String(this.page)}
						@input=${this.#h_page_input}
						min="1"
						label="page"
					></sl-input>
					<sl-icon-button
						label="next"
						.disabled=${this.is_last_page}
						name="chevron-right"
						@click=${this.#h_next_click}
						role="button"
					></sl-icon-button>
					<sl-icon-button
						label="last page"
						.disabled=${this.is_last_page}
						name="chevron-double-right"
						@click=${this.to_last_page}
					></sl-icon-button>
					<sl-input
						.size=${this.size}
						label="per page"
						class="per_page-input"
						id="per_page"
						type="number"
						min="1"
						.value=${String(this.per_page)}
						@input=${this.#h_per_page_input}
					></sl-input>
				</div>
				<span class="current-items-label"
					>${range !== null && this.n > 0
						? html` <p>${range[0]}-${range[1]} of ${this.n}</p> `
						: nothing}</span
				>
			</div>
		`;
	}

	#h_page_input(e: InputEvent) {
		const target = e.composedPath()[0] as HTMLInputElement;
		this.page = Number(target.value);
		this.dispatchEvent(new PageChangeEvent(this.page));
	}
	#h_per_page_input(e: InputEvent) {
		const target = e.composedPath()[0] as HTMLInputElement;
		this.per_page = Number(target.value);

		this.dispatchEvent(new PerPageChangeEvent(this.per_page));
	}
	#h_next_click() {
		this.increase_page();
		this.dispatchEvent(new PageChangeEvent(this.page));
	}
	#h_prev_click() {
		this.decrease_page();
		this.dispatchEvent(new PageChangeEvent(this.page));
	}

	increase_page(): void {
		this.page++;
	}
	decrease_page(): void {
		this.page > 1 && this.page--;
	}

	last_page_number(): number {
		return Math.ceil(this.n / this.per_page);
	}
	to_last_page(): void {
		this.page = this.last_page_number();
	}
	get is_last_page(): boolean {
		return this.page === this.last_page_number();
	}

	active_items_range(): [number, number] | null {
		const start = (this.page - 1) * this.per_page;
		let end = start + this.per_page;
		if (start + 1 <= 0 || end <= 0) {
			return null;
		}
		if (end > this.n) {
			end = this.n;
		}
		return [start + 1, end];
	}
}

export type Size = 'small' | 'medium' | 'large';

declare global {
	interface HTMLElementTagNameMap {
		'e-pagination': PaginationElement;
	}
}
