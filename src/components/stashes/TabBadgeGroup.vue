<script setup lang="ts">
import TabBadge from './TabBadge.vue';
import { StatefulStashTab } from '../../stores/stash';
import { ref, computed, watch } from 'vue';
import { League } from '../../types';
import { filter, shouldUnlockHideRemoveOnly, paginate } from './utils';
const props = defineProps<{ stashes: StatefulStashTab[]; league: League }>();
const shouldFilter = computed(() => props.stashes.length > 50);
const withHideRemoveOnly = computed(() => shouldUnlockHideRemoveOnly(props.league, props.stashes));
const hideRemoveOnly = ref(false);

const nameQuery = ref('');
const page = ref(1);
const perPage = ref(50);

const filtered = computed(() => filter(props.stashes, nameQuery.value, shouldFilter.value, hideRemoveOnly.value));
const paginated = computed(() => paginate(filtered.value, page.value, perPage.value));

watch(
	() => [nameQuery.value, perPage.value, hideRemoveOnly.value],
	() => {
		page.value = 1;
	}
);
</script>

<template>
	<div class="tab-badge-group">
		<div v-if="shouldFilter" style="display: flex; align-items: center; gap: 2rem">
			<div>
				<div class="filter" v-if="shouldFilter">
					<label for="filter-stashes-by-name">Filter by name</label>
					<input type="text" id="filter-stashes-by-name" v-model="nameQuery" />
				</div>
			</div>
			<div class="page-controls" v-if="shouldFilter">
				<button :disabled="page === 1" @click="page > 1 && page--">prev</button>
				<input type="text" v-model.number="page" />
				<button @click="page++">next</button>
				<span>per page</span>
				<input type="text" v-model.number="perPage" />
			</div>
			<div v-if="withHideRemoveOnly" class="hide-remove-only">
				<label for="hide-remove-only">Hide remove-only</label>
				<input type="checkbox" v-model="hideRemoveOnly" />
			</div>
		</div>
		<ul class="list">
			<li v-for="(tab, index) in paginated">
				<TabBadge
					:colour="tab.metadata?.colour ?? '#fff'"
					:name="tab.name"
					:tab-id="tab.id"
					v-model:selected="tab.selected"
				/>
			</li>
		</ul>
	</div>
</template>

<style scoped>
.tab-badge-group {
	display: grid;
	gap: 1rem;
}
.filter {
	display: flex;
	gap: 0.4rem;
}

.page-controls {
	display: flex;
	gap: 0.4rem;
	align-items: center;
}

.page-controls > input {
	width: 5ch;
	text-align: center;
}

.hide-remove-only {
	display: flex;
	align-items: center;
	gap: 0.2rem;
}

.list {
	display: flex;
	flex-wrap: wrap;
	list-style: none;
	gap: 5px;
	margin-inline: 1rem;
}
</style>
