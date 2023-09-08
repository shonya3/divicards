import { Column, DivinationCardsSample, Order, TablePreferences } from '@divicards/shared/types';

import { toOrderedBy } from '@divicards/shared/toOrderedBy';
import { isSheetsError } from './error';

export type Values<T = string | number | null> = Array<Array<T>>;

/** Prepare the sample for google sheets */
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
	const columnsArr = columnsToArray(options.columns);

	const headers = Array.from(columnsArr);
	values.push(headers);

	for (const card of sample.cards) {
		if (options.cardsMustHaveAmount && card.amount === 0) {
			continue;
		}

		const row = [];

		for (const column of columnsArr) {
			row.push(card[column]);
		}

		values.push(row);
	}

	return values;
};

/**
 * Convert columns to array to preserve the order of columns:
 *
 * name | amount | weight | price | sum
 */
const columnsToArray = (set: Set<Column>): Column[] => {
	const arr: Column[] = [];
	if (set.has('name')) arr.push('name');
	if (set.has('amount')) arr.push('amount');
	if (set.has('weight')) arr.push('weight');
	if (set.has('price')) arr.push('price');
	if (set.has('sum')) arr.push('sum');

	return arr;
};

// currently not using it. Using command("create_sheet_with_values", {...}) instead
export class SheetsApi {
	async writeValuesIntoSheet(
		spreadsheetId: string,
		title: string,
		values: Values,
		token: string
	): Promise<SendValuesResponse> {
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

	async createSheetWithSample(
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
	): Promise<SendSampleResponse> {
		const { sheetId } = await this.createSheet(spreadsheetId, title, token);
		const values = sampleIntoValues(sample, options);
		const sendValuesResponse = await this.writeValuesIntoSheet(spreadsheetId, title, values, token);
		return {
			...sendValuesResponse,
			url: this.sheetUrl(spreadsheetId, sheetId),
		};
	}

	sheetUrl(spreadsheetId: number | string, sheetId: number | string) {
		return `https://docs.google.com/spreadsheets/d/${spreadsheetId}/edit#gid=${sheetId}`;
	}
}

/**
 * Array of arrays(rows) for google sheets data representation
 */
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

type SendValuesResponse = {
	spreadsheetId: string;
	updatedCells: 802;
	updatedColumns: 2;
	updatedRange: string;
	updatedRows: number;
};

type SendSampleResponse = {
	url: string;
	spreadsheetId: string;
	updatedCells: 802;
	updatedColumns: 2;
	updatedRange: string;
	updatedRows: number;
};
