use divcord::{
    parse::{parse_dropses_from, parse_dropses_from2, parse_one_drops_from, RichColumnVariant},
    spreadsheet::{
        self,
        record::{Confidence, Dumb, GreyNote, RemainingWork},
        rich::DropsFrom,
    },
    PoeData, Source, Spreadsheet,
};

#[tokio::test]
#[cfg(feature = "fetch")]
async fn compare() {
    let poe_data = PoeData::load().await.unwrap();
    let spreadsheet = Spreadsheet::load().await.unwrap();

    for dumb in spreadsheet.dumb_records() {
        let dumb = dumb.unwrap();

        let s1 = parse_dropses_from(&dumb, &poe_data, RichColumnVariant::Sources).unwrap();
        let s2 = parse_dropses_from2(&dumb, &poe_data, RichColumnVariant::Sources).unwrap();
        assert_eq!(s1, s2);

        let v1 = parse_dropses_from(&dumb, &poe_data, RichColumnVariant::Verify).unwrap();
        let v2 = parse_dropses_from(&dumb, &poe_data, RichColumnVariant::Verify).unwrap();
        assert_eq!(v1, v2);
    }
}

// /// Parses all instances of record's drops_from and collects it into one Vec<Source>
// pub fn parse_dropses_from2(
//     dumb: &Dumb,
//     poe_data: &PoeData,
//     column: RichColumnVariant,
// ) -> Result<Vec<Source>, UnknownDropsFrom> {
//     let mut sources: Vec<Source> = vec![];
//     let drops_to_parse = match column {
//         RichColumnVariant::Sources => &dumb.drops,
//         RichColumnVariant::Verify => &dumb.drops_to_verify,
//     };

//     for d in drops_to_parse {
//         let inner_sources = parse_one_drops_from(d, dumb, poe_data)?;
//         sources.extend(inner_sources);
//     }

//     Ok(sources)
// }

#[tokio::test]
#[cfg(feature = "fetch")] // cargo test --features fetch
async fn parses_spreadsheet() {
    use divcord::Spreadsheet;

    let (poe_data, spreadsheet) = tokio::join!(PoeData::load(), Spreadsheet::fetch());
    let poe_data = poe_data.unwrap();
    let spreadsheet = spreadsheet.unwrap();
    let _records = divcord::records(&spreadsheet, &poe_data).unwrap();
}

fn create_dumb(card: &str, drops_from: DropsFrom) -> Dumb {
    Dumb {
        id: 1,
        greynote: GreyNote::Empty,
        card: card.to_owned(),
        tag_hypothesis: None,
        confidence: Confidence::Done,
        remaining_work: RemainingWork::NotApplicable,
        drops: vec![drops_from],
        drops_to_verify: vec![],
        notes: None,
    }
}

pub fn parse_drop(
    card: &str,
    drops_from: DropsFrom,
    poe_data: &PoeData,
) -> Result<Vec<divcord::Source>, divcord::parse::UnknownDropsFrom> {
    let clone = drops_from.clone();
    let dumb = create_dumb(card, drops_from);
    parse_one_drops_from(&clone, &dumb, poe_data)
}

#[tokio::test]
#[cfg(feature = "fetch")]
async fn main() {
    use divcord::spreadsheet::rich::{FontStyles, HexColor};
    let poe_data = PoeData::load().await.unwrap();

    let sources = parse_drop(
        "The Endurance",
        DropsFrom {
            name: "The Mines Level 1/2".to_owned(),
            styles: FontStyles {
                color: HexColor::White,
                italic: true,
                strikethrough: false,
            },
        },
        &poe_data,
    )
    .unwrap();
    assert!(!sources.is_empty());

    let sources = parse_drop(
        "The Endurance",
        DropsFrom {
            name: "The Mines Level 1/2, The Crystal Veins".to_owned(),
            styles: FontStyles {
                color: HexColor::White,
                italic: true,
                strikethrough: false,
            },
        },
        &poe_data,
    )
    .unwrap();
    assert!(sources.is_empty());
}
