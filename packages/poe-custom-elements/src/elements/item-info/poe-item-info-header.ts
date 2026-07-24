import { poeColorsCssVariables } from './../../styles/poe-colors-vars.style.js';
import { styles as poeColorsStyles } from '../../styles/poe-colors.style.js';
import { LitElement, html, css, TemplateResult, PropertyValueMap, CSSResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import type { FrameKind, PoeItem } from '../../poe.types.js';
import { classMap } from 'lit/directives/class-map.js';
import { basePath } from '../../lib/base_path.js';
import { frameKind } from '../../lib/internal.js';

@customElement('poe-item-info-header')
export class PoeItemInfoHeaderElement extends LitElement {
	@property({ type: Object }) item!: PoeItem;

	protected willUpdate(map: PropertyValueMap<this>): void {
		if (map.has('item')) {
			const influenceUrls = influenceSymbolsUrls(this.item);
			if (influenceUrls) {
				this.style.setProperty('--left-symbol-bg-image-url', influenceUrls.left);
				this.style.setProperty('--right-symbol-bg-image-url', influenceUrls.right);
			}
		}
	}

	protected render(): TemplateResult {
		const frame = frameKind(this.item.frameType);
		const size = frame ? singleOrDouble(frame, this.item.identified) : null;

		return html`<header
			class=${classMap({
				header: true,
				'header--single': size === 'single',
				'header--double': size === 'double',
				'header--necropolis': frame === 'necropolis',
				[`${frame}`]: true,
			})}
			style="background: ${headerBackgroundUrl(frame ?? 'normal', this.item.identified)}"
		>
			<div class="symbol left-symbol"></div>
			<div class="symbol right-symbol"></div>
			${size === 'double'
				? html`
						<div class="content mt-2">${this.item.name}</div>
						<div class="content mb-4">${this.item.baseType}</div>
					`
				: html`<div class="content">${this.item.baseType}</div>`}
		</header>`;
	}

	static styles: CSSResult = css`
		${poeColorsStyles}

		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}

		:host {
			--left-symbol-bg-image-url: none;
			--right-symbol-bg-image-url: none;
			${poeColorsCssVariables}
		}

		.header {
			font-family: 'fontin';
			display: flex;
			justify-content: center;
			align-items: center;
			flex-direction: column;
			position: relative;
			height: var(--height);
		}

		.header--single {
			--height: 33px;
		}

		.header--double {
			--height: 54px;
		}

		.header--necropolis {
			--height: 45px;
		}

		.content {
			padding-inline: 2.5rem;
			font-size: 19px;
			display: flex;
			align-items: center;
			justify-content: center;
			height: 100%;
		}

		.mt-2 {
			margin-top: 2px;
		}

		.mb-4 {
			margin-bottom: 0.25rem;
		}

		.symbol {
			width: calc(var(--cell-size) / var(--default-cell-size) * 27);
			height: var(--height, 33px);
			background-size: 100%;
			width: 27px;
		}
		.left-symbol {
			background: var(--left-symbol-bg-image-url) center no-repeat;
			position: absolute;
			left: 0px;
			z-index: 20000000000;
		}
		.right-symbol {
			background: var(--right-symbol-bg-image-url) center no-repeat;
			position: absolute;
			right: 0px;
			z-index: 20000000000;
		}
	`;
}

/**
 * Get css urls for item header influences or null
 * @param item Poe API item
 * @returns tuple of css strings url(url-of-influence-symbol) or null
 */
function influenceSymbolsUrls(item: PoeItem): {
	left: string;
	right: string;
} | null {
	const influenceNames = (): [string, string] | null => {
		if (item.influences) {
			const influences = Object.keys(item.influences);
			switch (influences.length) {
				case 0:
					return null;
				case 1:
					return [influences[0], influences[0]];
				case 2:
					return [influences[0], influences[1]];
			}
		}

		if (item.fractured) {
			return ['fractured', 'fractured'];
		}

		if (item.synthesised) {
			return ['synthesised', 'synthesised'];
		}

		return null;
	};

	const influences = influenceNames();
	if (!influences) {
		return null;
	}

	const _basePath = basePath();
	return {
		left: `url(${_basePath}/poe-images/${influences[0]}-symbol.png)`,
		right: `url(${_basePath}/poe-images/${influences[1]}-symbol.png)`,
	};
}

function headerBackgroundUrl(frameKind: FrameKind, identified: boolean): string {
	const size = singleOrDouble(frameKind, identified);
	const left = `url(${headerAssetUrl(frameKind, size, 'left')}) top left no-repeat`;
	const right = `url(${headerAssetUrl(frameKind, size, 'right')}) top right no-repeat`;
	const middle = `url(${headerAssetUrl(frameKind, size, 'middle')}) top center`;
	return `${left},${right},${middle}`;
}

function headerAssetUrl(
	frameKind: FrameKind,
	size: 'single' | 'double',
	side: 'left' | 'middle' | 'right',
	url = `${basePath()}/poe-images/`
): string {
	return `${url}${['header', size === 'double' ? 'double' : '', frameKind, side]
		.filter(s => s.length > 0)
		.join('-')}.png`;
}

function singleOrDouble(frameKind: FrameKind, identified: boolean): 'single' | 'double' {
	switch (frameKind) {
		case 'normal':
			return 'single';
		case 'magic':
			return 'single';
		case 'rare':
			return identified ? 'double' : 'single';
		case 'unique':
			return identified ? 'double' : 'single';
		case 'gem':
			return 'single';
		case 'currency':
			return 'single';
		case 'necropolis':
			return 'single';
		case 'divination':
			return 'single';
	}
}

declare global {
	interface HTMLElementTagNameMap {
		'poe-item-info-header': PoeItemInfoHeaderElement;
	}
}
