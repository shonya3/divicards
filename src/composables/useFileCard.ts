import { computed, reactive, ref, watch } from 'vue';
import { useSample } from './useSample';
import { useCsvFile } from './useCsvFile';
import { FileCardProps } from '../components/FileCard/FileCard.vue';
import { League } from '../types';
import { command } from '../command';
import { csvFile } from '../lib';

export const useFileCard = (file: File, league: League): FileCardProps => {
	const { text: csv, name: filename, href } = useCsvFile(file);
	const { data, error, isError } = useSample(csv, league);
	const selected = ref<boolean | null>(false);
	const valid = computed(() => !Boolean(error.value));
	const id = crypto.randomUUID();

	const props = reactive({
		id,
		valid,
		selected,
		sample: data,
		filename,
		href,
		error,
		isError,
		minimumCardPrice: 0,
		league,
	});

	watch(
		() => props.sample.polished,
		val => {
			props.href = URL.createObjectURL(csvFile(val, props.filename as `${string}.csv`));
		}
	);

	watch(
		() => props.minimumCardPrice,
		async val => {
			props.sample.chaos = await command('chaos', {
				sample: props.sample,
				min: val,
			});
		}
	);

	watch(
		() => props.league,
		async val => {
			props.sample = await command('league', { league: val, sample: props.sample });
		}
	);

	return props;
};
