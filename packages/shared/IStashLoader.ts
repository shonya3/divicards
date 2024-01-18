import { NoItemsTab, TabWithItems } from './poe.types';
import { DivinationCardsSample, League } from './types';

export interface IStashLoader {
	tabs(league: League): Promise<NoItemsTab[]>;
	sampleFromTab(tabId: string, league: League): Promise<DivinationCardsSample>;
}

export interface IDefaultStashLoader {
	tabs(league: League): Promise<NoItemsTab[]>;
	tab(tabId: string, league: League): Promise<TabWithItems>;
}

export class DefaultStashLoader implements IDefaultStashLoader {
	static API_URL = 'https://api.pathofexile.com' as const;
	/** Name of your application */
	#app: string;
	/** Actual version of your application */
	#version: string;
	/** Contact Email of developer */
	#contactEmail: string;
	/** Access Token with scope:stashes */
	#token: string;

	/**
	 *
	 * @param app Name of your application
	 * @param version Actual version of your application
	 * @param contactEmail Contact Email of developer
	 * @param token Access Token with scope:stashes
	 */
	constructor(app: string, version: string, contactEmail: string, token: string) {
		this.#app = app;
		this.#version = version;
		this.#contactEmail = contactEmail;
		this.#token = token;
	}
	async tabs(league: string): Promise<NoItemsTab[]> {
		const url = `${DefaultStashLoader.API_URL}/stash/${league}`;
		const response = await fetch(url, {
			headers: this.#authHeaders(),
		});

		const json = await response.json();
		return json;
	}
	async tab(league: string, tabId: string, subtabId?: string): Promise<TabWithItems> {
		let url = `${DefaultStashLoader.API_URL}/stash/${league}/${tabId}`;
		if (subtabId) {
			url = `${url}/${subtabId}`;
		}

		const response = await fetch(url, {
			headers: this.#authHeaders(),
		});
		type ApiTabResponse = { stash: TabWithItems };
		const json: ApiTabResponse = await response.json();
		return json.stash;
	}

	#authHeaders() {
		return new Headers({
			Authorization: `Bearer ${this.#token}`,
			'User-Agent': this.#userAgentHeader(),
		});
	}

	/** for "User-Agent" header
	 * ```
	 *  'User-Agent': #this.userAgent()
	 * ```
	 */
	#userAgentHeader() {
		return `$OAuth ${this.#app}/${this.#version} (contact: ${this.#contactEmail})`;
	}
}
