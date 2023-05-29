<script setup lang="ts">
import { leagues, League } from '../types';
const modelValue = defineModel<League>({ required: true });
</script>

<template>
	<div class="league-select">
		<label for="league">League</label>
		<select v-model="modelValue" ref="selectEl" id="league">
			<option v-for="league in leagues" :value="league">{{ league }}</option>
		</select>
	</div>
</template>

<style scoped>
.league-select {
	display: flex;
	gap: 0.2rem;
}
</style>
