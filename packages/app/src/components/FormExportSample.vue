<script setup lang="ts">
import {
	FormExportSampleElement,
	Props,
	Events,
} from '@divicards/wc/src/wc/e-sample-card/e-form-export-sample/e-form-export-sample';
import '@divicards/wc/src/wc/e-sample-card/e-form-export-sample/e-form-export-sample';
import { TablePreferences } from '@divicards/shared/types';
const props = withDefaults(defineProps<Props>(), { to: 'sheets', spreadsheetId: '', sheetTitle: '' });
const emit = defineEmits<{
	'update:sheetTitle': [FormExportSampleElement['sheetTitle']];
	'update:tablePreferences': [TablePreferences];
	'update:spreadsheetId': [FormExportSampleElement['spreadsheetId']];
	submit: [Props];
}>();

const handlers = {
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
	<e-form-export-sample
		v-bind="props"
		@upd:tablePreferences="handlers.onUpdTablePreferences"
		@upd:spreadsheetId="handlers.onUpdSpreedsheetId"
		@submit="handlers.onSubmit"
	></e-form-export-sample>
</template>

<style scoped>
wc-form-export-sample {
	padding-inline: 0;
}
</style>
