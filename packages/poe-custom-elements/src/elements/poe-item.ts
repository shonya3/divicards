import { LitElement, html, css, TemplateResult, PropertyValueMap, nothing, render, CSSResult } from 'lit';
import { customElement, property, query, state } from 'lit/decorators.js';
import type { Influence, ItemProperty, PoeItem, Requirement, Socket, SocketedItem } from '../poe.types.js';
import './poe-socket-chain.js';
import { classMap } from 'lit/directives/class-map.js';
import { SimpleTooltip } from './simple-tooltip.js';
import './simple-tooltip.js';
import './tooltip-json-icon.js';
import './item-info/poe-item-info.js';
import { JsonIconElement } from './tooltip-json-icon.js';
import { appendFontinStyle, capitalize, frameKind, parseDisplayMode3 } from '../lib/internal.js';
import { basePath } from '../lib/base_path.js';

/**
 * @cssprop --cell-size            - Size of one tab cell in pixels.
 * @cssprop --poe-item-size        - Size of item.
 * @cssprop --stack-size-font-size - Font size of stack size.
 */
@customElement('poe-item')
export class PoeItemElement extends LitElement {
	#itemIntoTextTransformer: ItemIntoTextTransformer | null = null;
	/** PoE API item data https://www.pathofexile.com/developer/docs/reference#stashes-get */
	@property({ type: Object }) item!: PoeItem;
	/** Controls the visibility of sockets.
	 *
	 *  If set to true, sockets are always visible.
	 *  If set to false, sockets are only visible when the Alt key is pressed or when hovered over.
	 */
	@property({ type: Boolean, reflect: true, attribute: 'always-show-sockets' }) alwaysShowSockets = false;
	/** Whether an item is displayed in inventory(or stash tab) or not. It gets blue background, if yes */
	@property({ type: Boolean }) placed = false;

	/** Main visibility state for sockets */
	@state() socketsVisible = false;
	@state() hovered = false;
	@state() altPressed = false;

	@query('tooltip-json-icon') iconJson?: JsonIconElement;

	protected async willUpdate(map: PropertyValueMap<this>): Promise<void> {
		if (map.has('item')) {
			this.style.setProperty('--w', this.item.w.toString());
			this.style.setProperty('--h', this.item.h.toString());
			if (!this.item.identified) {
				this.setAttribute('unidentified', '');
			}
			this.style.setProperty('--influence-background-image-url', influencesBackgroundVar(this.item));
		}

		if (map.has('alwaysShowSockets')) {
			this.socketsVisible = this.alwaysShowSockets;
		}
		if (map.has('altPressed')) {
			if (this.altPressed) {
				this.socketsVisible = true;
			} else {
				this.socketsVisible = this.alwaysShowSockets;
			}
		}
		if (map.has('hovered')) {
			if (this.hovered) {
				this.socketsVisible = true;
			} else {
				this.socketsVisible = this.alwaysShowSockets;
			}
		}
	}

	get tooltipElement(): SimpleTooltip | null {
		const next = this.nextElementSibling;
		if (next instanceof SimpleTooltip) {
			return next;
		} else {
			return null;
		}
	}

	protected render(): TemplateResult {
		if (!this.item) {
			return html`<p style="color: red">No Poe Api item data (.item)</p>`;
		}

		return html`
			<img alt=${this.item.baseType} .src=${this.item.icon} />
			${this.item.socketedItems && this.item.sockets
				? html`<poe-socket-chain
						@socketed-item-hover-change=${this.onHoveredSocketedItemChanged}
						class=${classMap({ hidden: !this.socketsVisible })}
						.socketedItems=${this.item.socketedItems}
						.sockets=${this.item.sockets}
						.w=${this.item.w}
				  ></poe-socket-chain>`
				: nothing}
			${this.item.stackSize
				? html`<p
						class=${classMap({
							stackSize: true,
							maxed:
								frameKind(this.item.frameType) === 'divination' &&
								this.item.stackSize === this.item.maxStackSize,
						})}
				  >
						${this.item.stackSizeText || this.item.stackSize}
				  </p>`
				: nothing}
		`;
	}

