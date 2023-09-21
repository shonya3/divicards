import { classMap } from 'lit/directives/class-map.js';
import { html, css, PropertyValues, nothing } from 'lit';
import { html as staticHtml, unsafeStatic } from 'lit/static-html.js';
import { BaseElement } from '../base-element';
import { property, state } from 'lit/decorators.js';
import { styleMap } from 'lit/directives/style-map.js';

declare global {
	interface HTMLElementTagNameMap {
		'wc-divination-card': DivinationCardElement;
	}
}

import { cardsDataMap } from './data';

export type CardSize = 'small' | 'medium' | 'large';

export interface Props {
	name: string;
	size: CardSize;
}
export interface Events {}

export class DivinationCardElement extends BaseElement {
	static override get defineList() {
		return [];
	}
	static override tag = 'wc-divination-card';
	static override styles = [this.baseStyles, styles()];

	@property({ reflect: true }) name: string = '';
	@property({ reflect: true }) size: CardSize = 'small';

	@state() stackSize: number = 0;
	@state()
	flavourText: string = ``;
	@state()
	artFilename: string = '';
	@state() rewardHtml: string = '';

	protected nameMarginTop(size: CardSize) {
		switch (size) {
			case 'small':
				return '0rem';
			case 'medium':
				return '0rem';
			case 'large':
				return '0.4rem';
		}
	}

	protected willUpdate(changedProperties: PropertyValues<this>): void {
		if (changedProperties.has('name')) {
			const cardData = cardsDataMap.get(this.name);
			if (cardData) {
				this.stackSize = cardData.stackSize ?? 1;
				this.flavourText = cardData.flavourText;
				this.artFilename = cardData.artFilename;
				this.rewardHtml = cardData.rewardHtml;
			}
		}
	}

	protected override render() {
		const sizeMap = styleMap({
			'--card-width': `var(--card-width-${this.size})`,
			'--card-height': `var(--card-height-${this.size})`,
		});

		const nameTopPadding = styleMap({
			'margin-top': this.nameMarginTop(this.size),
		});

		return html`<div id="element" style=${sizeMap}>
			<div id="skeleton" style=${sizeMap}></div>
			<header class="${classMap({ size22: this.size === 'small' })}" id="name" style=${nameTopPadding}>
				${this.name}
			</header>
			<div id="imageWrapper">
				<img loading="lazy" id="image" width="100%" src="/images/cards/${this.artFilename}.png" alt="" />
			</div>
			<div class=${classMap({ stackSize: true, 'stackSize--small': this.size === 'small' })}>
				${this.stackSize}
			</div>
			<div class="${classMap({ size25: this.size === 'small' })}" id="bottom-half">
				${staticHtml`${unsafeStatic(this.rewardHtml)}`}
				${this.size !== 'small' ? html`${this.divider()}${this.footer()}` : nothing}
			</div>
		</div>`;
	}

	protected divider() {
		return html`<div id="divider"></div>`;
	}

	protected footer() {
		return html`<footer>
			<p style="font-style: italic" class="flavourText">${this.flavourText}</p>
		</footer>`;
	}
}

