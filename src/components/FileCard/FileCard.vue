<script setup lang="ts">
import { FileContents, CardRecord, WeightedCardRecord } from '../../types';
import CSVIcon from '../icons/CSVIcon.vue';
import { ref, watch } from 'vue';
import DivTable from '../DivTable/DivTable.vue';
import { command } from '../../lib';
import FixedIcon from './FixedIcon.vue';
import BasePopup from '../BasePopup.vue';
import FixedNamesList from '../FixedNamesList.vue';

export interface Contents {
	fileContent: FileContents;
	selected: boolean | null;
	id: string;
	valid: boolean;
	error: string | null;
	price: number;
	records: WeightedCardRecord[];
	notCards: string[];
	fixedNames: Record<string, string>;
}

const props = defineProps<Contents>();
const emit = defineEmits<{
	(event: 'update:selected', e: InputEvent): void;
	(event: 'minimum-price-updated', price: number): void;
	(event: 'delete-me', id: string): void;
}>();

const nf = new Intl.NumberFormat('ru', { maximumFractionDigits: 0 });

const minimumChaosPrice = ref(50);
watch(
	() => minimumChaosPrice.value,
	async val => {
		if (!props.valid) return;
		const price = await command('total_chaos', {
			csvString: props.fileContent.text,
			minimumCardPrice: val,
		});

		emit('minimum-price-updated', price);
	},
	{ immediate: true }
);

const tablePopup = ref<typeof BasePopup | null>(null);
const fixedNamesPopup = ref<typeof BasePopup | null>(null);
</script>

<template>
	<pre v-if="notCards.length">{{ notCards }}</pre>
	<div class="file" :class="{ 'file-error': error, 'file-selected': selected }">
		<div class="minor-icons">
			<FixedIcon
				v-if="Object.keys(fixedNames).length"
				@click="fixedNamesPopup?.open()"
				:width="36"
				:height="36"
			/>
		</div>
		<p class="filename" :class="{ 'filename--error': error }">{{ fileContent.filename }}</p>
		<CSVIcon class="icon" :width="96" :height="96" @click="tablePopup?.open()" />
		<label class="slider-box" v-if="valid">
			<span>{{ minimumChaosPrice }}</span>
			<input class="slider" type="range" name="" id="" min="0" max="500" v-model.number="minimumChaosPrice" />
		</label>
		<div v-if="valid" class="total-price">
			<p>{{ nf.format(price) }}</p>
			<img width="35" height="35" class="chaos-img" src="/chaos.png" alt="chaos" />
		</div>
		<a class="download" v-if="valid" :download="fileContent.filename" :href="fileContent.href">Download</a>
		<button @click="$emit('delete-me', id)" class="btn-delete">X</button>
		<input
			class="checkbox"
			v-if="valid && selected != null"
			type="checkbox"
			:checked="selected"
			@change="e => $emit('update:selected', (e.target as HTMLInputElement).checked)"
		/>

		<base-popup ref="fixedNamesPopup">
			<fixed-names-list :fixed-names="fixedNames" />
		</base-popup>

		<base-popup ref="tablePopup">
			<div-table v-if="valid" :records="records" />
			<p v-else>{{ error }}</p>
		</base-popup>
	</div>
</template>

<style scoped>
.minor-icons {
	position: absolute;
	top: 30%;
	left: 20px;
}
.file {
	position: relative;
	padding: 1rem;
	padding-block: 1.4rem;
	display: flex;
	flex-direction: column;
	align-items: center;
	justify-content: center;
	gap: 1rem;
	width: fit-content;
	box-shadow: rgba(0, 0, 0, 0.02) 0px 1px 3px 0px, rgba(27, 31, 35, 0.15) 0px 0px 0px 1px;

	width: 250px;
	/* max-height: 320px; */
	height: 320px;

	border: 2px solid black;
	border-color: var(--border-color);
	transition: 0.2s border-color;
}

.icon {
	cursor: pointer;
}

.file-error {
	border-color: red;
}

.file-selected {
	border-color: green;
}

.filename {
	font-size: 1.5rem;
	letter-spacing: -0.4px;
	overflow: hidden;
	max-height: 30px;
	max-width: 100%;
}

.filename:hover {
	overflow: visible;
	/* position: absolute; */
}

.filename--error {
	color: red;
}

.slider-box {
	display: flex;
	justify-content: center;
	align-items: center;
	gap: 0.5rem;
}

.btn-delete {
	position: absolute;
	top: 0;
	right: 0;
	/* transform: translate(-50%, 50%); */
	padding: 0.2rem;
	border: none;
	background-color: transparent;
	cursor: pointer;
}

.checkbox {
	background-color: red;
	padding: 1rem;
	transform: scale(2);
	accent-color: green;
	cursor: pointer;

	position: absolute;
	bottom: 0;
	right: 0;
	transform: translate(15%, 15%) scale(2);
}

.download {
	/* position: absolute; */
	bottom: 0;
}

.total-price {
	display: flex;
	justify-content: center;
	align-items: center;
	font-size: 2rem;
}
</style>
