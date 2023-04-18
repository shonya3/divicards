import { invoke } from '@tauri-apps/api';
import { FileCardData } from './components/FileCard/FileCard.vue';
import { CardRecord, WeightedCardRecord } from './types';

export const command = <T extends keyof CommandList>(
	cmd: T,
	args: CommandList[T]['args']
): CommandList[T]['returnType'] => {
	return invoke(cmd, args) as CommandList[T]['returnType'];
};

export interface CommandList {
	create_file_card_data: {
		args: { csvString: string; minimumCardPrice: number };
		returnType: Promise<FileCardData>;
	};

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
	all_cards_price: {
		args: {
			/** Csv file as string */
			csvString: string;
			minimumCardPrice: number;
		};
		returnType: Promise<number>;
	};
}
