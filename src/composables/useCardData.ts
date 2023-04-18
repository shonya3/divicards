import { Ref, computed, ref, unref, watch } from 'vue';
import { command } from '../command';
import { FileCardData } from '../components/FileCard/FileCard.vue';

export const useCardData = (csv: Ref<string>, minimumCardPrice: number) => {
	const data: Ref<FileCardData> = ref<FileCardData>({
		minimumCardPrice,
		allCardsPrice: 0,
		csvPolished: '',
		fixedNames: {},
		notCards: [],
		records: [],
	});
	const error = ref('');
	const isError = computed(() => Boolean(error.value));
	const isReady = ref(false);

	const runCommand = async () => {
		isReady.value = false;
		error.value = '';
		try {
			data.value = await command('create_file_card_data', {
				csvString: unref(csv),
				minimumCardPrice,
			});
		} catch (err) {
			error.value = err as string;
		}
	};

	watch(
		() => csv.value,
		val => {
			if (val) {
				runCommand();
			}
		}
	);

	return {
		data,
		error,
		isError,
		isReady,
	};
};
