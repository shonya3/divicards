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
	| string;

export interface Metadata {
	colour?: string;
	public?: boolean;
	folder?: boolean;
	items?: boolean;
	map?: {
		series?: number;
	};
}

export interface IProperty {
	name: string;
	values: Array<Array<string | number>>;
	displayMode: number;
	progress?: number;
	type?: number;
}

export interface IRequirement {
	name: string;
	values: Array<Array<string | number>>;
	displayMode: number;
	type?: number;
}

export interface ISocket {
	group: number;
	attr: string;
	sColour: string;
}

export interface ICategory {
	gems: Array<string>;
	jewels: Array<string>;
}

export type PoeItem = {
	id: string;
	name: string;
	verified: boolean;
	inventoryId: string;
	frameType: number;
	x: number;
	y: number;
	w: number;
	h: number;
	ilvl: number;
	icon: string;
	league: string;
	sockets?: Array<ISocket>;
	shaper?: boolean;
	elder?: boolean;
	baseType: string;
	fractured?: boolean;
	synthesised?: boolean;
	typeLine: string;
	identified: boolean;
	corrupted?: boolean;
	lockedToCharacter?: boolean;
	requirements?: Array<IRequirement>;
	implicitMods?: Array<string>;
	explicitMods?: Array<string>;
	fracturedMods?: Array<string>;
	socketedItems?: Array<ISocketedItem>;
	properties?: Array<IProperty>;
	flavourText?: Array<string>;
	craftedMods?: Array<string>;
	enchantMods?: Array<string>;
	utilityMods?: Array<string>;
	descrText?: string;
	prophecyText?: string;
	socket?: number;
	stackSize?: number;
	maxStackSize?: number;
};

export interface ISocketedItem {
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
	requirements: Array<IRequirement>;
	nextLevelRequirements?: Array<IRequirement>;
	explicitMods: Array<string>;
	frameType: number;
	x?: number;
	y?: number;
	properties: Array<IProperty>;
	additionalProperties?: Array<IProperty>;
	descrText: string;
	secDescrText: string;
	socket: number;
}
