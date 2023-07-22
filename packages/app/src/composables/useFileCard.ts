import { computed, reactive, watch } from 'vue';
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

export const useFileCard = async (file: File, league: TradeLeague): Promise<FileCardProps> => {
	const fileCard = await createFileCard(file, league);
	const props = reactive<FileCardProps>(fileCard);

	watch(
		() => props.league,
		async val => {
			if (props.sample.type !== 'ok') return;
			props.sample = await command('league', { league: val, sample: props.sample.data });
			if (props.sample.type !== 'ok') return;
			props.href = URL.createObjectURL(new File([props.sample.data.polished], props.filename));
			props.filename = prefixFilename(props.filename, val);
		}
	);

	return props;
};

const createFileCard = async (csvSource: File, league: TradeLeague): Promise<FileCardProps> => {
	const csv = typeof csvSource === 'string' ? csvSource : await csvSource.text();
	let href: string;
	if (typeof csvSource === 'string') {
		throw new Error('TODO csvSource as string');
	} else {
		href = URL.createObjectURL(csvSource);
	}
	const uuid = crypto.randomUUID();
	const sample = await command('sample', { csv, league });
	const filename = prefixFilename(csvSource.name, league);

	const fileCard = {
		uuid,
		league,
		filename,
		sample,
		selected: false,
		href,
		minimumCardPrice: 0,
	};

	return fileCard;
};
