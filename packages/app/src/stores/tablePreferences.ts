import { Column, Order } from '@divicards/shared/types';

import { RemovableRef, useLocalStorage } from '@vueuse/core';
import { To } from '@divicards/wc/src/wc/form-export-sample/form-export-sample';
import { defineStore } from 'pinia';

export const useTablePreferencesStore = defineStore('tablePreferences', {
	state: (): {
		spreadsheetId: RemovableRef<string>;
		columns: RemovableRef<Set<Column>>;
		order: RemovableRef<Order>;
		orderedBy: RemovableRef<Column>;
		cardsMustHaveAmount: RemovableRef<boolean>;
		minPrice: RemovableRef<number>;
		sheetTitle: string;
		to: To;
	} => ({
		spreadsheetId: useLocalStorage<string>('spreadsheetId', ''),
		columns: useLocalStorage<Set<Column>>('columns', new Set(['name', 'amount'])),
		order: useLocalStorage<Order>('order', 'desc'),
		orderedBy: useLocalStorage<Column>('toOrderedBy', 'amount'),
		cardsMustHaveAmount: useLocalStorage<boolean>('cardsMustHaveAmount', false),
		minPrice: useLocalStorage<number>('minPrice', 0),
		sheetTitle: '',
		to: 'file',
	}),
	actions: {
		rememberSpreadsheetId(spreadsheetId: string) {
			this.spreadsheetId = spreadsheetId;
		},
	},
});
