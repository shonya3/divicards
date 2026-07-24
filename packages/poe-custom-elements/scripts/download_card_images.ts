import fs from 'node:fs';
import fsPromises from 'node:fs/promises';
import path from 'node:path';
import { Readable } from 'node:stream';
import sharp from 'sharp';
import * as v from 'valibot';



const CardElementSchema = v.array(
	v.object({
		slug: v.string(),
		name: v.string(),
		artFilename: v.string(),
	}),
);

const CONCURRENCY = 16;
process.env.UV_THREADPOOL_SIZE = CONCURRENCY.toString();
const ROOT_DIR = path.resolve(import.meta.dirname, '..');
const POE_CDN_CARDS = 'https://web.poecdn.com/image/divination-card';
const IMAGES_DIR = path.join(ROOT_DIR, 'public/poe-ce-assets/divination-card/cards/avif');
const CARD_DATA_PATH = path.resolve(ROOT_DIR, '../dump/out/cardElementData.json');

type CardElement = v.InferOutput<typeof CardElementSchema>[number];

async function main() {
	if (!fs.existsSync(IMAGES_DIR)) {
		await fsPromises.mkdir(IMAGES_DIR, { recursive: true });
	}

	const raw = JSON.parse(await fsPromises.readFile(CARD_DATA_PATH, 'utf-8'));
	const cards = v.parse(CardElementSchema, raw);
	console.log(`Read ${cards.length} cards`);

	const total = cards.length;
	let ok = 0;
	let fail = 0;
	let done = 0;

	const report = () => process.stdout.write(`\r${done}/${total} — ${ok} ok, ${fail} failed`);

	const workers = Array.from({ length: CONCURRENCY }, async () => {
		while (true) {
			const card = cards.pop();
			if (!card) break;
			try {
				await download_poecdn_image(card);
				ok++;
			} catch (e) {
				fail++;
			}
			done++;
			report();
		}
	});
	await Promise.all(workers);
	console.log(`\nDone — ${ok} downloaded, ${fail} failed`);
}
main();

async function download_poecdn_image(card: CardElement): Promise<void> {
	const url = `${POE_CDN_CARDS}/${card.artFilename}.png`;
	const response = await fetch(url);
	if (!response.ok) {
		throw new Error(`Could not download card. Code: ${response.status}`);
	}

	const image_path = path.join(IMAGES_DIR, `${card.artFilename}.avif`);
	const to_avif = sharp().avif();
	const write_file_stream = fs.createWriteStream(image_path, { flags: 'w' });
	await new Promise<void>((resolve, reject) => {
		//@ts-expect-error
		Readable.fromWeb(response.body)
			.pipe(to_avif)
			.pipe(write_file_stream)
			.on('finish', resolve)
			.on('error', reject);
	});
}
