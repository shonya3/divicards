import { Column, Order } from '@divicards/shared/types';
import { ref } from 'vue';

import { useLocalStorage } from '@vueuse/core';
import { To } from '@divicards/wc/src/wc/form-export-sample/form-export-sample';

export const usePreferences = () => {
	const spreadsheet = useLocalStorage<string>('spreadsheetId', '');
	const columns = useLocalStorage<Set<Column>>('columns', new Set(['name', 'amount']));
	const order = useLocalStorage<Order>('order', 'desc');
	const orderedBy = useLocalStorage<Column>('toOrderedBy', 'amount');
	const cardsMustHaveAmount = useLocalStorage<boolean>('cardsMustHaveAmount', false);
	const minPrice = useLocalStorage<number>('minPrice', 0);
	const sheetTitle = ref('');
	const to = ref<To>('sheets');

	return {
		spreadsheet,
		columns,
		order,
		orderedBy,
		cardsMustHaveAmount,
		sheetTitle,
		minPrice,
		to,
	};
};
