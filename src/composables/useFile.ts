import { ref } from 'vue';

export const useFile = (file: File) => {
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
