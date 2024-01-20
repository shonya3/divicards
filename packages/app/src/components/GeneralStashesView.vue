<script setup lang="ts">
import { GeneralStashesViewElement, Events } from '@divicards/wc/src/wc/stashes/general-stashes-view';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';
import type { Props } from '@divicards/wc/src/wc/stashes/general-stashes-view';
import type { League } from '@divicards/shared/types';
import { TabWithItems } from '../../../shared/poe.types';
GeneralStashesViewElement.define();
withDefaults(defineProps<Props>(), { league: ACTIVE_LEAGUE });
const emit = defineEmits<{
	close: [];
	'update:selectedTabs': [Set<string>];
	tab: [string, TabWithItems, League];
}>();

const onUpdSelectedTabs = (e: CustomEvent<Set<string>>) => {
	emit('update:selectedTabs', e.detail);
};

const onSampleFromTab = (e: CustomEvent<Events['tab']>) => {
	emit('tab', e.detail.name, e.detail.tab, e.detail.league);
};
</script>

<template>
	<wc-stashes-view
		:league="league"
		:stashLoader="stashLoader"
		@close="$emit('close')"
		@upd:selectedTabs="onUpdSelectedTabs"
		@tab="onSampleFromTab"
	></wc-stashes-view>
</template>
