import { defineStore } from 'pinia';
import { command } from '../command';

export const usePoeOAuth2Store = defineStore('auth', {
	state: (): {
		loggedIn: boolean;
		name: string;
	} => ({
		loggedIn: false,
		name: '',
	}),

	getters: {},
	actions: {
		async login(): Promise<string> {
			try {
				const loggedIn = await this.checkLoggedIn();
				if (loggedIn) throw new Error('Already logged in');

				this.name = await command('poe_auth');
				this.loggedIn = true;

				return this.name;
			} catch (err) {
				return err as string;
			}
		},

		async logout(): Promise<void> {
			await command('poe_logout');
			this.loggedIn = false;
		},

		async checkLoggedIn(): Promise<boolean> {
			return command('poe_authenticated');
		},

		async init() {
			this.loggedIn = await this.checkLoggedIn();
		},
	},
});
