use std::fs::read_to_string;

use divi::{
    prices::Prices,
    sample::{Column, DivinationCardsSample, SampleData},
};

fn main() -> Result<(), divi::error::Error> {
    let csv = read_to_string("example-2.csv").unwrap();
    let sample =
        DivinationCardsSample::create(SampleData::Csv(String::from(csv)), Some(Prices::default()))?;

    let values = sample.into_values(&[Column::Name, Column::Amount]);

    let json = serde_json::to_string(&values).unwrap();
    std::fs::write("values.json", json).unwrap();

    let body = serde_json::to_string(&serde_json::json!({
       "requests":[
          {
             "addSheet":{
                "properties":{
                   "sheetId":123456
                }
             }

          }
       ]
    }))
    .unwrap();

    dbg!(body);

    Ok(())
}
