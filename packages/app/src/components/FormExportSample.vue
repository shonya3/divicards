<script setup lang="ts">
import { FormExportSampleElement, Props, Events } from '@divicards/wc/src/wc/form-export-sample/form-export-sample';
import { TablePreferences } from '@divicards/shared/types';
FormExportSampleElement.define();
const props = withDefaults(defineProps<Props>(), { to: 'sheets', spreadsheetId: '', sheetTitle: '' });
const emit = defineEmits<{
	'update:columns': [FormExportSampleElement['columns']];
	'update:order': [FormExportSampleElement['order']];
	'update:orderedBy': [FormExportSampleElement['orderedBy']];
	'update:cardsMustHaveAmount': [FormExportSampleElement['cardsMustHaveAmount']];
	'update:sheetTitle': [FormExportSampleElement['sheetTitle']];
	'update:tablePreferences': [TablePreferences];
	'update:spreadsheetId': [FormExportSampleElement['spreadsheetId']];
	'update:minPrice': [FormExportSampleElement['minPrice']];
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
	onUpdMinPrice(e: CustomEvent<Events['upd:minPrice']>) {
		emit('update:minPrice', e.detail);
	},
	onSubmit(e: CustomEvent<Props>) {
		emit('submit', e.detail);
	},
};
</script>

<template>
	<wc-form-export-sample
		v-bind="props"
		@upd:columns="handlers.onUpdColumns"
		@upd:order="handlers.onUpdOrder"
		@upd:orderedBy="handlers.onUpdOrderedBy"
		@upd:cardsMustHaveAmount="handlers.onUpdCardsMustHaveAmount"
		@upd:tablePreferences="handlers.onUpdTablePreferences"
		@upd:spreadsheetId="handlers.onUpdSpreedsheetId"
		@upd:minPrice="handlers.onUpdMinPrice"
		@submit="handlers.onSubmit"
	></wc-form-export-sample>
</template>

<style scoped>
wc-form-export-sample {
	padding-inline: 0;
}
</style>
