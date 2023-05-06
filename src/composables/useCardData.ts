import { DivinationCardsSample } from './../types';
import { Ref, computed, ref, unref, watch } from 'vue';
import { command } from '../command';

export const useCardData = (csv: Ref<string>, minimumCardPrice: number) => {
	const data: Ref<DivinationCardsSample> = ref<DivinationCardsSample>({
		chaos: 0,
		polished: '',
		fixedNames: [],
		notCards: [],
		cards: [],
	});
	const error = ref('');
	const isError = computed(() => Boolean(error.value));
	const isReady = ref(false);

	const runCommand = async () => {
		isReady.value = false;
		error.value = '';
		try {
			// data.value = await command('create_file_card_data', {
			// 	csvString: unref(csv),
			// 	minimumCardPrice,
			// });

			data.value = await command('sample', { csv: unref(csv) });
			// console.log({ sample });
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
