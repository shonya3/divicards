<script setup lang="ts">
import TabBadge from './TabBadge.vue';
import { StatefulStashTab } from '../../stores/stash';
defineProps<{ stashes: StatefulStashTab[] }>();
</script>

<template>
	<div>
		<ul class="tab-badge-group">
			<li v-for="tab in stashes">
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
	display: flex;
	flex-wrap: wrap;
	list-style: none;
	gap: 5px;
	margin-inline: 1rem;

	padding: 2rem;
	/* border: 1px solid red; */
}
</style>
