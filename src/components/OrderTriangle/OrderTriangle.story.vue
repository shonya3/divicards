<script setup lang="ts">
import OrderTriangle from './OrderTriangle.vue';
</script>

<template>
	<Story>
		<order-triangle size="16px" order="asc" class="story" />
	</Story>
</template>

<style scoped>
.story {
	margin-left: 2rem;
	margin-top: 2rem;
}
</style>
