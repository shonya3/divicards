import { computed, reactive, ref, watch } from 'vue';
import { useSample } from './useSample';
import { useFile } from './useFile';
import { League, TradeLeague, leagues } from '@divicards/shared/types';
import { command } from '../command';
import { FileCardProps } from '@divicards/wc/src/wc/file-card/file-card';

const prefixFilename = (name: string, league: League): string => {
	const UNDERSCORE_GLUE = '_';

	for (const old of leagues) {
		if (name.startsWith(`${old}${UNDERSCORE_GLUE}`)) {
			return name.replace(old, league);
		}
	}

	return `${league}${UNDERSCORE_GLUE}${name}`;
};

export const useFileCard = (file: File, league: TradeLeague): FileCardProps => {
	const { text: csv, name: filename, href } = useFile(file);
	const { data, error, isError, isReady } = useSample(csv, league);
	const selected = ref<boolean | null>(false);
	const valid = computed(() => !Boolean(error.value));
	const uuid = crypto.randomUUID();

	const props = reactive({
		uuid,
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

	// watch(
	// 	() => props.minimumCardPrice,
	// 	async val => {
	// 		props.sample.chaos = await command('chaos', {
	// 			sample: props.sample,
	// 			min: val,
	// 		});
	// 	}
	// );

	watch(
		() => props.league,
		async val => {
			props.sample = await command('league', { league: val, sample: props.sample });
			props.filename = prefixFilename(props.filename, val);
			props.sample.chaos = await command('chaos', {
				sample: props.sample,
				min: props.minimumCardPrice,
			});
		}
	);

	return props;
};
