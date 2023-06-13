<script setup lang="ts">
import TabBadgeGroup from './TabBadgeGroup.vue';
import { useStashStore } from '../../stores/stash';
import { League } from '../../types';
import { storeToRefs } from 'pinia';
import { ref, watch } from 'vue';
import HelpTip from '../HelpTip.vue';
import LeagueSelect from '../LeagueSelect.vue';
import { useLoadStash } from '../../composables/useLoadStash';

const stashStore = useStashStore();
const { stashes, selectedTabsIds, league } = storeToRefs(stashStore);
const { fetchStashes, deleteTabs, unselectAllTabs } = stashStore;

const { msg, fetchingStash, fetchStashesContents } = useLoadStash();

const selectEl = ref<HTMLSelectElement | null>(null);

const onGetData = async () => {
	fetchStashesContents(selectedTabsIds.value, league.value);
	unselectAllTabs();
};

const noStashesMessage = ref('');

const onStashes = async (league: League) => {
	noStashesMessage.value = '';
	await fetchStashes(league);
	if (!stashes.value.length) {
		noStashesMessage.value = 'No stashes here. Try to change the league';
		selectEl.value?.focus();
	}
};

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
</script>

<template>
	<div class="main-stashes-component">
		<div class="controls">
			<div class="league-stashes">
				<LeagueSelect :trade="false" v-model="league" />
				<button @click="onStashes(league)">Stashes</button>
				<HelpTip>
					<p>Select tabs by clicking on them. Then click LOAD ITEMS button</p>
				</HelpTip>
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

		<TabBadgeGroup :stashes="stashes" :key="league" :league="league" />
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
