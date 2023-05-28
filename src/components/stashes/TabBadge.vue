<script setup lang="ts">
import { computed } from 'vue';

const props = withDefaults(
	defineProps<{
		colour: string;
		name: string;
		tabId: string;
		selected: boolean;
	}>(),
	{ selected: false }
);

defineEmits<{
	(event: 'update:selected', e: InputEvent): void;
}>();

const color = computed(() => `#${props.colour.padStart(6, '0')}`);
</script>

<template>
	<div class="tab-badge">
		<label :for="`tab-${tabId}`" class="name">{{ name }}</label>
		<input
			@change="(e) => $emit('update:selected', (e.target as HTMLInputElement).checked)"
			class="checkbox"
			type="checkbox"
			:name="`tab-${tabId}`"
			:id="`tab-${tabId}`"
			:checked="selected"
		/>
	</div>
</template>

<style scoped>
.tab-badge {
	--badge-color: v-bind(color);
	width: 8rem;
	height: 4rem;
	aspect-ratio: 1;
	display: flex;
	justify-content: center;
	align-items: center;

	border-radius: 2rem;
	border: 1px solid #000;
	overflow: clip;

	background-color: var(--badge-color);
	position: relative;
}

.tab-badge:hover {
	overflow: initial;
}
.tab-badge:hover .name {
	position: absolute;
}

.name {
	color: var(--badge-color);
	mix-blend-mode: difference;
	font-size: 0.9rem;
}

.checkbox {
	position: absolute;
	appearance: none;
	height: 100%;
	width: 100%;
	cursor: pointer;
}

input:checked {
	appearance: initial;
}

.tab-badge:has(.checkbox:checked) {
	transform: scale(1.4);
	z-index: 2;
}
</style>
