import { FrameKind, ItemProperty } from '../poe.types.js';
import { basePath } from './base_path.js';

/** https://www.pathofexile.com/developer/docs/reference#type-FrameType */
export function frameKind(frameType: number): FrameKind | null {
	switch (frameType) {
		case 0:
			return 'normal';
		case 1:
			return 'magic';
		case 2:
			return 'rare';
		case 3:
			return 'unique';
		case 4:
			return 'gem';
		case 5:
			return 'currency';
		case 6:
			return 'divination';
		case 11:
			return 'necropolis';
		default:
			return null;
	}
}

export function capitalize(s: string): string {
	const [first = '', ...rest] = s;
	return `${first.toUpperCase()}${rest.join('')}`;
}

export function parseDisplayMode3(property: ItemProperty): string;
export function parseDisplayMode3<T>(property: ItemProperty, mapFn: (val: string) => T): Array<T | string>;
export function parseDisplayMode3<T>(property: ItemProperty, mapFn?: (val: string) => T): Array<T | string> | string {
	if (property.displayMode !== 3) {
		throw new Error(`Expected displayMode 3, got ${property.displayMode}`);
	}

	const result = property.name.split(/\{(\d+)\}/g).map((part, index) => {
		if (index % 2 === 0) {
			return part;
		}

		const value = property.values[parseInt(part)]?.[0];
		if (value == null) {
			return part;
		}

		return mapFn ? mapFn(value) : value;
	});

	return mapFn ? result : result.join('');
}

export function appendFontinStyle(): void {
	const style = document.querySelector('style[data-description="poe-custom-elements-font"]');
	if (!style) {
		document.head.insertAdjacentHTML(
			'beforeend',
			`
            <style data-description="poe-custom-elements-font">
                @font-face {
				    font-family: 'fontin';
				    font-weight: normal;
				    font-style: normal;
				    src: url('${basePath()}/fontin.woff') format('woff');
			    }
            </style>
        `
		);
	}
}
