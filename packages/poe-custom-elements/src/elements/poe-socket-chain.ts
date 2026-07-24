import { classMap } from 'lit/directives/class-map.js';
import { LitElement, html, css, TemplateResult, nothing, CSSResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import type { Socket, SocketedItem } from '../poe.types.js';
import './poe-item-socket.js';
import { basePath } from '../lib/base_path.js';

type SocketLinkDirection = 'left-to-right' | 'top-to-bottom' | 'right-to-left';

/**
 * @event socketed-item-hover-change CustomEvent<SocketedItem | null> - Emits when the user hovers over or away from a socketed item.
 * @cssprop --default-cell-size - Size of one tab cell in pixels.
 * @cssprop --cell-size - Size of one tab cell in pixels.
 */
@customElement('poe-socket-chain')
export class PoeSocketChainElement extends LitElement {
	@property({ type: Array }) socketedItems: Array<SocketedItem> = [];
	@property({ type: Array }) sockets: Array<Socket> = [];
	@property({ type: Number }) w!: number;

	protected render(): TemplateResult {
		return html`<ul
			class=${classMap({
				'item-width--w1': this.w === 1,
				'item-width--w2': this.w === 2,
			})}
		>
			${Object.values(
				Object.groupBy(
					this.sockets.map((socket, i) => ({
						socketNo: i + 1,
						...socket,
						socketedItem: this.socketedItems.find(item => item.socket === i),
					})),
					socket => socket.group
				)
			).flatMap((sockets = []) => {
				return sockets.map(
					(socket, i) =>
						html`<li style="grid-area: s${socket.socketNo}">
							<poe-item-socket
								@pointerenter=${() => this.onSocketPointerEnter(socket.socketedItem ?? null)}
								@pointerleave=${this.onSocketPointerLeave}
								.socketedItem=${socket.socketedItem}
								.kind=${socket.sColour}
							></poe-item-socket>
							${i === sockets.length - 1
								? nothing
								: html`<div
										class=${classMap({
											'socket-link': true,
											[`socket-link--${this.socketLinkDirection(socket.socketNo)}`]: true,
										})}
									>
										<img
											class="socket-link-img"
											src="${basePath()}/poe-images/Socket_Link_Horizontal.png"
										/>
									</div>`}
						</li>`
				);
			})}
		</ul>`;
	}

	onSocketPointerEnter(socketedItem: SocketedItem | null): void {
		this.dispatchEvent(new CustomEvent('socketed-item-hover-change', { detail: socketedItem }));
	}
	onSocketPointerLeave(): void {
		this.dispatchEvent(new CustomEvent('socketed-item-hover-change', { detail: null }));
	}

	socketLinkDirection(socketNo: number): SocketLinkDirection {
		switch (this.w) {
			case 1: {
				return 'top-to-bottom';
			}

			case 2: {
				switch (socketNo) {
					case 1:
						return 'left-to-right';
					case 2:
						return 'top-to-bottom';
					case 3:
						return 'right-to-left';
					case 4:
						return 'top-to-bottom';
					case 5:
						return 'left-to-right';
					default: {
						throw new Error(`SocketNo can be 1 | 2 | 3 | 4 | 5, but not ${socketNo}`);
					}
				}
			}

			default: {
				throw new Error(`Item width can be 1 cell or 2 cells, but not ${this.w}`);
			}
		}
	}

	static styles: CSSResult = css`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}
		:host {
			--default-socket-link-image-width-px: 38;
			--socket-link-img-width: calc(
				var(--default-socket-link-image-width-px) * var(--cell-size) / var(--default-cell-size)
			);
			display: inline-block;
			display: flex;
			align-items: center;
			justify-content: center;
		}

		.item-width--w1 {
			grid-template-areas:
				's1'
				's2'
				's3'
				's4';
		}

		.item-width--w2 {
			grid-template-areas:
				's1 s2'
				's4 s3'
				's5 s6';
		}

		ul {
			list-style: none;
			display: grid;
			grid-template-areas:
				's1 s2'
				's4 s3'
				's5 s6';
		}

		li {
			position: relative;
			display: flex;
			justify-content: center;
			align-items: center;
		}

		.socket-link {
			position: absolute;
			z-index: 5;
		}

		.socket-link-img {
			width: var(--socket-link-img-width);
		}

		.socket-link--left-to-right {
			right: 0px;
			transform: translateX(50%);
		}

		.socket-link--right-to-left {
			left: 0px;
			transform: translateX(-50%);
		}

		.socket-link--top-to-bottom {
			rotate: -90deg;
			bottom: 0;
			translate: 2px 50%;
		}
	`;
}

declare global {
	interface HTMLElementTagNameMap {
		'poe-socket-chain': PoeSocketChainElement;
	}
}
