import { defineStore } from 'pinia';
import { command } from '../command';
import { ref, watch, Ref, computed } from 'vue';

const TEN_HOURS_AS_MILLIS = 10 * 3600 * 1000;
const EXPIRES_IN_MILLIS = TEN_HOURS_AS_MILLIS;

const useName = () => {
	const NAME_KEY = 'poe-name';
	const fromStorage = localStorage.getItem(NAME_KEY) ?? '';
	const name = ref(fromStorage);

	watch(
		() => name.value,
		nam => {
			localStorage.setItem(NAME_KEY, nam);
		}
	);

	return name;
};

export const useExpirationDate = (log = false) => {
	const EXPIRATION_KEY = 'auth-expiration';
	const item = localStorage.getItem(EXPIRATION_KEY);
	const fromStorage = item ? new Date(item) : null;
	let timer: ReturnType<typeof setTimeout> | null = null;
	let timeLeftInterval: ReturnType<typeof setInterval> | null = null;

	const expirationDate = ref<Date | null>(fromStorage);
	const loggedIn = computed(() => {
		if (expirationDate.value === null) return false;
		return new Date().getTime() < expirationDate.value.getTime();
	});

	const setExpiration = (expiresInMillis: number = EXPIRES_IN_MILLIS, date = new Date()) => {
		expirationDate.value = new Date(date.getTime() + expiresInMillis);
	};

	const timeLeft = ref(0);

	const manageTimers = (date: Date | null) => {
		if (date == null) {
			if (timer) {
				clearTimeout(timer);
				timer = null;
			}
			if (timeLeftInterval) {
				timeLeft.value = 0;
				clearInterval(timeLeftInterval);
			}
		} else if (date instanceof Date) {
			if (timer) {
				clearTimeout(timer);
				timer = null;
			}

			const left = date.getTime() - new Date().getTime();
			if (log) {
				timeLeft.value = Math.floor(left / 1000);
				timeLeftInterval = setInterval(() => {
					if (timeLeft.value <= 0) {
						clearInterval(timeLeftInterval!);
					}
					timeLeft.value -= 1;
				}, 1000);
			}
			timer = setTimeout(() => {
				expirationDate.value = null;
			}, left);
		}
	};

	watch(
		() => expirationDate.value,
		(date: Date | null) => {
			manageTimers(date);
			if (date == null) {
				localStorage.setItem(EXPIRATION_KEY, JSON.stringify(null));
			} else if (date instanceof Date) {
				localStorage.setItem(EXPIRATION_KEY, date.toJSON());
			} else console.warn('erroneus type');
		}
	);

	manageTimers(expirationDate.value);

	return { expirationDate, loggedIn, setExpiration, timeLeft, log };
};

const { expirationDate, loggedIn, setExpiration, timeLeft, log } = useExpirationDate();

export const usePoeOAuth2Store = defineStore('auth', {
	state: (): {
		name: Ref<string>;
		expiration: Ref<Date | null>;
	} => ({
		name: useName(),
		expiration: expirationDate,
	}),

	getters: {
		timeLeft(): number {
			return timeLeft.value;
		},
		loggedIn(): boolean {
			return loggedIn.value;
		},
		log() {
			return log;
		},
	},
	actions: {
		async login(): Promise<void> {
			try {
				if (this.loggedIn) throw new Error('Already logged in');

				this.name = await command('poe_auth');
				setExpiration(EXPIRES_IN_MILLIS);
			} catch (err) {
				console.log(err);
			}
		},

		async logout(): Promise<void> {
			await command('poe_logout');
			this.expiration = null;
		},
	},
});
