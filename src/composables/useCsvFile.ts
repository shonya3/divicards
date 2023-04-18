import { ref } from 'vue';

export const useCsvFile = (file: File) => {
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
