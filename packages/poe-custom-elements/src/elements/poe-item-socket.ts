import { LitElement, html, css, TemplateResult, CSSResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import type { SocketKind, SocketedItem } from '../poe.types.js';
import { basePath } from '../lib/base_path.js';
import { capitalize } from '../lib/internal.js';

/**
 * @cssprop --cell-size - Size of one tab cell in pixels.
 */
@customElement('poe-item-socket')
export class PoeItemSocketElement extends LitElement {
	/** Socket color or Abyss */
	@property({ reflect: true }) kind!: SocketKind;
	@property({ type: Object }) socketedItem?: SocketedItem;

	protected willUpdate(): void {
		this.style.setProperty('--background-image', this.gemImageSrc());
	}

	protected render(): TemplateResult {
		return html` <div class="highlight-ring"></div> `;
	}

	gemImageSrc() {
		const name = (kind: SocketKind): string => {
			const skillOrSupport = this.socketedItem?.support ? 'Support' : 'Skill';
			switch (kind) {
				case 'A':
					return this.socketedItem ? 'socketAbyss' : 'socketAbyssFull';
				case 'B':
					return this.socketedItem ? `intFull${skillOrSupport}` : 'int';
				case 'G':
					return this.socketedItem ? `dexFull${skillOrSupport}` : 'dex';
				case 'R':
					return this.socketedItem ? `strFull${skillOrSupport}` : 'str';
				case 'W': {
					const color = this.socketedItem?.colour;
					const whiteName = (gemColor: SocketKind) => {
						return `gen${capitalize(name(gemColor))}`;
					};

					if (!this.socketedItem) {
						return 'gen';
					}
					if (!color) {
						return whiteName('B');
					}
					switch (color) {
						case 'R':
						case 'G':
						case 'B':
							return whiteName(color);
						default:
							return whiteName('B');
					}
				}
			}
		};

		return `url('${basePath()}/poe-images/${name(this.kind)}.png')`;
	}

	static styles: CSSResult = css`
		:host {
			--background-image: '(computed) Image of empty or full socket';
			display: inline-block;
			width: var(--cell-size);
			height: var(--cell-size);
			position: relative;
			background: var(--background-image);
			background-size: 100%;
		}

		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}

		.highlight-ring {
			display: none;
			top: 50%;
			left: 50%;
			transform: translate(-50%, -50%);
			position: absolute;
			content: '';
			width: calc(var(--cell-size) / var(--default-cell-size) * 31);
			height: calc(var(--cell-size) / var(--default-cell-size) * 31);
			border-radius: 50%;

			box-shadow: 0px 0px 4px 2px #fff;
		}

		:host(:hover) .highlight-ring {
			display: initial;
		}

		img {
			width: 100%;
			display: block;
			height: var(--cell-size);
		}
	`;
}

declare global {
	interface HTMLElementTagNameMap {
		'poe-item-socket': PoeItemSocketElement;
	}
}
