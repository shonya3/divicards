<script setup lang="ts">
import { ACTIVE_LEAGUE } from '@divicards/shared/lib.js';
import type {
	ExtractCardsEvent,
	SampleFromStashtabEvent,
	StashesViewProps,
	StashtabFetchedEvent,
} from '@divicards/wc/stashes/e-stashes-view.js';
import '@divicards/wc/stashes/e-stashes-view';
import type { DivinationCardsSample, League } from '@divicards/shared/types.js';
import { TabWithItems } from 'poe-custom-elements/types.js';
withDefaults(defineProps<StashesViewProps>(), { league: ACTIVE_LEAGUE });
const emit = defineEmits<{
	close: [];
	'sample-from-tab': [stashtab_name: string, sample: DivinationCardsSample, league: League];
	'stashtab-fetched': [string, TabWithItems, League];
	'extract-cards': [TabWithItems, League];
}>();

const emit_sample_from_tab = ({ stashtab_name, sample, league }: SampleFromStashtabEvent) => {
	emit('sample-from-tab', stashtab_name, sample, league);
};

const emit_stashtabs_fetched = ({ stashtab, league }: StashtabFetchedEvent) => {
	emit('stashtab-fetched', stashtab.name, stashtab, league);
};

const emit_extract_cards = ({ tab, league }: ExtractCardsEvent) => {
	emit('extract-cards', tab, league);
};
</script>

<template>
	<e-stashes-view
		:league="league"
		:stashLoader="stashLoader"
		@stashes__close="$emit('close')"
		@stashes__sample-from-stashtab="emit_sample_from_tab"
		@stashes__stashtab-fetched="emit_stashtabs_fetched"
		@stashes__extract-cards="emit_extract_cards"
	></e-stashes-view>
</template>

<style scoped>
wc-stashes-view {
	margin-inline: auto;
}
</style>
