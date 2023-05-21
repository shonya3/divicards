export type Order = 'asc' | 'desc';
export type League = 'Crucible' | 'Standard' | 'Crucible-HC';
export const leagues = Object.freeze(['Crucible', 'Standard', 'Crucible-HC']) satisfies Readonly<League[]>;
export type CsvExt = `${string}.csv`;
export const isCsvExt = (s: string): s is CsvExt => {
	return s.endsWith('.csv');
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
