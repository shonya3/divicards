export type Order = 'asc' | 'desc';
export type League = 'Crucible' | 'Standard';
export type CsvExt = `${string}.csv`;

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
