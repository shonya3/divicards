import { downloadFile } from './../lib';
import { defineStore } from 'pinia';
import { FileCardProps } from '../components/FileCard/FileCard.vue';
import { csvFile } from '../lib';
import { useCreateFileCard } from '../composables/useCreateFileCard';
import { command } from '../command';
import { watch } from 'vue';

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
		downloadAll() {
			this.validFiles.forEach(({ filename, href }) => downloadFile(filename, href));
		},

		async merge() {
			const mergedCsv = await command('merge_csv', { csvFileStrings: this.selectedStrings });
			const file: File = csvFile(mergedCsv, 'merged.csv');

			const fileCard = useCreateFileCard(file, 50);
			watch(
				() => fileCard.minimumCardPrice,
				async val => {
					const price = await command('all_cards_price', {
						csvString: fileCard.data.csvPolished,
						minimumCardPrice: val,
					});

					fileCard.data.allCardsPrice = price;
				}
			);
			// No point to select merged file, `null` makes it nonselectable by removing checkbox
			// maybe should refactor later
			fileCard.selected = null;
			this.mergedFile = fileCard;
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
				const fileCard = useCreateFileCard(file, minimumCardPrice);
				watch(
					() => fileCard.minimumCardPrice,
					async val => {
						const price = await command('all_cards_price', {
							csvString: fileCard.data.csvPolished,
							minimumCardPrice: val,
						});

						fileCard.data.allCardsPrice = price;
					}
				);

				this.fileCards.push(fileCard);
			}
		},
	},
});
