import { PoeItem } from '../poe.types.js';
/**
 * Divination card with maxed stacksize
 */
export const maxedStackSize: PoeItem = {
	artFilename: 'TheEncroachingDarkness',
	baseType: 'The Encroaching Darkness',
	explicitMods: ['<uniqueitem>{Map}\r\n<corrupted>{Corrupted}'],
	flavourText: ['No matter where your dreams take you, Nightmare follows close behind.'],
	frameType: 6,
	h: 1,
	icon: 'https://web.poecdn.com/gen/image/WzI1LDE0LHsiZiI6IjJESXRlbXMvRGl2aW5hdGlvbi9JbnZlbnRvcnlJY29uIiwidyI6MSwiaCI6MSwic2NhbGUiOjF9XQ/f34bf8cbb5/InventoryIcon.png',
	id: 'a564f6693000f0e856f0979ff939d1b0800ab824e60e4f786b44f68005c577a9',
	identified: true,
	ilvl: 0,
	inventoryId: 'Stash1',
	league: 'Standard',
	maxStackSize: 8,
	name: '',
	properties: [
		{
			displayMode: 0,
			name: 'Stack Size',
			type: 32,
			values: [['8/8', 0]],
		},
	],
	stackSize: 8,
	typeLine: 'The Encroaching Darkness',
	verified: false,
	w: 1,
	x: 3,
	y: 2,
} satisfies PoeItem;
