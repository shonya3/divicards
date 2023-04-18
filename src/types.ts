export type DivCsvLine = [number, string, number | null, number | null];

export interface CardRecord {
	stackSize: number;
	name: string;
	calculated: number;
	total: number;
}

export type WeightedCardRecord = CardRecord & { realWeight: number };

export type Order = 'asc' | 'desc';
export type Column = 'price' | 'stackSize' | 'total';
export type SortState = {
	[col in Column]: Order;
} & { activeColumn: Column };
