import { ACTIVE_LEAGUE, downloadText } from '@divicards/shared/lib';
import { defineStore } from 'pinia';
import { SampleData, command } from '../command';
import { DivinationCardsSample, League, TradeLeague, isTradeLeague, leagues } from '@divicards/shared/types';
import { FileCardProps } from '@divicards/wc/src/wc/file-card/file-card';

const prefixFilename = (name: string, league: League): string => {
	const UNDERSCORE_GLUE = '_';

	for (const old of leagues) {
		if (name.startsWith(`${old}${UNDERSCORE_GLUE}`)) {
			return name.replace(old, league);
		}
	}

	return `${league}${UNDERSCORE_GLUE}${name}`;
};

export const createSampleCard = async (
	name: string,
	sampleData: SampleData,
	league: TradeLeague
): Promise<FileCardProps> => {
	const sample = await command('sample', { data: sampleData, league });
	console.log(sample);

	return {
		uuid: crypto.randomUUID(),
		filename: prefixFilename(name, league),
		league,
		sample,
		selected: false,
		minimumCardPrice: 0,
	};
};

export const createSampleCardFromSample = (
	name: string,
	sample: DivinationCardsSample,
	league: TradeLeague
): FileCardProps => {
	return {
		uuid: crypto.randomUUID(),
		filename: prefixFilename(name, league),
		league,
		sample,
		selected: false,
		minimumCardPrice: 0,
	};
};

export const useSampleStore = defineStore('sampleCards', {
	state: (): {
		sampleCards: FileCardProps[];
		merged: FileCardProps | null;
	} => ({
		sampleCards: [],
		merged: null,
	}),
	getters: {
		samples(): DivinationCardsSample[] {
			return this.sampleCards.map(c => c.sample);
		},

		selectedSampleCards(): FileCardProps[] {
			return this.sampleCards.filter(c => c.selected);
		},

		selectedSamples(): DivinationCardsSample[] {
			return this.selectedSampleCards.map(c => c.sample);
		},
	},
	actions: {
		fileById(id: string): FileCardProps | null {
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
			const merged = createSampleCardFromSample('merged.csv', sample, ACTIVE_LEAGUE);

			// No point to select merged file, `null` makes it nonselectable by removing checkbox
			// maybe should refactor later
			merged.selected = null;
			this.merged = merged;
		},

		downloadAll(): void {
			for (const file of this.sampleCards) {
				downloadText(file.filename, file.sample.csv);
			}
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
			console.log(sample);
			const sampleCard = createSampleCardFromSample(name, sample, isTradeLeague(league) ? league : ACTIVE_LEAGUE);
			this.sampleCards.push(sampleCard);
		},

		async replaceFileCard(league: League, oldSampleCard: FileCardProps): Promise<void> {
			if (!isTradeLeague(league)) return;

			const index = this.sampleCards.findIndex(c => c.uuid === oldSampleCard.uuid);
			if (index === -1) return;
			this.sampleCards[index] = await createSampleCard(oldSampleCard.filename, oldSampleCard.sample.csv, league);
		},

		async replaceMerged(league: League): Promise<void> {
			if (this.merged && isTradeLeague(league)) {
				this.merged = await createSampleCard(this.merged.filename, this.merged.sample.csv, league);
			}
		},
	},
});
