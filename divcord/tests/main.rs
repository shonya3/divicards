use divcord::{
    parse::parse_one_drops_from,
    spreadsheet::{
        record::{Confidence, Dumb, GreyNote, RemainingWork},
        rich::DropsFrom,
    },
    PoeData,
};

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
) -> Result<Vec<divcord::Source>, divcord::parse::ParseSourceError> {
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
