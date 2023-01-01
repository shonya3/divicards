<script setup lang="ts">
import ArrowRight from './icons/ArrowRight.vue';

defineProps<{
	fixedNames: Record<string, string>;
}>();
</script>

<template>
	<div class="fixed-names">
		<h2>Automatically fixed typos</h2>
		<ul class="fixed-names-list">
			<li class="list-item" v-for="[inputName, fixedName] in Object.entries(fixedNames)">
				<span class="input-name">{{ inputName }}</span>
				<ArrowRight />
				<span class="fixed-name">{{ fixedName }}</span>
			</li>
		</ul>
	</div>
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
</style>
