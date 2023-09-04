export type Order = 'asc' | 'desc' | 'unordered';

export const leagues = [
	'Ancestor',
	'Standard',
	'Hardcore Ancestor',
	'Hardcore',
	'SSF Standard',
	'SSF Hardcore',
	'SSF Ancestor',
	'HC SSF Ancestor',
] as const;
export const tradeLeagues = ['Ancestor', 'Standard', 'Hardcore Ancestor', 'Hardcore'] as const;
export const permanentLeagues: Readonly<League[]> = ['Standard', 'Hardcore', 'SSF Standard'] as const;

export type League = (typeof leagues)[number];
export type TradeLeague = (typeof tradeLeagues)[number];

export const isTradeLeague = (s: string): s is TradeLeague => {
	return tradeLeagues.includes(s as TradeLeague);
};

export interface DivinationCardsSample {
	cards: DivinationCardRecord[];
	notCards: string[];
	fixedNames: FixedName[];
	csv: string;
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
