export type SourceWithMember = { type: SourceType; id: string; kind: SourceWithMemberKind };
export type EmptySourceKind = 'empty-source';
export type SourceWithMemberKind = 'source-with-member';
export type Kind = EmptySourceKind | SourceWithMemberKind;
export type EmptySource = { type: SourceType; kind: EmptySourceKind };
export type ISource = SourceWithMember | EmptySource;
export const sourceTypes = ["Expedition Logbook","Chest","Delirium","Strongbox","Vendor","Unknown","Delirium Currency Rewards","Redeemer influenced maps","Disabled","Global Drop","Act","Map","Map Boss","Act Boss","Maven's Invitation: The Feared","Uul-Netol, Unburdened Flesh (in Breachstones)","The Vaal Omnitect","Metamorph","Null Portal","Vaal Flesh Merchant","All Incursion Architects in Alva missions/Alva's Memory","All Abyss Monsters","All (Scourge) beyond demons","All Rogue Exiles","Venarius","Argus","All Invasion Bosses","All Vaal Side Area Bosses","Breachlord Boss Domain","Architect","Shaper Guardian Boss","Syndicate Member","Elder Slayer","Elder Guardian Boss","Rogue Exile","Female Rogue Exile","Abyss Lich Boss","Maps Only","Act Boss","Harbinger Portal","Endgame Boss","Delve Boss","Beast Boss","Heist Boss","Beyond Boss","Expedition Logbook Boss","Shaper Mini-Boss","Betrayal Catarina","Oshabi Boss","Trial of Stinging Doubt","The Temple of Atzoatl","All Vaal side areas (need specific information)","Vaal Side Areas","Atziri Area","Area-Specific"] as const;

export type SourceType = (typeof sourceTypes)[number];
    