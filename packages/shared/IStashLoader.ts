import { NoItemsTab, TabWithItems } from './poe.types';
import { DivinationCardsSample, League } from './types';

export interface IStashLoader {
	tabs(league: League): Promise<NoItemsTab[]>;
	sampleFromTab(tabId: string, league: League): Promise<DivinationCardsSample>;
}

export interface IDefaultStashLoader {
	tab(league: League, tabId: string): Promise<TabWithItems>;
	tabs(league: League): Promise<NoItemsTab[]>;
}

/**
 * 
 * ```
 * const stashLoader = new DefaultStashLoader(
	'divicards',
	'0.5.3',
	'poeshonya3@gmail.com',
	'97b464048e88ad2c6433dacbefd67030c97523a5'
);

    ```
 */
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

	async tab(league: string, tabId: string, subtabId?: string): Promise<TabWithItems> {
		let url = `${DefaultStashLoader.API_URL}/stash/${league}/${tabId}`;
		if (subtabId) {
			url = `${url}/${subtabId}`;
		}

		const response = await fetch(url, {
			headers: this.#authHeaders(),
		});
		type ApiTabResponse = { stash: TabWithItems };
		const tabResponse: ApiTabResponse = await response.json();
		return tabResponse.stash;
	}

	async tabs(league: string): Promise<NoItemsTab[]> {
		const url = `${DefaultStashLoader.API_URL}/stash/${league}`;
		const response = await fetch(url, {
			headers: this.#authHeaders(),
		});
		type ApiTabsResponse = { stashes: NoItemsTab[] };

		const tabs: ApiTabsResponse = await response.json();
		return this.#flattenTabs(tabs.stashes);
	}

	#flattenTabs(tabs: NoItemsTab[]): NoItemsTab[] {
		const flat: NoItemsTab[] = [];

		for (const tab of tabs) {
			if (tab.type !== 'Folder') {
				flat.push(tab);
			}

			if (tab.children) {
				for (const childTab of tab.children) {
					flat.push(childTab);
				}
			}
		}

		return flat;
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
