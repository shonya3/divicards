<script setup lang="ts">
import { ref } from 'vue';
import DivTable from './DivTable.vue';
import { dummyRecords } from './dummyRecords';

const records = ref(dummyRecords);
</script>

<template>
	<Story>
		<DivTable :records="records" />
	</Story>
</template>

<style scoped></style>
