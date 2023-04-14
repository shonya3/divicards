<script setup lang="ts">
import { SortState, Column, WeightedCardRecord } from '../../types';
import { ref, computed, onMounted, watch } from 'vue';
import { orderBy } from './orderBy';
import OrderTriangle from '../OrderTriangle/OrderTriangle.vue';
const nf = new Intl.NumberFormat('ru', { maximumFractionDigits: 0 });
const props = defineProps<{ records: WeightedCardRecord[] }>();
const nameQuery = ref('');
const minPrice = ref(0);
const hideZeroStackSize = ref(true);

const filteredRecords = computed(() => {
	return props.records.slice().filter(({ name, calculated, stackSize }) => {
		if (hideZeroStackSize.value) {
			if (stackSize == 0) return false;
		}

		return (
			name.toLocaleLowerCase().includes(nameQuery.value.trim().toLocaleLowerCase()) &&
			calculated >= minPrice.value
		);
	});
});
const summary = computed(() => {
	return filteredRecords.value.reduce(
		({ stackSize, total }, current) => {
			stackSize += current.stackSize;
			total += current.total;
			return { stackSize, total };
		},
		{
			stackSize: 0,
			total: 0,
		}
	);
});
const order = ref<SortState>({
	activeColumn: 'price',
	stackSize: 'asc',
	price: 'asc',
	total: 'asc',
});

watch(
	[() => minPrice.value, () => order.value, () => nameQuery.value],
	() => orderBy(order.value.activeColumn, order.value[order.value.activeColumn], filteredRecords.value),
	{ deep: true }
);

const toggleOrder = (column: Column) => {
	order.value[column] = order.value[column] === 'asc' ? 'desc' : 'asc';
	order.value.activeColumn = column;
};

onMounted(() => {
	toggleOrder('total');
});
</script>

<template>
	<div class="table-container">
		<header class="header">
			<label for="filter-card-name">Enter name</label>
			<input autofocus type="text" id="filter-card-name" v-model="nameQuery" />
			<span class="stats"
				>found
				<span class="ch-6">{{ filteredRecords.length }} </span>
				card names
				<span class="ch-6">({{ summary.stackSize }}</span>
				cards,
				<span class="ch-7">{{ nf.format(summary.total) }}</span>
				<img width="20" height="20" class="chaos-img" src="/chaos.png" alt="chaos" />)</span
			>
			<label class="slider-box">
				<span>min price </span>
				<input class="slider" type="range" name="" id="" min="0" max="500" v-model.number="minPrice" />
				<span class="ch-3">{{ minPrice }}</span>
				<img width="20" height="20" class="chaos-img" src="/chaos.png" alt="chaos" />
			</label>
			<div style="display: flex; gap: 0.8rem">
				<span>hide names with zero stack size</span>
				<input type="checkbox" name="" id="" v-model="hideZeroStackSize" />
			</div>
			<!-- <div>download filtered file</div> -->
		</header>
		<table class="table">
			<colgroup>
				<col span="1" class="col" />
				<col span="1" class="col" />
				<col span="1" class="col-name" />
				<col span="1" class="col" />
				<col span="1" class="col" />
				<col span="1" class="col" />
			</colgroup>

			<thead>
				<tr>
					<th>&numero;</th>
					<th>
						<span class="column__name"> Stack Size </span>
						<order-triangle
							:active="order.activeColumn === 'stackSize'"
							:order="order.stackSize"
							@click="toggleOrder('stackSize')"
						/>
					</th>
					<th>
						<span class="column__name"> Name </span>
					</th>
					<th>
						<span class="column__name"> Price </span>
						<order-triangle
							:active="order.activeColumn === 'price'"
							:order="order.price"
							@click="toggleOrder('price')"
						/>
					</th>
					<th>
						<span class="column__name"> Total </span>
						<order-triangle
							:active="order.activeColumn === 'total'"
							:order="order.total"
							@click="toggleOrder('total')"
						/>
					</th>
					<th>
						<span class="column__name"> Weight </span>
					</th>
				</tr>
			</thead>

			<tbody>
				<tr
					class="columns"
					v-for="({ stackSize, name, calculated, total, realWeight }, index) in filteredRecords"
				>
					<td class="row">{{ index + 1 }}</td>
					<td class="row">{{ stackSize }}</td>
					<td class="name-row">{{ name }}</td>
					<td class="row">{{ nf.format(calculated) }}</td>
					<td class="row">{{ nf.format(total) }}</td>
					<td class="row">
						{{
							// ((70_000 * stackSize) / allStackSize) ** (3 / 2)
							// nf.format(calcRecordRealWeight({ stackSize, name, calculated, total }))
							nf.format(realWeight)
						}}
					</td>
				</tr>
			</tbody>
		</table>
	</div>
</template>

<style scoped>
.ch-3 {
	/* display: block; */
	text-align: center;
	min-width: 3ch;
}
.ch-6 {
	text-align: center;
	min-width: 6ch;
}
.ch-7 {
	text-align: center;
	min-width: 7ch;
}
.slider-box {
	display: flex;
	justify-content: center;
	align-items: center;
	gap: 0.5rem;
}
.stats {
	display: flex;
	align-items: center;
}
.table-container {
	display: flex;
	flex-direction: column;
	gap: 2rem;
	/* --col-name-width: 650px; */
	/* --col-width: calc((100% - var(--col-name-width)) / 4); */
	height: 100%;
	max-width: 1200px;

	color: var(--color);
	background-color: var(--bg-color);
}

.table {
	width: 100%;
}

.header {
	display: flex;
	gap: 1rem;
	align-items: center;
	flex-wrap: wrap;
}

tr {
	display: grid;
	grid-template-columns: 0.5fr 1.2fr 3fr 1fr 1fr 1fr;
}

.table > thead > tr > th {
	display: flex;
	gap: 0.5rem;
}

.column__name {
	overflow-x: hidden;
	white-space: nowrap;
	/* font-size: 18px; */
	/* min-width: 50px; */
}
</style>
