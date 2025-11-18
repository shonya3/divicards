import { ACTIVE_LEAGUE } from '@divicards/shared/lib.js';
import { defineStore } from 'pinia';
import { SampleData, command } from '../command';
import { DivinationCardsSample, League, TradeLeague, isTradeLeague, leagues } from '@divicards/shared/types.js';
import { Props as SampleCardProps } from '@divicards/wc/e-sample-card/e-sample-card.js';

const sampleCardsAmount = (sample: DivinationCardsSample): number => {
    const cards = Array.isArray((sample as any)?.cards) ? (sample as any).cards : [];
    return cards.reduce((total: number, { amount }: { amount: number }) => total + amount, 0);
};

const prefixFilename = (name: string, league: League, sample: DivinationCardsSample): string => {
	const UNDERSCORE_GLUE = '_';

	for (const old of leagues) {
		if (name.startsWith(`${old}`) && name.includes(UNDERSCORE_GLUE)) {
			const numberOfUnderscores = (name.match(/_/g) ?? []).length;
			if (numberOfUnderscores === 1) {
				const underscoreIndex = name.indexOf('_');
				const unprefixedName = name.slice(underscoreIndex + 1);
				return `${league}-${sampleCardsAmount(sample)}${UNDERSCORE_GLUE}${unprefixedName}`;
			}
		}
	}

	return `${league}-${sampleCardsAmount(sample)}${UNDERSCORE_GLUE}${name}`;
};

async function prepareCsvDataForDrag(sample: DivinationCardsSample): Promise<string> {
	return await command('sample_into_csv', {
		sample,
		preferences: {
			order: 'desc',
			columns: ['name', 'amount'],
			orderedBy: 'amount',
			cardsMustHaveAmount: false,
			minPrice: 0,
		},
	});
}

export const createSampleCard = async (
	name: string,
	sampleData: SampleData,
	league: TradeLeague
): Promise<SampleCardProps> => {
	const sample = await command('sample', { data: sampleData, league });
	const csv = await prepareCsvDataForDrag(sample);

	const props = {
		uuid: crypto.randomUUID(),
		filename: prefixFilename(name, league, sample),
		league,
		sample,
		selected: false,
		minimumCardPrice: 0,
		csvDataForDrag: csv,
	};

	return props;

	return {
		uuid: crypto.randomUUID(),
		filename: prefixFilename(name, league, sample),
		league,
		sample,
		selected: false,
		minimumCardPrice: 0,
		csvDataForDrag: csv,
	};
};

export const createSampleCardFromSample = async (
	name: string,
	sample: DivinationCardsSample,
	league: TradeLeague
): Promise<SampleCardProps> => {
	const csv = await prepareCsvDataForDrag(sample);
	return {
		uuid: crypto.randomUUID(),
		filename: prefixFilename(name, league, sample),
		league,
		sample,
		selected: false,
		minimumCardPrice: 0,
		csvDataForDrag: csv,
	};
};

export const useSampleStore = defineStore('sampleCards', {
	state: (): {
		sampleCards: SampleCardProps[];
		merged: SampleCardProps | null;
	} => ({
		sampleCards: [],
		merged: null,
	}),
	getters: {
		samples(): DivinationCardsSample[] {
			return this.sampleCards.map(c => c.sample);
		},

		selectedSampleCards(): SampleCardProps[] {
			return this.sampleCards.filter(c => c.selected);
		},

		selectedSamples(): DivinationCardsSample[] {
			return this.selectedSampleCards.map(c => c.sample);
		},
	},
	actions: {
		fileById(id: string): SampleCardProps | null {
			if (this.merged && this.merged.uuid === id) return this.merged;
			return this.sampleCards.find(c => c.uuid === id) ?? null;
		},

		async addCard(filename: string, sampleData: SampleData, league: TradeLeague = ACTIVE_LEAGUE): Promise<void> {
			const sampleCard = await createSampleCard(filename, sampleData, league);
			this.sampleCards.push(sampleCard);
		},

		async mergeSelected(): Promise<void> {
			this.merge(this.selectedSamples);
		},

		async mergeAll(): Promise<void> {
			this.merge(this.samples);
		},

		async merge(samples: DivinationCardsSample[]): Promise<void> {
			const sample = await command('merge', { samples });
			const merged = await createSampleCardFromSample('merged.csv', sample, ACTIVE_LEAGUE);

			// No point to select merged file, `null` makes it nonselectable by removing checkbox
			// maybe should refactor later
			merged.selected = null;
			this.merged = merged;
		},

		deleteMerged(): void {
			this.merged = null;
		},

		deleteFile(id: string): void {
			this.sampleCards = this.sampleCards.filter(c => c.uuid !== id);
		},

		deleteAllFiles(): void {
			this.sampleCards = [];
		},

		async addFromFile(file: File) {
			const text = await file.text();
			this.addCard(file.name, text, ACTIVE_LEAGUE);
		},

		async addFromDragAndDrop(e: DragEvent): Promise<PromiseSettledResult<void>[]> {
			return Promise.allSettled(Array.from(e.dataTransfer?.files ?? []).map(f => this.addFromFile(f)));
		},

		async addSample(name: string, sample: DivinationCardsSample, league: League): Promise<void> {
			const sampleCard = await createSampleCardFromSample(
				name,
				sample,
				isTradeLeague(league) ? league : ACTIVE_LEAGUE
			);
			this.sampleCards.push(sampleCard);
		},

		async replaceFileCard(league: League, oldSampleCard: SampleCardProps): Promise<void> {
			if (!isTradeLeague(league)) return;

			const index = this.sampleCards.findIndex(c => c.uuid === oldSampleCard.uuid);
			if (index === -1) return;
			this.sampleCards[index] = await createSampleCard(oldSampleCard.filename, oldSampleCard.sample, league);
		},

		async replaceMerged(league: League): Promise<void> {
			if (this.merged && isTradeLeague(league)) {
				this.merged = await createSampleCard(this.merged.filename, this.merged.sample, league);
			}
		},
	},
});
