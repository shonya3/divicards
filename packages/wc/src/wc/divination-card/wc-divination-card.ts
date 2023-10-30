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
	static override tag = 'wc-divination-card';
	static override styles = styles();

	@property({ reflect: true }) name: string = '';
	@property({ reflect: true }) size: CardSize = 'small';

	@state() stackSize: number = 0;
	@state() flavourText: string = ``;
	@state() artFilename: string = '';
	@state() rewardHtml: string = '';

	get imageUrl() {
		// return `/images/cards/${this.artFilename}.png`;
		return `https://web.poecdn.com/image/divination-card/${this.artFilename}.png`;
	}

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

		return html`<div
			class=${classMap({
				'divination-card': true,
				[`divination-card--${this.size}`]: true,
			})}
		>
			<div
				class=${classMap({
					skeleton: true,
					[`skeleton--${this.size}`]: true,
				})}
				style=${sizeMap}
			></div>
			<header class="${classMap({ name: true, size22: this.size === 'small' })}" style=${nameTopPadding}>
				${this.name}
			</header>
			<div class="imageWrapper">
				<img loading="lazy" class="image" width="100%" src=${this.imageUrl} alt="" />
			</div>
			<div class=${classMap({ stackSize: true, 'stackSize--small': this.size === 'small' })}>
				${this.stackSize}
			</div>
			<div class="${classMap({ 'bottom-half': true, size25: this.size === 'small' })}">
				${staticHtml`${unsafeStatic(this.rewardHtml)}`}
				${this.size !== 'small' ? html`${this.divider()}${this.footer()}` : nothing}
			</div>
		</div>`;
	}

	protected divider() {
		return html`<div class="divider"></div>`;
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
			--card-width-medium: 268px;
			--card-width-large: 326px;
			--card-font-size: 1rem;
			--card-aspect-ratio: 0.668329;

			--font-small: 0.8rem;

			--item-normal: 0, 0%, 78%;
			--item-rare: 60, 100%, 73%;
			--item-magic: 240, 100%, 77%;
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

		.divination-card {
			font-family: 'fontin', Verdana, Arial;

			width: var(--card-width, var(--card-width-medium));
			aspect-ratio: var(--card-aspect-ratio);

			text-align: center;
			overflow: hidden;

			display: flex;
			flex-direction: column;

			position: relative;
		}

		.divination-card--small {
			--card-width: var(--card-width-small);
		}

		.divination-card--medium {
			--card-width: var(--card-width-medium);
		}

		.divination-card--large {
			--card-width: var(--card-width-large);
		}

		.skeleton {
			background: rgba(0, 0, 0, 0) url(/images/cards/divination-card.png) no-repeat center;
			background-size: 105%;
			z-index: 3;
			position: absolute;

			width: var(--card-width, var(--card-width-medium));
			aspect-ratio: var(--card-aspect-ratio);
		}

		.name {
			line-height: 24px;
			font-size: var(--card-font-size);
			z-index: 4;
			color: #111;
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

		.bottom-half {
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
			font-size: 0.8rem;
			display: flex;
			flex-direction: column;
			align-items: center;
			justify-content: center;
		}

		.flavourText {
			font-size: 1rem;
			color: rgba(167, 90, 27, 1);
			text-wrap: balance;
			font-style: italic;
			line-height: 1.2rem;
		}

		.divider {
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

		.divination {
			color: #0ebaff;
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
	`;
}
