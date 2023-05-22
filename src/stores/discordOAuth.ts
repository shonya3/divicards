import { defineStore } from 'pinia';
import { command } from '../command';
import { DiscordIdentity } from '../types';

export const useDiscordOAuthStore = defineStore('auth', {
	state: (): {
		loggedIn: boolean;
		identity: DiscordIdentity | null;
	} => ({
		loggedIn: false,
		identity: null,
	}),

	getters: {
		name(): string {
			if (!this.identity) return '';
			return `${this.identity.username}${this.identity.discriminator ? `#${this.identity.discriminator}` : ''}`;
		},
	},
	actions: {
		async login(): Promise<string> {
			try {
				const loggedIn = await this.checkLoggedIn();
				if (loggedIn) throw new Error('Already logged in');

				const message = await command('discord_auth');
				this.loggedIn = true;

				this.identity = await this.getIdentity();
				return message;
			} catch (err) {
				return err as string;
			}
		},

		async getIdentity(): Promise<DiscordIdentity> {
			return command('discord_identity');
		},

		async logout(): Promise<void> {
			await command('discord_logout');
			this.loggedIn = false;
		},

		async checkLoggedIn(): Promise<boolean> {
			return command('discord_authenticated');
		},

		async init() {
			this.loggedIn = await this.checkLoggedIn();

			if (!this.loggedIn) return;

			this.identity = await this.getIdentity();
		},
	},
});
