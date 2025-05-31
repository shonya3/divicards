import { DefineComponent } from 'vue';
import { ChangeThemeEvent, Events } from './events.js';
import { styles } from './styles.js';
import { sunmoon } from './sunmoon.js';
import { template as themeToggleTemplate } from './template.js';
import { VueEventHandlers } from '../../event-utils.js';
export type ColorTheme = 'light' | 'dark';

const themeUtils = Object.freeze({
	LOCAL_STORAGE_KEY: 'theme-preference',
	getSystemPreference(): ColorTheme {
		return window.matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'dark';
	},
	getStorageValue(): string | null {
		return localStorage.getItem(this.LOCAL_STORAGE_KEY);
	},

	getTheme(): ColorTheme {
		const storagePreference = this.getStorageValue();
		if (!storagePreference) {
			return this.getSystemPreference();
		}

		if (storagePreference !== 'dark' && storagePreference !== 'light') return 'light';
		return storagePreference;
	},

	addRootThemeAttr(theme: ColorTheme): void {
		document.documentElement.setAttribute('data-theme', theme);
	},

	setStorageValue(val: ColorTheme): void {
		localStorage.setItem(this.LOCAL_STORAGE_KEY, val);
	},
});

/**
 * @cssproperty --size
 * @cssproperty	--icon-fill
 * @cssproperty	--icon-fill
 */
export class ThemeToggle extends HTMLElement {
	static define(tag = 'e-theme-toggle'): void {
		if (!customElements.get(tag)) {
			customElements.define('e-theme-toggle', ThemeToggle);
		}
	}

	get theme(): ColorTheme {
		const theme = this.getAttribute('theme');
		if (theme !== 'dark' && theme !== 'light') return 'dark';
		return theme;
	}

	set theme(val: ColorTheme) {
		this.setAttribute('theme', val);
		if (val === 'dark') {
			document.documentElement.classList.add('sl-theme-dark');
		} else {
			document.documentElement.classList.remove('sl-theme-dark');
		}
	}

	static observedAttributes = ['theme'] as const;
	attributeChangedCallback(name: 'theme', _: string | null, val: string | null): void {
		switch (name) {
			case 'theme':
				if (val !== 'dark' && val !== 'light') return;
				themeUtils.setStorageValue(val);
				document.documentElement.setAttribute('data-theme', val);
				this.$button?.setAttribute('aria-label', val);
		}
	}

	get $button(): HTMLButtonElement | null {
		return this.#shadowRoot.querySelector('button');
	}

	#shadowRoot: ShadowRoot;
	constructor() {
		super();
		const { shadowRoot } = this.attachInternals();
		if (!shadowRoot) {
			const template = document.createElement('template');
			template.innerHTML = themeToggleTemplate;

			this.#shadowRoot = this.attachShadow({ mode: 'open' });
			this.#shadowRoot.adoptedStyleSheets = [styles.styleSheet!, sunmoon.styleSheet!];
			this.#shadowRoot.append(template.content.cloneNode(true));
		} else {
			this.#shadowRoot = shadowRoot;
		}

		this.$button?.addEventListener('click', this.#handle_click);
	}
	connectedCallback(): void {
		this.theme = themeUtils.getTheme();
		this.dispatchEvent(new ChangeThemeEvent(this.theme));
	}

	toggleTheme(): void {
		this.theme = this.theme === 'dark' ? 'light' : 'dark';
	}

	#handle_click = () => {
		this.toggleTheme();
		this.dispatchEvent(new ChangeThemeEvent(this.theme));
	};
}

customElements.define('e-theme-toggle', ThemeToggle);

declare global {
	interface HTMLElementTagNameMap {
		'e-theme-toggle': ThemeToggle;
	}
}

declare module 'vue' {
	interface GlobalComponents {
		'e-theme-toggle': DefineComponent<VueEventHandlers<Events>>;
	}
}
