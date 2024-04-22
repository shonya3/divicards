<script setup lang="ts">
import { TabWithItems } from '@divicards/shared/poe.types';
import { CardNameAmount } from '@divicards/shared/types';
import { Ref, computed, ref, watch } from 'vue';
const props = defineProps<{ tab: TabWithItems }>();
const nameAmountPairs: Ref<CardNameAmount[]> = ref([]);
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

const onMergeStacksClick = () => {
	nameAmountPairs.value = Object.entries(Object.groupBy(nameAmountPairs.value, ({ name }) => name)).flatMap(
		([name, pairs = []]) => ({
			name,
			amount: pairs.reduce((sum, { amount = 0 }) => (sum += amount), 0),
		})
	);
};
</script>

<template>
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
</template>

<style scoped></style>
