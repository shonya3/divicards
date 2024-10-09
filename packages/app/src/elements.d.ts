import type { DefineComponent } from 'vue';
import type { MyEvent, MyCustomEvent } from './events';
import { MinimumCardsPriceChangeEvent } from '@divicards/wc/src/wc/e-sample-card/e-sample-card';
import { TradeLeague } from '@divicards/shared/types';
import { DivinationCardsSample } from '@divicards/shared/types';

declare module 'vue' {
	// Extending Vue's GlobalComponents interface
	interface GlobalComponents extends CustomElements {}
}

interface CustomElements {
	'my-element': DefineComponent<{
		name: string;
		onMyEvent?: (e: MyEvent) => void;
		onMyCustomEvent?: (e: MyCustomEvent) => void;
	}>;
	'my-test-sample-card': DefineComponent<
		{
			league?: TradeLeague;
			filename: string;
			selected: boolean | null;
			uuid: string;
			minimumCardPrice: number;
			sample: DivinationCardsSample;
		},
		{},
		{},
		{},
		{},
		{},
		{},
		{ 'sample__change:minimum_card_price': (e: MinimumCardsPriceChangeEvent) => void }
	>;
}

// For JSX/TSX support, extend the IntrinsicElements
// declare global {
// 	namespace JSX {
// 		interface IntrinsicElements {
// 			'my-element': {
// 				/** Name for element */
// 				name: string;
// 			};
// 		}
// 	}
// }
