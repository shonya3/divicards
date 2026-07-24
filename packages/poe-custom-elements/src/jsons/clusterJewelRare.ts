import { PoeItem } from '../poe.types.js';

export const clusterJewelRare = {
	baseType: 'Large Cluster Jewel',
	descrText:
		'Place into an allocated Large Jewel Socket on the Passive Skill Tree. Added passives do not interact with jewel radiuses. Right click to remove from the Socket.',
	enchantMods: [
		'Adds 8 Passive Skills',
		'2 Added Passive Skills are Jewel Sockets',
		'Added Small Passive Skills grant: Claw Attacks deal 12% increased Damage with Hits and Ailments\nAdded Small Passive Skills grant: Dagger Attacks deal 12% increased Damage with Hits and Ailments',
	],
	explicitMods: [
		'Added Small Passive Skills also grant: +3 to Maximum Life',
		'1 Added Passive Skill is Calamitous',
		'1 Added Passive Skill is Smite the Weak',
	],
	frameType: 2,
	h: 1,
	icon: 'https://web.poecdn.com/gen/image/WzI1LDE0LHsiZiI6IjJESXRlbXMvSmV3ZWxzL05ld0dlbUJhc2UzIiwidyI6MSwiaCI6MSwic2NhbGUiOjF9XQ/db35e60885/NewGemBase3.png',
	id: '864a3241a53834f36d3be516b0899e16b7fdbb7f1dc69dfa23d568e8414105ea',
	identified: true,
	ilvl: 59,
	inventoryId: 'Stash1',
	league: 'Hardcore Necropolis',
	name: 'Spirit Glimmer',
	rarity: 'Rare',
	requirements: [
		{
			displayMode: 0,
			name: 'Level',
			type: 62,
			values: [['40', 0]],
		},
	],
	typeLine: 'Large Cluster Jewel',
	verified: false,
	w: 1,
	x: 11,
	y: 1,
} as PoeItem;
