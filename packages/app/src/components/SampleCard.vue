<script setup lang="ts">
import { SampleCardElement, Props, Events } from '@divicards/wc/src/wc/e-sample-card/e-sample-card';
import '@divicards/wc/src/wc/e-sample-card/e-sample-card';
import { DivinationCardsSample } from '@divicards/shared/types';
import { League } from '@divicards/shared/types';
const props = defineProps<Props>();
const emit = defineEmits<{
	'update:selected': [SampleCardElement['selected']];
	'update:league': [SampleCardElement['league']];
	'update:minimumCardPrice': [SampleCardElement['minimumCardPrice']];
	delete: [SampleCardElement['uuid']];
	'google-sheets-clicked': [DivinationCardsSample, League];
	'save-to-file-clicked': [DivinationCardsSample, League, string];
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

const onSaveToFileClicked = (e: CustomEvent<Events['save-to-file-clicked']>) => {
	emit('save-to-file-clicked', e.detail.sample, e.detail.league, e.detail.filename);
};
</script>

<template>
	<e-sample-card
		v-bind="props"
		@google-sheets-clicked="onGoogleSheetsClicked"
		@save-to-file-clicked="onSaveToFileClicked"
		@upd:league="onUpdLeague"
		@upd:selected="onUpdSelected"
		@upd:minimumCardPrice="onUpdPrice"
		@delete="onDelete"
	></e-sample-card>
</template>
