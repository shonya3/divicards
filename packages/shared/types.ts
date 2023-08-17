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
	price: number | null;
	amount: number;
	sum: number | null;
	weight: number;
}

export interface FixedName {
	old: string;
	fixed: string;
}

export interface DiscordIdentity {
	id: string;
	username: string;
	global_name: string | null;
	display_name: string | null;
	avatar: string | null;
	discriminator: string | null;
	public_flags: number;
	flags: number;
	banner: string | null;
	banner_color: string | null;
	locale: string;
	mfa_enabled: boolean;
	premium_type: number;
	avatar_decoration: string | null;
}

export interface GoogleIdentity {
	name: string;
	id: string;
	picture: string | null;
	locale: string | null;
	given_name: string;
}

export type CardNameAmount = Pick<DivinationCardRecord, 'name' | 'amount'>;
export type Ok<T> = { type: 'ok'; data: T };
export type Err<E = string> = { type: 'err'; error: E };
export type Result<T, E = string> = Ok<T> | Err<E>;
