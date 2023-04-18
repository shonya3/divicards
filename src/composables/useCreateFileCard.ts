import { computed, reactive, ref, watch } from 'vue';
import { useCardData } from './useCardData';
import { useCsvFile } from './useCsvFile';
import { FileCardProps } from '../components/FileCard/FileCard.vue';
import { command } from '../command';

export const useCreateFileCard = (file: File, minimumCardPrice = 50): FileCardProps => {
	const { text: csv, name: filename, href } = useCsvFile(file);
	const { data, error, isError } = useCardData(csv, minimumCardPrice);
	const selected = ref<boolean | null>(false);
	const valid = computed(() => !Boolean(error.value));
	const id = crypto.randomUUID();

	const fileCard = reactive({
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

	watch(
		() => fileCard.minimumCardPrice,
		async val => {
			const price = await command('all_cards_price', {
				csvString: fileCard.data.csvPolished,
				minimumCardPrice: val,
			});

			fileCard.data.allCardsPrice = price;
		}
	);

	return fileCard;
};
