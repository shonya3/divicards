<script setup lang="ts">
import { DivinationCardsSample, League, leagues, DivinationCardRecord } from '../../types';
import GridIcon from '../icons/GridIcon.vue';
import { ref, computed, Ref, ComputedRef } from 'vue';
import DivTable from '../DivTable/DivTable.vue';
import BasePopup from '../BasePopup.vue';
import FixedNamesList from './FixedNamesList/FixedNamesList.vue';
import NotCardsList from './NotCardsList/NotCardsList.vue';
import LeagueSelect from '../LeagueSelect.vue';
import { A } from '@tauri-apps/api/path-c062430b';

const league = defineModel<League>('league', { required: true });
const minimumCardPrice = defineModel<number>('minimumCardPrice', { required: true });
const selected = defineModel<boolean>('selected', { required: true });

export interface FileCardProps {
	league: League;
	filename: string;
	href: string;
	selected: boolean | null;
	id: string;
	valid: boolean;
	error: string | null;
	minimumCardPrice: number;
	sample: DivinationCardsSample;
	isReady: boolean;
}

const props = defineProps<FileCardProps>();
console.log({ fileCardProps: props });
defineEmits<{
	(event: 'update:selected', e: InputEvent): void;
	(event: 'delete', id: string): void;
	(event: 'update:minimumCardPrice', newPrice: number): void;
	(event: 'update:league', league: League): void;
}>();

const { format } = new Intl.NumberFormat('ru', { maximumFractionDigits: 0 });

const tablePopup = ref<typeof BasePopup | null>(null);

const filteredCards = computed(() => {
	return props.sample.cards.filter(card => {
		return (card.price ?? 0) >= props.minimumCardPrice;
	});
});
const filteredSummary = computed(() => {
	let value = 0;
	let amount = 0;

	for (const card of filteredCards.value) {
		value += card.sum ?? 0;
		amount += card.amount;
	}

	return {
		amount,
		value,
	};
});
</script>

<template>
	<div class="file" :class="{ 'file-error': error, 'file-selected': selected }">
		<p class="filename" :class="{ 'filename--error': error }">{{ filename }}</p>
		<button @click="$emit('delete', id)" class="btn-delete">X</button>

		<template v-if="isReady">
			<p v-if="error">{{ error }}</p>
			<template v-else-if="!error">
				<div class="minor-icons">
					<fixed-names-list :fixed-names="sample.fixedNames" />
					<not-cards-list :not-cards="sample.notCards"></not-cards-list>
				</div>
				<GridIcon class="icon" :width="96" :height="96" @click="tablePopup?.open()" />
				<label class="slider-box" v-if="valid">
					<span>{{ minimumCardPrice }}</span>
					<input class="slider" type="range" name="" id="" min="0" max="500" v-model="minimumCardPrice" />
				</label>
				<div v-if="valid" class="total-price">
					<p>{{ format(filteredSummary.value) }}</p>
					<img width="35" height="35" class="chaos-img" src="/chaos.png" alt="chaos" />
				</div>
				<div class="cards-amount">
					<p>{{ filteredSummary.amount }}</p>
					<img width="35" height="35" src="/divination-card.png" alt="Divination card" />
				</div>

				<LeagueSelect v-model="league" />

				<a class="download" v-if="valid" :download="filename" :href="href">Download</a>
				<input
					class="checkbox"
					v-if="valid && selected != null"
					type="checkbox"
					:checked="selected"
					@change="(e) => $emit('update:selected', (e.target as HTMLInputElement).checked)"
				/>

				<base-popup ref="tablePopup">
					<div-table v-if="valid" :cards="sample.cards" />
					<p v-else>{{ error }}</p>
				</base-popup>
			</template>
		</template>
	</div>
</template>

<style scoped>
.league {
	display: flex;
	gap: 0.4rem;
}

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
	min-height: 400px;

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
	font-size: 1rem;
	letter-spacing: -0.4px;
	overflow: hidden;
	max-height: 60px;
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

.total-price,
.cards-amount {
	display: flex;
	justify-content: center;
	align-items: center;
	font-size: 2rem;
}
</style>
