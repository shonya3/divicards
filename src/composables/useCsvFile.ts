import { ref, watch } from 'vue';
import { csvFile } from '../lib';

export const useCsvFile = (file: File) => {
	console.log(file.name);
	const text = ref('a,b\n1,2');
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

// watch(
// 	() => text.value,
// 	val => {
// 		console.log('text is changed');
// 		href.value = URL.createObjectURL(file);
// 	}
// );
