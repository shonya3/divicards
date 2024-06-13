<script setup lang="ts">
import { StashesViewElement, Events } from '@divicards/wc/src/wc/stashes/stashes-view';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';
import type { StashesViewProps } from '@divicards/wc/src/wc/stashes/stashes-view';
import type { DivinationCardsSample, League } from '@divicards/shared/types';
import { TabWithItems } from '@divicards/shared/poe.types';
StashesViewElement.define();
withDefaults(defineProps<StashesViewProps>(), { league: ACTIVE_LEAGUE });
const emit = defineEmits<{
	close: [];
	'update:selectedTabs': [Set<string>];
	'sample-from-tab': [string, DivinationCardsSample, League];
	'tab-with-items-loaded': [string, TabWithItems, League];
}>();

const onUpdSelectedTabs = (e: CustomEvent<Set<string>>) => {
	emit('update:selectedTabs', e.detail);
};

const onSampleFromTab = (e: CustomEvent<Events['sample-from-tab']>) => {
	emit('sample-from-tab', e.detail.name, e.detail.sample, e.detail.league);
};

const onTabWithItemsLoaded = (e: CustomEvent<Events['tab-with-items-loaded']>) => {
	emit('tab-with-items-loaded', e.detail.name, e.detail.tab, e.detail.league);
};
</script>

<template>
	<wc-stashes-view
		:league="league"
		:stashLoader="stashLoader"
		@close="$emit('close')"
		@upd:selectedTabs="onUpdSelectedTabs"
		@sample-from-tab="onSampleFromTab"
		@tab-with-items-loaded="onTabWithItemsLoaded"
	></wc-stashes-view>
</template>

<style scoped>
wc-stashes-view {
	margin-inline: auto;
}
</style>
