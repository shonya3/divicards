<script setup lang="ts">
import { FileCardElement, FileCardProps, Events } from '@divicards/wc/src/wc/file-card/file-card';
import { DivinationCardsSample } from '../../../shared/types';
import { League } from '@divicards/shared/types';
FileCardElement.define();
const props = defineProps<FileCardProps>();
const emit = defineEmits<{
	'update:selected': [FileCardElement['selected']];
	'update:league': [FileCardElement['league']];
	'update:minimumCardPrice': [FileCardElement['minimumCardPrice']];
	delete: [FileCardElement['uuid']];
	'google-sheets-clicked': [DivinationCardsSample, League];
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

const onGoogleSheetsClicked = (e: CustomEvent<Events['google-sheets-clicked']>) => {
	emit('google-sheets-clicked', e.detail.sample, e.detail.league);
};
</script>

<template>
	<wc-file-card
		v-bind="props"
		@google-sheets-clicked="onGoogleSheetsClicked"
		@upd:league="onUpdLeague"
		@upd:selected="onUpdSelected"
		@upd:minimumCardPrice="onUpdPrice"
		@delete="onDelete"
	></wc-file-card>
</template>
