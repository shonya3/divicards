use std::{collections::HashMap, fs::File};

// use csv::Reader;
use tracing::instrument;

use crate::error::Error;

use crate::types::record::{self, Record};

#[instrument]
pub fn read(path: &str, minimum_card_price: Option<f32>) -> Result<Vec<Record>, Error> {
    let mut reader: csv::Reader<File> = match csv::Reader::from_path(path) {
        Ok(r) => r,
        Err(err) => {
            tracing::event!(tracing::Level::ERROR, "{:?}", err);
            return Err(Error::CSVError(err));
        }
    };

    let price = minimum_card_price;

    let mut records: Vec<Record> = vec![];
    for record in reader.deserialize::<Record>() {
        let record = match record {
            Ok(r) => r,
            Err(err) => return Err(Error::CSVError(err)),
        };

        if record.calculated.unwrap_or_default() >= price.unwrap_or_default() {
            records.push(record);
        }
    }

    Ok(records)
}

pub fn write(records: &Vec<Record>, path: &str) -> Result<(), Error> {
    let mut writer = match csv::Writer::from_path(path) {
        Ok(writer) => writer,
        Err(err) => {
            tracing::event!(tracing::Level::ERROR, "{:?}", err);
            return Err(Error::CSVError(err));
        }
    };

    for record in records {
        match writer.serialize(record) {
            Ok(r) => r,
            Err(err) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", err);
                return Err(Error::CSVError(err));
            }
        }
    }

    Ok(())
}

// / Merges correct csv divination card files into one.
// / # Example usage:
// /
// /  ```rust
// /   merge_n_files(
// /     starter_map,
// /     &[
// /         "ready_files/5k.csv",
// /         "ready_files/6066.csv",
// /         "ready_files/40k.csv",
// /     ],
// /     "ready_files/output.csv",
// / )?;
// / ```
fn merge_n_files(
    starter_map: HashMap<&'static str, Record>,
    file_paths: &[&str],
    output_path: &str,
) -> Result<(), Error> {
    let mut vectors: Vec<Vec<Record>> = vec![];
    let mut maps: Vec<HashMap<&str, Record>> = vec![];
    for path in file_paths {
        let v: Vec<Record> = read(path, None)?;
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

    write(&new_vec, output_path)?;

    Ok(())
}

pub fn write_empty_table(output_path: &str) -> Result<(), Error> {
    let mut records: Vec<Record> = vec![];
    for name in record::NAMES {
        let mut new_record = Record::default();
        new_record.name = name.to_string();

        records.push(new_record);
    }

    write(&records, output_path)?;

    Ok(())
}

pub fn read_polish_write(
    input_path: &str,
    output_path: &str,
    starter_map: HashMap<&'static str, Record>,
) -> Result<(), Error> {
    let rec = read(input_path, None)?;
    let rec = record::polish_records(rec, starter_map)?;
    write(&rec, output_path)?;
    Ok(())
}
