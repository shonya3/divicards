import { downloadFile } from './../lib';
import { defineStore } from 'pinia';
import { FileCardProps } from '../components/FileCard/FileCard.vue';
import { csvFile } from '../lib';
import { useCreateFileCard } from '../composables/useCreateFileCard';
import { command } from '../command';
import { watch } from 'vue';
import { DivinationCardsSample } from '../types';

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
		downloadAll() {
			this.validFiles.forEach(({ filename, href }) => downloadFile(filename, href));
		},

		async merge() {
			const sample = await command('merge', { samples: this.selectedSamples });
			const file: File = csvFile(sample.polished, 'merged.csv');

			const fileCard = useCreateFileCard(file, 0);
			watch(
				() => fileCard.minimumCardPrice,
				async val => {
					fileCard.sample.chaos = await command('chaos', {
						csv: fileCard.sample.polished,
						min: val,
					});
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

		addCards(files: File[], minimumCardPrice = 0): void {
			for (const file of files) {
				const fileCard = useCreateFileCard(file, minimumCardPrice);
				watch(
					() => fileCard.minimumCardPrice,
					async val => {
						fileCard.sample.chaos = await command('chaos', {
							csv: fileCard.sample.polished,
							min: val,
						});
					}
				);

				this.fileCards.push(fileCard);
			}
		},
	},
});
