import { html, css, PropertyValues, PropertyValueMap } from 'lit';
import { html as staticHtml, unsafeStatic } from 'lit/static-html.js';
import { BaseElement } from '../base-element';
import { property, state, query } from 'lit/decorators.js';
import { classMap } from 'lit/directives/class-map.js';
import { Column, Order, TablePreferences } from '@divicards/shared/types';
import { HelpTipElement } from '../help-tip';

declare global {
	interface HTMLElementTagNameMap {
		'wc-divination-card': DivinationCardElement;
	}
}

import { cardsDataMap } from './data';

export interface Props {
	name: string;
	// artFilename: string;
	// stackSize: number;
	// flavourText: string;
}
export interface Events {}

export class DivinationCardElement extends BaseElement {
	static override get defineList() {
		return [];
	}
	static override tag = 'wc-divination-card';
	static override styles = [this.baseStyles, styles()];

	@property({ reflect: true }) name: string = '';

	@state() stackSize: number = 0;
	@state()
	flavourText: string = ``;
	@state()
	artFilename: string = '';
	@state() rewardHtml: string = '';

	protected willUpdate(changedProperties: PropertyValues<this>): void {
		if (changedProperties.has('name')) {
			// if (this.name === 'Fire of Unknown Origin') {
			// 	this.artFilename = 'FireOfUnknownOrigin';
			// }
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
		return html`<div id="element">
			<div id="skeleton"></div>
			<header id="name">${this.name}</header>
			<div id="imageWrapper">
				<img id="image" width="100%" src="/images/cards/${this.artFilename}.png" alt="" />
			</div>
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

// <div id="imageWrapper">
// 	<img id="image" width="100%" src="/images/cards/${this.artFilename}.png" alt="" />
// </div>

function styles() {
	return css`
		:host {
			display: block;

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
		#element {
			width: 10vw;
			min-height: 25vh;
			height: 401px;

			width: 326px;
			height: 501px;

			padding: 0.15rem;

			text-align: center;
			overflow: hidden;

			display: flex;
			flex-direction: column;
			background-color: hsla(var(--coolgrey-1000), 0.95);

			position: relative;
		}

		#skeleton {
			background: rgba(0, 0, 0, 0) url(/images/cards/divination-card.png) no-repeat center;
			background-size: 105%;
			z-index: 3;
			position: absolute;
			height: 401px;
			width: 100%;

			width: 326px;
			height: 501px;
		}

		#name {
			color: hsl(var(--item-divination));
			padding-block: 0.4rem;
			padding-bottom: 0.6rem;
			z-index: 4;
			color: #111;
			font-size: 1.2rem;
			font-weight: 500;
		}

		#imageWrapper {
			overflow-y: hidden;
			height: 100px;

			position: absolute;
			left: 4%;
			top: 8%;
			height: 42%;
			width: 90%;
			background: #1e1e1e;
			overflow: hidden;
		}

		#image {
			display: block;
			object-fit: contain;
			min-width: 100%;
			height: 100%;
			overflow-y: hidden;
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
