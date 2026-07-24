import { PoeItem } from '../poe.types.js';

export const rogueMarkers = {
	baseType: "Rogue's Marker",
	descrText: 'Right click on this item while in a Town or Hideout to use it.',
	explicitMods: [
		'Creates a portal to the Rogue Harbour from a Town or Hideout\nUsed as Currency for services in the Rogue Harbour',
	],
	frameType: 5,
	h: 1,
	icon: 'https://web.poecdn.com/gen/image/WzI1LDE0LHsiZiI6IjJESXRlbXMvQ3VycmVuY3kvSGVpc3QvSGVpc3RDb2luQ3VycmVuY3kiLCJ3IjoxLCJoIjoxLCJzY2FsZSI6MX1d/987a8953f9/HeistCoinCurrency.png',
	id: 'fb94a96094c755c7bd4608228016eac462c5ab99b0bef6e61f1d644a6f73fef5',
	identified: true,
	ilvl: 0,
	inventoryId: 'Stash1',
	league: 'Hardcore Necropolis',
	maxStackSize: 50000,
	name: '',
	properties: [
		{
			displayMode: 0,
			name: 'Stack Size',
			type: 32,
			values: [['2815/50000', 0]],
		},
	],
	stackSize: 2815,
	typeLine: "Rogue's Marker",
	verified: false,
	w: 1,
	x: 7,
	y: 6,
} as PoeItem;
