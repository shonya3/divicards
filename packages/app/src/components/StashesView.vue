<script setup lang="ts">
import { StashesViewElement, Events } from '@divicards/wc/src/wc/stashes/stashes-view';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';
import type { StashesViewProps } from '@divicards/wc/src/wc/stashes/stashes-view';
import type { League } from '@divicards/shared/types';
import type { StashTab } from '@divicards/shared/poe.types';
StashesViewElement.define();
withDefaults(defineProps<StashesViewProps>(), { league: ACTIVE_LEAGUE });
const emit = defineEmits<{
	close: [];
	'update:selectedTabs': [Set<string>];
	'tab-tada': [StashTab, League];
}>();

const onUpdSelectedTabs = (e: CustomEvent<Set<string>>) => {
	emit('update:selectedTabs', e.detail);
};

const onTabData = (e: CustomEvent<Events['tab-data']>) => {
	emit('tab-tada', e.detail.tab, e.detail.league);
};
</script>

<template>
	<wc-stashes-view
		:league="league"
		@close="$emit('close')"
		@upd:selectedTabs="onUpdSelectedTabs"
		@tab-data="onTabData"
	></wc-stashes-view>
</template>

<style scoped></style>