function styles() {
	return css`
		:host {
			display: block;

			--card-width-small: 134px;
			--card-height-small: 200px;

			--card-width-medium: 268px;
			--card-height-medium: 401px;

			--card-width-large: 326px;
			--card-height-large: 501px;

			--item-normal: 0, 0%, 78%;
			--item-magic: 240, 100%, 77%;
			--item-rare: 60, 100%, 73%;
			--item-unique-contrast: 25, 63%, 48%;
			--item-unique: 26, 65%, 42%;
			--item-gem: 177, 72%, 37%;
			--item-relic: 0, 0%, 78%;
			--item-currency: 42, 19%, 59%;
			--item-prophecy: 275, 100%, 65%;
			--item-divination: 0, 0%, 50%;
			--item-keystone: 46, 52%, 74%;
			--item-explicit: 240, 100%, 77%;
			--item-implicit: var(--item-explicit);
			--item-crafted: 240, 100%, 85%;
			--item-enchanted: var(--item-crafted);
			--item-fractured: 44, 26%, 51%;
			--item-corrupted: 0, 100%, 41%;
			--item-scourge: 20, 100%, 57%;
			--item-physical: 0, 0%, 58%;
			--item-fire: 0, 100%, 29%;
			--item-cold: 210, 46%, 39%;
			--item-lightning: 51, 100%, 50%;
			--item-chaos: 322, 73%, 47%;
			--item-augmented: rgb(138, 138, 255);
			--coolgrey-1000: 206, 24%, 7%;
		}

		* {
			margin: 0;
			padding: 0;
		}

		#element {
			font-family: 'fontin', Verdana, Arial;

			width: var(--card-width, var(--card-width-medium));
			height: var(--card-height, var(--card-height-medium));

			text-align: center;
			overflow: hidden;

			display: flex;
			flex-direction: column;

			position: relative;
		}

		#skeleton {
			background: rgba(0, 0, 0, 0) url(/images/cards/divination-card.png) no-repeat center;
			background-size: 105%;
			z-index: 3;
			position: absolute;
			height: 401px;
			width: 100%;

			width: var(--card-width, var(--card-width-medium));
			height: var(--card-height, var(--card-height-medium));
		}

		#name {
			color: hsl(var(--item-divination));
			z-index: 4;
			color: #111;
		}

		#imageWrapper {
			overflow-y: hidden;
			height: 100px;

			position: absolute;
			left: 4%;
			top: 8%;
			height: 42%;
			width: 90%;
			overflow: hidden;
		}

		#image {
			display: block;
			object-fit: contain;
			min-width: 100%;
			height: 100%;
			overflow-y: hidden;
		}

		.stackSize {
			display: flex;
			align-items: center;
			justify-content: center;

			position: absolute;
			color: #c8c8c8;
			left: 8%;
			top: 46.8%;
			z-index: 4;
			width: 16%;
			font-size: 1rem;
			height: 26px;
		}

		.stackSize--small {
			top: 42.8%;
			font-size: 0.6rem;
		}

		#bottom-half {
			position: absolute;
			top: 52%;
			height: 44%;
			width: 90%;
			z-index: 4;
			margin: 0 6% 6%;

			margin-top: 0.4rem;
			display: flex;
			flex-direction: column;
			justify-content: space-evenly;

			flex: 1;
		}

		.reward {
			display: flex;
			flex-direction: column;
			align-items: center;
			justify-content: center;
		}

		.flavourText {
			color: rgba(167, 90, 27, 1);
			text-wrap: balance;
			font-style: italic;
			line-height: 1.2rem;
		}

		#divider {
			height: 1px;
			width: 50%;
			transform: translateX(50%);

			background-image: linear-gradient(to right, transparent, rgba(255, 255, 255, 0.5), transparent);
		}

		.currencyItem {
			color: hsla(var(--item-currency));
		}

		.uniqueItem {
			color: hsla(var(--item-unique));
		}

		.fractured {
			color: hsla(var(--item-fractured));
		}

		.enchanted {
			color: hsla(var(--item-enchanted));
		}

		.normal {
			color: hsla(var(--item-normal));
		}

		.default {
			color: #7f7f7f;
		}

		.magicItem {
			color: hsla(var(--item-magic));
		}

		.rareItem {
			color: hsla(var(--item-rare));
		}
		.corrupted {
			color: hsla(var(--item-corrupted));
		}
		.rare {
			color: hsla(var(--item-rare));
		}

		.augmented {
			color: var(--item-augmented);
		}

		.gemItem {
			color: hsla(var(--item-gem));
		}

		.size22 {
			font-size: 11px;
			line-height: 0.8rem;
		}

		.size25 {
			font-size: 12.5px;
			line-height: 0.9rem;
		}

		.size26 {
			font-size: 13px;
		}

		.size27 {
			font-size: 13.5px;
		}
		.size28 {
			font-size: 14px;
		}

		.size29 {
			font-size: 14.5px;
		}

		.size30 {
			font-size: 15px;
		}

		.size31 {
			font-size: 15.5px;
		}

		p {
			line-height: inherit;
		}

		p:has(.size25) {
			line-height: 0.9rem;
		}

		p:has(.size26) {
			line-height: 0.95rem;
		}

		p:has(.size27) {
			line-height: 1rem;
		}

		p:has(.size28) {
			line-height: 1.05rem;
		}

		p:has(.size29) {
			line-height: 1.1rem;
		}

		p:has(.size30) {
			line-height: 1.15rem;
		}

		footer {
		}
	`;
}
