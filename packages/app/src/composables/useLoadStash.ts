import { ACTIVE_LEAGUE } from '@divicards/shared/lib';
import { ref } from 'vue';
import { League, isTradeLeague } from '@divicards/shared/types';
import { command } from '../command';
import { useFileCardsStore } from '../stores/fileCards';
import { StashTab } from '@divicards/shared/poe.types';
import { cardsFromTab } from '../poe/cards';

export const useLoadStash = () => {
	const msg = ref('');
	const fetchingStash = ref(false);
	const countDown = ref(0);
	let countdownTimer: ReturnType<typeof setInterval> | null = null;

	const fetchStashesContents = async (ids: string[], league: League) => {
		const tradeLeague = isTradeLeague(league) ? league : ACTIVE_LEAGUE;
		const SLEEP_SECS = 10;
		const LOAD_AT_ONE_ITERATION = 5;
		const stashIds = ids.slice();
		const result: StashTab[] = [];
		fetchingStash.value = true;
		while (stashIds.length > 0) {
			const chunkIds = stashIds.splice(0, LOAD_AT_ONE_ITERATION);
			msg.value = `${new Date().toLocaleTimeString('ru')}: Loading ${chunkIds.length} tabs data`;
			const r = await Promise.all(
				chunkIds.map(async stashId => {
					const { stash: tab } = await command('stash', { stashId, league });
					const sample = await command('sample_cards', {
						cards: cardsFromTab(tab),
						league: tradeLeague,
					});
					const file = new File([sample.polished], `${tab.name}.csv`);
					useFileCardsStore().addCards([file], tradeLeague);
					return tab;
				})
			);
			result.push(...r);
			if (stashIds.length === 0) break;

			// Countdown
			if (countdownTimer) {
				clearInterval(countdownTimer);
				countdownTimer = null;
			}
			countDown.value = SLEEP_SECS;
			countdownTimer = setInterval(() => {
				if (countDown.value <= 0) {
					if (countdownTimer) {
						clearInterval(countdownTimer);
						countdownTimer = null;
					}
				} else {
					countDown.value--;
					msg.value = `Loaded. Now ${countDown.value}s sleep`;
				}
			}, 1000);

			msg.value = `Loaded. Now ${SLEEP_SECS}s sleep`;
			await new Promise(r => setTimeout(r, SLEEP_SECS * 1000));
		}

		fetchingStash.value = false;
		msg.value = '';

		return result;
	};

	return {
		countDown,
		msg,
		fetchingStash,
		fetchStashesContents,
	};
};
