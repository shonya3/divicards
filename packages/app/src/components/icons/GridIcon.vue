<script setup lang="ts">
withDefaults(
	defineProps<{
		width?: number;
		height?: number;
	}>(),
	{ width: 16, height: 16 }
);
</script>

<!-- <template>
	<svg xmlns="http://www.w3.org/2000/svg" :width="width" class="svg" viewBox="0 0 48 48">
		<g fill="none" stroke="currentColor" stroke-width="4">
			<path
				d="M39.3 6H8.7A2.7 2.7 0 0 0 6 8.7v30.6A2.7 2.7 0 0 0 8.7 42h30.6a2.7 2.7 0 0 0 2.7-2.7V8.7A2.7 2.7 0 0 0 39.3 6Z"
			/>
			<path stroke-linecap="round" d="M24 6v36M6 24h36" />
		</g>
	</svg>
</template> -->

<template>
	<svg xmlns="http://www.w3.org/2000/svg" :width="width" class="svg" viewBox="0 0 512 512">
		<path
			fill="currentColor"
			d="M47.547 63.547v384.906a16 16 0 0 0 16 16h384.906a16 16 0 0 0 16-16V63.547a16 16 0 0 0-16-16H63.547a16 16 0 0 0-16 16Zm288.6 16h96.3v96.3h-96.3Zm0 128.3h96.3v96.3h-96.3Zm0 128.3h96.3v96.3h-96.3Zm-128.3-256.6h96.3v96.3h-96.3Zm0 128.3h96.3v96.3h-96.3Zm0 128.3h96.3v96.3h-96.3Zm-128.3-256.6h96.3v96.3h-96.3Zm0 128.3h96.3v96.3h-96.3Zm0 128.3h96.3v96.3h-96.3Z"
		/>
	</svg>
</template>

<style scoped>
.svg {
	min-width: 36px;
	min-height: 36px;
}
</style>
