import { ref, watch } from 'vue';
import { CsvExt } from '../types';

export const useCsvFile = (file: File) => {
	const n: CsvExt = file.name.endsWith('.csv') ? (file.name as CsvExt) : `${file.name}.csv`;
	const text = ref('');
	const name = ref<CsvExt>(n);
	const href = ref(URL.createObjectURL(file));

	const updateText = async () => {
		text.value = await file.text();
	};

	updateText();

	return {
		text,
		name,
		href,
	};
};
