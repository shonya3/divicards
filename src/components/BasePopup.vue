<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';

const opened = ref(false);

const open = () => (opened.value = true);
const close = () => (opened.value = false);

defineExpose({
	open: () => (opened.value = true),
});

const escapeHandler = (e: KeyboardEvent) => {
	if (e.code === 'Escape') {
		close();
	}
};

onMounted(() => {
	window.addEventListener('keydown', escapeHandler);
});

onUnmounted(() => {
	window.removeEventListener('keydown', escapeHandler);
});
</script>

<template>
	<Teleport to="body">
		<div v-if="opened" class="popup">
			<div @click="opened = false" class="backdrop"></div>
			<div class="popup_content">
				<slot />
			</div>
		</div>
	</Teleport>
</template>

<style scoped>
@keyframes blur-in {
	from {
		backdrop-filter: blur(0px) brightness(100%);
	}
	to {
		backdrop-filter: blur(3px) brightness(40%);
	}
}

.popup {
	/* backdrop-filter: brightness(40%) blur(3px); */
	animation: blur-in 200ms forwards;

	position: absolute;
	top: 0;
	left: 0;
	width: 100%;
	height: 100%;
	display: flex;
	justify-content: center;
	align-items: center;
}
.backdrop {
	opacity: 70%;
	width: 100%;
	height: 100%;
	top: 0;
	left: 0;
	position: absolute;

	z-index: 2;
}

.popup_content {
	color: var(--color);
	background-color: var(--bg-color);
	width: min(95%, 1200px);
	padding: 5rem;

	overflow-y: scroll;
	height: 90vh;

	/* background-color: #fff; */
	border-radius: 4px;
	z-index: 10;
	top: 0;

	z-index: 3;
}
</style>
