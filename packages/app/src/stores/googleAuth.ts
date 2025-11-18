import { defineStore } from 'pinia';
import { command } from '../command';
import { ref, watch, Ref, computed } from 'vue';
import { addRustListener } from '../event';
import { useLocalStorage } from '@vueuse/core';

const ONE_HOUR_IN_MILLIS = 3600 * 1000;
const EXPIRES_IN_MILLIS = ONE_HOUR_IN_MILLIS;
const GOOGLE_AVATAR_KEY = 'google-avatar-url';
const GOOGLE_NAME_KEY = 'google-name';

export const useExpirationDate = (log = false) => {
	const EXPIRATION_KEY = 'google-auth-expiration';
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

export const useGoogleAuthStore = defineStore('google-auth', {
    state: (): {
        name: Ref<string>;
        picture: Ref<string>;
        expiration: Ref<Date | null>;
        loggingIn: boolean;
        auth_url: string | null;
        spreadsheetId: Ref<string>;
    } => ({
        name: useLocalStorage(GOOGLE_NAME_KEY, ''),
        picture: useLocalStorage(GOOGLE_AVATAR_KEY, ''),
        expiration: expirationDate,
        loggingIn: false,
        auth_url: null,
        spreadsheetId: useLocalStorage('sheets-spreadsheet-id', ''),
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
			if (this.loggingIn) {
				console.log('Already logging in');
				if (!this.auth_url) return;
				await command('open_url', { url: this.auth_url });
				return;
			}
			if (this.loggedIn) {
				console.log('Already logged in');
				return;
			}

			this.loggingIn = true;

			const unlisten = await addRustListener('auth-url', e => {
				this.auth_url = e.payload.url;
			});

			try {
				await command('google_auth');
				const identity = await command('google_identity');
				this.name = identity.given_name;
				this.picture = identity.picture ?? '';

				setExpiration(EXPIRES_IN_MILLIS);
			} finally {
				this.loggingIn = false;
				this.auth_url = null;
				unlisten();
			}
        },

        async logout(): Promise<void> {
            await command('google_logout');
            this.expiration = null;
        },

        setSpreadsheetId(id: string) {
            this.spreadsheetId = id.trim();
        },
    },
});
