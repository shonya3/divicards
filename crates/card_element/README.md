# card_element

Pure data model for the divination card custom element, consumed by the
[poe-custom-elements](https://github.com/shonya3/poe-custom-elements) library
(used by both divicards-site and the divicards app).

Contains the serde model types only — no fetching, no I/O:

- [`DivinationCardElementData`] — the payload of `cardElementData.json`, consumed
  by the site's `<divination-card>` custom element
- [`UniqueReward`] — the unique item a card can reward, with its resolved item class

## Model

| Type | Purpose |
|------|---------|
| `DivinationCardElementData` | Per-card element data: `slug`, `name`, `artFilename`, `rewardHtml`, `flavourText`, `stackSize`, `minLevel`, `unique` |
| `UniqueReward` | `name` + `item_class` of the unique item rewarded by a card |

`DivinationCardElementData` serializes with `camelCase` field names; the nested
`UniqueReward` uses `item_class` (snake_case). Both match the JSON consumed by
the site.

## Producer

The data is produced by the extraction pipeline in the
[`poe_data`](../poe_data) crate (`cards::card_element_data`), which enriches raw
game-file data with reward descriptions, item classes, weights and prices from
poewiki.net, RePoE, poe.ninja and Google Sheets.

## Consumers

- [`poe_data`](../poe_data) — produces instances
- [`generate_website_files`](../generate_website_files) — writes `cardElementData.json`
  to the divicards-site `gen/json/` directory and syncs it (pretty-printed) to the
  poe-custom-elements repository

## Output shape

```json
{
  "slug": "the-doctors",
  "name": "The Doctor",
  "artFilename": "TheDoctor",
  "rewardHtml": "<div class=reward><p><span class=uniqueItem>Headhunter</span></p></div>",
  "flavourText": "...",
  "stackSize": 8,
  "minLevel": 35,
  "unique": { "name": "Headhunter", "item_class": "Leather Belt" }
```
