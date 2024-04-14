export type Order = 'asc' | 'desc' | 'unordered';
export type Column = 'price' | 'amount' | 'sum' | 'name' | 'weight';
export type TablePreferences = {
	columns: Set<Column>;
	orderedBy: Column;
	order: Order;
	cardsMustHaveAmount: boolean;
	minPrice: number;
};

export const leagues = [
	'Necropolis',
	'Standard',
	'Hardcore Necropolis',
	'Hardcore',
	'Solo Self-Found',
	'SSF Necropolis',
	'HC SSF Necropolis',
] as const;
export const tradeLeagues = ['Necropolis', 'Standard', 'Hardcore Necropolis', 'Hardcore'] as const;
export const permanentLeagues = ['Standard', 'Hardcore', 'Solo Self-Found', 'Hardcore SSF'] as const;

export function isPermanentLeague(league: unknown): league is PermanentLeague | never {
	return typeof league === 'string' && permanentLeagues.includes(league as PermanentLeague);
}

export type League = (typeof leagues)[number] | string;
export type TradeLeague = (typeof tradeLeagues)[number];
export type PermanentLeague = (typeof permanentLeagues)[number];

export const isTradeLeague = (s: string): s is TradeLeague => {
	return tradeLeagues.includes(s as TradeLeague);
};

export interface DivinationCardsSample {
	cards: DivinationCardRecord[];
	notCards: string[];
	fixedNames: FixedName[];
}

export interface DivinationCardRecord {
	name: string;
	amount: number;
	price: number | null;
	sum: number | null;
	weight: number | null;
}

export interface FixedName {
	old: string;
	fixed: string;
}

export type CardNameAmount = Pick<DivinationCardRecord, 'name' | 'amount'>;
export interface GoogleIdentity {
	name: string;
	id: string;
	picture: string | null;
	locale: string | null;
	given_name: string;
}
