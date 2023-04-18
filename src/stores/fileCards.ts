import { downloadFile } from './../lib';
import { defineStore } from 'pinia';
import { FileCardProps } from '../components/FileCard/FileCard.vue';
import { csvFile } from '../lib';
import { useCreateFileCard } from '../composables/useCreateFileCard';
import { command } from '../command';

type State = {
	fileCards: FileCardProps[];
	mergedFile: FileCardProps | null;
};

export const useFileCardsStore = defineStore('filecardsStore', {
	state: (): State => ({
		fileCards: [],
		mergedFile: null,
	}),
	getters: {
		selectedFiles(): FileCardProps[] {
			return this.fileCards.filter(file => file.selected && file.valid);
		},

		validFiles(): FileCardProps[] {
			return this.fileCards.filter(file => file.valid);
		},

		selectedStrings(): string[] {
			return this.selectedFiles.map(file => file.data.csvPolished);
		},

		getFileById: state => {
			return (id: string) => [...state.fileCards, state.mergedFile].find(file => file?.id === id);
		},
	},
	actions: {
		async updateAllCardsPrice(id: string, newMinimumPrice: number) {
			const file = this.getFileById(id);
			if (!file) return;
			const price = await command('all_cards_price', {
				csvString: file.data.csvPolished,
				minimumCardPrice: newMinimumPrice,
			});

			file.data.allCardsPrice = price;
		},

		downloadAll() {
			this.validFiles.forEach(({ filename, href }) => downloadFile(filename, href));
		},

		async merge() {
			const mergedCsv = await command('merge_csv', { csvFileStrings: this.selectedStrings });
			const file: File = csvFile(mergedCsv, 'merged.csv');

			this.mergedFile = useCreateFileCard(file, 50);
			// No point to select merged file, `null` makes it nonselectable by removing checkbox
			// maybe should refactor later
			this.mergedFile.selected = null;
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

		addCards(files: File[], minimumCardPrice = 50): void {
			for (const file of files) {
				this.fileCards.push(useCreateFileCard(file, minimumCardPrice));
			}
		},
	},
});
