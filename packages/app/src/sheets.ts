import { Column, DivinationCardsSample, Order, TablePreferences } from '@divicards/shared/types';
import { Values } from './command';
import { toOrderedBy } from '@divicards/shared/toOrderedBy';
import { isSheetsError } from './error';

type Reply = {
	addSheet?: {
		properties: {
			sheetId: number;
			title: string;
			index: number;
			sheetType: 'GRID' | string;
			GridProperties: {
				columnCount: 26;
				rowCount: 1000;
			};
		};
	};
};

type BatchResponse = {
	spreadsheetId: string;
	replies: Reply[];
};

type NewSheetResponse = {
	sheetId: number;
	title: string;
	index: number;
	sheetType: 'GRID' | string;
	GridProperties: {
		columnCount: 26;
		rowCount: 1000;
	};
};

export class SheetsApi {
	async writeValuesIntoSheet(spreadsheetId: string, title: string, values: Values, token: string): Promise<unknown> {
		const url = `https://sheets.googleapis.com/v4/spreadsheets/${spreadsheetId}/values/${title}?valueInputOption=RAW`;
		const body = JSON.stringify({
			range: title,
			majorDimension: 'ROWS',
			values,
		});

		const response = await fetch(url, {
			method: 'PUT',
			body,
			headers: new Headers({
				Authorization: `Bearer ${token}`,
			}),
		});

		const data = response.json();
		if (isSheetsError(data)) {
			throw data;
		}
		return data;
	}

	async createSheet(spreadsheetId: string, title: string, token: string): Promise<NewSheetResponse> {
		const url = `https://sheets.googleapis.com/v4/spreadsheets/${spreadsheetId}:batchUpdate`;
		const body = JSON.stringify({
			requests: [
				{
					addSheet: {
						properties: {
							title,
						},
					},
				},
			],
		});

		const response = await fetch(url, {
			method: 'POST',
			body,
			headers: new Headers({
				Authorization: `Bearer ${token}`,
			}),
		});

		const data = (await response.json()) as BatchResponse;

		if (isSheetsError(data)) {
			console.log('SHEETS IS ERROR ALRIGHT');
			throw data;
		}

		const addResponse = data.replies[0];
		if (addResponse.addSheet) {
			return addResponse.addSheet.properties;
		} else {
			throw new Error('Unknown error creating sheet');
		}
	}

	async createSheetAndWriteSample(
		spreadsheetId: string,
		title: string,
		sample: DivinationCardsSample,
		token: string,
		options: TablePreferences = {
			columns: new Set(['name', 'amount']),
			orderedBy: 'amount',
			order: 'desc',
			cardsMustHaveAmount: false,
		}
	) {
		const newSheet = await this.createSheet(spreadsheetId, title, token);
		const values = sampleIntoValues(sample, options);
		const result = await this.writeValuesIntoSheet(spreadsheetId, title, values, token);
		return result;
	}

	sheetUrl(spreadsheetId: number | string, sheetId: number | string) {
		return `https://docs.google.com/spreadsheets/d/${spreadsheetId}/edit#gid=${sheetId}`;
	}
}

export const sampleIntoValues = (
	sample: DivinationCardsSample,
	options: TablePreferences = {
		columns: new Set(['name', 'amount']),
		orderedBy: 'amount',
		order: 'desc',
		cardsMustHaveAmount: false,
	}
): Values => {
	const values: Values = [];

	sample.cards = toOrderedBy(sample.cards, options.orderedBy, options.order);

	const headers: string[] = [];
	for (const column of options.columns.values()) {
		headers.push(column);
	}
	values.push(headers);

	for (const card of sample.cards) {
		if (options.cardsMustHaveAmount && card.amount === 0) {
			continue;
		}

		const row = [];

		for (const column of options.columns.values()) {
			row.push(card[column]);
		}

		values.push(row);
	}

	return values;
};
