<script setup lang="ts">
import { leagues, tradeLeagues, League } from '../types';
const modelValue = defineModel<League>({ required: true });
defineProps<{
	trade: boolean;
}>();
</script>

<template>
	<div class="league-select">
		<label for="league">League</label>
		<select v-model="modelValue" ref="selectEl" id="league">
			<option v-for="league in trade ? tradeLeagues : leagues" :value="league">{{ league }}</option>
		</select>
	</div>
</template>

<style scoped>
.league-select {
	display: flex;
	gap: 0.2rem;
}
</style>
