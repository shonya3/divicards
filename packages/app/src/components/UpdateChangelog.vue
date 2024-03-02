<script setup lang="ts">
import type { UpdateManifest } from '@tauri-apps/api/updater';
import UpdateButton from './UpdateButton.vue';
import { command } from '../command';
defineProps<{
	manifest: UpdateManifest;
}>();
defineEmits<{
	'update-clicked': [];
}>();

const LATEST_RELEASE = 'https://github.com/shonya3/divicards/releases/latest';
</script>

<template>
	<div class="changelog">
		<h1 class="heading">Divicards v{{ manifest?.version }}</h1>
		<a
			class="link-release-notes"
			@click.prevent="command('open_url', { url: LATEST_RELEASE })"
			:href="LATEST_RELEASE"
			>Check release notes</a
		>

		<UpdateButton class="update-btn" @click="$emit('update-clicked')">Update</UpdateButton>
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
	color: white;
	text-decoration: underline;
	text-align: center;
	font-size: 1.1rem;
}

.link-release-notes:hover {
	color: lightblue;
}

.update-btn {
	display: block;
	margin-inline: auto;
	margin-top: 3rem;
}

.heading {
	text-align: center;
}
</style>
