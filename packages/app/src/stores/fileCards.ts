import { downloadFile } from './../lib';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';
import { defineStore } from 'pinia';
import { useFileCard } from '../composables/useFileCard';
import { command } from '../command';
import { DivinationCardsSample, Result, TradeLeague } from '@divicards/shared/types';
import { FileCardProps } from '@divicards/wc/src/wc/file-card/file-card';

export const useFileCardsStore = defineStore('filecardsStore', {
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
		async addCards(files: File[], league: TradeLeague = ACTIVE_LEAGUE): Promise<void> {
			for (const file of files) {
				this.fileCards.push(await useFileCard(file, league));
			}
		},

		async merge() {
			const sample = await command('merge', { samples: this.selectedSamples });
			const file: File = new File([sample.csv], 'merged.csv');

			const fileCard = await useFileCard(file, ACTIVE_LEAGUE);

			// No point to select merged file, `null` makes it nonselectable by removing checkbox
			// maybe should refactor later
			fileCard.selected = null;
			this.mergedFile = fileCard;
		},

		downloadAll() {
			this.validFiles.forEach(({ filename, href }) => downloadFile(filename, href));
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
	},
});
