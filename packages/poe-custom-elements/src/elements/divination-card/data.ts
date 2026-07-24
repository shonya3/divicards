import json from './cardElementData.json' with {type: 'json'};;
export const cardElementData: CardElementData[] = json;

/**
 * https://github.com/shonya3/divicards/tree/main/crates/card_element
 */
export type CardElementData = {
  slug: string;
	name: string;
	artFilename: string;
	flavourText: string;
	stackSize: number | null;
	rewardHtml: string;
	minLevel: number;
	unique: UniqueReward | null;
};

export type UniqueReward = {
	name: string;
	item_class: string;
};


export function findCardBySlug(slug: string): CardElementData | null {
	return cardElementData.find(card => card.slug === slug) ?? null;
}
