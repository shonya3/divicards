export type DivCsvLine = [number, string, number | null, number | null];

export type Order = 'asc' | 'desc';
export type Column = 'price' | 'amount' | 'sum';
export type SortState = {
	[col in Column]: Order;
} & { activeColumn: Column };

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
