<script setup lang="ts">
import { fileCardProps } from './dummy';
import FileCard from './FileCard.vue';
import { logEvent } from 'histoire/client';
</script>

<template>
	<Story>
		<FileCard
			@price-updated="logEvent('price-updated', $event)"
			@update:selected="logEvent('update:selected', $event)"
			@delete-me="logEvent('delete-me', $event)"
			v-bind="fileCardProps"
		/>
	</Story>
</template>

<style scoped></style>
