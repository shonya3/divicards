import { css, CSSResult } from 'lit';

export const styles: CSSResult = css`
	:host {
		display: block;

		--divination-card-background-url: computed;
		--source-color: hsl(240 7.3% 84%);

		--card-width-small: 168px;
		--card-width-medium: 268px;
		--card-width-large: 326px;
		--card-font-size: 1rem;
		--card-aspect-ratio: 0.668329;

		--font-small: 0.8rem;

		--flavour-text-color: rgba(167, 90, 27, 1);
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

		width: fit-content;
		--padding-inline: 0;
		--padding-block: 0;

		display: block;
	}

	* {
		margin: 0;
		padding: 0;
	}

	img {
		color: white;
		line-height: 100px;
		text-transform: none !important;
		font-size: 10px;
	}

	.element {
		padding-inline: var(--padding-inline);
		padding-block: var(--padding-block);
	}

	.min-level {
		position: absolute;
		z-index: 4;
		bottom: 0;
		right: 0.75rem;
		font-size: var(--digits-font-size);
		color: #eee;
	}

	.boss {
		position: absolute;
		z-index: 4;
		bottom: 0;
		left: 0rem;
		font-size: var(--digits-font-size);
	}

	.divination-card {
		font-family: 'fontin', Verdana, Arial;
		border-radius: 0.3rem;

		width: var(--card-width, var(--card-width-medium));
		aspect-ratio: var(--card-aspect-ratio);

		text-align: center;
		overflow: hidden;

		display: flex;
		flex-direction: column;

		position: relative;
	}

	.divination-card--50 {
		--card-width: 50px;
		--reward-font-size: 3.2px;
		--digits-font-size: 3.2px;
		--flavour-font-size: 3.2px;
		--flavour-line-height: 3.2px;
		--name-font-size: 5px;
		--name-line-height: 5px;
		--stack-size-font-size: 8px;
		--stack-size-top: 34%;
		--bottom-half-margin-top: 0;
	}

	.divination-card--75 {
		--card-width: 75px;
		--name-font-size: 7px;
		--name-line-height: 6.5px;
		--reward-font-size: 7px;
		--flavour-font-size: 5px;
		--flavour-line-height: 4.5px;
		--stack-size-font-size: 10px;
		--stack-size-top: 38%;
		--bottom-half-margin-top: 0;
		--digits-font-size: 3.2px;
	}

	.divination-card--small {
		--card-width: var(--card-width-small);
		--name-font-size: 12px;
		--name-line-height: 15px;
		--reward-font-size: 0.8rem;
		--digits-font-size: 0.6rem;
		--flavour-font-size: 0.7rem;
		--flavour-line-height: 0.7rem;
	}

	.divination-card--medium {
		--reward-font-size: 1rem;
		--card-width: var(--card-width-medium);
		--name-font-size: 18px;
		--digits-font-size: ;
	}

	.divination-card--large {
		--reward-font-size: 1.2rem;
		--reward-line-height: 1.15rem;
		--reward-letter-spacing: -0.4px;
		--card-width: var(--card-width-large);
		--name-font-size: 24px;
		--name-line-height: 17px;
	}

	.skeleton {
		background: rgba(0, 0, 0, 0) var(--divination-card-background-url) no-repeat center;
		background-size: 105%;
		z-index: 3;
		position: absolute;

		width: var(--card-width, var(--card-width-medium));
		aspect-ratio: var(--card-aspect-ratio);
	}

	a {
		color: #000;
		text-decoration: none;
	}
	a:hover {
		color: #083344;
		text-decoration: underline;
	}

	.name {
		line-height: var(--name-line-height, 1.5rem);
		font-size: var(--name-font-size, 1rem);
		letter-spacing: -0.6px;
		z-index: 4;
	}
	.name--large {
		margin-top: 0.4rem;
	}

	.stackSize {
		display: flex;
		align-items: center;
		justify-content: center;

		position: absolute;
		color: #c8c8c8;
		left: 8%;
		top: var(--stack-size-top, 46.8%);
		z-index: 4;
		width: 16%;
		font-size: var(--stack-size-font-size, 1rem);
		height: 26px;
	}
	.stackSize--small {
		top: 44.2%;
		font-size: 0.6rem;
	}
	.stackSize--medium {
		top: 46.3%;
	}

	.bottom-half {
		position: absolute;
		top: 52%;
		height: 44%;
		width: 90%;
		z-index: 4;
		margin: 0 6% 6%;

		margin-top: var(--bottom-half-margin-top, 0.4rem);
		display: flex;
		flex-direction: column;
		justify-content: space-evenly;

		flex: 1;
	}

	.reward {
		color: #ccc;
		font-size: var(--reward-font-size, 0.8rem);
		line-height: var(--reward-line-height);
		letter-spacing: var(--reward-letter-spacing);
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}

	.link {
		position: absolute;
		top: 30px;
		left: 0;
		width: 100%;
		height: calc(50% - 30px);
		z-index: 20;
	}

	.flavourText {
		font-size: var(--flavour-font-size, 1rem);
		line-height: var(--flavour-line-height, 1.2rem);
		color: var(--flavour-text-color);
		text-wrap: balance;
		font-style: italic;
	}

	.divider {
		height: 1px;
		width: 50%;
		transform: translateX(50%);

		background-image: linear-gradient(to right, transparent, rgba(255, 255, 255, 0.5), transparent);
	}

	::slotted(e-source) {
		--source-font-size: 0.8rem;
		margin-left: 0.8rem;
	}

	.default {
		color: #7f7f7f;
	}
	.fractured {
		color: hsla(var(--item-fractured));
	}
	.enchanted {
		color: hsla(var(--item-enchanted));
	}
	.normal,
	.normalItem {
		color: hsla(var(--item-normal));
	}
	.magic,
	.magicItem {
		color: hsla(var(--item-magic));
	}
	.rare,
	.rareItem {
		color: hsla(var(--item-rare));
	}
	.unique,
	.uniqueItem {
		color: hsla(var(--item-unique));
	}
	.gem,
	.gemItem {
		color: hsla(var(--item-gem));
	}
	.currency,
	.currencyItem {
		color: hsla(var(--item-currency));
	}
	.corrupted {
		color: hsla(var(--item-corrupted));
	}
	.divination {
		color: #0ebaff;
	}
	.augmented {
		color: var(--item-augmented);
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
