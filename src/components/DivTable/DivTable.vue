<script setup lang="ts">
import { DivinationCardRecord, Order } from '../../types';
import { ref, computed, onMounted, watch } from 'vue';
import { orderBy } from './orderBy';
import { SortState, Column } from './types';
import { OrderTriangleElement } from '../wc/order-triangle';
OrderTriangleElement.define();

export interface DivTableProps {
	cards: DivinationCardRecord[];
}

const props = defineProps<DivTableProps>();
const { format } = new Intl.NumberFormat('ru', { maximumFractionDigits: 0 });
const nameQuery = ref('');
const minPrice = ref(0);
const hideZerosum = ref(false);

const filteredRecords = computed(() => {
	return props.cards.slice().filter(({ name, price, sum }) => {
		if (hideZerosum.value) {
			if (sum === 0 || sum === null) return false;
		}

		return (
			name.toLocaleLowerCase().includes(nameQuery.value.trim().toLocaleLowerCase()) &&
			(price ?? 0) >= minPrice.value
		);
	});
});
const summary = computed(() => {
	let sum = 0;
	let amount = 0;

	for (const record of filteredRecords.value) {
		sum += record.sum ?? 0;
		amount += record.amount;
	}

	return {
		amount,
		sum,
	};
});
const order = ref<SortState>({
	activeColumn: 'price',
	amount: 'asc',
	price: 'asc',
	sum: 'asc',
	name: 'asc',
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
	toggleOrder('sum');
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
				<span class="ch-6">({{ summary.amount }}</span>
				cards,
				<span class="ch-7">{{ format(summary.sum) }}</span>
				<img width="20" height="20" class="chaos-img" src="/chaos.png" alt="chaos" />)</span
			>
			<label class="slider-box">
				<span>min price </span>
				<input class="slider" type="range" name="" id="" min="0" max="500" v-model.number="minPrice" />
				<span class="ch-3">{{ minPrice }}</span>
				<img width="20" height="20" class="chaos-img" src="/chaos.png" alt="chaos" />
			</label>
			<div style="display: flex; gap: 0.8rem">
				<span>hide names with zero sum</span>
				<input type="checkbox" name="" id="" v-model="hideZerosum" />
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
						<span class="column__name"> Amount </span>
						<wc-order-triangle
							:active="order.activeColumn === 'amount'"
							:order="order.amount"
							@click="toggleOrder('amount')"
						></wc-order-triangle>
					</th>
					<th>
						<span class="column__name"> Name </span>
						<wc-order-triangle
							:active="order.activeColumn === 'name'"
							:order="order.name"
							@click="toggleOrder('name')"
						/>
					</th>
					<th>
						<span class="column__name"> Price </span>
						<wc-order-triangle
							:active="order.activeColumn === 'price'"
							:order="order.price"
							@click="toggleOrder('price')"
						/>
					</th>
					<th>
						<span class="column__name"> Sum </span>
						<wc-order-triangle
							:active="order.activeColumn === 'sum'"
							:order="order.sum"
							@click="toggleOrder('sum')"
						/>
					</th>
					<th>
						<span class="column__name"> Weight </span>
					</th>
				</tr>
			</thead>

			<tbody>
				<tr class="columns" v-for="({ amount, name, price, sum, weight }, index) in filteredRecords">
					<td class="row">{{ index + 1 }}</td>
					<td class="row">{{ amount }}</td>
					<td class="name-row">{{ name }}</td>
					<td class="row">{{ format(price ?? 0) }}</td>
					<td class="row">{{ format(sum ?? 0) }}</td>
					<td class="row">{{ format(weight) }}</td>
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
