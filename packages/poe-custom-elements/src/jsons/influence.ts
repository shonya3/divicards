import { PoeItem } from '../poe.types.js';

export const influence: PoeItem = {
	baseType: 'Agate Amulet',
	explicitMods: [
		'Grants Level 22 Determination Skill',
		'+23 to Strength',
		'12% increased Dexterity',
		'11% increased maximum Energy Shield',
	],
	frameType: 2,
	h: 1,
	icon: 'https://web.poecdn.com/gen/image/WzI1LDE0LHsiZiI6IjJESXRlbXMvQW11bGV0cy9BZ2F0ZUFtdWxldCIsInciOjEsImgiOjEsInNjYWxlIjoxfV0/710a5a8ec4/AgateAmulet.png',
	id: 'df3caa57ed265cb237174dab64d8522d2bd782a9147ed834c42afbfc8fc1561c',
	identified: true,
	ilvl: 86,
	implicitMods: ['+18 to Strength and Intelligence'],
	influences: {
		redeemer: true,
		shaper: true,
	},
	inventoryId: 'Stash1',
	league: 'Standard',
	name: 'Soul Idol',
	rarity: 'Rare',
	requirements: [
		{
			displayMode: 0,
			name: 'Level',
			type: 62,
			values: [['60', 0]],
		},
	],
	shaper: true,
	typeLine: 'Agate Amulet',
	verified: false,
	w: 1,
	x: 5,
	y: 0,
} as PoeItem;
