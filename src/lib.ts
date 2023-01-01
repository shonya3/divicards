import { CommandList, Contents, FileContents, CardRecord, WeightedCardRecord } from './types';
import { invoke } from '@tauri-apps/api';

export const createCSVLink = (contents: string): string => {
	const blob = new Blob([contents], { type: 'csv/text' });
	const href = URL.createObjectURL(blob);
	return href;
};

export const downloadFiles = (hrefList: FileContents[]): void => {
	const links: HTMLAnchorElement[] = [];
	hrefList.forEach(({ filename, href }) => {
		links.push(createDownloadLink(href, filename));
	});
	links.forEach(a => {
		a.click();
	});
};

export const createDownloadLink = (href: string, filename: string): HTMLAnchorElement => {
	const a = document.createElement('a');
	a.download = `${filename}`;
	a.href = href;
	return a;
};

export const createCsvFile = (csvString: string, filename: string): File =>
	new File([csvString], filename, { type: 'text/csv' });

export const command = <T extends keyof CommandList>(
	cmd: T,
	args: CommandList[T]['args']
): CommandList[T]['returnType'] => {
	return invoke(cmd, args) as CommandList[T]['returnType'];
};

const calcRecordWeight = (record: CardRecord, allStackSize: number): number => record.stackSize / allStackSize;
const calcRecordRealWeight = (
	record: CardRecord,
	realStackedSummaryWeight: number,
	condenseFactor: number,
	allStackSize: number
): number => (realStackedSummaryWeight * calcRecordWeight(record, allStackSize)) ** (1 / condenseFactor);

export const createContents = async (file: File): Promise<Contents> => {
	let valid = false;
	let error = null;
	let text = await file.text();
	let href = '';
	let records: CardRecord[] = [];
	let notCards: string[] = [];
	let fixedNames: Record<string, string> = {};

	try {
		const data = await command('read_polish_csv', { csvString: text });
		records = data.records;
		text = data.csv;
		notCards = data.notCards;
		fixedNames = data.fixedNames;
		href = createCSVLink(text);
		valid = true;
	} catch (err) {
		error = err as string;
	}

	// const allStackSize = records.reduce((summ, { stackSize }) => (summ += stackSize), 0);
	// const REAL_STACKED_RAIN_OF_CHAOS_WEIGHT = 2452.65513;
	// const CONDENSE_FACTOR = 2 / 3;
	// const rainOfChaos = records.find(({ name }) => name === 'Rain of Chaos');
	// if (!rainOfChaos) throw new Error('No Rain of Chaos card');
	// const weight = rainOfChaos.stackSize / allStackSize;
	// const realStackedSummaryWeight = REAL_STACKED_RAIN_OF_CHAOS_WEIGHT / weight;
	// const weightedRecords: WeightedCardRecord[] = records.map(record => {
	// 	const realWeight = calcRecordRealWeight(record, realStackedSummaryWeight, CONDENSE_FACTOR, allStackSize);
	// 	return {
	// 		...record,
	// 		realWeight,
	// 	};
	// });

	let weightedRecords: WeightedCardRecord[] = [];
	try {
		const { csv, records: weighted } = await command('weight_records_to_csv', { records });
		text = csv;
		href = createCSVLink(text);
		valid = true;
		weightedRecords = weighted;
	} catch (err) {
		error = err as string;
	}

	const fileContent: FileContents = {
		text,
		filename: file.name,
		href,
	};
	const contents: Contents = {
		id: crypto.randomUUID(),
		fileContent,
		valid,
		error,
		selected: false,
		price: 0,
		records: weightedRecords,
		notCards,
		fixedNames,
	};

	return contents;
};
