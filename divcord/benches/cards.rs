use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use divcord::{PoeData, Source, Spreadsheet};

pub fn criterion_benchmark(c: &mut Criterion) {
    let spreadsheet: Spreadsheet =
        serde_json::from_str(&std::fs::read_to_string("benches/spreadsheet.json").unwrap())
            .unwrap();
    let poe_data: PoeData =
        serde_json::from_str(&std::fs::read_to_string("benches/poeData.json").unwrap()).unwrap();

    let records = divcord::records(&spreadsheet, &poe_data).unwrap();

    let all_source_types = Source::types();

    c.bench_function("cards", |b| {
        b.iter(|| {
            divcord::cards::cards_by_source_types(
                black_box(&all_source_types),
                black_box(&records),
                black_box(&poe_data),
            )
        })
    });
}

fn configured_criterion() -> Criterion {
    Criterion::default()
}

criterion_group!(
    name = benches;
    config = configured_criterion();
    targets = criterion_benchmark
);
criterion_main!(benches);
