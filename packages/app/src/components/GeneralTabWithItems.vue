<script setup lang="ts">
import { TabWithItems } from '@divicards/shared/poe.types';
import { NameAmount } from '@divicards/shared/types';
import '@shoelace-style/shoelace/dist/components/icon-button/icon-button.js';
import { Ref, computed, ref, watch } from 'vue';
const props = defineProps<{ tab: TabWithItems }>();
const emits = defineEmits(['close']);
const nameAmountPairs: Ref<NameAmount[]> = ref([]);
const mergeable = computed(() => {
	return Object.values(Object.groupBy(nameAmountPairs.value, ({ name }) => name)).some((arr = []) => arr.length > 1);
});
const tableAsClipboardString = computed(
	() => 'name\tamount\n' + nameAmountPairs.value.map(({ name, amount = '' }) => `${name}\t${amount}`).join('\n')
);

watch(
	() => props.tab.items,
	items => {
		nameAmountPairs.value = items.map(({ baseType, stackSize }) => ({ name: baseType, amount: stackSize ?? 0 }));
	},
	{ immediate: true }
);

function onMergeStacksClick() {
	nameAmountPairs.value = Object.entries(Object.groupBy(nameAmountPairs.value, ({ name }) => name)).flatMap(
		([name, pairs = []]) => ({
			name,
			amount: pairs.reduce((sum, { amount = 0 }) => (sum += amount), 0),
		})
	);
}
</script>

<template>
	<div class="component">
		{{ tab.name }}
		<details>
			<summary>
				Amounts table
				<sl-copy-button :value="tableAsClipboardString"></sl-copy-button>
				<sl-button v-if="mergeable" @click="onMergeStacksClick"> Merge stacks </sl-button>
			</summary>
			<table id="cards" class="table is-bordered is-narrow is-hoverable">
				<thead>
					<tr>
						<th>Card</th>
						<th>Count</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="{ name, amount } in nameAmountPairs">
						<td>{{ name }}</td>
						<td>{{ amount }}</td>
					</tr>
				</tbody>
			</table>
		</details>
		<details>
			<summary>Json <sl-copy-button :value="JSON.stringify(tab)"></sl-copy-button></summary>
			<pre>{{ tab }}</pre>
		</details>
		<sl-icon-button @click="$emit('close')" class="btn-close" name="x-lg"></sl-icon-button>
	</div>
</template>

<style scoped>
.component {
	position: relative;
	margin: 1px solid red;
}
.component {
	min-width: 400px;
	max-width: 100%;
	overflow-x: hidden;
	position: relative;
	padding-inline: 1rem;
	padding-top: 1.4rem;
	padding-bottom: 0.4rem;
	gap: 1rem;
	width: fit-content;
	box-shadow: rgba(0, 0, 0, 0.02) 0px 1px 3px 0px, rgba(27, 31, 35, 0.15) 0px 0px 0px 1px;
	border: 1px solid black;
	border-color: var(--border-color);
	border-radius: var(--border-radius);
	background-color: var(--bg-color);
	transition: 0.2s border-color;
}
.btn-close {
	position: absolute;
	top: 0;
	left: 400px;
	transform: translateX(-100%);
	padding: 0.2rem;
	border: none;
	background-color: transparent;
	cursor: pointer;
}
</style>
