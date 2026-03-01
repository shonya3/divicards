import { file } from "bun";

const maps: Array<any> = await file("../maps.json").json();




console.log(maps.filter(m => m.series === "Keepers" && !m.unique).length)