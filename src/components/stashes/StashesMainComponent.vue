<script setup lang="ts">
import TabBadgeGroup from './TabBadgeGroup.vue';
import { useStashStore } from '../../stores/stash';
import { useLoadStash } from '../../poe/useLoadStash';
import { League, leagues } from '../../types';
import { storeToRefs } from 'pinia';
import { ACTIVE_LEAGUE } from '../../lib';
import { ref, watch } from 'vue';
import HelpTip from '../HelpTip.vue';

const stashStore = useStashStore();
const { stashes, selectedTabsIds, league } = storeToRefs(stashStore);
const { fetchStashes, deleteTabs, unselectAllTabs } = stashStore;

const { msg, fetchingStash, fetchStashesContents } = useLoadStash();

const onGetData = async () => {
	fetchStashesContents(selectedTabsIds.value, league.value);
	unselectAllTabs();
};

defineEmits<{
	(event: 'close'): void;
}>();

watch(
	() => league.value,
	() => deleteTabs()
);
</script>

<template>
	<div class="main-stashes-component">
		<div class="controls">
			<div class="league-stashes">
				<div class="league">
					<label for="league">League</label>
					<select id="league" v-model="league">
						<option v-for="league in leagues" :value="league">{{ league }}</option>
					</select>
				</div>
				<button @click="fetchStashes(league)">Stashes</button>
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

		<TabBadgeGroup :stashes="stashes" :key="league" />
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
