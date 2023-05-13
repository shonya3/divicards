import { ref, watch } from 'vue';
import { csvFile } from '../lib';

export const useCsvFile = (file: File) => {
	const text = ref('');
	const name = ref(file.name);
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
