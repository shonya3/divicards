#[cfg(test)]
mod tests {
    use divcord::table::{rich::DropsFrom, table_record::DivcordTableRecord};
    use poe_data::PoeData;

    #[tokio::test]
    async fn two_dropsources_should_throw_error() {
        use divcord::dropsource::parse::*;
        use divcord::table::{
            rich::{FontStyles, HexColor},
            table_record::{Confidence, GreyNote, RemainingWork},
        };

        let poe_data = PoeData::load().await.unwrap();
        let record =  DivcordTableRecord {
    id: 230,
    greynote: GreyNote::Empty,
    card: "The Demon".to_owned(),
    tag_hypothesis: Some(
        "kitava_area".to_owned(),
    ),
    confidence: Confidence::Ok,
    remaining_work: RemainingWork::UnclearHypothesis,
    sources_drops_from: vec![
        DropsFrom {
            name: "Kitava, The Destroyer (The Destroyer's Heart), Lord of the Grey".to_owned(),
            styles: FontStyles {
                color: HexColor(
                    "#FFFFFF".to_owned(),
                ),
                italic: false,
                strikethrough: false,
            },
        },
    ],
    verify_drops_from: vec![],
    wiki_disagreements: Some(
        "Megaera".to_owned(),
    ),
    sources_with_tag_but_not_on_wiki: None,
    notes: Some(
        "We recently got video evidence of The Demon dropping from Kitava, the Destroyer in Lava Lake Map.\nBut Lava Lake Map was not on-Atlas in 3.17 Archnemesis.\nSo following GGG's rule, it must have had a secondary drop location -- presumably via `kitava_map_boss`, which applies to \"The Destroyer's Heart\" (the heart of Kitava, the Destroyer in Lava Lake) and \"Lord of the Grey\" in Belfry.\n\nSo presumably The Demon should also drop from Lord of the Grey, as some people had speculated in the past.\n3.23: Added Crater - is this now kitava_area? Or new tag? Patch notes indirectly confirmed that The Wrath and The Demon most likely share tags - is this an area tag or boss tag?".to_owned(),
    ),
};

        let source = parse_one_drops_from(&record.sources_drops_from[0], &record, &poe_data);
        assert!(source.is_err());
    }
}
