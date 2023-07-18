export type Order = 'asc' | 'desc' | 'unordered';
export type League =
	| 'Crucible'
	| 'Standard'
	| 'Crucible-HC'
	| 'Hardcore'
	| 'SSF Standard'
	| 'SSF Hardcore'
	| 'SSF Crucible'
	| 'HC SSF Crucible';
export const leagues = Object.freeze([
	'Crucible',
	'Standard',
	'Crucible-HC',
	'Hardcore',
	'SSF Standard',
	'SSF Hardcore',
	'SSF Crucible',
	'HC SSF Crucible',
]) satisfies Readonly<League[]>;
export type TradeLeague = 'Crucible' | 'Standard' | 'Crucible-HC' | 'Hardcore';
export const tradeLeagues = Object.freeze(['Crucible', 'Standard', 'Crucible-HC', 'Hardcore']) satisfies Readonly<
	TradeLeague[]
>;

export const permanentLeagues: Readonly<League[]> = Object.freeze(['Standard', 'Hardcore', 'SSF Standard']);

export const isTradeLeague = (s: string): s is TradeLeague => {
	return tradeLeagues.includes(s as TradeLeague);
};

export interface DivinationCardsSample {
	cards: DivinationCardRecord[];
	notCards: string[];
	fixedNames: FixedName[];
	chaos: number;
	polished: string;
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
