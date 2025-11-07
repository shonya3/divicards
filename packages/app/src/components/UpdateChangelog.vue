<script setup lang="ts">
import { command } from '../command';
defineProps<{
	version: string;
}>();
defineEmits<{
	'update-clicked': [];
}>();

const LATEST_RELEASE = 'https://github.com/shonya3/divicards/releases/latest';
</script>

<template>
	<div class="changelog">
		<h1 class="heading">Divicards v{{ version }}</h1>
		<a
			class="link-release-notes"
			@click.prevent="command('open_url', { url: LATEST_RELEASE })"
			:href="LATEST_RELEASE"
			>Check release notes</a
		>

		<sl-button variant="success" class="update-btn" @click="$emit('update-clicked')">Update</sl-button>
	</div>
</template>

<style scoped>
.changelog {
	padding: 3rem;
	padding-block: 2rem;
	width: 600px;
}

.link-release-notes {
	display: block;
	margin-top: 1rem;
	color: var(--sl-color-gray-700);
	text-decoration: underline;
	text-align: center;
}

.link-release-notes:hover {
	color: lightblue;
}

.update-btn {
	display: block;
	margin-inline: auto;
	margin-top: 2rem;
}

.heading {
	text-align: center;
}
</style>
