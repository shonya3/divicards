import type { Contents } from './components/FileCard/FileCard.vue';
import { CommandList, FileContents, CardRecord, WeightedCardRecord } from './types';
import { invoke } from '@tauri-apps/api';

export const csvFile = (csv: string, name: `${string}.csv`): File => new File([csv], name, { type: 'text/csv' });

export const createDownloadAnchor = (href: string, filename: string): HTMLAnchorElement => {
	const a = document.createElement('a');
	a.download = `${filename}`;
	a.href = href;
	return a;
};

export const downloadFiles = (hrefList: FileContents[]): void => {
	for (const { name, href } of hrefList) {
		createDownloadAnchor(href, name).click();
	}
};

export const command = <T extends keyof CommandList>(
	cmd: T,
	args: CommandList[T]['args']
): CommandList[T]['returnType'] => {
	return invoke(cmd, args) as CommandList[T]['returnType'];
};

export const createContents = async (file: File): Promise<Contents> => {
	let error: string | null = null;
	let text = await file.text();
	let href = URL.createObjectURL(file);
	let records: CardRecord[] = [];
	let notCards: string[] = [];
	let fixedNames: Record<string, string> = {};

	try {
		const data = await command('read_polish_csv', { csvString: text });
		records = data.records;
		text = data.csv;
		notCards = data.notCards;
		fixedNames = data.fixedNames;
	} catch (err) {
		error = err as string;
		console.log(err);
	}

	let weightedRecords: WeightedCardRecord[] = [];
	if (!error) {
		try {
			const { csv, records: weighted } = await command('weight_records_to_csv', { records });
			text = csv;
			weightedRecords = weighted;
		} catch (err) {
			error = err as string;
		}
	}

	const fileContent: FileContents = {
		text,
		name: file.name,
		href,
	};

	const contents: Contents = {
		id: crypto.randomUUID(),
		fileContent,
		valid: !Boolean(error),
		error,
		selected: false,
		allCardsPrice: 0,
		records: weightedRecords,
		notCards,
		fixedNames,
	};

	return contents;
};
