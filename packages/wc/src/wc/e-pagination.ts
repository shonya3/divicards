import { LitElement, html, css, nothing, TemplateResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import '@shoelace-style/shoelace/dist/components/button/button.js';
import '@shoelace-style/shoelace/dist/components/input/input.js';
import '@shoelace-style/shoelace/dist/components/icon-button/icon-button.js';
import { PageChangeEvent } from './events/change/page';
import { PerPageChangeEvent } from './events/change/per_page';

export interface Events {
	'change:page': PageChangeEvent;
	'change:per_page': PerPageChangeEvent;
}

@customElement('e-pagination')
export class PaginationElement extends LitElement {
	@property({ reflect: true, type: Number }) page = 1;
	@property({ reflect: true, type: Number, attribute: 'per-page' }) per_page = 10;
	/** Number of items */
	@property({ type: Number }) n: number = 0;

	render(): TemplateResult {
		const range = this.get_active_pages_range();
		return html`
			<div class="page-controls">
				<div class="buttons">
					<sl-icon-button
						aria-label="prev"
						name="chevron-left"
						?disabled=${this.page === 1}
						@click=${this.decrease_page}
						>prev</sl-icon-button
					>
					<sl-input
						class="page-input"
						.helpText=${'page'}
						id="page"
						type="number"
						.value=${String(this.page)}
						@input=${this.#handle_page_input}
						min="1"
					></sl-input>
					<sl-icon-button
						aria-label="next"
						.disabled=${this.is_last_page}
						name="chevron-right"
						@click=${this.increase_page}
						>next</sl-icon-button
					>
					<sl-icon-button
						.disabled=${this.is_last_page}
						name="chevron-double-right"
						@click=${this.move_to_last_page}
						>next</sl-icon-button
					>
					<sl-input
						aria-label="to last page"
						class="per-page-input"
						.helpText=${'per page'}
						id="per-page"
						type="number"
						min="1"
						.value=${String(this.per_page)}
						@input=${this.#handle_per_page_input}
					></sl-input>
				</div>
				${range !== null && this.n > 0 ? html` <p>${range[0]} - ${range[1]} of ${this.n}</p> ` : nothing}
			</div>
		`;
	}

	#handle_page_input(e: InputEvent): void {
		const target = e.composedPath()[0] as HTMLInputElement;
		this.#set_page_and_emit(Number(target.value));
	}
	#handle_per_page_input(e: InputEvent): void {
		const target = e.composedPath()[0] as HTMLInputElement;
		this.#set_per_page_and_emit(Number(target.value));
	}

	#set_page_and_emit(page: number): void {
		this.page = page;
		this.dispatchEvent(new PageChangeEvent(page));
	}
	#set_per_page_and_emit(per_page: number): void {
		this.per_page = per_page;
		this.dispatchEvent(new PerPageChangeEvent(per_page));
	}

	decrease_page(): void {
		if (this.page <= 1) {
			return;
		}

		this.#set_page_and_emit(this.page - 1);
	}
	increase_page(): void {
		this.#set_page_and_emit(this.page + 1);
	}
	get_last_page(): number {
		return Math.ceil(this.n / this.per_page);
	}
	move_to_last_page(): void {
		this.#set_page_and_emit(this.get_last_page());
	}
	get_active_pages_range(): [number, number] | null {
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
	get is_last_page(): boolean {
		return this.page === this.get_last_page();
	}

	static styles = css`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}

		.page-controls {
			display: flex;
			align-items: center;
			flex-wrap: wrap;
			@media (width >= 640px) {
				gap: 1rem;
			}
		}

		.buttons {
			display: flex;
			gap: 0.4rem;
			align-items: center;
		}

		sl-icon-button {
			font-size: 1.2rem;
		}

		.per-page-input,
		.page-input {
			margin-top: 1.1rem;
			width: 10ch;
		}
	`;
}

declare global {
	interface HTMLElementTagNameMap {
		'e-pagination': PaginationElement;
	}
}
