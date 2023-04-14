<script setup lang="ts">
import BasePopup from '../../BasePopup.vue';
import WarningIcon from './WarningIcon.vue';
import { ref } from 'vue';

const notCardPopup = ref<typeof BasePopup | null>(null);

defineProps<{
	notCards: string[];
}>();
</script>

<template>
	<WarningIcon v-if="Object.keys(notCards).length" @click="notCardPopup?.open()" :width="24" :height="24" />
	<BasePopup ref="notCardPopup">
		<div class="fixed-names">
			<h2>Probably not cards</h2>
			<ul class="list">
				<li v-for="(notCard, i) in notCards">{{ notCard }}</li>
			</ul>
		</div>
	</BasePopup>
</template>

<style scoped>
.fixed-names-list {
	margin-top: 2rem;
	/* max-width: 400px; */
}

.list-item {
	display: flex;
	align-items: center;
	gap: 20px;
}

.input-name {
	overflow-x: hidden;
	width: 200px;
	/* background-color: red; */
	opacity: 60%;
}

.list {
	margin-top: 1rem;
}
</style>
