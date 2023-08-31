<script setup lang="ts">
import { FileCardElement, FileCardProps, Events } from '@divicards/wc/src/wc/file-card/file-card';
FileCardElement.define();
const props = defineProps<FileCardProps>();
const emit = defineEmits<{
	'update:selected': [FileCardElement['selected']];
	'update:league': [FileCardElement['league']];
	'update:minimumCardPrice': [FileCardElement['minimumCardPrice']];
	delete: [FileCardElement['uuid']];
}>();

const onUpdLeague = (e: CustomEvent<Events['upd:league']>) => {
	emit('update:league', e.detail);
};

const onUpdSelected = (e: CustomEvent<Events['upd:selected']>) => {
	emit('update:selected', e.detail);
};

const onUpdPrice = (e: CustomEvent<Events['upd:minimumCardPrice']>) => {
	emit('update:minimumCardPrice', e.detail);
};

const onDelete = (e: CustomEvent<Events['delete']>) => {
	emit('delete', e.detail);
};
</script>

<template>
	<wc-file-card
		v-bind="props"
		@upd:league="onUpdLeague"
		@upd:selected="onUpdSelected"
		@upd:minimumCardPrice="onUpdPrice"
		@delete="onDelete"
	></wc-file-card>
</template>
