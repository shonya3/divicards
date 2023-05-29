<script setup lang="ts">
import { computed } from 'vue';

const props = withDefaults(
	defineProps<{
		colour: string;
		name: string;
		tabId: string;
		selected: boolean;
		index: number;
	}>(),
	{ selected: false }
);

defineEmits<{
	(event: 'update:selected', e: InputEvent): void;
}>();

const color = computed(() => `#${props.colour.padStart(6, '0')}`);
const id = computed(() => `tab-id-${props.tabId}`);
</script>

<template>
	<div class="tab-badge">
		<label :for="id" class="name">{{ name }}</label>
		<input
			@change="(e) => $emit('update:selected', (e.target as HTMLInputElement).checked)"
			class="checkbox"
			type="checkbox"
			:name="id"
			:id="id"
			:checked="selected"
		/>
	</div>
</template>

<style scoped>
.tab-badge {
	--tab-index: v-bind(` '${index}' `);
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

	&:has(.checkbox:checked) {
		transform: scale(1.4);
		z-index: 2;
	}

	&:after {
		display: block;
		position: absolute;
		bottom: 0;
		right: 0;
		background-color: rgba(255, 255, 255, 0.3);
		color: #000;
		content: var(--tab-index);
		width: 2.8rem;
		text-align: center;
		border-top-left-radius: 2rem;
		font-size: 0.8rem;
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
}
</style>
