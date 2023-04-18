import { computed, reactive, ref } from 'vue';
import { useCardData } from './useCardData';
import { useCsvFile } from './useCsvFile';
import { FileCardProps } from '../components/FileCard/FileCard.vue';

export const useCreateFileCard = (file: File, minimumCardPrice = 50): FileCardProps => {
	const { text: csv, name: filename, href } = useCsvFile(file);
	const { data, error, isError } = useCardData(csv, minimumCardPrice);
	const selected = ref<boolean | null>(false);
	const valid = computed(() => !Boolean(error.value));
	const id = crypto.randomUUID();

	return reactive({
		id,
		valid,
		selected,
		data,
		filename,
		href,
		error,
		isError,
		minimumCardPrice,
	});
};
