type CardData = {
	name: string;
	artFilename: string;
	flavourText: string;
	stackSize: number | null;
	rewardHtml: string;
};

import json from './data.json' assert { type: 'json' };

const data: CardData[] = json;
export const cardsDataMap = new Map<string, CardData>();
for (const card of data) {
	cardsDataMap.set(card.name, card);
}
