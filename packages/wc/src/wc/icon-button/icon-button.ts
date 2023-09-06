import { html, css, svg } from 'lit';
import { property, query, state } from 'lit/decorators.js';
import { BaseElement } from '../base-element';
import { classMap } from 'lit/directives/class-map.js';
declare global {
	interface HTMLElementTagNameMap {
		'wc-icon-button': IconButtonElement;
	}
}

export class IconButtonElement extends BaseElement {
	static override tag = 'wc-icon-button';
	static override styles = [this.baseStyles, iconButtonStyles(), base()];

	@query('.icon-button') button!: HTMLButtonElement | HTMLLinkElement;

	@state() private hasFocus = false;

	/** The name of the icon to draw. Available names depend on the icon library being used. */
	@property() name?: string;

	/** The name of a registered custom icon library. */
	@property() library?: string;

	/**
	 * An external URL of an SVG file. Be sure you trust the content you are including, as it will be executed as code and
	 * can result in XSS attacks.
	 */
	@property() src?: string;

	/** When set, the underlying button will be rendered as an `<a>` with this `href` instead of a `<button>`. */
	@property() href?: string;

	/** Tells the browser where to open the link. Only used when `href` is set. */
	@property() target?: '_blank' | '_parent' | '_self' | '_top';

	/** Tells the browser to download the linked file as this filename. Only used when `href` is set. */
	@property() download?: string;

	/**
	 * A description that gets read by assistive devices. For optimal accessibility, you should always include a label
	 * that describes what the icon button does.
	 */
	@property() label = '';

	/** Disables the button. */
	@property({ type: Boolean, reflect: true }) disabled = false;

	private handleBlur() {
		this.hasFocus = false;
		this.emit('sl-blur');
	}

	private handleFocus() {
		this.hasFocus = true;
		this.emit('sl-focus');
	}

	private handleClick(event: MouseEvent) {
		if (this.disabled) {
			event.preventDefault();
			event.stopPropagation();
		}
	}

	/** Simulates a click on the icon button. */
	click() {
		this.button.click();
	}

	/** Sets focus on the icon button. */
	focus(options?: FocusOptions) {
		this.button.focus(options);
	}

	/** Removes focus from the icon button. */
	blur() {
		this.button.blur();
	}

	render() {
		/* eslint-disable lit/binding-positions, lit/no-invalid-html */
		return html`
			<button
				part="base"
				class=${classMap({
					'icon-button': true,
					'icon-button--disabled': this.disabled,
					'icon-button--focused': this.hasFocus,
				})}
				.disabled=${this.disabled}
				type="button"
				role="button"
				aria-disabled=${this.disabled ? 'true' : 'false'}
				aria-label="${this.label}"
				tabindex=${this.disabled ? '-1' : '0'}
				@blur=${this.handleBlur}
				@focus=${this.handleFocus}
				@click=${this.handleClick}
			>
				${this.icon()}
			</button>
		`;
	}

	protected icon() {
		switch (this.name) {
			case 'close': {
				return html`<svg
					xmlns="http://www.w3.org/2000/svg"
					width="16"
					height="16"
					fill="currentColor"
					class="bi bi-x-lg"
					viewBox="0 0 16 16"
				>
					<path
						d="M2.146 2.854a.5.5 0 1 1 .708-.708L8 7.293l5.146-5.147a.5.5 0 0 1 .708.708L8.707 8l5.147 5.146a.5.5 0 0 1-.708.708L8 8.707l-5.146 5.147a.5.5 0 0 1-.708-.708L7.293 8 2.146 2.854Z"
					/>
				</svg>`;
			}

			default:
				throw new Error('unsupported icon name');
		}
	}
}

function iconButtonStyles() {
	return css`
		.icon-button {
			/* Focus rings */
			--sl-focus-ring-color: rgb(105, 208, 255);
			--sl-focus-ring-style: solid;
			--sl-focus-ring-width: 3px;
			--sl-focus-ring: var(--sl-focus-ring-style) var(--sl-focus-ring-width) var(--sl-focus-ring-color);
			--sl-focus-ring-offset: 1px;

			display: inline-block;
			color: rgb(142, 142, 154);

			flex: 0 0 auto;
			display: flex;
			align-items: center;
			background: none;
			border: none;
			border-radius: var(--sl-border-radius-medium);
			font-size: inherit;
			color: inherit;
			padding: var(--sl-spacing-x-small);
			cursor: pointer;
			transition: var(--sl-transition-x-fast) color;
			-webkit-appearance: none;
		}

		.icon-button:hover:not(.icon-button--disabled),
		.icon-button:focus-visible:not(.icon-button--disabled) {
			color: rgb(39, 186, 253);
		}

		.icon-button:active:not(.icon-button--disabled) {
			color: rgb(105, 208, 255);
		}

		.icon-button:focus {
			outline: none;
		}

		.icon-button--disabled {
			opacity: 0.5;
			cursor: not-allowed;
		}

		.icon-button:focus-visible {
			outline: var(--sl-focus-ring);
			outline-offset: var(--sl-focus-ring-offset);
		}

		.icon-button__icon {
			pointer-events: none;
		}
	`;
}

function base() {
	return css`
		:host {
			box-sizing: border-box;
		}

		:host *,
		:host *::before,
		:host *::after {
			box-sizing: inherit;
		}

		[hidden] {
			display: none !important;
		}
	`;
}
