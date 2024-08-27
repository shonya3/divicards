#[cfg(test)]
mod tests {
    use std::time::Duration;

    use divcord::parse::*;
    use divcord::spreadsheet::{
        record::{Confidence, GreyNote, RemainingWork},
        rich::{FontStyles, HexColor},
    };
    use divcord::{
        cards::FromChild,
        records,
        spreadsheet::{
            load::{DataFetcher, SpreadsheetFetcher, Stale},
            record::Dumb,
            rich::DropsFrom,
        },
        Source,
    };
    use poe_data::act::ActAreaId;
    use poe_data::PoeData;

    async fn load_spreadsheet() -> divcord::Spreadsheet {
        let mut fetcher = SpreadsheetFetcher::default();
        fetcher.0.stale = Stale::After(Duration::from_secs(81400));
        fetcher.load().await.unwrap()
    }

    #[tokio::test]
    async fn dropsources_comma_separated_produce_error() {
        let poe_data = PoeData::load().await.unwrap();
        let record =  Dumb {
    id: 230,
    confidence: Confidence::Ok,
    greynote: GreyNote::Empty,
    card: "The Demon".to_owned(),
    tag_hypothesis: Some(
        "kitava_area".to_owned(),
    ),
    remaining_work: RemainingWork::UnclearHypothesis,
    drops: vec![
        DropsFrom {
            name: "Kitava, The Destroyer (The Destroyer's Heart), Lord of the Grey".to_owned(),
            styles: FontStyles {
                color: HexColor::White,
                italic: false,
                strikethrough: false,
            },
        },
    ],
    drops_to_verify: vec![],
    notes: Some(
        "We recently got video evidence of The Demon dropping from Kitava, the Destroyer in Lava Lake Map.\nBut Lava Lake Map was not on-Atlas in 3.17 Archnemesis.\nSo following GGG's rule, it must have had a secondary drop location -- presumably via `kitava_map_boss`, which applies to \"The Destroyer's Heart\" (the heart of Kitava, the Destroyer in Lava Lake) and \"Lord of the Grey\" in Belfry.\n\nSo presumably The Demon should also drop from Lord of the Grey, as some people had speculated in the past.\n3.23: Added Crater - is this now kitava_area? Or new tag? Patch notes indirectly confirmed that The Wrath and The Demon most likely share tags - is this an area tag or boss tag?".to_owned(),
    ),
};

        let source = parse_one_drops_from(&record.drops[0], &record, &poe_data);
        assert!(source.is_err());
    }

    #[tokio::test]
    async fn child_sources_for_acts() {
        let poe_data = PoeData::load().await.unwrap();
        let spreadsheet = load_spreadsheet().await;

        let records = records(&spreadsheet, &poe_data).unwrap();
        let dried_lake = Source::Act(ActAreaId::new("1_4_2".to_owned()));

        let vec_from_child =
            divcord::cards::cards_from_child_sources(&dried_lake, &records, &poe_data);

        assert!(vec_from_child.contains(&FromChild {
            source: dried_lake.to_owned(),
            card: "The Fletcher".to_owned(),
            column: divcord::parse::RichColumnVariant::Verify,
            child: Source::ActBoss("Nightwane".to_owned())
        }))
    }
}
