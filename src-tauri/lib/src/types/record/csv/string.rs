use std::collections::HashMap;
use std::vec;

use csv::Reader;
use tracing::instrument;
// use tracing::instrument;

use crate::error::Error;

use crate::types::record::{self, Record};

pub fn read(content_string: &str, minimum_card_price: Option<f32>) -> Result<Vec<Record>, Error> {
    let mut reader = Reader::from_reader(content_string.as_bytes());

    let mut records: Vec<Record> = vec![];
    for record in reader.deserialize::<Record>() {
        if let Ok(record) = record {
            if record.calculated.unwrap_or_default() >= minimum_card_price.unwrap_or_default() {
                records.push(record);
            }
        }
    }

    Ok(records)
}

pub fn write(records: &Vec<Record>) -> Result<String, Error> {
    let mut writer = csv::Writer::from_writer(vec![]);
    for record in records {
        writer.serialize(record)?;
    }
    let content_string = String::from_utf8(writer.into_inner().expect("Error with csv serialize"))?;
    Ok(content_string)
}

#[instrument]
pub fn read_polish_write(
    content_string: &str,
    map: HashMap<&'static str, Record>,
    minimum_card_price: Option<f32>,
) -> Result<String, Error> {
    let records = read(content_string, minimum_card_price)?;
    let records = record::polish_records(records, map)?;
    let output_content_string = write(&records)?;
    Ok(output_content_string)
}

pub fn total_chaos(content_string: &str, minimum_card_price: Option<f32>) -> Result<f32, Error> {
    let records = read(content_string, minimum_card_price)?;
    Ok(record::total_price_chaos(&records))
}

pub fn merge(
    starter_map: HashMap<&'static str, Record>,
    csv_file_strings: &[&str],
) -> Result<String, Error> {
    let mut vectors: Vec<Vec<Record>> = vec![];
    let mut maps: Vec<HashMap<&str, Record>> = vec![];

    for content_string in csv_file_strings {
        let v = read(content_string, None)?;
        vectors.push(v);
    }

    for vector in vectors {
        let map: HashMap<&str, Record> = record::vec_to_map(vector, starter_map.clone())?;
        maps.push(map);
    }

    let mut new_vec: Vec<Record> = vec![];
    for (name, record) in starter_map {
        let mut same_name_records: Vec<&Record> = vec![];
        for map in &maps {
            let record: &Record = map.get(name).unwrap();
            same_name_records.push(record);
        }

        let mut new_record = Record::default();
        new_record.name = name.to_string();

        let calculated = match record.calculated {
            Some(price) => price,
            None => match name {
                _ if record::is_legacy_card(name) => 0.0,
                _ => return Err(Error::NoPriceError(name.to_string())),
            },
        };

        new_record.calculated = Some(calculated);

        new_record.stack_size = same_name_records.iter().map(|r| r.stack_size).sum();
        new_record.total = Some(new_record.stack_size as f32 * calculated);

        new_vec.push(new_record);
    }

    Ok(write(&new_vec)?)
}

mod test {
    #![allow(unused_variables)]

    use std::collections::HashMap;

    use crate::types::record::Record;

    fn merge() {
        let doctor_price = 4000.0;
        let nurse_price = 400.0;
        let dragonheart_price = 50.0;

        let mut map = HashMap::<&'static str, Record>::new();
        map.insert(
            "The Doctor",
            Record {
                name: "The Doctor".into(),
                calculated: Some(doctor_price),
                ..Default::default()
            },
        );

        map.insert(
            "The Nurse",
            Record {
                name: "The Nurse".into(),
                calculated: Some(nurse_price),
                ..Default::default()
            },
        );

        map.insert(
            "The Dragon's Heart",
            Record {
                name: "The Dragon's Heart".into(),
                calculated: Some(dragonheart_price),
                ..Default::default()
            },
        );

        let doctor = Record {
            stack_size: 1,
            name: "The Doctor".into(),
            calculated: Some(doctor_price),
            total: Some(doctor_price),
        };

        let nurse = Record {
            stack_size: 2,
            name: "The Nurse".into(),
            calculated: Some(nurse_price),
            total: Some(2 as f32 * nurse_price),
        };

        let vec1: Vec<Record> = vec![doctor.clone(), nurse.clone()];

        let doctor2 = Record {
            stack_size: 2,
            name: "The Doctor".into(),
            calculated: Some(doctor_price),
            total: Some(2 as f32 * doctor_price),
        };

        let dragonsheart = Record {
            stack_size: 1,
            name: "The Dragon's Heart".into(),
            calculated: Some(dragonheart_price),
            total: Some(dragonheart_price),
        };

        let vec2: Vec<Record> = vec![doctor2, dragonsheart];

        todo!();
    }
}

// pub fn read_csv_from_string(data: &str) -> Result<String, Error> {
//     let mut reader = Reader::from_reader(data.as_bytes());

//     let mut writer = csv::Writer::from_writer(vec![]);
//     let mut records: Vec<Record> = vec![];
//     for record in reader.deserialize::<Record>() {
//         let record = record?;
//         writer.serialize(record.clone())?;
//         records.push(record);
//     }

//     // let serialized = serde_json::to_string(&records)?;

//     let data = String::from_utf8(writer.into_inner().unwrap())?;

//     Ok(data)
// }
