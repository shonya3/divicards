import { computed, reactive, ref, watch } from 'vue';
import { useSample } from './useSample';
import { useFile } from './useFile';
import { FileCardProps } from '../components/FileCard/FileCard.vue';
import { League, leagues } from '../types';
import { command } from '../command';

const prefixFilename = (name: string, league: League): string => {
	const UNDERSCORE_GLUE = '_';

	for (const old of leagues) {
		if (name.startsWith(`${old}${UNDERSCORE_GLUE}`)) {
			return name.replace(old, league);
		}
	}

	return `${league}${UNDERSCORE_GLUE}${name}`;
};

export const useFileCard = (file: File, league: League): FileCardProps => {
	const { text: csv, name: filename, href } = useFile(file);
	const { data, error, isError, isReady } = useSample(csv, league);
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
		isReady,
	});

	watch(
		() => isReady.value,
		val => {
			if (!isError.value) {
				props.filename = prefixFilename(filename.value, league);
			}
		}
	);

	watch(
		() => props.sample.polished,
		val => {
			props.href = URL.createObjectURL(new File([val], props.filename));
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
			props.filename = prefixFilename(props.filename, val);
		}
	);

	return props;
};
