<script setup lang="ts">
import { CardRecord, Order } from '../../types';
import { ref, computed, onMounted, watch, ComputedRef } from 'vue';
import { orderBy, byPrice, byStackSize, byTotal } from './orderBy';
const props = defineProps<{ records: CardRecord[] }>();
const nameQuery = ref('');
const records = computed(() => props.records.slice());

const minPrice = ref(0);

console.log(props.records);

const sortedColumn = ref<'price' | 'stackSize' | 'total'>('total');

const orderBy = (column: 'price' | 'stackSize' | 'total', order: Order, records: CardRecord[]): CardRecord[] => {
	sortedColumn.value = column;
	switch (column) {
		case 'price':
			return byPrice(order, records);
		case 'stackSize':
			return byStackSize(order, records);
		case 'total':
			return byTotal(order, records);
		default:
			throw new Error('Invalid column name');
	}
};

let filteredRecords = computed(() => {
	return props.records
		.slice()
		.filter(
			({ name, calculated }) =>
				name.toLocaleLowerCase().includes(nameQuery.value.trim().toLocaleLowerCase()) &&
				calculated >= minPrice.value
		);
});

const allCardsStackSizeAndTotal = computed(() => {
	let allStackSize = 0;
	let allTotal = 0;
	for (let i = 0; i < filteredRecords.value.length; i++) {
		const { stackSize, total } = filteredRecords.value[i];
		allStackSize += Number(stackSize ?? 0);
		allTotal += Number(total ?? 0);
	}
	return {
		stackSize: allStackSize,
		total: allTotal,
	};
});

const nf = new Intl.NumberFormat('ru', { maximumFractionDigits: 0 });

const priceOrder = ref<Order>('asc');
const priceOrderTransform = ref(0);
const rot = computed(() => `rotate(${priceOrderTransform.value}deg)`);
watch([() => priceOrder.value], val => {
	console.log(filteredRecords.value[0].name);
	console.log(records.value[0].name);
	console.log('price order value');
	priceOrderTransform.value += 180;
	orderBy('price', val[0], filteredRecords.value);
});

watch(
	() => minPrice.value,
	val => {
		orderBy(sortedColumn.value, priceOrder.value, filteredRecords.value);
	}
);

const stackSizeOrder = ref<Order>('asc');
const stackSizeOrderTransform = ref(0);
const stackSizeRotate = computed(() => `rotate(${stackSizeOrderTransform.value}deg)`);
watch(
	() => stackSizeOrder.value,
	val => {
		console.log('order');
		stackSizeOrderTransform.value += 180;
		orderBy('stackSize', val, records.value);
	}
);

const order = {
	activeOrder: 'price',
	stackSize: 'asc',
	price: 'asc',
	total: 'asc',
};

const totalOrder = ref<Order>('asc');
const totalOrderTransform = ref(0);
const totalRotate = computed(() => `rotate(${totalOrderTransform.value}deg)`);
watch(
	() => totalOrder.value,
	val => {
		console.log('order');
		totalOrderTransform.value += 180;
		orderBy('total', val, records.value);
	}
);

const toggleOrder = (order: Order): Order => {
	return order === 'asc' ? 'desc' : 'asc';
};

watch(
	() => filteredRecords.value,
	() => {
		// orderBy('total', 'desc', filteredRecords.value);
	}
);

totalOrder.value = 'desc';
</script>

<template>
	<div class="table-container">
		<header class="header">
			<label for="filter-card-name">Enter name</label>
			<input autofocus type="text" id="filter-card-name" v-model="nameQuery" />
			<span class="stats"
				>found
				<span class="ch-6"> {{ filteredRecords.length }} </span>
				card names (
				<span class="ch-6">{{ allCardsStackSizeAndTotal['stackSize'] }}</span>
				cards,
				<span class="ch-7">{{ nf.format(allCardsStackSizeAndTotal['total']) }}</span>
				<img width="20" height="20" class="chaos-img" src="/chaos.png" alt="chaos" />)</span
			>
			<label class="slider-box">
				<span>min price </span>
				<input class="slider" type="range" name="" id="" min="0" max="500" v-model.number="minPrice" />
				<span class="ch-3">{{ minPrice }}</span>
				<img width="20" height="20" class="chaos-img" src="/chaos.png" alt="chaos" />
			</label>
		</header>
		<table class="table">
			<colgroup>
				<col span="1" class="col" />
				<col span="1" class="col" />
				<col span="1" class="col-name" />
				<col span="1" class="col" />
				<col span="1" class="col" />
			</colgroup>

			<thead>
				<tr>
					<th>&numero;</th>
					<th>
						Stack Size
						<div
							:style="{ '--rot': stackSizeRotate }"
							class="order"
							@click="stackSizeOrder = toggleOrder(stackSizeOrder)"
						></div>
					</th>
					<th>Name</th>
					<th>
						Price
						<div
							:style="{ '--rot': rot }"
							class="order"
							@click="priceOrder = toggleOrder(priceOrder)"
						></div>
					</th>
					<th>
						Total
						<div
							:style="{ '--rot': totalRotate }"
							class="order"
							@click="totalOrder = toggleOrder(totalOrder)"
						></div>
					</th>
				</tr>
			</thead>

			<tbody>
				<tr v-for="({ stackSize, name, calculated, total }, index) in filteredRecords">
					<td class="row">{{ index + 1 }}</td>
					<td class="row">{{ stackSize }}</td>
					<td class="name-row">{{ name }}</td>
					<td class="row">{{ nf.format(calculated) }}</td>
					<td class="row">{{ nf.format(total) }}</td>
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
}

.table {
	width: 100%;
}

.header {
	display: flex;
	gap: 1rem;
}

.order {
	/* position: absolute;
	right: 0;
	top: 0; */
	width: 16px;
	height: 16px;
	clip-path: polygon(0% 100%, 50% 0%, 100% 100%);
	background-color: #000;

	transition: transform 300ms;
	transform: var(--rot);
	cursor: pointer;
}

tr {
	display: grid;
	grid-template-columns: 1fr 1fr 3fr 1fr 1fr;
}

.table > thead > tr > th {
	display: flex;
	gap: 0.5rem;
}
</style>
