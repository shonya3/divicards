// import { maxedStackSize } from './jsons/maxedStackSize.js';
import { divination } from './jsons/tabs/divination.js';
import { fragments } from './jsons/tabs/myfragments.js';
import { blight } from './jsons/tabs/blight.js';
import { essence } from './jsons/tabs/essence.js';
import { influence } from './jsons/influence.js';
import { fracturedGloves } from './jsons/fracturedGloves.js';
import { socketed } from './jsons/socketed.js';
import { rogueMarkers } from './jsons/rogueMarkers.js';
import { allflame } from './jsons/allflame.js';
import { elementalBow } from './jsons/elementalBow.js';
import { LitElement, html, css, TemplateResult, CSSResult } from 'lit';
import { customElement, state } from 'lit/decorators.js';
import type { PoeItem } from './poe.types.js';
import './elements/poe-item.js';
import './elements/poe-stash-tab.js';
import './elements/item-info/poe-item-info.js';
import './elements/item-info/poe-item-info-header.js';
import { TabWithItems } from './poe.types.js';
import quadJson from './jsons/QuadStash.json' with { type: 'json' };
import premJson from './jsons/PremiumStash.json' with { type: 'json' };
import influenceJson from './jsons/influence.json' with { type: 'json' };
import garbageJson from './jsons/garbage.json' with { type: 'json' };
import { a as aTab } from './jsons/tabs/a.js';
import { quadStd } from './jsons/tabs/quadStd.js';
import { currencyTab } from './jsons/tabs/currencyTab.js';

@customElement('app-root')
export class AppRoot extends LitElement {
	@state() quad = quadJson as TabWithItems;
	@state() prem = premJson as TabWithItems;
	@state() item: PoeItem = item();
	@state() influenceTab = influenceJson as TabWithItems;
	@state() garbageTab = garbageJson as TabWithItems;

	protected render(): TemplateResult {
		return html`<poe-divination-card name="The Silly Boy"></poe-divination-card>`;
		// return html`<poe-item .item=${maxedStackSize}></poe-item>`;
	}

	protected Items(): TemplateResult {
		return html` <poe-item .item=${aTab.items!.find(i => i.baseType === 'Plated Maul')!}></poe-item>
			<poe-item .item=${socketed}></poe-item>
			<poe-item .item=${aTab.items!.find(i => i.baseType === 'Plated Maul')!}></poe-item>
			<poe-item .item=${fracturedGloves}></poe-item>
			<poe-item .item=${rogueMarkers}></poe-item>`;
	}

	protected Tabs(): TemplateResult {
		return html`
			<div style="display:flex; flex-wrap:wrap; gap:0.5rem">
				<!--
                			<poe-stash-tab .tab=${essence}></poe-stash-tab>
			<poe-stash-tab .tab=${currencyTab}></poe-stash-tab>
            <poe-stash-tab .tab=${this.quad}></poe-stash-tab>
            <poe-stash-tab style="margin-top: 2px" .tab=${this.prem}></poe-stash-tab>
            <poe-stash-tab .tab=${this.garbageTab}></poe-stash-tab>
            <poe-stash-tab .tab=${this.influenceTab}></poe-stash-tab>
            -->
				<poe-stash-tab .tab=${divination}></poe-stash-tab>
				<poe-stash-tab .tab=${blight}></poe-stash-tab>
				<poe-stash-tab .tab=${fragments}></poe-stash-tab>
				<poe-stash-tab .tab=${quadStd}></poe-stash-tab>
			</div>
		`;
	}

	protected ItemInfos(): TemplateResult {
		return html`<poe-item-info .item=${influence}></poe-item-info>
			<poe-item-info .item=${socketed}></poe-item-info>
			<poe-item-info .item=${allflame}></poe-item-info>
			<poe-item-info .item=${elementalBow}></poe-item-info>
			<poe-item-info .item=${aTab.items!.find(i => i.baseType === 'Plated Maul')!}></poe-item-info>`;
	}

	static styles: CSSResult = css`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}
	`;
}

function item(): PoeItem {
	return {
		baseType: 'Glorious Leather',
		explicitMods: [
			'+13% chance to Suppress Spell Damage',
			'+100 to Evasion Rating',
			'+77 to maximum Life',
			'+28% to Fire Resistance',
			'+39% to Lightning Resistance',
		],
		frameType: 2,
		h: 3,
		icon: 'https://web.poecdn.com/gen/image/WzI1LDE0LHsiZiI6IjJESXRlbXMvQXJtb3Vycy9Cb2R5QXJtb3Vycy9Cb2R5RGV4MUMiLCJ3IjoyLCJoIjozLCJzY2FsZSI6MX1d/47360dcc4a/BodyDex1C.png',
		id: 'd30b5d8f1b66c43e24051760aff4afc71b1e00f21b299baffeb7b7394fef6575',
		identified: true,
		ilvl: 62,
		inventoryId: 'Stash1',
		league: 'Hardcore Necropolis',
		name: 'Empyrean Shelter',
		properties: [
			{
				displayMode: 0,
				name: 'Evasion Rating',
				type: 17,
				values: [['617', 1]],
			},
		],
		rarity: 'Rare',
		requirements: [
			{
				displayMode: 0,
				name: 'Level',
				type: 62,
				values: [['48', 0]],
			},
			{
				displayMode: 1,
				name: 'Dex',
				type: 64,
				values: [['124', 0]],
			},
		],
		socketedItems: [],
		sockets: [
			{
				attr: 'S',
				group: 0,
				sColour: 'R',
			},
			{
				attr: 'S',
				group: 0,
				sColour: 'R',
			},
			{
				attr: 'I',
				group: 1,
				sColour: 'B',
			},
			{
				attr: 'D',
				group: 1,
				sColour: 'G',
			},
			{
				attr: 'D',
				group: 2,
				sColour: 'G',
			},
			{
				attr: 'D',
				group: 2,
				sColour: 'G',
			},
		],
		typeLine: 'Glorious Leather',
		verified: false,
		w: 2,
		x: 22,
		y: 12,
	} as PoeItem;
}

declare global {
	interface HTMLElementTagNameMap {
		'app-root': AppRoot;
	}
}
