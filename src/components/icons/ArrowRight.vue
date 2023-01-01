<script setup lang="ts">
withDefaults(
	defineProps<{
		width?: number;
		height?: number;
	}>(),
	{ width: 16, height: 16 }
);
</script>

<template>
	<svg :width="width" :height="height" viewBox="0 0 24 24">
		<path fill="currentColor" d="m14 18l-1.4-1.45L16.15 13H4v-2h12.15L12.6 7.45L14 6l6 6Z"></path>
	</svg>
</template>
