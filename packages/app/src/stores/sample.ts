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

export const createFileCard = async (
	name: string,
	sampleData: SampleData,
	league: TradeLeague
): Promise<FileCardProps> => {
	const sample = await command('sample', { data: sampleData, league });

	return {
		uuid: crypto.randomUUID(),
		filename: prefixFilename(name, league),
		league,
		sample,
		selected: false,
		minimumCardPrice: 0,
	};
};

export const createFileCardFromSample = (
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
		mergedFile: FileCardProps | null;
	} => ({
		sampleCards: [],
		mergedFile: null,
	}),
	getters: {
		samples(): DivinationCardsSample[] {
			return this.sampleCards.map(f => f.sample);
		},

		selectedFiles(): FileCardProps[] {
			return this.sampleCards.filter(file => file.selected);
		},

		selectedSamples(): DivinationCardsSample[] {
			return this.selectedFiles.map(f => f.sample);
		},
	},
	actions: {
		fileById(id: string): FileCardProps | null {
			if (this.mergedFile && this.mergedFile.uuid === id) return this.mergedFile;
			return this.sampleCards.find(f => f.uuid === id) ?? null;
		},

		async addCard(filename: string, sampleData: SampleData, league: TradeLeague = ACTIVE_LEAGUE): Promise<void> {
			const fileCard = await createFileCard(filename, sampleData, league);
			this.sampleCards.push(fileCard);
		},

		async merge() {
			const sample = await command('merge', { samples: this.selectedSamples });
			const fileCard = await createFileCard('merged.csv', sample.csv, ACTIVE_LEAGUE);

			// No point to select merged file, `null` makes it nonselectable by removing checkbox
			// maybe should refactor later
			fileCard.selected = null;
			this.mergedFile = fileCard;
		},

		downloadAll() {
			for (const file of this.sampleCards) {
				downloadText(file.filename, file.sample.csv);
			}
		},

		deleteMergedFile() {
			this.mergedFile = null;
		},

		deleteFile(id: string): void {
			this.sampleCards = this.sampleCards.filter(file => file.uuid !== id);
		},

		deleteAllFiles(): void {
			this.sampleCards = [];
		},

		async addFromFile(file: File) {
			const text = await file.text();
			this.addCard(file.name, text, ACTIVE_LEAGUE);
		},

		async addFromDragAndDrop(e: DragEvent) {
			return Promise.allSettled(Array.from(e.dataTransfer?.files ?? []).map(f => this.addFromFile(f)));
		},

		async addSample(name: string, sample: DivinationCardsSample, league: League) {
			console.log(sample);
			const fileCard = createFileCardFromSample(name, sample, isTradeLeague(league) ? league : ACTIVE_LEAGUE);
			this.sampleCards.push(fileCard);
		},

		async replaceFileCard(league: League, oldFileCard: FileCardProps) {
			if (!isTradeLeague(league)) return;

			const index = this.sampleCards.findIndex(file => file.uuid === oldFileCard.uuid);
			if (index === -1) return;
			this.sampleCards[index] = await createFileCard(oldFileCard.filename, oldFileCard.sample.csv, league);
		},

		async replaceMerged(league: League) {
			if (this.mergedFile && isTradeLeague(league)) {
				this.mergedFile = await createFileCard(this.mergedFile.filename, this.mergedFile.sample.csv, league);
			}
		},
	},
});
