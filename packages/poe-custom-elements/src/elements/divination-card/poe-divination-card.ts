import { classMap } from 'lit/directives/class-map.js';
import { html, PropertyValues, nothing, LitElement, TemplateResult, CSSResult } from 'lit';
import { html as staticHtml, unsafeStatic } from 'lit/static-html.js';
import { customElement, property, state } from 'lit/decorators.js';
import { cardElementData } from './data.js';
import { styles } from './divination-card.styles.js';
import { basePath } from '../../lib/base_path.js';

/**
 * @summary Divination Card

 * @cssproperty --padding-inline - The inline padding to use for for element.
 * @cssproperty --padding-block  - The block padding to use for for element.
 * @event       navigate Event   - Emits when on click of one of link elements.
 */
@customElement('poe-divination-card')
export class DivinationCardElement extends LitElement {
	@property({ reflect: true }) name: string = '';
	@property({ reflect: true }) size: CardSize = 'medium';
	@property({ reflect: true }) boss?: string;
	@property({ attribute: 'base-url' }) baseUrl = 'https://divicards-site.pages.dev';
	@property({ reflect: true }) href = '';
	@property({ reflect: true, attribute: 'href-pattern' }) hrefPattern = '{{base}}/card/{{slug}}';

	@state() stackSize: number = 0;
	@state() flavourText = ``;
	@state() artFilename = '';
	@state() rewardHtml = '';
	@state() dropLevel = '';
	@state() slug = '';

	#parseHrefPattern() {
		return this.hrefPattern.replaceAll('{{base}}', this.baseUrl).replaceAll('{{slug}}', this.slug);
	}

	#getHref() {
		if (this.href) {
			return this.href;
		}

		return this.#parseHrefPattern();
	}

	protected willUpdate(changedProperties: PropertyValues<this>): void {
		this.style.setProperty(
			'--divination-card-background-url',
			`url(${basePath()}/divination-card/cards/avif/divination-card.avif)`
		);
		if (changedProperties.has('name')) {
			const name = this.name === 'Fire of Unknown Origin' ? 'Fire Of Unknown Origin' : this.name;
			const cardData = cardElementData.find(card => card.name === name);
			if (cardData) {
				this.stackSize = cardData.stackSize ?? 1;
				this.flavourText = cardData.flavourText;
				this.artFilename = cardData.artFilename;
				this.rewardHtml = cardData.rewardHtml;
				this.dropLevel = `${cardData.minLevel}+`;
				this.slug = cardData.slug;
			}
		}
	}

	protected render(): TemplateResult {
		const href = this.#getHref();
		return html`<div class="element">
			<div
				class=${classMap({
					'divination-card': true,
					[`divination-card--${this.size}`]: true,
				})}
			>
				<a .ariaLabel=${this.name} class="link" @click=${this.#dispatchNavigate} href=${href}></a>
				<div class="skeleton"></div>
				<header
					class=${classMap({
						name: true,
						[`name--${this.size}`]: true,
					})}
				>
					<a @click=${this.#dispatchNavigate} href=${href}> ${this.name} </a>
				</header>
				<div class="imageWrapper">
					<a .ariaLabel=${this.name} @click=${this.#dispatchNavigate} href=${href}>
						<img
							loading="lazy"
							class="image"
							width="100%"
							src=${imageurl(this.artFilename)}
							alt="card's image"
						/>
					</a>
				</div>
				<div
					class=${classMap({
						stackSize: true,
						[`stackSize--${this.size}`]: this.size,
					})}
				>
					${this.stackSize}
				</div>
				<div class="${classMap({ 'bottom-half': true, size25: this.size === 'small' })}">
					${staticHtml`${unsafeStatic(this.rewardHtml)}`}
					<div class="divider"></div>
					<footer>
						<p class="flavourText">${this.flavourText}</p>
					</footer>
				</div>
				<div title="drop level" class="min-level">${this.dropLevel}</div>

				<div class="boss">
					<slot name="boss"> ${this.boss ? html`${this.boss}` : nothing} </slot>
				</div>
			</div>
		</div>`;
	}

	#dispatchNavigate() {
		this.dispatchEvent(new Event('navigate'));
	}
	static override styles: CSSResult = styles;
}

export type CardSize = (typeof CARD_SIZE_VARIANTS)[number];
export const CARD_SIZE_VARIANTS = ['50', '75', 'small', 'medium', 'large'] as const;

function imageurl(artFilename?: string): string {
	if (!artFilename) {
		// console.warn(`Divination Card. No artFilename ${this.name}`);
		return '';
	}
	return `${basePath()}/divination-card/cards/avif/${artFilename}.avif`;
	// return `https://web.poecdn.com/image/divination-card/${artFilename}.png`;
}

declare global {
	interface HTMLElementTagNameMap {
		'poe-divination-card': DivinationCardElement;
	}
}
