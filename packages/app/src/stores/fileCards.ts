import { ACTIVE_LEAGUE, downloadText } from '@divicards/shared/lib';
import { defineStore } from 'pinia';
import { SampleData, command } from '../command';
import { DivinationCardsSample, League, Result, TradeLeague, isTradeLeague, leagues } from '@divicards/shared/types';
import { FileCardProps } from '@divicards/wc/src/wc/file-card/file-card';
import { StashTab } from '@divicards/shared/poe.types';

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
		filename: sample.type === 'ok' ? prefixFilename(name, league) : name,
		league,
		sample,
		selected: false,
		minimumCardPrice: 0,
	};
};

export const createFileCardFromSample = (
	name: string,
	sample: Result<DivinationCardsSample>,
	league: TradeLeague
): FileCardProps => {
	return {
		uuid: crypto.randomUUID(),
		filename: sample.type === 'ok' ? prefixFilename(name, league) : name,
		league,
		sample,
		selected: false,
		minimumCardPrice: 0,
	};
};

export const useFileCardsStore = defineStore('fileCards', {
	state: (): {
		fileCards: FileCardProps[];
		mergedFile: FileCardProps | null;
	} => ({
		fileCards: [],
		mergedFile: null,
	}),
	getters: {
		selectedFiles(): FileCardProps[] {
			return this.fileCards.filter(file => file.selected && file.sample.type === 'ok');
		},

		samples(): Result<DivinationCardsSample>[] {
			return this.fileCards.map(f => f.sample);
		},

		selectedSamples(): DivinationCardsSample[] {
			const selectedSamples: DivinationCardsSample[] = [];
			for (const file of this.selectedFiles) {
				if (file.sample.type === 'ok' && file.selected === true) {
					selectedSamples.push(file.sample.data);
				}
			}
			return selectedSamples;
		},

		validFiles(): FileCardProps[] {
			return this.fileCards.filter(file => file.sample.type === 'ok');
		},

		getFileById: state => {
			return (id: string) => [...state.fileCards, state.mergedFile].find(file => file?.uuid === id);
		},
	},
	actions: {
		async addCard(filename: string, sampleData: SampleData, league: TradeLeague = ACTIVE_LEAGUE): Promise<void> {
			const fileCard = await createFileCard(filename, sampleData, league);
			this.fileCards.push(fileCard);
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
			for (const file of this.fileCards) {
				if (file.sample.type === 'ok') {
					downloadText(file.filename, file.sample.data.csv);
				}
			}
		},

		deleteMergedFile() {
			this.mergedFile = null;
		},

		deleteFile(id: string): void {
			this.fileCards = this.fileCards.filter(file => file.uuid !== id);
		},

		deleteAllFiles(): void {
			this.fileCards = [];
		},

		async addFromFile(file: File) {
			try {
				const text = await file.text();
				this.addCard(file.name, text, ACTIVE_LEAGUE);
			} catch (err) {
				if (typeof err === 'string') {
					this.addCard('error', err, ACTIVE_LEAGUE);
				}
			}
		},

		async addFromTab(tab: StashTab, league: League) {
			const tradeLeague = isTradeLeague(league) ? league : ACTIVE_LEAGUE;
			const sample = await command('sample_from_tab', { league, stashId: tab.id });
			const fileCard = createFileCardFromSample(tab.name, sample, tradeLeague);
			this.fileCards.push(fileCard);
			console.log(sample);
			console.log(this.fileCards.at(-1));
		},

		async sampleFromTab(sample: Result<DivinationCardsSample>, league: League, name: string) {
			const fileCard = createFileCardFromSample(name, sample, league as TradeLeague);
			this.fileCards.push(fileCard);
		},

		async replaceFileCard(league: League, oldFileCard: FileCardProps) {
			if (!isTradeLeague(league)) return;

			const index = this.fileCards.findIndex(file => file.uuid === oldFileCard.uuid);
			if (index === -1) return;
			if (oldFileCard.sample.type === 'err') return;
			this.fileCards[index] = await createFileCard(oldFileCard.filename, oldFileCard.sample.data.csv, league);
		},

		async replaceMerged(league: League) {
			const merged = this.mergedFile;
			if (!merged || merged.sample.type === 'err') return;
			if (!isTradeLeague(league)) return;
			this.mergedFile = await createFileCard(merged.filename, merged.sample.data.csv, league);
		},
	},
});
