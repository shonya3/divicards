//@ts-check

/**
 * @typedef {Object} MapBoss
 * @property {string} name
 * @property {string[]} maps
 */

/**
 * Fetch bosses from https://www.poewiki.net/wiki/Map_bosses
 * @returns {MapBoss[]}
 */
const fetchMapBosses = () => {
	const tableRows = document.querySelector('table')?.querySelector('tbody')?.querySelectorAll('tr');
	if (tableRows == null) {
		throw new Error('No table');
	}

	/** @type Map<string, string[]> */
	const dataMap = new Map();

	for (const row of tableRows) {
		const [nameCol, mapCol] = row.querySelectorAll('td');
		const boss = nameCol.innerText.trim();
		const map = mapCol.innerText.trim();

		const maps = dataMap.get(boss) ?? [];
		maps.push(map);
		dataMap.set(boss, maps);
	}

	/** @type{MapBoss[]} */
	const mapBosses = [];
	for (const [n, maps] of dataMap) {
		let name = n;

		switch (name) {
			case 'Drought Maddened Rhoa':
				name = 'Drought-Maddened Rhoa';
				break;
			case 'Hephaestus, the Hammer':
				name = 'Hephaeus, The Hammer';
				break;
			case 'Kitava, the Destroyer':
				name = 'Kitava, The Destroyer';
				break;
			case 'Sanctum Guard':
				name = 'Sanctum Guardian';
				break;
			case 'Unraveling Horror':
				name = 'Unravelling Horror';
				break;
		}

		mapBosses.push({ name, maps });
	}

	const cleansingLight = { name: 'The Cleansing Light', maps: ['Basilica Map'] };
	const opid = { name: "Opid, Helial's Herald", maps: ['The Twilight Temple'] };
	const vindicatedQueen = { name: 'The Vindicated Queen', maps: ['Caldera Map'] };

	mapBosses.push(cleansingLight, opid, vindicatedQueen);
	mapBosses.push({ name: 'Nightmare of the Depraved Trinity', maps: ['Abomination Map'] });
	mapBosses.push({ name: 'Nightmare of Catarina', maps: ['Ziggurat Map'] });

	return mapBosses;
};
