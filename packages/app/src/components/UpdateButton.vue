<script setup lang="ts"></script>

<template>
	<button class="update-btn">
		<slot>Update</slot>
	</button>
</template>

<style scoped>
.update-btn {
	width: fit-content;
	background-color: #15803d;
}

.update-btn:hover {
	background-color: #065f46;
}
</style>
