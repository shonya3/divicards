import type { DefineComponent } from 'vue';
import type { MyEvent, MyCustomEvent } from './events';

declare module 'vue' {
	// Extending Vue's GlobalComponents interface
	interface GlobalComponents extends CustomElements {}
}

interface CustomElements {
	'my-element': DefineComponent<{
		name: string;
		onMyEvent: (e: MyEvent) => void;
		onMyCustomEvent: (e: MyCustomEvent) => void;
	}>;
}

// For JSX/TSX support, extend the IntrinsicElements
declare global {
	namespace JSX {
		interface IntrinsicElements {
			'my-element': {
				/** Name for element */
				name: string;
			};
		}
	}
}
