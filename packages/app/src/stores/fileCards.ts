import { ACTIVE_LEAGUE, downloadText } from '@divicards/shared/lib';
import { defineStore } from 'pinia';
import { command } from '../command';
import { DivinationCardsSample, League, Result, TradeLeague, isTradeLeague, leagues } from '@divicards/shared/types';
import { FileCardProps } from '@divicards/wc/src/wc/file-card/file-card';
import { StashTab } from '@divicards/shared/poe.types';
import { cardsFromTab } from '../cards';

const prefixFilename = (name: string, league: League): string => {
	const UNDERSCORE_GLUE = '_';

	for (const old of leagues) {
		if (name.startsWith(`${old}${UNDERSCORE_GLUE}`)) {
			return name.replace(old, league);
		}
	}

	return `${league}${UNDERSCORE_GLUE}${name}`;
};

export const createFileCard = async (name: string, csv: string, league: TradeLeague): Promise<FileCardProps> => {
	const sample = await command('sample', { csv, league });

	return {
		uuid: crypto.randomUUID(),
		filename: sample.type === 'ok' ? prefixFilename(name, league) : name,
		league,
		sample,
		selected: false,
		minimumCardPrice: 0,
	};
};

export const useFileCardsStore = defineStore('this', {
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
		async addCard(filename: string, csv: string, league: TradeLeague = ACTIVE_LEAGUE): Promise<void> {
			this.fileCards.push(await createFileCard(filename, csv, league));
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

		async addFromTab(league: League, tab: StashTab) {
			const tradeLeague = isTradeLeague(league) ? league : ACTIVE_LEAGUE;

			const sample = await command('sample_cards', {
				cards: cardsFromTab(tab),
				league: tradeLeague,
			});

			const data = sample.type === 'ok' ? sample.data.csv : sample.error;
			this.addCard(`${tab.name}.csv`, data, tradeLeague);
		},

		async replaceFileCard(league: League, oldFileCard: FileCardProps) {
			if (!isTradeLeague(league)) return;

			const index = this.fileCards.findIndex(file => file.uuid === oldFileCard.uuid);
			if (index === -1) return;
			if (oldFileCard.sample.type === 'err') return;

			const newSample = await command('league', { league, sample: oldFileCard.sample.data });
			const data = newSample.type === 'ok' ? newSample.data.csv : newSample.error;
			const newFileCard = await createFileCard(oldFileCard.filename, data, league);
			this.fileCards[index] = newFileCard;
			this.fileCards = Array.from(this.fileCards);
		},

		async replaceMerged(league: League) {
			const merged = this.mergedFile;
			if (!merged || merged.sample.type === 'err') return;
			if (!isTradeLeague(league)) return;
			const newSample = await command('league', { league, sample: merged.sample.data });
			const data = newSample.type === 'ok' ? newSample.data.csv : newSample.error;
			this.mergedFile = await createFileCard(merged.filename, data, league);
		},
	},
});
