import { DivinationCardsSample } from './../types';
import { Ref, computed, ref, unref, watch } from 'vue';
import { command } from '../command';
import { League } from '../types';

export const useSample = (csv: Ref<string>, league: League) => {
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
			data.value = await command('sample', { csv: unref(csv), league });
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
