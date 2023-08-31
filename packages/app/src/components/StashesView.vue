<script setup lang="ts">
import { StashesViewElement, Events } from '@divicards/wc/src/wc/stashes/stashes-view';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';
import type { StashesViewProps } from '@divicards/wc/src/wc/stashes/stashes-view';
import type { DivinationCardsSample, League } from '@divicards/shared/types';
StashesViewElement.define();
withDefaults(defineProps<StashesViewProps>(), { league: ACTIVE_LEAGUE });
const emit = defineEmits<{
	close: [];
	'update:selectedTabs': [Set<string>];
	'sample-from-tab': [string, DivinationCardsSample, League];
}>();

const onUpdSelectedTabs = (e: CustomEvent<Set<string>>) => {
	emit('update:selectedTabs', e.detail);
};

const onSampleFromTab = (e: CustomEvent<Events['sample-from-tab']>) => {
	emit('sample-from-tab', e.detail.name, e.detail.sample, e.detail.league);
};
</script>

<template>
	<wc-stashes-view
		:league="league"
		:stashLoader="stashLoader"
		@close="$emit('close')"
		@upd:selectedTabs="onUpdSelectedTabs"
		@sample-from-tab="onSampleFromTab"
	></wc-stashes-view>
</template>
