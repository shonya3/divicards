export interface FileContents {
	text: string;
	name: string;
	href: string;
}

export type DivCsvLine = [number, string, number | null, number | null];

export interface CommandList {
	weight_records_to_csv: {
		args: { records: CardRecord[] };
		returnType: Promise<{
			csv: string;
			records: WeightedCardRecord[];
		}>;
	};
	update_prices: {
		args: {};
		returnType: Promise<void>;
	};
	merge_csv: {
		args: {
			/** List of Csv files as strings */
			csvFileStrings: string[];
		};
		returnType: Promise<string>;
	};
	read_polish_csv: {
		args: {
			/** Csv file as string */
			csvString: string;
		};
		returnType: Promise<{
			csv: string;
			records: CardRecord[];
			notCards: string[];
			fixedNames: Record<string, string>;
		}>;
	};
	total_chaos: {
		args: {
			/** Csv file as string */
			csvString: string;
			minimumCardPrice: number;
		};
		returnType: Promise<number>;
	};
}

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
