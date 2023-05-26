<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { usePoeOAuth2Store } from '../stores/poeOAuth2Store';
withDefaults(
	defineProps<{
		logs: boolean;
	}>(),
	{ logs: false }
);

const authStore = usePoeOAuth2Store();
const { timeLeft, loggedIn, name, log } = storeToRefs(authStore);
</script>

<template>
	<div class="poe-auth">
		<div class="logged-in" v-if="loggedIn">
			<template v-if="logs && log">
				<div class="logs">
					<p>{{ timeLeft }} seconds left</p>
				</div>
			</template>
			<p>{{ name }}</p>
			<button @click="authStore.logout">Logout</button>
		</div>
		<div v-else>
			<button @click="authStore.login">Login</button>
		</div>
	</div>
</template>

<style scoped>
.poe-auth {
	position: relative;
}
.logged-in {
	display: flex;
	align-items: center;
	justify-self: center;
	gap: 1rem;
}

.logs {
	opacity: 0.6;
	font-size: 80%;
}
</style>
