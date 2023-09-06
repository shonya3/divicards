<script setup lang="ts">
import { ToGoogleSheetsElement, Props, Events } from '../../../wc/src/wc/to-google-sheets/to-google-sheets';
import { TablePreferences } from '@divicards/shared/types';
ToGoogleSheetsElement.define();
const props = defineProps<Props>();
const emit = defineEmits<{
	'update:columns': [ToGoogleSheetsElement['columns']];
	'update:order': [ToGoogleSheetsElement['order']];
	'update:orderedBy': [ToGoogleSheetsElement['orderedBy']];
	'update:cardsMustHaveAmount': [ToGoogleSheetsElement['cardsMustHaveAmount']];
	'update:sheetTitle': [ToGoogleSheetsElement['sheetTitle']];
	'update:tablePreferences': [TablePreferences];
	'update:spreadsheetId': [ToGoogleSheetsElement['spreadsheetId']];
	submit: [Props];
}>();

const handlers = {
	onUpdColumns(e: CustomEvent<Events['upd:columns']>) {
		emit('update:columns', e.detail);
	},
	onUpdOrder(e: CustomEvent<Events['upd:order']>) {
		emit('update:order', e.detail);
	},
	onUpdOrderedBy(e: CustomEvent<Events['upd:orderedBy']>) {
		emit('update:orderedBy', e.detail);
	},
	onUpdCardsMustHaveAmount(e: CustomEvent<Events['upd:cardsMustHaveAmount']>) {
		emit('update:cardsMustHaveAmount', e.detail);
	},
	onUpdSheetTitle(e: CustomEvent<Events['upd:order']>) {
		emit('update:order', e.detail);
	},
	onUpdTablePreferences(e: CustomEvent<Events['upd:tablePreferences']>) {
		emit('update:tablePreferences', e.detail);
	},
	onUpdSpreedsheetId(e: CustomEvent<Events['upd:spreadsheetId']>) {
		emit('update:spreadsheetId', e.detail);
	},
	onSubmit(e: CustomEvent<Props>) {
		emit('submit', e.detail);
	},
};
</script>

<template>
	<wc-to-google-sheets
		v-bind="props"
		@upd:columns="handlers.onUpdColumns"
		@upd:order="handlers.onUpdOrder"
		@upd:orderedBy="handlers.onUpdOrderedBy"
		@upd:cardsMustHaveAmount="handlers.onUpdCardsMustHaveAmount"
		@upd:tablePreferences="handlers.onUpdTablePreferences"
		@upd:spreadsheetId="handlers.onUpdSpreedsheetId"
		@submit="handlers.onSubmit"
	></wc-to-google-sheets>
</template>

<style scoped>
wc-to-google-sheets {
	padding-inline: 0;
}
</style>
