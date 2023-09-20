import { html, css, PropertyValues } from 'lit';
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

export type CardSize = 'medium' | 'large';

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
	@property({ reflect: true }) size: CardSize = 'large';

	@state() stackSize: number = 0;
	@state()
	flavourText: string = ``;
	@state()
	artFilename: string = '';
	@state() rewardHtml: string = '';

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
			'padding-top': `${this.size === 'medium' ? '0rem' : '0.4rem'}`,
		});

		return html`<div id="element" style=${sizeMap}>
			<div id="skeleton" style=${sizeMap}></div>
			<header id="name" style=${nameTopPadding}>${this.name}</header>
			<div id="imageWrapper">
				<img id="image" width="100%" src="/images/cards/${this.artFilename}.png" alt="" />
			</div>
			<div id="stackSize">${this.stackSize}</div>
			<div id="bottom-half">
				${staticHtml`${unsafeStatic(this.rewardHtml)}`}
				<div id="divider"></div>
				<footer>
					<div id="flavourText">${this.flavourText}</div>
				</footer>
			</div>
		</div>`;
	}
}

function styles() {
	return css`
		:host {
			display: block;

			--card-width-large: 326px;
			--card-height-large: 501px;

			--card-width-medium: 268px;
			--card-height-medium: 401px;

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

		* {
			margin: 0;
			padding: 0;
		}
		@font-face {
			font-family: FontinSmallCaps;
			src: url(/fontin.otf) format('opentype');
		}
		#element {
			font-family: FontInSmallCaps;
			width: 10vw;
			min-height: 25vh;
			height: 401px;

			width: var(--card-width, var(--card-width-medium));
			height: var(--card-height, var(--card-height-medium));

			padding: 0.15rem;

			text-align: center;
			overflow: hidden;

			display: flex;
			flex-direction: column;

			position: relative;
			text-transform: uppercase;
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
			padding-top: 0.4rem;
			padding-bottom: 0.6rem;
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

		#stackSize {
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

			line-height: 1.3rem;
		}

		.reward {
			display: flex;
			flex-direction: column;
			align-items: center;
			justify-content: center;
		}

		#flavourText {
			color: rgba(167, 90, 27, 1);
			font-style: italic;
			text-wrap: balance;
		}

		#divider {
			height: 1px;
			width: 50%;
			transform: translateX(50%);

			background-image: linear-gradient(to right, transparent, rgba(255, 255, 255, 0.5), transparent);
		}

		footer {
		}
	`;
}
