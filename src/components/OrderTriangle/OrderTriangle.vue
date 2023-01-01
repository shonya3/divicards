<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { Order } from '../../types';

type CssSize = 'px' | 'rem';
const props = withDefaults(defineProps<{ size?: `${number}${CssSize}`; order?: Order; active?: boolean }>(), {
	size: '16px',
	order: 'asc',
	active: false,
});

const rotationValue = ref(props.order === 'asc' ? 0 : 180);
const rotation = computed(() => `rotate(${rotationValue.value}deg)`);

watch(
	() => props.order,
	() => {
		rotationValue.value += 180;
	}
);
</script>

<template>
	<div
		:style="{ width: size, height: size, transform: rotation }"
		class="order"
		:class="{ 'order--active': active }"
		title="Order"
	></div>
</template>

<style scoped>
.order {
	color: var(--color);
	background-color: var(--bg-color);
	/* position: absolute;
	right: 0;
	top: 0; */
	width: v-bind(size);
	height: v-bind(size);
	clip-path: polygon(0% 100%, 50% 0%, 100% 100%);
	background-color: var(--color);
	border-radius: 16px;

	transition: 300ms;
	transition-property: background-color, transform, filter;
	filter: brightness(0.8);
	transform: var(--rotation);
	cursor: pointer;
}

.order--active {
	filter: brightness(1);
	background-color: cyan;
	transform: var(--rotation);
}
</style>
