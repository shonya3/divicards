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

export const createContents = async (file: File): Promise<Contents> => {
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
	} catch (err) {
		error = err as string;
		console.log(err);
	}

	let weightedRecords: WeightedCardRecord[] = [];
	if (!error) {
		try {
			const { csv, records: weighted } = await command('weight_records_to_csv', { records });
			text = csv;
			href = createCSVLink(text);
			weightedRecords = weighted;
		} catch (err) {
			error = err as string;
		}
	}

	const fileContent: FileContents = {
		text,
		filename: file.name,
		href,
	};
	const contents: Contents = {
		id: crypto.randomUUID(),
		fileContent,
		valid: !Boolean(error),
		error,
		selected: false,
		price: 0,
		records: weightedRecords,
		notCards,
		fixedNames,
	};

	return contents;
};
