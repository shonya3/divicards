<script setup lang="ts">
import { useStashStore } from '../../stores/stash';
import { League } from '@divicards/shared/types';
import { storeToRefs } from 'pinia';
import { ref, watch } from 'vue';
import { useLoadStash } from '../../composables/useLoadStash';
import { HelpTipElement } from '@divicards/wc/src/wc/help-tip';
import { LeagueSelectElement } from '@divicards/wc/src/wc/league-select';
import { TabBadgeGroupElement } from '@divicards/wc/src/wc/stashes/tab-badge-group';
import { TabSelectEvent } from '@divicards/wc/src/wc/stashes/tab-badge';
HelpTipElement.define();
LeagueSelectElement.define();
TabBadgeGroupElement.define();

const stashStore = useStashStore();
const { stashes, selectedTabsIds, league } = storeToRefs(stashStore);
const { fetchStashes, deleteTabs, unselectAllTabs } = stashStore;

const { msg, fetchingStash, fetchStashesContents } = useLoadStash();

const selectEl = ref<HTMLSelectElement | null>(null);

const noStashesMessage = ref('');

defineEmits<{
	(event: 'close'): void;
}>();

watch(
	() => league.value,
	() => {
		deleteTabs();
		noStashesMessage.value = '';
	}
);

const onStashes = async (league: League) => {
	noStashesMessage.value = '';
	await fetchStashes(league);
	if (!stashes.value.length) {
		noStashesMessage.value = 'No stashes here. Try to change the league';
		selectEl.value?.focus();
	}
};

const onGetData = async () => {
	fetchStashesContents(selectedTabsIds.value, league.value);
	unselectAllTabs();
	stashStore.stashes = Array.from(stashStore.stashes);
};

const onTabSelected = (e: TabSelectEvent) => {
	const { tabId, selected } = e.detail;
	const tab = stashes.value.find(t => t.id === tabId);

	console.log({ tabId, selected, e, tab });

	if (tab) {
		tab.selected = selected;
		stashes.value = Array.from(stashes.value);
	}
};
</script>

<template>
	<div class="main-stashes-component">
		<div class="controls">
			<div class="league-stashes">
				<wc-league-select
					:league="league"
					@league-change="(e: CustomEvent<League>) => (league = e.detail)"
				></wc-league-select>
				<button @click="onStashes(league)">Stashes</button>
				<wc-help-tip>
					<p>Select tabs by clicking on them. Then click LOAD ITEMS button</p>
				</wc-help-tip>
			</div>

			<button
				:class="{ visible: selectedTabsIds.length > 0 }"
				class="btn-load-items"
				:disabled="selectedTabsIds.length === 0 || fetchingStash"
				@click="onGetData"
			>
				load items
			</button>

			<button @click="$emit('close')" class="btn-close">Close</button>
		</div>

		<p class="msg" :class="{ visible: fetchingStash }">{{ msg }}</p>
		<p class="msg" :class="{ visible: noStashesMessage.length > 0 }">{{ noStashesMessage }}</p>

		<wc-tab-badge-group
			@tab-select="onTabSelected"
			:stashes="stashes"
			:key="league"
			:league="league"
		></wc-tab-badge-group>
	</div>
</template>

<style scoped>
.main-stashes-component {
	position: relative;
	padding: 1rem;
	border: 2px solid #000;
	border-radius: 0.25rem;
}

.btn-load-items {
	border: 2px solid transparent;
	text-transform: uppercase;
	visibility: hidden;
}

.btn-load-items:not(:disabled) {
	transform: scale(1.25);
	border-color: purple;
}

.league-stashes {
	max-width: max-content;
	display: flex;
	align-items: center;
	gap: 1rem;
}

.msg {
	font-size: 2rem;
	max-width: max-content;
	margin-inline: auto;
	margin-top: 1rem;
	visibility: hidden;
	min-height: 2rem;
}

.visible {
	visibility: visible;
}

.controls {
	display: flex;
	justify-content: space-between;
}
</style>