	constructor() {
		super();
		this.addEventListener('mouseenter', this.onMouseEnter);
		this.addEventListener('mouseleave', this.onMouseLeave);
	}

	private onHoveredSocketedItemChanged(e: CustomEvent<SocketedItem>) {
		if (this.tooltipElement) {
			const socketedItemContainer = this.tooltipElement.querySelector('.socketed-item');
			if (socketedItemContainer instanceof HTMLElement) {
				socketedItemContainer.innerHTML = '';
				const info = document.createElement('poe-item-info');
				if (e.detail) {
					info.item = e.detail as PoeItem;
					socketedItemContainer.append(info);
				}
			}
		}
	}

	protected firstUpdated(): void {
		SimpleTooltip.lazy(this, tooltip => {
			render(
				html`<div
					style="display:flex;align-items:flex-start;flex-wrap:wrap;gap:1.2rem;z-index:500;padding:0;margin:0"
				>
					<poe-item-info .item=${this.item}></poe-item-info>
					<div class="socketed-item"></div>
				</div>`,
				tooltip
			);
		});
	}

	private onJClick = (e: KeyboardEvent) => {
		if (this.hovered) {
			if (e.code === 'KeyJ') {
				const icon = this.iconJson ?? document.createElement('tooltip-json-icon');
				if (!this.iconJson) {
					this.shadowRoot!.append(icon);
				}

				navigator.clipboard.writeText(JSON.stringify(this.item, null, 4));
				icon.showing = true;
				setTimeout(() => {
					icon.showing = false;
				}, 2000);
			}
		}
	};

	private onHoverCtrlCClick = (e: KeyboardEvent) => {
		if (this.hovered) {
			if (e.ctrlKey && e.code === 'KeyC') {
				console.log('ctrl c clicked!');
				if (!this.#itemIntoTextTransformer) {
					this.#itemIntoTextTransformer = new ItemIntoTextTransformer(this.item);
				}
				window.navigator.clipboard.writeText(this.#itemIntoTextTransformer.transform());
			}
		}
	};

	private onMouseEnter() {
		this.hovered = true;
	}
	private onMouseLeave() {
		this.hovered = false;
	}
	private onAltPressed = (e: KeyboardEvent) => {
		if (e.key === 'Alt') {
			e.preventDefault();
			this.altPressed = true;
		}
	};
	private onAltReleased = () => {
		this.altPressed = false;
	};

	connectedCallback(): void {
		super.connectedCallback();
		appendFontinStyle();
		window.addEventListener('keydown', this.onAltPressed);
		window.addEventListener('keyup', this.onAltReleased);
		window.addEventListener('keydown', this.onJClick);
		window.addEventListener('keydown', this.onHoverCtrlCClick);
	}
	disconnectedCallback(): void {
		super.disconnectedCallback();
		window.removeEventListener('keydown', this.onAltPressed);
		window.removeEventListener('keyup', this.onAltReleased);
		window.removeEventListener('keydown', this.onJClick);
		window.removeEventListener('keydown', this.onHoverCtrlCClick);
	}

	static styles: CSSResult = css`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}
		:host {
			--influence-background-image-url: none;
			--background-color: none;
			--default-cell-size: 47;
			--cell-size: 47px; /** css prop */
			--w: '(computed) number of horizontal cells';
			--h: '(computed) number of vertical cells';
			width: var(--poe-item-size, calc(var(--cell-size) * var(--w)));
			height: var(--poe-item-size, calc(var(--cell-size) * var(--h)));
			background: var(--influence-background-image-url);
			background-color: var(--background-color);

			position: relative;
			display: flex;
			justify-content: center;
			align-items: center;
			font-family: fontin;
		}

		:host([placed]) {
			background-color: rgba(25, 26, 150, 0.25);
		}

		:host([unidentified]) {
			background-color: rgba(210, 0, 0, 0.18) !important;
		}

		.socketed-item {
			display: flex;
			flex-wrap: wrap;
			gap: 2000px;
		}

		.stackSize {
			font-size: var(--stack-size-font-size, calc(var(--cell-size) / var(--default-cell-size) * 18));
			font-weight: bold;
			color: #fff;
			position: absolute;
			top: -1px;
			left: 5%;
			text-shadow: -1px -1px 0 #000, 0 -1px 0 #000, 1px -1px 0 #000, 1px 0 0 #000, 1px 1px 0 #000, 0 1px 0 #000,
				-1px 1px 0 #000, -1px 0 0 #000, -1px -1px 3px #000, 0 -1px 3px #000, 1px -1px 0 #000, 1px 0 3px #000,
				1px 1px 3px #000, 0 1px 3px #000, -1px 1px 3px #000, -1px 0 3px #000;
			pointer-events: none;
		}
		.maxed {
			color: #00bafe;
		}

		img {
			display: block;
			width: 100%;
		}

		poe-socket-chain {
			position: absolute;
		}

		.hidden {
			display: none !important;
		}
	`;
}

function influencesBackgroundVar(item: PoeItem): string {
	if (!item.influences) {
		return '';
	}
	const influences = Object.keys(item.influences) as Array<Influence>;
	const influenceImageUrl = (influence: Influence) => {
		switch (influence) {
			case 'shaper':
			case 'elder':
				return `url(${basePath()}/poe-images/${capitalize(influence)}Backgroundw${item.w}h${
					item.h
				}.png) no-repeat`;
			default:
				return '';
		}
	};
	return influences.map(influenceImageUrl).filter(Boolean).join(', ');
}

export class ItemIntoTextTransformer {
	#separator = '\n--------\n' as const;
	item: PoeItem;
	constructor(item: PoeItem) {
		this.item = item;
	}

