import { LitElement, html, css, TemplateResult, nothing, CSSResult } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import type { ItemProperty } from '../../poe.types.js';
import { classMap } from 'lit/directives/class-map.js';
import { parseDisplayMode3 } from '../../lib/internal.js';

@customElement('poe-item-info-property')
export class PoeItemInfoPropertyElement extends LitElement {
	@property({ type: Object }) property!: ItemProperty;

	protected render(): TemplateResult {
		if (!this.property) {
			return html`<p style="color: red">No item property provided</p>`;
		}

		return html`<div class="property">${Values(this.property)}</div>`;
	}

	static styles: CSSResult = css`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}
		.property {
			color: #7f7f7f;
		}
		.value {
			color: #fff;
		}
		.fire {
			color: #960000;
		}
		.cold {
			color: #366492;
		}
		.lightning {
			color: gold;
		}
		.augmented {
			color: #88f;
		}
	`;
}

function colorValueClass(num: number): string {
	switch (num) {
		case 1:
			return 'augmented';
		case 4:
			return 'fire';
		case 5:
			return 'cold';
		case 6:
			return 'lightning';
		default:
			return '';
	}
}

function Values(property: ItemProperty) {
	if (property.displayMode === 0) {
		if (!property.values.length) {
			return property.name;
		}

		const values = property.values.map((value, i) => {
			return html`<span
					class=${classMap({
						value: true,
						[`${colorValueClass(value[1])}`]: true,
					})}
					>${value[0]}</span
				>${i === property.values.length - 1 ? nothing : ','} `;
		});
		return html`${property.name}: ${values}`;
	}

	if (property.displayMode === 3) {
		return parseDisplayMode3(property, value => html`<span class="value">${value}</span>`);
	}

	return nothing;
}

// displayMode 0 - with. If value, with :, if no, nothing
/* 
{ "displayMode": 0, "name": "Attack, Totem, AoE, Slam, Melee", "values": [] },
{ "displayMode": 0, "name": "Level", "type": 5, "values": [["17", 0]] },
{ "displayMode": 0, "name": "Cost", "values": [["10 Mana", 0]] },
{ "displayMode": 0, "name": "Attack Speed", "values": [["90% of base", 0]] },
{ "displayMode": 0, "name": "Attack Damage", "values": [["157.2% of base", 0]] },
{ "displayMode": 0, "name": "Effectiveness of Added Damage", "values": [["157%", 0]] }
 */
/*  displayMode 1 -
{ "displayMode": 1, "name": "Str", "type": 63, "values": [["142", 0]] }
becomes 142 Str, where 142 - white, Str - grey
*/

/* "displayMode": 2
"additionalProperties": [
	{
		"displayMode": 2,
		"name": "Experience",
		"progress": 0.47,
		"type": 20,
	    "values": [["7660059/16159983", 0]]
	}
],
*/

/*
Item Class: Two Hand Maces
Rarity: Rare
Honour Mangler
Plated Maul
--------
Two Handed Mace
Physical Damage: 161-241 (augmented)
Elemental Damage: 92-187 (augmented)
Critical Strike Chance: 5.00%
Attacks per Second: 1.47 (augmented)
Weapon Range: 1.3 metres
--------
Requirements:
Level: 66
Str: 164
Int: 46
--------
Sockets: R-R-R-R 
--------
Item Level: 58
--------
30% increased Stun Duration on Enemies (implicit)
--------
+17 to Strength
53% increased Physical Damage
Adds 92 to 187 Fire Damage
22% increased Attack Speed
27% increased Stun Duration on Enemies
+104 to Accuracy Rating
70% increased Physical Damage (crafted) 
 */

/*
Item Class: Bows
Rarity: Rare
Foe Volley
Decimation Bow
--------
Bow
Physical Damage: 44-116
Elemental Damage: 85-176 (augmented), 101-156 (augmented), 3-47 (augmented)
Critical Strike Chance: 7.30% (augmented)
Attacks per Second: 1.20
--------
Requirements:
Level: 53
Dex: 170
--------
Sockets: G-G-G-G 
--------
Item Level: 55
--------
46% increased Critical Strike Chance (implicit)
--------
Adds 85 to 176 Fire Damage
Adds 101 to 156 Cold Damage
Adds 3 to 47 Lightning Damage
+24% to Global Critical Strike Multiplier
--------
Note: ~b/o 2 chaos

*/

declare global {
	interface HTMLElementTagNameMap {
		'poe-item-info-property': PoeItemInfoPropertyElement;
	}
}
