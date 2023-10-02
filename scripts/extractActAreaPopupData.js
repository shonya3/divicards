//@ts-check

/**
 * @typedef {Object} ActArea
 * @property {string} id
 * @property {string} name
 * @property {number} act
 * @property {number} areaLevel
 * @property {string} imageUrl
 * @property {string} poedbImageUrl
 * @property {boolean} hasLabyrinthTrial
 * @property {boolean} isTown
 * @property {Bossfight[]} bossfights
 * @property {string} flavourText
 */

/**
 * @typedef {Object} Bossfight
 * @property {string} name
 * @property {string} url
 */

/**
 * @param {HTMLElement} actAreaPopup
 * @returns {ActArea}
 */
const extractActAreaPopupData = actAreaPopup => {
	const TOWN_IMAGE_URL = 'https://cdn.poedb.tw/image/Art/2DArt/UIImages/InGame/WorldPanelTownPinIcon.webp';
	const WAYPOINT_IMAGE_URL = 'https://cdn.poedb.tw/image/Art/2DArt/UIImages/InGame/WorldPanelWaypointIcon.webp';
	const LABYRINTH_TRIAL_IMAGE_URL =
		'https://cdn.poedb.tw/image/Art/2DArt/UIImages/InGame/WorldPanelLabyrinthWaypointPinIcon.webp';

	const img = actAreaPopup.querySelector('.itemboximage img');
	const stats = document.querySelector('.Stats');
	if (!(img instanceof HTMLImageElement)) throw new Error('no img element');
	if (!stats) throw new Error('no stats element');
	const hasWaypoint = stats?.querySelector(`[src="${WAYPOINT_IMAGE_URL}"]`) !== null;
	const isTown = stats?.querySelector(`[src="${TOWN_IMAGE_URL}"]`) !== null;
	const hasLabyrinthTrial = stats?.querySelector(`[src="${LABYRINTH_TRIAL_IMAGE_URL}"]`) !== null;

	const itemHeader = actAreaPopup.querySelector('.itemHeader');
	if (!(itemHeader instanceof HTMLElement)) throw new Error('no itemHeader element');
	const name = itemHeader.innerText;
	let id = null;
	let act = null;
	let areaLevel = null;
	let bossfights = [];
	let flavourText = stats.querySelector('.FlavourText')?.textContent ?? null;
	const poedbImageUrl = img.src;
	const slashIndex = poedbImageUrl.lastIndexOf('/');
	const filename = poedbImageUrl.slice(slashIndex);
	const imageUrl = `/images/acts${filename}`;

	const props = stats.querySelectorAll('.property');
	for (const prop of props) {
		if (!(prop instanceof HTMLElement)) {
			throw new Error(`==${name}== prop should be HTMLElement`);
		}

		const text = prop.innerText;

		const valueElement = prop.querySelector('.text-type0');
		if (!(valueElement instanceof HTMLElement)) {
			continue;
		}

		if (text.includes('Id:')) {
			id = valueElement.innerText;
		}

		if (text.includes('Act: ')) {
			act = Number(valueElement.innerText);
		}

		if (text.includes('Area Level')) {
			areaLevel = Number(valueElement.innerText);
		}

		if (text.includes('Boss Fights')) {
			const span = prop.querySelector('span');
			if (!span) {
				continue;
			}
			const aElements = Array.from(prop.querySelector('span')?.querySelectorAll('a') ?? []);
			for (const a of aElements) {
				bossfights.push({
					name: a.innerText,
					url: a.href,
				});
			}
		}
	}

	if (!id) throw new Error(`==${name}== No id`);
	if (!act) throw new Error(`==${name}== no act`);
	if (!areaLevel) throw new Error(`==${name}== no area level`);
	if (!flavourText) throw new Error(`==${name}== No flavourText`);

	const data = {
		id,
		name,
		act,
		areaLevel,
		imageUrl,
		poedbImageUrl,
		hasWaypoint,
		hasLabyrinthTrial,
		isTown,
		bossfights,
		flavourText,
	};

	return data;
};
