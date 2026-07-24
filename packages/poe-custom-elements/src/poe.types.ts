export interface TabWithItems extends NoItemsTab {
	items: PoeItem[];
}

export interface NoItemsTab {
	id: string;
	index: number;
	name: string;
	type: StashType;
	folder?: string;
	metadata?: Metadata;
	children?: NoItemsTab[];
	parent?: string;
}

export type StashType =
	| 'PremiumStash'
	| 'CurrencyStash'
	| 'MapStash'
	| 'QuadStash'
	| 'FragmentStash'
	| 'EssenceStash'
	| 'BlightStash'
	| 'Folder'
	| 'NormalStash'
	| 'DivinationCardStash';

export interface Metadata {
	colour?: string;
	public?: boolean;
	folder?: boolean;
	items?: boolean;
	map?: {
		series?: number;
	};
}

export type ItemProperty = {
	name: string;
	values: Array<[string, number]>;
	displayMode: number;
	progress?: number;
	type?: number;
};

export type Requirement = {
	name: string;
	values: Array<[string, number]>;
	displayMode: number;
	type?: number;
	suffix?: string;
};

export interface Socket {
	group: number;
	attr: string;
	sColour: SocketKind;
}

export const SOCKET_KINDS = ['R', 'G', 'B', 'A', 'W'] as const;
export type SocketKind = (typeof SOCKET_KINDS)[number];

export interface ICategory {
	gems: Array<string>;
	jewels: Array<string>;
}

/** PoE API item data https://www.pathofexile.com/developer/docs/reference#stashes-get */
export type PoeItem = {
	/** Monster level. Appears on allflames */
	itemLevel?: number;
	id: string;
	name: string;
	verified: boolean;
	inventoryId: string;
	frameType: number;
	x: number;
	y: number;
	w: number;
	h: number;
	rarity?: ItemRarity;
	ilvl: number;
	icon: string;
	league: string;
	sockets?: Array<Socket>;
	shaper?: boolean;
	elder?: boolean;
	baseType: string;
	fractured?: boolean;
	synthesised?: boolean;
	typeLine: string;
	identified: boolean;
	corrupted?: boolean;
	lockedToCharacter?: boolean;
	requirements?: Array<Requirement>;
	implicitMods?: Array<string>;
	explicitMods?: Array<string>;
	fracturedMods?: Array<string>;
	socketedItems?: Array<SocketedItem>;
	properties?: Array<ItemProperty>;
	flavourText?: Array<string>;
	craftedMods?: Array<string>;
	enchantMods?: Array<string>;
	utilityMods?: Array<string>;
	descrText?: string;
	prophecyText?: string;
	socket?: number;
	stackSize?: number;
	/** Textual representation of the stack size for display purposes.
	 *  This property will be present on items that have 5-digit stack sizes (ie. > 9999).
	 *  https://www.pathofexile.com/forum/view-thread/2936225
	 */
	stackSizeText?: string;
	maxStackSize?: number;
	additionalProperties?: Array<ItemProperty>;
	secDescrText?: string;
	artFilename?: string;
	cisRaceReward?: boolean;
	colour?: string;
	support?: boolean;
	talismanTier?: number;
	influences?: Record<Influence, boolean>;
	incubatedItem?: IncubatedItem;
	nextLevelRequirements?: Array<Requirement>;
	note?: string;
	abyssJewel?: boolean;
	hybrid?: HybridGem;
};

export interface SocketedItem {
	id: string;
	verified: boolean;
	w: number;
	h: number;
	ilvl: number;
	icon: string;
	name: string;
	typeLine: string;
	corrupted?: boolean;
	lockedToCharacter?: boolean;
	category?: ICategory;
	requirements: Array<Requirement>;
	nextLevelRequirements?: Array<Requirement>;
	explicitMods: Array<string>;
	frameType: number;
	x?: number;
	y?: number;
	properties: Array<ItemProperty>;
	additionalProperties?: Array<ItemProperty>;
	descrText: string;
	secDescrText: string;
	socket: number;
	baseType?: string;
	colour?: string;
	identified?: boolean;
	league?: string;
	support?: boolean;
	hybrid?: HybridGem;
}

export type IncubatedItem = {
	name: string;
	level: number;
	progress: number;
	total: number;
};

export type HybridGem = {
	baseTypeName: string;
	explicitMods?: Array<string>;
	properties: Array<ItemProperty>;
	secDescrText: string;
	isVaalGem?: boolean;
};

export const INFLUENCE_VARIANTS = ['shaper', 'elder', 'redeemer', 'warlord', 'hunter', 'crusader'] as const;
export type Influence = (typeof INFLUENCE_VARIANTS)[number];

export const RARITY_VARIANTS = ['Normal', 'Magic', 'Rare', 'Unique'] as const;
export type ItemRarity = (typeof RARITY_VARIANTS)[number];

/** https://www.pathofexile.com/developer/docs/reference#type-FrameType */
export type FrameKind = (typeof FRAME_KIND_VARIANTS)[number];

export const FRAME_KIND_VARIANTS = [
	'normal',
	'magic',
	'rare',
	'unique',
	'gem',
	'currency',
	'necropolis',
	'divination',
] as const;
