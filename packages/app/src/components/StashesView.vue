<script setup lang="ts">
import { Events } from '@divicards/wc/src/wc/stashes/e-stashes-view';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';
import type { ExtractCardsEvent, StashesViewProps } from '@divicards/wc/src/wc/stashes/e-stashes-view';
import '@divicards/wc/src/wc/stashes/e-stashes-view';
import type { DivinationCardsSample, League } from '@divicards/shared/types';
import { TabWithItems } from 'poe-custom-elements/types.js';
withDefaults(defineProps<StashesViewProps>(), { league: ACTIVE_LEAGUE });
const emit = defineEmits<{
	close: [];
	'sample-from-tab': [name: string, sample: DivinationCardsSample, league: League];
	'tab-with-items-loaded': [string, TabWithItems, League];
	'extract-cards': [TabWithItems, League];
}>();

const onSampleFromTab = (e: CustomEvent<Events['sample-from-tab']>) => {
	emit('sample-from-tab', e.detail.name, e.detail.sample, e.detail.league);
};

const onTabWithItemsLoaded = (e: CustomEvent<Events['tab-with-items-loaded']>) => {
	emit('tab-with-items-loaded', e.detail.name, e.detail.tab, e.detail.league);
};

const onExtractCards = ({ tab, league }: ExtractCardsEvent) => {
	emit('extract-cards', tab, league);
};
</script>

<template>
	<e-stashes-view
		:league="league"
		:stashLoader="stashLoader"
		@close="$emit('close')"
		@sample-from-tab="onSampleFromTab"
		@tab-with-items-loaded="onTabWithItemsLoaded"
		@extract-cards="onExtractCards"
	></e-stashes-view>
</template>

<style scoped>
wc-stashes-view {
	margin-inline: auto;
}
</style>