	transform(): string {
		return [
			[
				this.item.rarity ? `Rarity: ${this.item.rarity}` : '',
				`${this.item.name === this.item.baseType}` ? '' : this.item.name,
				this.item.baseType,
			]
				.filter(s => s.length > 0)
				.join('\n'),
			this.properties.length ? this.properties.map(parseProperty).join('\n') : '',
			this.requirements.length
				? `Requirements: \n${this.requirements
						.map(({ name, values }) => `${name}: ${values[0][0]}`)
						.join('\n')}`
				: '',
			this.sockets.length
				? `Sockets: ${Object.values(Object.groupBy(this.sockets, socket => socket.group))
						.flatMap((s = []) => s.map(({ sColour }) => sColour).join('-'))
						.join(' ')}`
				: '',
			this.enchantments.length ? this.enchantments.join('\n') : '',
			this.implicits.length ? this.implicits.join('\n') : '',
			this.fracturedMods.length || this.explicits.length || this.crafts.length
				? [...this.fracturedMods, ...this.explicits, ...this.crafts].join('\n')
				: '',
			this.item.identified ? '' : 'Unidentified',
		]
			.filter(el => el.length > 0)
			.flatMap((el, index, arr) => (index === arr.length - 1 ? [el] : [el, this.#separator]))
			.join('');
	}

	groupSockets(): void {
		Object.values(Object.groupBy(this.sockets, socket => socket.group))
			.flatMap((s = []) => s.map(({ sColour }) => sColour).join('-'))
			.map(s => {
				console.log(s);
				return s;
			})
			.join(' ');
	}

	get sockets(): Array<Socket> {
		return this.item.sockets ?? [];
	}

	get enchantments(): Array<string> {
		return this.item.enchantMods ?? [];
	}

	get properties(): Array<ItemProperty> {
		return this.item.properties ?? [];
	}

	get requirements(): Array<Requirement> {
		return this.item.requirements ?? [];
	}

	get implicits(): Array<string> {
		return this.item.implicitMods ?? [];
	}

	get explicits(): Array<string> {
		return this.item.explicitMods ?? [];
	}

	get crafts(): Array<string> {
		return this.item.craftedMods ?? [];
	}

	get fracturedMods(): Array<string> {
		return this.item.fracturedMods ?? [];
	}
}

function parseProperty(property: ItemProperty): string {
	switch (property.displayMode) {
		case 0: {
			if (!property.values.length) {
				return property.name;
			}
			return `${property.name}: ${property.values.map(value => value[0]).join(', ')}`;
		}
		case 3:
			return parseDisplayMode3(property);
		default:
			return '';
	}
}

declare global {
	interface HTMLElementTagNameMap {
		'poe-item': PoeItemElement;
	}
}
