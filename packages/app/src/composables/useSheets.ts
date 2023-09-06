import { Column, Order } from '@divicards/shared/types';
import { ref } from 'vue';

import { useLocalStorage } from '@vueuse/core';

export const useSheets = () => {
	const spreadsheet = useLocalStorage<string>('spreadsheetId', '');
	const columns = useLocalStorage<Set<Column>>('columns', new Set(['name', 'amount']));
	const order = useLocalStorage<Order>('order', 'desc');
	const orderedBy = useLocalStorage<Column>('toOrderedBy', 'amount');
	const cardsMustHaveAmount = useLocalStorage<boolean>('cardsMustHaveAmount', false);
	const sheetTitle = ref('');

	return {
		spreadsheet,
		columns,
		order,
		orderedBy,
		cardsMustHaveAmount,
		sheetTitle,
	};
};
