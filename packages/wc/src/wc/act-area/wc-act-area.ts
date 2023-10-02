import { classMap } from 'lit/directives/class-map.js';
import { html, css, PropertyValues, nothing } from 'lit';
import { html as staticHtml, unsafeStatic } from 'lit/static-html.js';
import { BaseElement } from '../base-element';
import { property, state } from 'lit/decorators.js';
import { styleMap } from 'lit/directives/style-map.js';

declare global {
	interface HTMLElementTagNameMap {
		'wc-act-area': ActAreaElement;
	}
}

export type Size = 'small' | 'large';

export interface Props {
	name: string;
	size: Size;
}
export interface Events {}

export class ActAreaElement extends BaseElement {
	static override get defineList() {
		return [];
	}
	static override tag = 'wc-act-area';
	static override styles = [this.baseStyles, styles()];

	@property({ reflect: true }) name: string = 'The Vastiri Desert';
	@property({ reflect: true }) size: Size = 'small';
	@property({ type: Number, reflect: true }) act: number = 9;
	@property() img: string = '/images/acts/9_3.webp';

	@state()
	flavourText: string = ``;
	@state()
	artFilename: string = '';

	protected override render() {
		return html`<div
			style="--act-area-background-image: url(${this.img})"
			class=${classMap({
				'act-area': true,
				'act-area--small': this.size === 'small',
				'act-area--large': this.size === 'large',
			})}
		>
			<div class="name">${this.name} (Act ${this.act})</div>
			<div class="monster-level">Monster level: 61</div>
		</div>`;
	}
}

function styles() {
	return css`
		:host {
			display: inline-block;
			/* 507x98  */
			--act-area-background-image: url(/images/acts/1_1.webp);
			--act-area-name-color: #fec076;
			--act-area-width-small: 261px;
			--act-area-width-large: 507px;
			--act-area-font-size-small: 15px;
			--act-area-font-size-large: 24px;
		}

		/* 
			<img class="skeleton-img" src="/images/acts/1_1.webp"></img>
        */

		.act-area {
			width: var(--act-area-width);
			background: rgba(0, 0, 0, 0) no-repeat center;
			background-size: 100%;
			background-image: var(--act-area-background-image);
			position: absolute;
			aspect-ratio: 5.173;
		}

		.act-area--small {
			--act-area-width: var(--act-area-width-small);
			--act-area-font-size: var(--act-area-font-size-small);
			--act-area-name-top: 4px;
			--act-area-name-left: 20px;
			--act-area-monster-level-top: 24px;
			--act-area-monster-level-left: 20px;
			--act-area-monster-level-font-size: 12px;
		}

		.act-area--large {
			--act-area-width: var(--act-area-width-large);
			--act-area-font-size: var(--act-area-font-size-large);
			--act-area-name-top: 20px;
			--act-area-name-left: 44px;
			--act-area-monster-level-top: 62px;
			--act-area-monster-level-left: 44px;
			--act-area-monster-level-font-size: 20px;
		}

		img {
			width: 100%;
		}

		.name {
			position: absolute;
			font-size: var(--act-area-font-size, 20px);
			top: var(--act-area-name-top);
			left: var(--act-area-name-left);
			color: var(--act-area-name-color, red);
		}

		.monster-level {
			position: absolute;
			top: var(--act-area-monster-level-top);
			left: var(--act-area-monster-level-left);
			font-size: var(--act-area-monster-level-font-size);
		}
	`;
}
