import { NoItemsTab } from './poe.types';
import { DivinationCardsSample, League } from './types';

export interface IStashLoader {
	tabs(league: League): Promise<NoItemsTab[]>;
	sampleFromTab(tabId: string, league: League): Promise<DivinationCardsSample>;
}
