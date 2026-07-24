import { css, CSSResult } from 'lit';

export const styles: CSSResult = css`
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
	.necropolis,
	.necropolisItem {
		color: hsla(var(--item-necropolis));
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
`;
