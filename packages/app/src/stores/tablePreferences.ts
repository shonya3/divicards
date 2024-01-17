import { Column, Order } from '@divicards/shared/types';

import { RemovableRef, useLocalStorage } from '@vueuse/core';
import { To } from '@divicards/wc/src/wc/form-export-sample/form-export-sample';
import { defineStore } from 'pinia';
import { Ref, ref } from 'vue';

export const useTablePreferencesStore = defineStore('tablePreferences', {
	state: (): {
		spreadsheetId: RemovableRef<string>;
		columns: RemovableRef<Set<Column>>;
		order: RemovableRef<Order>;
		orderedBy: RemovableRef<Column>;
		cardsMustHaveAmount: RemovableRef<boolean>;
		minPrice: RemovableRef<number>;
		sheetTitle: Ref<string>;
		to: Ref<To>;
	} => ({
		spreadsheetId: useLocalStorage<string>('spreadsheetId', ''),
		columns: useLocalStorage<Set<Column>>('columns', new Set(['name', 'amount'])),
		order: useLocalStorage<Order>('order', 'desc'),
		orderedBy: useLocalStorage<Column>('toOrderedBy', 'amount'),
		cardsMustHaveAmount: useLocalStorage<boolean>('cardsMustHaveAmount', false),
		minPrice: useLocalStorage<number>('minPrice', 0),
		sheetTitle: ref(''),
		to: ref('file'),
	}),
	actions: {
		rememberSpreadsheetId(spreadsheetId: string) {
			this.spreadsheetId = spreadsheetId;
		},
	},
});
