import { downloadFile } from './../lib';
import { ACTIVE_LEAGUE } from '@divicards/shared/lib';
import { defineStore } from 'pinia';
import { FileCardProps } from '../components/FileCard/FileCard.vue';
import { useFileCard } from '../composables/useFileCard';
import { command } from '../command';
import { DivinationCardsSample, TradeLeague } from '@divicards/shared/types';

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
			return this.fileCards.filter(file => file.selected && file.valid);
		},

		samples(): DivinationCardsSample[] {
			return this.fileCards.map(f => f.sample);
		},

		selectedSamples(): DivinationCardsSample[] {
			return this.selectedFiles.map(({ sample }) => sample);
		},

		validFiles(): FileCardProps[] {
			return this.fileCards.filter(file => file.valid);
		},

		selectedStrings(): string[] {
			return this.selectedFiles.map(file => String(file.sample.chaos));
		},

		getFileById: state => {
			return (id: string) => [...state.fileCards, state.mergedFile].find(file => file?.id === id);
		},
	},
	actions: {
		addCards(files: File[], league: TradeLeague = ACTIVE_LEAGUE): void {
			for (const file of files) {
				this.fileCards.push(useFileCard(file, league));
			}
		},

		async merge() {
			const sample = await command('merge', { samples: this.selectedSamples });
			const file: File = new File([sample.polished], 'merged.csv');

			const fileCard = useFileCard(file, ACTIVE_LEAGUE);

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
			this.fileCards = this.fileCards.filter(file => file.id !== id);
		},

		deleteAllFiles(): void {
			this.fileCards = [];
		},
	},
});
